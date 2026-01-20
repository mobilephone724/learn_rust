use bitvec::prelude::*;
use bitvec::slice::Iter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};

/*
 * High Level Layout
 *
 *      (1): TreeNode And Tree
 *      (2): Compressed Code
 *      (3): (1) + (2) Bidirection Map
 *      (4): Public API: Compress and Decompress
 *
 */

/// Custom error type for Huffman compression operations.
#[derive(Debug)]
pub enum HuffmanError {
    /// Invalid input data.
    InvalidInput(String),
    /// Error building the Huffman tree.
    TreeBuildError(String),
    /// Error during compression.
    CompressionError(String),
    /// Error during decompression.
    DecompressionError(String),
}

impl fmt::Display for HuffmanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HuffmanError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            HuffmanError::TreeBuildError(msg) => write!(f, "Tree build error: {}", msg),
            HuffmanError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            HuffmanError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
        }
    }
}

impl Error for HuffmanError {}

/* (1) Tree Node and Tree */
/// A node in the Huffman tree.
#[derive(Eq, PartialEq)]
struct HuffmanTreeNode {
    /// The weight (frequency) of this node.
    pub weight: u64,
    /// The value(s) this node represents. For leaves, a single byte; for internal, combined.
    pub val: Vec<u8>,
    /// Left child.
    pub left: Option<Box<HuffmanTreeNode>>,
    /// Right child.
    pub right: Option<Box<HuffmanTreeNode>>,
}

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap behavior in BinaryHeap
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MAX_CHAR: usize = 255;
type HuffmanFrequency = [u64; MAX_CHAR + 1];

/// The Huffman tree, represented as a binary heap with the root at the top.
struct HuffmanTree {
    pub tree: BinaryHeap<HuffmanTreeNode>,
}

impl HuffmanTree {
    fn generate_huffman_tree_nodes_with_frequency(
        frequency: &HuffmanFrequency,
    ) -> Result<Vec<HuffmanTreeNode>, HuffmanError> {
        let mut res: Vec<HuffmanTreeNode> = Vec::new();

        for i in 0..frequency.len() {
            if frequency[i] == 0 {
                continue;
            }

            let idx: u8 = i.try_into().map_err(|_| HuffmanError::InvalidInput("Failed to convert index to u8".to_string()))?;

            res.push(HuffmanTreeNode {
                weight: frequency[i],
                val: vec![idx],
                left: None,
                right: None,
            });
        }

        Ok(res)
    }

    pub fn from_nodes(huffman_tree_nodes: Vec<HuffmanTreeNode>) -> Result<Self, HuffmanError> {
        let mut tree = BinaryHeap::new();

        for node in huffman_tree_nodes {
            tree.push(node);
        }

        while tree.len() >= 2 {
            let Some(n1) = tree.pop() else {
                return Err(HuffmanError::TreeBuildError("fail to get n1 while generating huffman tree".to_string()));
            };

            let Some(n2) = tree.pop() else {
                return Err(HuffmanError::TreeBuildError("fail to get n2 while generating huffman tree".to_string()));
            };

            let mut new_val = n1.val.clone();
            new_val.extend_from_slice(&n2.val);

            tree.push(HuffmanTreeNode {
                weight: n1.weight + n2.weight,
                val: new_val,
                left: Some(Box::new(n1)),
                right: Some(Box::new(n2)),
            });
        }

        Ok(HuffmanTree { tree })
    }

    pub fn from_frequency(frequency: &HuffmanFrequency) -> Result<Self, HuffmanError> {
        let nodes = HuffmanTree::generate_huffman_tree_nodes_with_frequency(frequency)?;

        HuffmanTree::from_nodes(nodes)
    }
}

/* (2) Compressed Code */
/// Represents a Huffman compressed code as a sequence of bits.
#[derive(Clone, PartialEq, Eq, Hash)]
struct HuffmanCompressedCode {
    pub val: BitVec<u8, Msb0>,
}

impl HuffmanCompressedCode {
    pub fn new() -> Self {
        HuffmanCompressedCode { val: BitVec::new() }
    }

    pub fn is_empty(&self) -> bool {
        return self.val.is_empty();
    }

    pub fn get_left(&self) -> HuffmanCompressedCode {
        let mut left_code = self.clone();
        left_code.val.push(false);

        left_code
    }

    pub fn get_right(&self) -> HuffmanCompressedCode {
        let mut left_code = self.clone();
        left_code.val.push(true);

        return left_code;
    }

    pub fn push(&mut self, value: bool) {
        self.val.push(value);
    }

    pub fn iter(&self) -> Iter<'_, u8, Msb0> {
        self.val.iter()
    }

    pub fn clear(&mut self) {
        self.val.clear();
    }
}

