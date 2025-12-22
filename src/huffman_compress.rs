use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::format;
use std::string;

#[derive(Eq, PartialEq)]
pub struct HuffmanTreeNode {
    pub weight: i32,
    pub substr: String,
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

// pub fn build_sample_vector() -> Vec<&'static str> {
//     let mut values = Vec::new();
//     values.push("alpha");
//     values.push("beta");
//     values.push("gamma");
//     values
// }

pub type HuffmanTree = BinaryHeap<HuffmanTreeNode>;

// Using a tuple (most common for pairs)

// Or using a struct for more clarity
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

impl std::fmt::Debug for HaffmanCompressedCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HaffmanCompressedCode {{ val: {}, mask: {} }}", self.val, self.mask)
    }
}

// For your specific use case with Huffman encoding:


pub fn generate_haffman_tree_nodes() -> Vec<HuffmanTreeNode> {
    let mut res: Vec<HuffmanTreeNode> = Vec::new();

    res.push(HuffmanTreeNode {
        weight: 10,
        substr: "A".to_string(),
        left: None,
        right: None,
    });

    res.push(HuffmanTreeNode {
        weight: 20,
        substr: "B".to_string(),
        left: None,
        right: None,
    });

    res.push(HuffmanTreeNode {
        weight: 25,
        substr: "C".to_string(),
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

        res.push(HuffmanTreeNode {
            weight: n1.weight + n2.weight,
            substr: format!("{}{}", n1.substr, n2.substr),
            left: Some(Box::new(n1)),
            right: Some(Box::new(n2)),
        });
    }

    res
}


pub fn generate_haffman_dic(node_tree: &mut BinaryHeap<HuffmanTreeNode>) -> HashMap<char, String> {
    let Some(root) = node_tree.pop() else {
        panic!("no in haffman tree");
    };

    generate_haffman_dic_internal(&Box::new(root), &"0".to_string())
}

fn generate_haffman_dic_internal(node: &Box<HuffmanTreeNode>, prefix: &String) -> HashMap<char, String> {
    println!("In haffman dic recursion: cur node is {} with prefix {}", node.substr, prefix);

    if node.substr.len() == 1 {
        let mut res: HashMap<char, String> = HashMap::new();
        let Some(ch) = node.substr.chars().nth(0) else {
            panic!("cannot get the first char in string {}", node.substr);
        };

        res.insert(ch, prefix.clone());

        println!("length is 1, return single value {}-{}", ch, prefix);
        return res;
    }

    let Some(left) = node.left.as_ref() else {
        panic!("cannot get the left child in node {} with weight {}", node.substr, node.weight);
    };

    let Some(right) = node.right.as_ref() else {
        panic!("cannot get the right child in node {} with weight {}", node.substr, node.weight);
    };

    let mut left_map = generate_haffman_dic_internal(left, &format!("{}1", prefix));
    let right_map = generate_haffman_dic_internal(right, &format!("{}0", prefix));

    left_map.extend(right_map);

    left_map
}