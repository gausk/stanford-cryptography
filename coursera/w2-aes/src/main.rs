use aes::Aes128;
use aes::cipher::BlockDecrypt;
use aes::cipher::BlockEncrypt;
use aes::cipher::KeyInit;
use aes::cipher::generic_array::GenericArray;

fn decrypt_aes_cbc_block(cipher: &Aes128, block: &[u8], prev_block: &[u8]) -> Vec<u8> {
    let mut buf = GenericArray::clone_from_slice(block);
    cipher.decrypt_block(&mut buf);
    buf.into_iter()
        .zip(prev_block.iter())
        .map(|(d, p)| d ^ p)
        .collect()
}

fn decrypt_aes_cbc(key: &[u8], ciphertext: &[u8]) -> String {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut padded_message: Vec<u8> = ciphertext
        .chunks(16)
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| decrypt_aes_cbc_block(&cipher, w[1], &w[0]))
        .flatten()
        .collect();
    padded_message.truncate(padded_message.len() - *padded_message.last().unwrap() as usize);
    String::from_utf8(padded_message).unwrap_or_else(|_| "Decryption failed".to_string())
}

fn bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut fixed: [u8; 16] = Default::default();
    fixed.copy_from_slice(bytes);
    u128::from_be_bytes(fixed)
}

fn decrypt_aes_ctr(key: &[u8], ciphertext: &[u8]) -> String {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut blocks = ciphertext.chunks(16);
    let iv = bytes_to_u128(blocks.next().unwrap());

    let decrypted = blocks
        .enumerate()
        .map(|(i, block)| {
            let ctr_blk = (iv + i as u128).to_be_bytes();
            let mut buf = GenericArray::from(ctr_blk);
            cipher.encrypt_block(&mut buf);
            buf.into_iter()
                .zip(block.iter())
                .map(|(c, b)| c ^ b)
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    String::from_utf8(decrypted).unwrap_or_else(|_| "Decryption failed".to_string())
}

fn main() {
    println!("Hello, Week 2 Assignment!\n");
    println!("Question 1: Decrypted plain text using CBC mode");

    let key = hex::decode("140b41b22a29beb4061bda66b6747e14").unwrap();
    let ciphertext = hex::decode("4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81").unwrap();
    let decrypted = decrypt_aes_cbc(&key, &ciphertext);
    println!("{}\n", decrypted);

    println!("Question 2: Decrypted plain text using CBC mode");
    let key = hex::decode("140b41b22a29beb4061bda66b6747e14").unwrap();
    let ciphertext = hex::decode("5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253").unwrap();
    let decrypted = decrypt_aes_cbc(&key, &ciphertext);
    println!("{}\n", decrypted);

    println!("Question 3: Decrypted plain text using ctr mode");

    let key = hex::decode("36f18357be4dbd77f050515c73fcf9f2").unwrap();
    let ciphertext = hex::decode("69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329").unwrap();
    let decrypted = decrypt_aes_ctr(&key, &ciphertext);
    println!("{}\n", decrypted);

    println!("Question 4: Decrypted plain text using ctr mode");
    let key = hex::decode("36f18357be4dbd77f050515c73fcf9f2").unwrap();
    let ciphertext = hex::decode("770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451").unwrap();
    let decrypted = decrypt_aes_ctr(&key, &ciphertext);
    println!("{}\n", decrypted);
}
