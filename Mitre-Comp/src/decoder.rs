//
// @file    decoder.rs
// @author  Maxwell Jones
// @brief   eCTF Decoder Example Design Implementation
// @date    2025
//
// This source file is part of an example system for MITRE's 2025 Embedded System CTF (eCTF).
// This code is basically a translation of the official code, just translated into Rust
// Original Code: https://github.com/mitre-cyber-academy/2025-ectf-insecure-example/blob/release/decoder/src/decoder.c

include! ("simple_crypto.rs");
include! ("simple_flash.rs");

enum msg_type_t {
    DECODE_MSG = 'D',     // 'D' - 0x44
    SUBSCRIBE_MSG = 'S',  // 'S' - 0x53
    LIST_MSG = 'L',       // 'L' - 0x4c
    ACK_MSG = 'A',        // 'A' - 0x41
    DEBUG_MSG = 'G',      // 'G' - 0x47
    ERROR_MSG = 'E',      // 'E' - 0x45
}



//@brief - Starts up the system boot
fn init(){
    let ret: u32;

    flash_simple_init();
}

//----------------------------------------
//---------------MAIN LOOP----------------
//----------------------------------------

fn main (){
    let output_buff: [char; 128] = {0};
    let uart_buf: [u8; 100];
    let cmd = msg_type_t;
    let result: i8;
    let pkt_len: u16;

    //Initialize Device
    init();     
}
