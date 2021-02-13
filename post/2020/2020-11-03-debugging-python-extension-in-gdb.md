---
title: "Debugging python extension in gdb"
date: "2020-11-03"
categories: 
  - "notes"
tags: 
  - "gdb"
  - "python"
---

```bash
$ gdb -ex r --args python myscript.py
```

If you are using virtual environment then you are likely to run into the following error.

`/home/dilawars/PY38/shims/python": not in executable format: File format not recognized`

Because, in virtualenv, the command `python` is a bash script wrapping the real python executable. To bypass this, just do the following:

```bash
$ gdb -ex r --args bash python myscript.py
```
