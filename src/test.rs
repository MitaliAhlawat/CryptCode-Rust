#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = b"0123456789abcdef"; // 16 bytes
        let plaintext = "Hello, CryptCode!";
        
        let ciphertext = encrypt(plaintext, key).unwrap();
        let decrypted = decrypt(&ciphertext, key).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_invalid_key_length() {
        let short_key = b"shortkey"; // 8 bytes
        let plaintext = "Test message";
        
        assert!(encrypt(plaintext, short_key).is_err());
    }

    #[test]
    fn test_decrypt_invalid_ciphertext() {
        let key = b"0123456789abcdef";
        let invalid_ciphertext = "not a valid ciphertext";
        
        assert!(decrypt(invalid_ciphertext, key).is_err());
    }

    #[test]
    fn test_key_generation() {
        let key1 = generate_key();
        let key2 = generate_key();
        
        assert_eq!(key1.len(), 24); // base64 encoded 16-byte key
        assert_ne!(key1, key2); // keys should be different
    }
}