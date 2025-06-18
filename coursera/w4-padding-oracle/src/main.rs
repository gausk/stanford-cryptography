use hex;
use reqwest::blocking::get;

fn get_status_code(iv: &[u8], cipher_str: &str) -> u16 {
    let response = get(format!(
        "http://crypto-class.appspot.com/po?er={}{}",
        hex::encode(&iv),
        cipher_str
    ))
    .unwrap();
    response.status().as_u16()
}

fn padding_oracle(cipher_text: &str) -> String {
    let decoded = hex::decode(cipher_text).expect("Invalid hex string");
    let message = decoded
        .chunks(16)
        .collect::<Vec<&[u8]>>()
        .windows(2)
        .map(|chunk| padding_oracle_for_each_ciphertext(chunk[0], chunk[1]))
        .flatten()
        .collect();
    String::from_utf8(message).unwrap()
}

fn guess_iter() -> impl Iterator<Item = u8> {
    (1..=16) // padding
        .chain(32..=32) // space
        .chain(97..=122) // lowercase letters
        .chain(65..=90) // uppercase letters
}

fn padding_oracle_for_each_ciphertext(iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut decoded_message = Vec::new();
    let mut iv = iv.to_vec();
    let cipher_str = hex::encode(ciphertext);
    for i in 0..16 as usize {
        let padding_value = (i + 1) as u8;
        iv.iter_mut().for_each(|x| {
            *x = *x ^ padding_value ^ (padding_value - 1);
        });
        for guess in guess_iter() {
            iv[16 - 1 - i] = iv[16 - 1 - i] ^ guess;
            match get_status_code(&iv, &cipher_str) {
                403 => {
                    // Padding is incorrect, try next guess
                    iv[16 - 1 - i] = iv[16 - 1 - i] ^ guess;
                    continue;
                }
                _ => {
                    decoded_message.push(guess);
                    break;
                }
            }
        }
    }
    decoded_message.into_iter().rev().collect()
}

fn main() {
    println!("Hello, Week 4 Assignment!\n");
    println!(
        "{}",
        padding_oracle(
            "f20bdba6ff29eed7b046d1df9fb7000058b1ffb4210a580f748b4ac714c001bd4a61044426fb515dad3f21f18aa577c0bdf302936266926ff37dbf7035d5eeb4"
        )
    );
}
