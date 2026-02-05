---
title: Rust and kernel development 0 2ade579bff89801a9e40f54a68897d8e
updated: 2026-01-14 22:55:02Z
created: 2026-01-15 04:51:09Z
---

# Rust and kernel development: 0

Published On: November 16, 2025
Author: dilawar
Status: Published
Type: Note
Tags: article, note

Today, I finally started doing kernel development in Rust in free time. First, I subscribed to a few mailing lists using NNTP (I am using Thunderbird, https://brennan.io/2021/05/05/kernel-mailing-lists-thunderbird-nntp/). I could not find any NNTP client for android! 

I also joined Zulip group for rust for Linux!

After that, I followed the official guide, ⁣ buthttps://docs.kernel.org/rust/quick-start.html could not get Rust specific options in `make LLVM=1 menuconfig` on Debian Trixie. 

When searched for `rust` in the pager, I found out that a few flags were not set. Now I have no clue how to set these. 

![image.png](../../../_resources/image-11.png)

After searching for a while, I landed on, https://rust-exercises.ferrous-systems.com/latest/book/building-linux-kernel-driver which is a more detailed guide. I missed one step (missing from official site).

```bash
make LLVM=1 rustavailable
make LLVM=1 defconfig 
make LLVM=1 menuconfig  # General setup / [*] Rust support
```

Worked. I saw the `Rust support` options in general settings and manage to tweak options in `Rust hacking` option.

![image.png](../../../_resources/image%201.png)

![image.png](../../../_resources/image%202.png)

Great!