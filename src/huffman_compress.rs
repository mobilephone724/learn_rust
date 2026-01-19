use bitvec::prelude::*;
use bitvec::slice::Iter;
use std::array;
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

/* (1) Tree Node and Tree */
#[derive(Eq, PartialEq)]
struct HuffmanTreeNode {
    pub weight: u64,
    pub val: Vec<u8>,
    pub left: Option<Box<HuffmanTreeNode>>,
    pub right: Option<Box<HuffmanTreeNode>>,
}

impl Ord for HuffmanTreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // reserve
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for HuffmanTreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MAX_CHAR: usize = 255;
type HaffmanFrequency = [u64; MAX_CHAR + 1];

struct HuffmanTree {
    pub tree: BinaryHeap<HuffmanTreeNode>,
}

impl HuffmanTree {
    fn generate_haffman_tree_nodes_with_frequency(
        frequency: &HaffmanFrequency,
    ) -> Result<Vec<HuffmanTreeNode>, String> {
        let mut res: Vec<HuffmanTreeNode> = Vec::new();

        for i in 0..frequency.len() {
            if frequency[i] == 0 {
                continue;
            }

            let idx: u8 = i.try_into().map_err(|_| "Failed to convert index to u8")?;

            res.push(HuffmanTreeNode {
                weight: frequency[i],
                val: vec![idx],
                left: None,
                right: None,
            });
        }

        Ok(res)
    }

    pub fn from_nodes(huffman_tree_nodes: Vec<HuffmanTreeNode>) -> Result<Self, String> {
        let mut tree = BinaryHeap::new();

        for node in huffman_tree_nodes {
            tree.push(node);
        }

        while tree.len() >= 2 {
            let Some(n1) = tree.pop() else {
                return Err("fail to get n1 while generating huffman tree".to_string());
            };

            let Some(n2) = tree.pop() else {
                return Err("fail to get n2 while generating huffman tree".to_string());
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

    pub fn from_frequency(frequency: &HaffmanFrequency) -> Result<Self, String> {
        let nodes = HuffmanTree::generate_haffman_tree_nodes_with_frequency(frequency)?;

        HuffmanTree::from_nodes(nodes)
    }
}

/* (2) Compressed Code */
#[derive(Clone, PartialEq, Eq, Hash)]
struct HaffmanCompressedCode {
    pub val: BitVec<u8, Msb0>,
}

impl HaffmanCompressedCode {
    pub fn new() -> Self {
        HaffmanCompressedCode { val: BitVec::new() }
    }

    pub fn is_empty(&self) -> bool {
        return self.val.is_empty();
    }

    pub fn get_left(&self) -> HaffmanCompressedCode {
        let mut left_code = self.clone();
        left_code.val.push(false);

        left_code
    }

    pub fn get_right(&self) -> HaffmanCompressedCode {
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
type HaffmanCompressedDict = HashMap<u8, HaffmanCompressedCode>;
type HaffmanOriginDict = HashMap<HaffmanCompressedCode, u8>;

struct HuffmanMap {
    pub compressed_dict: HaffmanCompressedDict,
    pub original_dict: HaffmanOriginDict,
}

impl HuffmanMap {
    pub fn from_haffman_tree(haffman_tree: &HuffmanTree) -> Result<Self, String> {
        let root = haffman_tree.tree.peek().ok_or("no root in huffman tree")?;

        HuffmanMap::generate_haffman_dic_internal(root, HaffmanCompressedCode::new())
    }

    fn new() -> Self {
        HuffmanMap {
            compressed_dict: HaffmanCompressedDict::new(),
            original_dict: HaffmanOriginDict::new(),
        }
    }

    fn extend(&mut self, another_map: HuffmanMap) {
        self.compressed_dict.extend(another_map.compressed_dict);
        self.original_dict.extend(another_map.original_dict);
    }

    fn insert(&mut self, origin: u8, compressed: &HaffmanCompressedCode) {
        self.compressed_dict.insert(origin, compressed.clone());
        self.original_dict.insert(compressed.clone(), origin);
    }

    fn generate_haffman_dic_internal(
        node: &HuffmanTreeNode,
        mut current_compress_code: HaffmanCompressedCode,
    ) -> Result<HuffmanMap, String> {
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
            return Err("cannot get the left child while traverse haffman tree".to_string());
        };

        let Some(right) = node.right.as_ref() else {
            return Err("cannot get the right child while traverse haffman tree".to_string());
        };

        let mut left_map =
            HuffmanMap::generate_haffman_dic_internal(left, current_compress_code.get_left())?;
        let right_map =
            HuffmanMap::generate_haffman_dic_internal(right, current_compress_code.get_right())?;

        left_map.extend(right_map);

        return Ok(left_map);
    }
}

/* public API */
pub fn compress(content: &Vec<u8>) -> Result<Vec<u8>, String> {
    let mut compressed_result: Vec<u8> = Vec::new();

    /* get and write frequency */
    let mut frequency: HaffmanFrequency = [0u64; 256];

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
    let dic = HuffmanMap::from_haffman_tree(&tree)?;

    /* and append compressed data */
    let mut compressed_content_length_in_bit: i64 = 0;
    let mut byte: u8 = 0;
    let mut filled: u8 = 0;
    for ch in content {
        let Some(compressed_code) = dic.compressed_dict.get(&ch) else {
            return Err("charactor doesn't in dictionary".to_string());
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

pub fn decompress(content: &Vec<u8>) -> Result<Vec<u8>, String> {
    let header_len = 8 * (256 + 1);
    const FREQUENCY_LEN: usize = 256;

    // get length first
    if content.len() < header_len {
        return Err("content is too small to contain header".to_string());
    }

    let mut frequency: HaffmanFrequency = [0u64; FREQUENCY_LEN];
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
    let dic = HuffmanMap::from_haffman_tree(&tree)?;
    let dic = dic.original_dict;

    let mut bit_read = 0;
    let mut compressed_code: HaffmanCompressedCode = HaffmanCompressedCode::new();

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
        return Err("some content remains".to_string());
    }

    Ok(decompressed_content)
}

#[derive(Debug)]
pub enum FileCompressError {
    Io(std::io::Error),
    Internal(String),
}

impl fmt::Display for FileCompressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileCompressError::Io(ref err) => write!(f, "IO error: {}", err),
            FileCompressError::Internal(ref err) => write!(f, "Internal error: {}", err),
        }
    }
}

impl Error for FileCompressError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            FileCompressError::Io(ref err) => Some(err),
            FileCompressError::Internal(_) => None,
        }
    }
}

impl From<std::io::Error> for FileCompressError {
    fn from(err: std::io::Error) -> Self {
        FileCompressError::Io(err)
    }
}

impl From<String> for FileCompressError {
    fn from(err: String) -> Self {
        FileCompressError::Internal(err)
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


