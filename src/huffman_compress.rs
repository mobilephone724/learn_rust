use bitvec::prelude::*;
use bitvec::slice::Iter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::{Read, Write};

#[derive(Eq, PartialEq)]
pub struct HuffmanTreeNode {
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

pub type HuffmanTree = BinaryHeap<HuffmanTreeNode>;
pub type CompressedContent = BitVec<u8, Msb0>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HaffmanCompressedCode {
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

pub type HaffmanCompressedDict = HashMap<u8, HaffmanCompressedCode>;
pub type HaffmanOriginDict = HashMap<HaffmanCompressedCode, u8>;

pub struct HuffmanMap {
    pub compressed_dict: HaffmanCompressedDict,
    pub original_dict: HaffmanOriginDict,
}

impl HuffmanMap {
    pub fn new() -> Self {
        HuffmanMap {
            compressed_dict: HaffmanCompressedDict::new(),
            original_dict: HaffmanOriginDict::new(),
        }
    }

    pub fn extend(&mut self, another_map: HuffmanMap) {
        self.compressed_dict.extend(another_map.compressed_dict);
        self.original_dict.extend(another_map.original_dict);
    }

    pub fn insert(&mut self, origin: u8, compressed: &HaffmanCompressedCode) {
        self.compressed_dict.insert(origin, compressed.clone());
        self.original_dict.insert(compressed.clone(), origin);
    }
}

pub fn generate_haffman_tree_nodes() -> Vec<HuffmanTreeNode> {
    let mut res: Vec<HuffmanTreeNode> = Vec::new();

    res.push(HuffmanTreeNode {
        weight: 10,
        val: vec![b'A'],
        left: None,
        right: None,
    });

    res.push(HuffmanTreeNode {
        weight: 20,
        val: vec![b'B'],
        left: None,
        right: None,
    });

    res.push(HuffmanTreeNode {
        weight: 25,
        val: vec![b'C'],
        left: None,
        right: None,
    });

    res
}

pub fn generate_haffman_tree(nodes: Vec<HuffmanTreeNode>) -> HuffmanTree {
    let mut res: HuffmanTree = BinaryHeap::new();

    for node in nodes {
        res.push(node)
    }

    while res.len() > 1 {
        let Some(n1) = res.pop() else {
            eprintln!("Error: failed to pop first node");
            break;
        };

        let Some(n2) = res.pop() else {
            eprintln!("Error: failed to pop second node");
            break;
        };

        let mut new_val = n1.val.clone();
        new_val.extend_from_slice(&n2.val);

        res.push(HuffmanTreeNode {
            weight: n1.weight + n2.weight,
            val: new_val,
            left: Some(Box::new(n1)),
            right: Some(Box::new(n2)),
        });
    }

    res
}

pub fn generate_haffman_dic(node_tree: &mut BinaryHeap<HuffmanTreeNode>) -> HuffmanMap {
    let Some(root) = node_tree.pop() else {
        panic!("no in haffman tree");
    };

    generate_haffman_dic_internal(&Box::new(root), HaffmanCompressedCode::new())
}

fn generate_haffman_dic_internal(
    node: &Box<HuffmanTreeNode>,
    mut current_compress_code: HaffmanCompressedCode,
) -> HuffmanMap {
    if node.val.len() == 1 {
        let mut res = HuffmanMap::new();

        if current_compress_code.is_empty() {
            current_compress_code = current_compress_code.get_left();
        }

        res.insert(node.val[0], &current_compress_code);

        return res;
    }

    let Some(left) = node.left.as_ref() else {
        panic!(
            "cannot get the left child in node {:?} with weight {}",
            node.val, node.weight
        );
    };

    let Some(right) = node.right.as_ref() else {
        panic!(
            "cannot get the right child in node {:?} with weight {}",
            node.val, node.weight
        );
    };

    let left_code = current_compress_code.get_left();
    let right_code = current_compress_code.get_right();

    let mut left_map = generate_haffman_dic_internal(left, left_code);
    let right_map = generate_haffman_dic_internal(right, right_code);

    left_map.extend(right_map);

    return left_map;
}

pub fn generate_haffman_tree_nodes_with_frequency(frequency: &Vec<u64>) -> Vec<HuffmanTreeNode> {
    let mut res: Vec<HuffmanTreeNode> = Vec::new();

    for i in 0..frequency.len() {
        if frequency[i] == 0 {
            continue;
        }

        res.push(HuffmanTreeNode {
            weight: frequency[i],
            val: vec![
                i.try_into()
                    .expect("size of frequency must not be greater than 255"),
            ],
            left: None,
            right: None,
        });
    }

    res
}

pub fn generate_haffman_dic_from_file(file_path: &str) -> (HuffmanMap, Vec<u64>) {
    let mut file = File::open(file_path).expect("failed to open data.bin");
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let mut frequency: Vec<u64> = vec![0u64; 256];
    for ch in contents {
        frequency[ch as usize] += 1;
    }

    let tree_nodes = generate_haffman_tree_nodes_with_frequency(&frequency);
    let mut tree = generate_haffman_tree(tree_nodes);
    let dic = generate_haffman_dic(&mut tree);
    // let dic = generate_haffman_dic(&mut generate_haffman_tree(generate_haffman_tree_nodes_with_frequency(&frequency)));

    return (dic, frequency);
}

pub fn generate_new_content_from_file(
    file_path: &str,
    dic: &HaffmanCompressedDict,
) -> CompressedContent {
    let mut file = File::open(file_path).expect("failed to open data.bin");
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let mut res: CompressedContent = BitVec::new();

    println!("length of original content is: {}", contents.len());

    for ch in contents {
        let compressed_code = dic.get(&ch).expect("unrecognized charactor");

        // res.extend_from_bitslice(compressed_code.as_bitslice());
        res.extend(compressed_code.iter());
    }

    return res;
}

pub fn generate_new_file(
    origin_file_path: &str,
    compressed_file_path: &str,
    dic: &HaffmanCompressedDict,
    frequency: &Vec<u64>,
) -> std::io::Result<()> {
    let mut origin_file = OpenOptions::new().read(true).open(origin_file_path)?;
    let mut compressed_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(compressed_file_path)?;

    // (1) write frequency first
    for f in frequency {
        compressed_file.write_all(&f.to_le_bytes())?;
    }

    // read orignal file
    let mut original_contents: Vec<u8> = Vec::new();
    origin_file
        .read_to_end(&mut original_contents)
        .expect("failed to read file contents");
    println!("length of original content is: {}", original_contents.len());

    let mut compressed_data: CompressedContent = BitVec::new();
    for ch in original_contents {
        let compressed_code = dic.get(&ch).expect("unrecognized charactor");
        compressed_data.extend(compressed_code.iter());
    }

    // (2) write length so that we can know how many bits are valid in the last byte
    let bit_len = compressed_data.len() as u64;
    compressed_file.write_all(&bit_len.to_le_bytes())?;

    // (3) write compressed data
    let mut byte: u8 = 0;
    let mut filled: u8 = 0;
    for bit in compressed_data.iter() {
        if *bit {
            // Msb0: first bit goes to the highest position in the byte
            byte |= 1 << (7 - filled);
        }
        filled += 1;

        if filled == 8 {
            compressed_file.write_all(&[byte])?;
            byte = 0;
            filled = 0;
        }
    }

    if filled != 0 {
        // Flush the remaining bits in the last byte
        compressed_file.write_all(&[byte])?;
    }

    // write_bitvec_to_file(compressed_file_path, &compressed_data).expect("can not write new file");

    Ok(())
}

pub fn read_compressed_file(file_path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    // let mut file = File::open(file_path).expect("failed to open data.bin");
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .expect("failed to read file contents");

    let header_len = 8 * (256 + 1);

    // get length first
    if contents.len() < header_len {
        panic!("File too small to contain bit length header");
    }

    println!("content len is {}", contents.len());
    let frequency: Vec<u64> = contents
        .chunks_exact(8) // Take 8 bytes at a time
        .take(256) // Ensure we only take 256 groups
        .map(|chunk| {
            // Convert the slice [u8] into a fixed array [u8; 8] then to u64
            u64::from_le_bytes(chunk.try_into().unwrap())
        })
        .collect(); // Gather into the Vec

    // let bit_len = u64::from_be_bytes(contents[(256 * 8)..(256 * 8 + 8)].try_into().unwrap());
    let bit_len = u64::from_le_bytes(contents[(256 * 8)..(256 * 8 + 8)].try_into().unwrap());

    let tree_nodes = generate_haffman_tree_nodes_with_frequency(&frequency);
    let mut tree = generate_haffman_tree(tree_nodes);
    let dic = generate_haffman_dic(&mut tree);
    let dic = dic.original_dict;

    let mut bit_read = 0;
    let mut compressed_code: HaffmanCompressedCode = HaffmanCompressedCode::new();
    let mut new_contents: Vec<u8> = Vec::new();

    for (index, &ch) in contents.iter().enumerate() {
        if index < header_len {
            continue;
        }

        let remain = std::cmp::min(bit_len - bit_read, 8);

        if remain <= 0 {
            break;
        }

        let bits: BitVec<u8, Msb0> = BitVec::from_element(ch);
        for (i, b) in bits.iter().enumerate() {
            if (i as u64) >= remain {
                // must be empty
                // assert_eq!()
                break;
            }

            compressed_code.push(*b);

            if let Some(decompressed_value) = dic.get(&compressed_code) {
                new_contents.push(*decompressed_value);
                compressed_code.clear();
            }
        }

        bit_read += remain;
    }

    let string = String::from_utf8_lossy(&new_contents);
    println!("string is {}", string);

    Ok(())
}
