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

#[derive(Copy, Clone)]
pub struct HaffmanCompressedCode {
    pub val: i32,
    pub mask: i32
}

impl std::fmt::Display for HaffmanCompressedCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = Default::default();

        for i in 0..32 {
            let single_bit_mask: i32 = 1 << i;
            if self.mask & single_bit_mask == 0 {
                break;

                
            } else {
                if self.val & single_bit_mask == 0 {
                    out.push('0');
                } else {
                    out.push('1');
                }
            }
        }
        out = out.chars().rev().collect();

        write!(f, "HaffmanCompressedCode {{ val: {}, mask: {}, output {} }}", self.val, self.mask, out)
    }
}

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

    generate_haffman_dic_internal(&Box::new(root), HaffmanCompressedCode {val: 0, mask: 0})
}

fn generate_haffman_dic_internal(node: &Box<HuffmanTreeNode>, mut current_compress_code: HaffmanCompressedCode) -> HaffmanCompressedDict {
    println!("In haffman dic recursion: cur node is {:?} with prefix {}", node.val, current_compress_code);

    if node.val.len() == 1 {
        let mut res: HaffmanCompressedDict = HashMap::new();

        if current_compress_code.mask == 0 {
            current_compress_code.mask = 1;
        }

        res.insert(node.val[0], current_compress_code);

        println!("length is 1, return single value {}-{}", node.val[0], current_compress_code);
        return res;
    }

    let Some(left) = node.left.as_ref() else {
        panic!("cannot get the left child in node {:?} with weight {}", node.val, node.weight);
    };

    let Some(right) = node.right.as_ref() else {
        panic!("cannot get the right child in node {:?} with weight {}", node.val, node.weight);
    };

    let mut left_map = generate_haffman_dic_internal(left, HaffmanCompressedCode {val: (current_compress_code.val << 1) + 1, mask: (current_compress_code.mask << 1) + 1});
    let right_map = generate_haffman_dic_internal(right, HaffmanCompressedCode {val: (current_compress_code.val << 1) + 0, mask: (current_compress_code.mask << 1) + 1});

    left_map.extend(right_map);

    return left_map;
}