type CompressedContent = BitVec<u8, Msb0>;

/* (3) bidirection map */
type HuffmanCompressedDict = HashMap<u8, HuffmanCompressedCode>;
type HuffmanOriginDict = HashMap<HuffmanCompressedCode, u8>;

/// Bidirectional mapping between original bytes and their Huffman codes.
struct HuffmanMap {
    pub compressed_dict: HuffmanCompressedDict,
    pub original_dict: HuffmanOriginDict,
}

impl HuffmanMap {
    pub fn from_huffman_tree(huffman_tree: &HuffmanTree) -> Result<Self, HuffmanError> {
        let root = huffman_tree.tree.peek().ok_or(HuffmanError::TreeBuildError("no root in huffman tree".to_string()))?;

        HuffmanMap::generate_huffman_dict_internal(root, HuffmanCompressedCode::new())
    }

    fn new() -> Self {
        HuffmanMap {
            compressed_dict: HuffmanCompressedDict::new(),
            original_dict: HuffmanOriginDict::new(),
        }
    }

    fn extend(&mut self, another_map: HuffmanMap) {
        self.compressed_dict.extend(another_map.compressed_dict);
        self.original_dict.extend(another_map.original_dict);
    }

    fn insert(&mut self, origin: u8, compressed: &HuffmanCompressedCode) {
        self.compressed_dict.insert(origin, compressed.clone());
        self.original_dict.insert(compressed.clone(), origin);
    }

    fn generate_huffman_dict_internal(
        node: &HuffmanTreeNode,
        mut current_compress_code: HuffmanCompressedCode,
    ) -> Result<HuffmanMap, HuffmanError> {
        /* Have reached the bottom? */
        if node.val.len() == 1 {
            let mut res = HuffmanMap::new();

            if current_compress_code.is_empty() {
                current_compress_code = current_compress_code.get_left();
            }

            res.insert(node.val[0], &current_compress_code);

            return Ok(res);
        }

        /* Recursive left and right node */
        let Some(left) = node.left.as_ref() else {
            return Err(HuffmanError::TreeBuildError("cannot get the left child while traverse huffman tree".to_string()));
        };

        let Some(right) = node.right.as_ref() else {
            return Err(HuffmanError::TreeBuildError("cannot get the right child while traverse huffman tree".to_string()));
        };

        let mut left_map =
            HuffmanMap::generate_huffman_dict_internal(left, current_compress_code.get_left())?;
        let right_map =
            HuffmanMap::generate_huffman_dict_internal(right, current_compress_code.get_right())?;

        left_map.extend(right_map);

        return Ok(left_map);
    }
}

/* public API */
pub fn compress(content: &Vec<u8>) -> Result<Vec<u8>, HuffmanError> {
    let mut compressed_result: Vec<u8> = Vec::new();

    /* get and write frequency */
    let mut frequency: HuffmanFrequency = [0u64; 256];

    for ch in content {
        frequency[*ch as usize] += 1;
    }

    for f in frequency {
        compressed_result.extend_from_slice(&f.to_le_bytes());
    }

    /* leave a i64 size space for length */
    let length_pos = compressed_result.len();
    compressed_result.extend_from_slice(&[0u8; 8]);

    /* Can NOT build the huffman tree from empty input */
    if content.len() == 0 {
        return Ok(compressed_result);
    }

    /* generate dictionary */
    let tree = HuffmanTree::from_frequency(&frequency)?;
    let dic = HuffmanMap::from_huffman_tree(&tree)?;

    /* and append compressed data */
    let mut compressed_content_length_in_bit: i64 = 0;
    let mut byte: u8 = 0;
    let mut filled: u8 = 0;
    for ch in content {
        let Some(compressed_code) = dic.compressed_dict.get(&ch) else {
            return Err(HuffmanError::CompressionError("character not in dictionary".to_string()));
        };

        for bit in compressed_code.val.iter() {
            if *bit {
                // Msb0: first bit goes to the highest position in the byte
                byte |= 1 << (7 - filled);
            }
            filled += 1;

            if filled == 8 {
                compressed_result.extend_from_slice(&[byte]);
                compressed_content_length_in_bit += 8;
                byte = 0;
                filled = 0;
            }
        }
    }

    if filled != 0 {
        // Flush the remaining bits in the last byte
        compressed_result.extend_from_slice(&[byte]);
        compressed_content_length_in_bit += filled as i64;
    }

    compressed_result[length_pos..length_pos + 8]
        .copy_from_slice(&compressed_content_length_in_bit.to_le_bytes());

    Ok(compressed_result)
}

