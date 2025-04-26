use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_padding::Pkcs7;
use rand::Rng;
use base64::{encode, decode};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const KEY_SIZE: usize = 16;
const IV_SIZE: usize = 16;

fn encrypt(_plaintext: &str, _key: &[u8]) -> Result<String, &'static str> {

    if _key.len() != KEY_SIZE {
        return Err("The key must be 16 bytes long")
    }

    let mut iv = [0u8; IV_SIZE];
    rand::thread_rng().fill(&mut iv);

    let cipher = Aes128Cbc::new_from_slices(_key, &iv).map_err(|_|"Cipher initialization failed")?;

    let ciphertext= cipher.encrypt_vec(_plaintext.as_bytes());

    let mut result = iv.to_vec();
    result.extend(ciphertext);

    Ok(encode(&result))
}

fn decrypt(_ciphertext: &str, _key: &[u8]) -> Result<String, &'static str> {

    if _key.len() != KEY_SIZE {
        return Err("The key must be 16 bytes long")
    }
    let data = decode(_ciphertext).map_err(|_| "Base64 decoding failed")?;

    if data.len() < IV_SIZE {
        return Err("Data too short to contain IV and ciphertext");
    }
    let (iv, ciphertext) = data.split_at(IV_SIZE);

    let cipher = Aes128Cbc::new_from_slices(_key, iv).map_err(|_| "Cipher initialization failed")?;

    let decrypted_data = cipher.decrypt_vec(ciphertext).map_err(|_| "Decryption failed")?;

    String::from_utf8(decrypted_data).map_err(|_| "Invalid UTF-8")
}

fn main() {
    println!("Encryption and decryption functions initialized.");
}

