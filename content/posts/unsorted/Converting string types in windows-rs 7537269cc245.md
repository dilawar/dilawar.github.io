---
title: Converting string types in windows-rs 7537269cc245457bb68fc9259887100b
updated: 2026-01-14 22:55:24Z
created: 2026-01-15 04:51:09Z
---

# Converting string types in windows-rs

Published On: March 7, 2024
Author: dilawar
Status: Published
Type: Note
Tags: rust, windows

| `PWSTR` → `PCWSTR`  | `let s: PWSTR=...; let r:PCWSTR = PCWSTR(s.0)` |
| --- | --- |
| `String` → `PCWSTR` | `let s: String=...; let r:PCWSTR = PCWSTR(HSTRING::from(s).as_ptr())` |
| `&str` → `PCWSTR` | `let r:PCWSTR = w!("example")` |

```rust
//! Trait for converting &str/String into PWSTR.
//! Thanks https://github.com/microsoft/windows-rs/issues/973#issue-942298423

#![cfg(windows)]

use windows::core::PWSTR;

pub trait IntoPWSTR {
    fn into_pwstr(self) -> (PWSTR, Vec<u16>);
}

impl IntoPWSTR for &str {
    fn into_pwstr(self) -> (PWSTR, Vec<u16>) {
        let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();

        (PWSTR(encoded.as_mut_ptr()), encoded)
    }
}

impl IntoPWSTR for String {
    fn into_pwstr(self) -> (PWSTR, Vec<u16>) {
        let mut encoded = self.encode_utf16().chain([0u16]).collect::<Vec<u16>>();

        (PWSTR(encoded.as_mut_ptr()), encoded)
    }
}
```

## References

- https://github.com/microsoft/windows-rs/issues/973