---
title: Playing with fingerprint sensor in Rust 2dde579bff8980e79aa6fcdf7e922572
updated: 2026-01-14 22:55:16Z
created: 2026-01-15 04:51:09Z
---

# Playing with fingerprint sensor in Rust

Published On: January 3, 2026
Author: dilawar
Status: Draft
Type: Note

I got a fingerprint sensor connected to a Teensy 3.2 board from a friend. He has already soldered it to the board and put it inside a 3d printed case. He sent me the following URL to get me started.

https://how2electronics.com/interfacing-r502-r503-capacitive-fingerprint-sensor-with-arduino/

![IMG_20260103_125912.jpg](../../../_resources/182579c7-62bf-4779-9a56-d9848316dfc9.png)

I am doing this small project to 

- use rust in embedded/firmware setting
- write a challenging algorithm (fingerprint matching) in rust
- learn a bit more about firmware specific concepts.

## Getting started

It was easy, thanks to  https://branan.github.io/teensy/ 🙏🏾.

## Notes

- Use `nightly` because you are going to use some unstable library features e.g.
    
    ```bash
    error[E0658]: use of unstable library feature `stdarch_arm_hints`
     --> src/watchdog.rs:1:5
      |
    1 | use core::arch::arm::__nop;
      |     ^^^^^^^^^^^^^^^^^^^^^^
      |
      = note: see issue #117218 <https://github.com/rust-lang/rust/issues/117218> for more information
    ```