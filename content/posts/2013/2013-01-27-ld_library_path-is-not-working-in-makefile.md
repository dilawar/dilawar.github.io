---
title: "LD_LIBRARY_PATH is not working in Makefile"
date: "2013-01-27"
categories: 
  - "linux"
tags: 
  - "library_path-vs-ld_library_path"
  - "makefile"
---

If LD\_LIBRARY\_PATH is exported in Makefile and makefile is not working as you have expected, then you should try updating LIBRARY\_PATH.

> LIBRARY\_PATH is used by gcc before compilation to search for directories containing libraries that need to be linked to your program.

If you wish to link static libraries, you should update LIBRARY\_PATH and not LD\_LIBRARY\_PATH. I made a mistake and suffered for 2 hours.

> LD\_LIBRARY\_PATH is used by your program to search for directories containing the libraries after it has been successfully compiled and linked.

See [this question.](http://stackoverflow.com/questions/4250624/ld-library-path-vs-library-path)
