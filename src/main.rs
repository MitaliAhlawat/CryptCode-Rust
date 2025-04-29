use aes::Aes128;
use cbc::{Encryptor, Decryptor};
use cipher::{BlockEncryptMut, BlockDecryptMut, generic_array::GenericArray};
use cipher::KeyIvInit;
use rand::Rng;
use base64::{engine::general_purpose, Engine as _};
use clap::{Parser, Subcommand};

const KEY_SIZE: usize = 16;
const IV_SIZE: usize = 16;

#[derive(Parser)]
#[clap(name = "CryptCode")]
#[clap(version = "1.0")]
#[clap(about = "AES-128 encryption/decryption tool", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a message
    Encrypt {
        /// Message to encrypt
        message: String,
        
        /// Encryption key (16 bytes, base64 encoded)
        #[clap(short, long)]
        key: String,
    },
    /// Decrypt a message
    Decrypt {
        /// Message to decrypt
        message: String,
        
        /// Decryption key (16 bytes, base64 encoded)
        #[clap(short, long)]
        key: String,
    },
    /// Generate a random 16-byte key (base64 encoded)
    GenerateKey,
}

fn encrypt(plaintext: &str, key: &[u8]) -> Result<String, &'static str> {
    if key.len() != KEY_SIZE {
        return Err("The key must be 16 bytes long");
    }

    let mut iv = [0u8; IV_SIZE];
    rand::thread_rng().fill(&mut iv);

    let key = GenericArray::from_slice(key);
    let iv = GenericArray::from_slice(&iv);
    let mut cipher = Encryptor::<Aes128>::new(key, iv);

    let mut buffer = plaintext.as_bytes().to_vec();
    // PKCS7 padding
    let pad_len = KEY_SIZE - (buffer.len() % KEY_SIZE);
    buffer.extend(vec![pad_len as u8; pad_len]);

    // Convert to blocks and encrypt
    let blocks = buffer.chunks_exact_mut(KEY_SIZE);
    for block in blocks {
        cipher.encrypt_block_mut(GenericArray::from_mut_slice(block));
    }

    let mut result = iv.to_vec();
    result.extend(buffer);

    Ok(general_purpose::STANDARD.encode(&result))
}

fn decrypt(ciphertext: &str, key: &[u8]) -> Result<String, &'static str> {
    if key.len() != KEY_SIZE {
        return Err("The key must be 16 bytes long");
    }
    
    let data = general_purpose::STANDARD.decode(ciphertext).map_err(|_| "Base64 decoding failed")?;

    if data.len() < IV_SIZE {
        return Err("Data too short to contain IV and ciphertext");
    }
    
    let (iv, ciphertext) = data.split_at(IV_SIZE);
    let key = GenericArray::from_slice(key);
    let iv = GenericArray::from_slice(iv);
    let mut cipher = Decryptor::<Aes128>::new(key, iv);

    let mut buffer = ciphertext.to_vec();
    // Convert to blocks and decrypt
    let blocks = buffer.chunks_exact_mut(KEY_SIZE);
    for block in blocks {
        cipher.decrypt_block_mut(GenericArray::from_mut_slice(block));
    }

    // Remove PKCS7 padding
    let pad_len = buffer[buffer.len() - 1] as usize;
    if pad_len > KEY_SIZE {
        return Err("Invalid padding");
    }
    buffer.truncate(buffer.len() - pad_len);

    String::from_utf8(buffer).map_err(|_| "Invalid UTF-8")
}

fn generate_key() -> String {
    let mut key = [0u8; KEY_SIZE];
    rand::thread_rng().fill(&mut key);
    general_purpose::STANDARD.encode(key)
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { message, key } => {
            let key_bytes = general_purpose::STANDARD.decode(&key).unwrap_or_else(|_| {
                eprintln!("Error: Key must be base64 encoded");
                std::process::exit(1);
            });
            
            match encrypt(&message, &key_bytes) {
                Ok(ciphertext) => println!("{}", ciphertext),
                Err(e) => eprintln!("Encryption failed: {}", e),
            }
        }
        Commands::Decrypt { message, key } => {
            let key_bytes = general_purpose::STANDARD.decode(&key).unwrap_or_else(|_| {
                eprintln!("Error: Key must be base64 encoded");
                std::process::exit(1);
            });
            
            match decrypt(&message, &key_bytes) {
                Ok(plaintext) => println!("{}", plaintext),
                Err(e) => eprintln!("Decryption failed: {}", e),
            }
        }
        Commands::GenerateKey => {
            println!("{}", generate_key()); 
        }
    }
}


