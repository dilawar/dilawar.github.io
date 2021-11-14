---
title: "Clean up mess in terminal after ncurses crashes"
date: "2012-08-01"
categories: 
  - "linux"
tags: 
  - "ncurses"
  - "terminal"
---

If your terminal is left in mess (line-break are messed up and newlines are not visible etc) due to ncurses-crash or seg-fault, you can make your terminal behave normally again. Type

stty sane^j

In current implementation of stty.

stty sane