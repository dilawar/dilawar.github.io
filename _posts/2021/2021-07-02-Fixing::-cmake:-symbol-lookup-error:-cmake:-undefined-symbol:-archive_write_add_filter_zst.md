---
title: Fixing 'cmake\: symbol lookup error\: cmake\: undefined symbol\: archive_write_add_filter_zst'
date: 2021-07-02
comments: true
---

I get the following error during build. I think it is specific to CentOS-8.


```sh
$ cmake -DCMAKE_BUILD_TYPE=Release .. && make
cmake: symbol lookup error: cmake: undefined symbol: archive_write_add_filter_zstd
```

It can be fixed easily by installing/updating `libarchive`.


```sh
$ sudo yum install -y cmake libarchive
```
