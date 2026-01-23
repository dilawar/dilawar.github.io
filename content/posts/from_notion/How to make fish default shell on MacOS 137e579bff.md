---
title: How to make fish default shell on MacOS 137e579bff8980209d11e4f662d4cb70
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# How to make fish default shell on MacOS

Published On: November 8, 2024
Author: dilawar
Status: Published
Type: Note
Tags: note

- Find the path of `fish` using `which fish`
    
    ```bash
    dilawar@halwa ~> which fish
    /usr/local/bin/fish
    ```
    
- Append this path to file `/etc/shells`
- Execute `chsh -s /usr/local/bin/fish`

# References

- https://stackoverflow.com/a/26321141
- https://gist.github.com/gagarine/cf3f65f9be6aa0e105b184376f765262