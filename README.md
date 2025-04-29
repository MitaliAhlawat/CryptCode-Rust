# CS-128-Honors-Final-Project

Group Name: CryptCode

Group member Names, NetID: Mitali Ahlawat (mitalia2), Divyanshi Bansal (dbansal5)

Our project focuses on developing a tailored encryption and decryption function with Rust. Our
vision is to design an effective and secure encryption technique that will enable confidentiality in
the information without losing computability feasibility. The reason why our project is embraced
is that encryption plays a priceless role in today's cybersecurity and the knowledge of its
implementation by Rust makes us more aware of cryptographic concepts as well as the Rust
environment.

Goals and Objectives:
- Develop a working encryption and decryption system in Rust.
- Implement a custom or existing encryption algorithm using Rust crates.
- Ensure the algorithm balances security and performance.
- Develop a user-friendly interface for testing encryption and decryption.
- Gain practical experience in Rust, cryptography, and secure coding practices.

Major Components:

  1. Encryption Algorithm:
     - We will design a symmetric/asymmetric encryption algorithm, or modify an existing one (e.g., AES, RSA, or a custom algorithm).
     - We will utilize Rust cryptographic crates such as aes, rsa, or ring for secure encryption.
     - The algorithm should support variable-length inputs and key management.

  3. Decryption Algorithm:
     - The decryption process should accurately retrieve the original plaintext.
     - Implement key verification and error handling to prevent unauthorized decryption.

  5. User Interface (Optional):
     - A simple CLI or GUI using Rust frameworks to allow users to encrypt and decrypt messages easily.
     -  Provide feedback on encryption success, key validity, and decryption output.

  7. Testing and Security Analysis:
     - Implement test cases using Rust’s quickcheck for validation.
     - Analyze the strength of the encryption against common attacks

Project Checkpoints
- Checkpoint 1 : Finalize encryption approach.
- Checkpoint 2: Implement encryption and decryption logic, develop CLI/GUI, and conduct initial testing.
- Final Submission: Deliver a fully functional Rust-based encryption/decryption program with documentation and security analysis.

Possible Challenges
- Understanding and implementing cryptographic principles correctly in Rust.
- Learning and using Rust cryptographic crates effectively.
- Ensuring secure key storage and management.
- Handling performance trade-offs between security and efficiency.
- Debugging encryption/decryption mismatches.

References
- Rust Crypto Crates: https://crates.io/categories/cryptography
- Rust ring crate documentation: https://docs.rs/ring/latest/ring/
- AES and RSA Wikipedia articles :
  - https://en.wikipedia.org/wiki/Advanced_Encryption_Standard,
  - https://en.wikipedia.org/wiki/RSA_cryptosystem]


## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/CS-128-Honors-Final-Project.git
   cd CS-128-Honors-Final-Project

## Build: 
cargo build --release

## Usage:
Generate a Key:

cargo run -- generate-key

Encrypt a Message:

cargo run -- encrypt "Your secret message" --key YOUR_BASE64_KEY

Decrypt:
cargo run -- decrypt BASE64_CIPHERTEXT --key YOUR_BASE64_KEY

Example Workflow
Generate a key:

cargo run -- generate-key > secret.key
Encrypt a message:

cargo run -- encrypt "Hello World" --key $(cat secret.key)
Decrypt the message:

cargo run -- decrypt "BASE64_OUTPUT_FROM_STEP_2" --key $(cat secret.key)

Technical Implementation
Major Components
Encryption Algorithm:

AES-128 CBC implementation

Uses Rust cryptographic crates (aes, cbc, cipher)

Supports variable-length inputs with PKCS7 padding

Decryption Algorithm:

Accurate plaintext retrieval

Key verification and error handling

Padding removal

Testing:

Unit tests for encryption/decryption

Error case handling