pub fn decompress(content: &Vec<u8>) -> Result<Vec<u8>, HuffmanError> {
    let header_len = 8 * (256 + 1);
    const FREQUENCY_LEN: usize = 256;

    // get length first
    if content.len() < header_len {
        return Err(HuffmanError::DecompressionError("content is too small to contain header".to_string()));
    }

    let mut frequency: HuffmanFrequency = [0u64; FREQUENCY_LEN];
    for (i, chunk) in content.chunks_exact(8).enumerate() {
        if i >= FREQUENCY_LEN {
            break;
        }
        // Convert the 8-byte slice into a fixed-size array [u8; 8]
        // then convert that to a u64
        frequency[i] = u64::from_le_bytes(chunk.try_into().unwrap());
    }

    let bit_len = u64::from_le_bytes(content[(256 * 8)..(256 * 8 + 8)].try_into().unwrap());

    if bit_len == 0 {
        return Ok(Vec::new());
    }

    let tree = HuffmanTree::from_frequency(&frequency)?;
    let dic = HuffmanMap::from_huffman_tree(&tree)?;
    let dic = dic.original_dict;

    let mut bit_read = 0;
    let mut compressed_code: HuffmanCompressedCode = HuffmanCompressedCode::new();

    let mut decompressed_content: Vec<u8> = Vec::new();
    for (index, &ch) in content.iter().enumerate() {
        if index < header_len {
            continue;
        }

        let remain = std::cmp::min(bit_len - bit_read, 8);

        if remain <= 0 {
            break;
        }

        let bits: BitVec<u8, Msb0> = BitVec::from_element(ch);
        for (i, b) in bits.iter().enumerate() {
            /* is this the last u8 */
            if (i as u64) >= remain {
                break;
            }

            compressed_code.push(*b);

            if let Some(decompressed_value) = dic.get(&compressed_code) {
                decompressed_content.push(*decompressed_value);
                compressed_code.clear();
            }
        }

        bit_read += remain;
    }

    if !compressed_code.is_empty() {
        return Err(HuffmanError::DecompressionError("some content remains".to_string()));
    }

    Ok(decompressed_content)
}

#[derive(Debug)]
pub enum FileCompressError {
    Io(std::io::Error),
    Huffman(HuffmanError),
}

impl fmt::Display for FileCompressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileCompressError::Io(ref err) => write!(f, "IO error: {}", err),
            FileCompressError::Huffman(ref err) => write!(f, "Huffman error: {}", err),
        }
    }
}

impl Error for FileCompressError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            FileCompressError::Io(ref err) => Some(err),
            FileCompressError::Huffman(ref err) => Some(err),
        }
    }
}

impl From<std::io::Error> for FileCompressError {
    fn from(err: std::io::Error) -> Self {
        FileCompressError::Io(err)
    }
}

impl From<HuffmanError> for FileCompressError {
    fn from(err: HuffmanError) -> Self {
        FileCompressError::Huffman(err)
    }
}

pub fn file_compress(
    file_in_path: &str,
    file_out_path: &str,
) -> Result<(), FileCompressError> {
    let mut file_in = OpenOptions::new().read(true).open(file_in_path)?;

    let mut contents: Vec<u8> = Vec::new();
    file_in.read_to_end(&mut contents)?;

    let compressed = compress(&contents)?;

    let mut file_out = File::create(file_out_path)?;
    file_out.write_all(&compressed)?;

    Ok(())
}

pub fn file_decompress(
    file_in_path: &str,
    file_out_path: &str,
) -> Result<(), FileCompressError> {
    let mut file_in = OpenOptions::new().read(true).open(file_in_path)?;

    let mut contents: Vec<u8> = Vec::new();
    file_in.read_to_end(&mut contents)?;

    let decompressed = decompress(&contents)?;

    let mut file_out = File::create(file_out_path)?;
    file_out.write_all(&decompressed)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_simple() {
        let input = b"hello world";
        let compressed = compress(&input.to_vec()).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_compress_decompress_empty() {
        let input = b"";
        let compressed = compress(&input.to_vec()).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_compress_decompress_single_char() {
        let input = b"a";
        let compressed = compress(&input.to_vec()).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_compress_decompress_repeated() {
        let input = b"aaaaa";
        let compressed = compress(&input.to_vec()).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_compress_decompress_all_different() {
        let input = b"abcdefghijklmnopqrstuvwxyz";
        let compressed = compress(&input.to_vec()).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }

    #[test]
    fn test_compress_decompress_binary() {
        let input = vec![0, 1, 2, 255, 128];
        let compressed = compress(&input).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(input, decompressed);
    }
}


