---
title: >-
  Making a Large Number by Stacking Combining Smalle
  716ba2dd5c7a4aec99a8fd05db6e21c5
updated: 2026-01-14 22:55:24Z
created: 2026-01-15 04:51:09Z
---

# Making a Large Number by Stacking/Combining Smaller Numbers in Rust

Published On: August 17, 2023
Author: dilawar
Status: Published
Type: Note
Tags: programming, rust

1. Stacking two `u32`s into one `u64`
    
    [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=48defd8d9b23f5fbfffab7d303423fec)
    
    ```rust
    fn main() {
        let a = 1u32;
        let b = 1u32;
        let ab = [a.to_be_bytes(), b.to_be_bytes()].concat();
        let ab : [u8; 8] = ab.try_into().unwrap();
        let c = u64::from_be_bytes(ab);
        println!("{a} {b} {c}");
    }
    ```
    
2. Combine four  `u32` into one `i128`
    
    [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=47ff2a71b83f643a5e61494fea0b9293)
    
    ```rust
    fn four_u32_to_i128(a: u32, b: u32, c: u32, d: u32) -> i128 {
        let abcd = [a.to_be_bytes(), b.to_be_bytes(), c.to_be_bytes(), d.to_be_bytes()].concat();
        let abcd : [u8; 16] = abcd.try_into().unwrap();
        i128::from_be_bytes(abcd)
    }
    
    fn main() {
        let x = four_u32_to_i128(1u32, 2u32, 3u32, 4u32);
        println!("{x}");
    }
    ```