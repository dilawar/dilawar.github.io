---
title: Prepending Appending to PATH using Rust b173c501a4054de0b88de9cd2976751c
updated: 2026-01-14 22:58:26Z
created: 2026-01-15 04:51:09Z
---

# Prepending/Appending to PATH using Rust

Published On: July 25, 2024
Author: dilawar
Status: Published
Type: Note
Tags: rust

```rust
/// Add to to PATH environment variable. If `append` is false, add to the 
/// front of list.
pub fn add_to_path(path: &str, append: bool) -> anyhow::Result<()> {
    use anyhow::Context;
    use std::env;
    use std::path::PathBuf;

    let paths = env::var_os("PATH").context("empty PATH")?;
    let mut paths = env::split_paths(&paths).collect::<Vec<_>>();
    if append {
        paths.push(PathBuf::from(path));
    } else {
        paths.insert(0, PathBuf::from(path));
    }

    let new_path = env::join_paths(paths)?;
    env::set_var("PATH", new_path);

    Ok(())
}
```