mod bstree;
mod huffman_compress;
mod option_test;
// use crate::huffman_compress::HuffmanTreeNode;
// use std::collections::BinaryHeap;
// use bstree::*;

fn main() {
    println!("Hello, world!");

    // let some_content = "hello".to_string();
    // println!("Original content: {:?}", some_content);

    // let Ok(compressed_content) = huffman_compress::compress(&some_content.into_bytes()) else {
    //     panic!("compress failed");
    // };

    // let Ok(decompressed_content) = huffman_compress::decompress(&compressed_content) else {
    //     panic!("decompress failed");
    // };

    // let Ok(decompressed_string) = String::from_utf8(decompressed_content) else {
    //     panic!("tranform to string failed");
    // };
    // println!("decompress result is {:?}", decompressed_string);


    // let _ = huffman_compress::file_compress("assets/img_2026-01-20T13:57:11.833Z.png", "data.bin.compressed");

    // let _ = huffman_compress::file_decompress("data.bin.compressed", "assets/img_2026-01-20T13:57:11.833Z.origin.png");



            // match decompressed_content {
            //     Ok(content) => match String::from_utf8(content) {
            //         Ok(string_content) => println!("Decompression successful: {:?}", string_content),
            //         Err(e) => eprintln!("Failed to convert decompressed content to string: {:?}", e),
            //     },
            //     Err(e) => eprintln!("Decompression failed: {:?}", e),
            // }
}
