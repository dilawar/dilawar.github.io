---
title: Creating a global rng in Rust 13ce579bff898004b8a6fbfdb7bb5ba1
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# Creating a global rng in Rust

Published On: November 12, 2024
Author: dilawar
Status: Published
Type: Note
Tags: rust

How to create a global RNG (module level) that can be reused between function calls e.g. whenever I call `get_random_u32()` , I get a random `u32` . Say that `thread_rng` is not an option.

Here is a sample program.

[https://gist.github.com/rust-play/7985c99ace004093c63369d5e3219818](https://gist.github.com/rust-play/7985c99ace004093c63369d5e3219818)