---
title: >-
  Flatten a HashMap String, Vec String to Vec String
  145e579bff898008ba1af3e1f5d67c49
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# Flatten a HashMap<String, Vec<String>> to Vec<String> in Rust

Published On: November 21, 2024
Author: dilawar
Status: Published
Type: Note
Tags: rust

I have somethat that may look like the output of `tree` like command. I want it a plain list wihtout nesting for further processing. I want to convert the following on the left to the one on the right.

```
{
    "/opt": [
        "a",
        "b",
        "c",
    ],
    "/root": [
        "a",
        "b",
        "c",
    ],
}
```

```
[
    "/opt/a",
    "/opt/b",
    "/opt/c",
    "/root/a",
    "/root/b",
    "/root/c",
]
```

The following gist shows how to do it. You can play with code on the playground here https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=dfdbc1036a4bc502488598a742552fba.

Line `12` converts HashMap to a vector of vector where inner most vector is join of key with individual values. `flatten` converts vector of vector to a vector (akin to `concat`).

## Gist

[https://gist.github.com/rust-play/3a11b9e4d16e992e995c8aa0d26c6141](https://gist.github.com/rust-play/3a11b9e4d16e992e995c8aa0d26c6141)