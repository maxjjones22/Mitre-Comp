
//  @file "simple_crypto.rs"
//  @author Maxwell Jones
//  @brief Simplified Crypto API Implementatio
//  @date 1/23/2025
//
//  This is an example system for MITRE's 2025 Embedded System CTF (eCTF).
//  This code is a translation of the original explanation, but translated into Rust
//  Not exactly pretty, but its my first time
//  Original code: https://github.com/mitre-cyber-academy/2025-ectf-insecure-example/blob/release/decoder/src/simple_crypto.c



//------------Function Protos------------

//  @brief Emcrypts plaintext using a symmetric cipher
//
//  @return 0 on success, -1 on bad length, and other non-zero for other error
//  I am using a signed 8 bit, since the error lenth will never be large, therefore does not need as much memory
fn encrpyt_sym(uint8_t *plaintext) -> i8 {
    Aes ctx;
}
