---
title: >-
  Fix Rg not launching search window on vim+windows+
  12be579bff89805998d5f279023ad453
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# Fix :Rg not launching search window on vim+windows+msys2

Published On: October 26, 2024
Author: dilawar
Status: Published
Type: Note
Tags: note

When you run `:Rg` in vim in a `msys` terminal on Windows and it doesn’t launch the search window but rather drops you into `less` .

Check your `TERM` variable `echo $TERM`. If it not `cygwin`, set it to `cygwin` and that it. Reference https://github.com/junegunn/fzf/issues/2153