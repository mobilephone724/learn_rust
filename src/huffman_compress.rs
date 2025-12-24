use bitvec::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::format;
use std::string;

#[derive(Eq, PartialEq)]
pub struct HuffmanTreeNode {
    pub weight: i32,
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

pub type HaffmanCompressedCode = BitVec<u8, Msb0>;

pub type HaffmanCompressedDict = HashMap<u8, HaffmanCompressedCode>;

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

pub fn generate_haffman_dic(node_tree: &mut BinaryHeap<HuffmanTreeNode>) -> HaffmanCompressedDict {
    let Some(root) = node_tree.pop() else {
        panic!("no in haffman tree");
    };

    generate_haffman_dic_internal(&Box::new(root), BitVec::new())
}

fn generate_haffman_dic_internal(
    node: &Box<HuffmanTreeNode>,
    mut current_compress_code: HaffmanCompressedCode,
) -> HaffmanCompressedDict {
    println!(
        "In haffman dic recursion: cur node is {:?} with prefix {}",
        node.val, &current_compress_code
    );

    if node.val.len() == 1 {
        let mut res: HaffmanCompressedDict = HashMap::new();

        if current_compress_code.is_empty() {
            current_compress_code.push(false);
        }

        println!(
            "length is 1, return single value {}-{}",
            node.val[0], &current_compress_code
        );

        res.insert(node.val[0], current_compress_code);

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

    let mut left_code = current_compress_code.clone();
    let mut right_code = current_compress_code.clone();

    left_code.push(false);
    right_code.push(true);

    let mut left_map = generate_haffman_dic_internal(left, left_code);
    let right_map = generate_haffman_dic_internal(right, right_code);

    left_map.extend(right_map);

    return left_map;
}
