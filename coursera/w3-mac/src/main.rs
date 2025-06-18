use std::fs::File;
use std::io::Read;
use sha2::{Sha256, Digest};
use hex;

fn read_mp4_file(path: &str) -> Vec<u8> {
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    buffer
}


fn final_sha256_hash(path: &str) -> String {
    let mut current_hash = Vec::new();
    let data = read_mp4_file(path);
    data.chunks(1024).rev().for_each(|chunk| {
        let mut hasher = Sha256::new();
        hasher.update(chunk);
        hasher.update(&current_hash);
        current_hash = hasher.finalize().to_vec();
    });
    hex::encode(current_hash)
}


fn main() {
    println!("Hello, Week 3 Assignment!\n");
    print!("Let's validate first for a video file: ");
    let path = "data/6.2.birthday.mp4_download";
    let expected_hash = "03c08f4ee0b576fe319338139c045c89c3e8e9409633bea29442e21425006ea8";
    let hash = final_sha256_hash(path);
    if hash != expected_hash {
        println!("Hash mismatch! Expected: {}, Got: {}", expected_hash, hash);
        panic!();
    } else {
        println!("Hash matches: {}", hash);
    }

    print!("\nHash h0 for video file 6.1.intro.mp4_download: ");
    let path = "data/6.1.intro.mp4_download";
    println!("{}", final_sha256_hash(path));
}
