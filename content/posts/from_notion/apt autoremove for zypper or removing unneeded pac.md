---
title: >-
  apt autoremove for zypper or removing unneeded pac
  1efe579bff89804e8e9cd35045c9e818
updated: 2026-01-14 22:54:16Z
created: 2026-01-15 04:51:09Z
---

# apt autoremove for zypper or removing unneeded packages using zypper

Published On: May 10, 2025
Author: dilawar
Status: Published
Type: TIL
Tags: note

```bash
zypper pa --unneeded | grep '^i' | cut -d\| -f3 | xargs sudo zypper -n remove
```

- `zypper pa --unneeded` lists unneeded packages.
- `grep '^i'` selects lines that starts with `i` (installed package)
- `cut d\| -f3` selects third field after cutting line at delimited `|`
- And the last one runs `sudo zypper -n remove` on the output.

On older version of `zypper`, following script may be useful.

[https://github.com/dilawar/Scripts/blob/master/zypper_autoremove.sh](https://github.com/dilawar/Scripts/blob/master/zypper_autoremove.sh)