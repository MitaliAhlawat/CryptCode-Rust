## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/CS-128-Honors-Final-Project.git
   cd CS-128-Honors-Final-Project

## Build: 
cargo build --release

## Usage:
### Generate a Key:

cargo run -- generate-key

### Encrypt a Message:

cargo run -- encrypt "Your secret message" --key YOUR_BASE64_KEY

### Decrypt:
cargo run -- decrypt BASE64_CIPHERTEXT --key YOUR_BASE64_KEY

## Example Workflow
### Generate a key:

cargo run -- generate-key > secret.key
### Encrypt a message:

cargo run -- encrypt "Hello World" --key $(cat secret.key)
### Decrypt the message:

cargo run -- decrypt "BASE64_OUTPUT_FROM_STEP_2" --key $(cat secret.key)

## Technical Implementation
### Major Components
### Encryption Algorithm:

- AES-128 CBC implementation
- Uses Rust cryptographic crates (aes, cbc, cipher)
- Supports variable-length inputs with PKCS7 padding
- Decryption Algorithm:
- Accurate plaintext retrieval
- Key verification and error handling
- Padding removal

### Testing:
- Unit tests for encryption/decryption
- Error case handling
