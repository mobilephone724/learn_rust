mod bstree;
mod huffman_compress;
mod option_test;
use bitvec::prelude::*;
// use crate::huffman_compress::HuffmanTreeNode;
// use std::collections::BinaryHeap;
// use bstree::*;

fn main() {
    println!("Hello, world!");

    let (dic, frequency) = huffman_compress::generate_haffman_dic_from_file("data.bin");

    for (key, value) in &dic.compressed_dict {
        println!("key: {}, value: {}", key, value.val);
    }

    match huffman_compress::generate_new_file(
        "data.bin",
        "data.bin.compressed",
        &dic.compressed_dict,
        &frequency
    ) {
        Ok(()) => println!("compress successfull!"),
        Err(e) => eprintln!("Failed to write: {}", e),
    }

    let _ = huffman_compress::read_compressed_file("data.bin.compressed");
}
