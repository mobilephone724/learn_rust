mod bstree;
mod huffman_compress;
mod option_test;
// use crate::huffman_compress::HuffmanTreeNode;
// use std::collections::BinaryHeap;
// use bstree::*;

fn main() {
    println!("Hello, world!");

    let mut heap = huffman_compress::generate_haffman_tree(huffman_compress::generate_haffman_tree_nodes());

    // if let Some(n) = heap.pop() {
    //     println!("result of pop is {} with weight {}" , n.substr, n.weight);
    // } else {
    //     println!("heap is empty");
    // }

    // if let Some(n) = heap.pop() {
    //     println!("result of pop is {} with weight {}" , n.substr, n.weight);
    // } else {
    //     println!("heap is empty");
    // }

    // if let Some(n) = heap.pop() {
    //     println!("result of pop is {} with weight {}" , n.substr, n.weight);
    // } else {
    //     println!("heap is empty");
    // }

    let haffman_map = huffman_compress::generate_haffman_dic(&mut heap);

    for (key, value) in &haffman_map {
        println!("key: {}, value: {}", key, value);
    }

    // println!("map is {}", haffman_map);



}
