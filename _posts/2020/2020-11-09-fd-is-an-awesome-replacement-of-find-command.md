---
title: "fd is an awesome replacement of find command"
date: "2020-11-09"
categories: 
  - "linux"
  - "notes"
  - "utility"
tags: 
  - "fd"
  - "find"
  - "rust"
  - "tools"
  - "unix"
---

Recently, I came across [fd](https://github.com/sharkdp/fd) command. It is written in Rust and suppose to replace `find` command.

`fd` is amazing, PERIOD. It has much better user experience. Check out the official demo below,

![OFFICIAL DEMO](https://raw.githubusercontent.com/sharkdp/fd/master/doc/screencast.svg)

My pet peeve with `find` command is not that it has too many options, but that these options are hard to figure out. I don't use `find` so often to build a muscle memory. So a simpler version is most welcome.

To give one particular example, yesterday I wanted to know the list of sub-directories I've touched 2 days ago. I played with `-mmin` and `-mtimes` options and got lost. With `fd`, I could just do the following

`$ fd --changed-within 2d`

Do I need to say more?

## `tldr fd`

  **fd**

  **An alternative to find.**
 **Aims to be faster and easier to use than find.**
 **More information: https://github.com/sharkdp/fd.**

  - Find files matching the given pattern in the current directory:
    fd pattern

  - Find files that begin with "foo":
    fd '^foo'

  - Find files with a specific extension:
    fd --extension txt

  - Find files in a specific directory:
    fd pattern path/to/dir

  - Include ignored and hidden files in the search:
    fd --hidden --no-ignore pattern
