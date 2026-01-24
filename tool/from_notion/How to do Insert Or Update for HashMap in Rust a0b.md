---
title: >-
  How to do Insert Or Update for HashMap in Rust
  a0b97ee63f0b4feea7950bb81f347201
updated: 2026-01-14 22:55:26Z
created: 2026-01-15 04:51:09Z
---

# How to do Insert Or Update for HashMap in Rust

Published On: August 8, 2024
Author: dilawar
Status: Published
Type: Note
Tags: note, rust

```rust
let mut map: HashMap<String, usize> = HashMap::new();

// Use entry API
*map.entry(key.to_string()).or_insert_with(0) += 1;

// Or
let values = map.entry(key).or_default();
*values += 1;

// Or, least verbose
*map.entry(key).or_default() += 1
```