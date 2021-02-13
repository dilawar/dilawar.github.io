---
title: "Use getmail to read only new messages"
date: "2010-12-26"
tags: 
  - "getmail"
  - "multiple-copies-of-email"
  - "random"
---

If your getmail is giving multiple copies of messages are each run, then do this in your configuration file in ~/.getmail/ directory.

\[options\]
verbose = 0
read\_all = false
delete = true
message\_log = ~/.getmail/log

read\_all

is by default True. You need to set it false else it will create many copies of same message and will make your life miserable.
