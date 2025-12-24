mod bstree;
mod huffman_compress;
mod option_test;
use bitvec::prelude::*;
// use crate::huffman_compress::HuffmanTreeNode;
// use std::collections::BinaryHeap;
// use bstree::*;

fn main() {
    println!("Hello, world!");

    // let mut heap = huffman_compress::generate_haffman_tree(huffman_compress::generate_haffman_tree_nodes());

    // let haffman_map = huffman_compress::generate_haffman_dic(&mut heap);

    // for (key, value) in &haffman_map {
    //     println!("key: {}, value: {}", key, value);
    // }


    let dic = huffman_compress::generate_haffman_dic_from_file("data.bin");

    for (key, value) in &dic {
        println!("key: {}, value: {}", key, value);
    }

}
