---
title: Display git branches sorted by activity 3f39811f5b5f4cd0aa4a094fede12cc3
updated: 2026-01-14 22:55:24Z
created: 2026-01-15 04:51:09Z
---

# Display git branches sorted by activity

Published On: July 26, 2024
Author: dilawar
Status: Published
Type: Note
Tags: git, tips

Many times I have to check the branch I touched most recently. `git-extras` didn’t have a useful command to list the branches sorted by activity. I search around and found the following script that works perfectly.

```bash
#!/bin/sh

set -ex

git branch \
    -va \
    --sort=-committerdate \
    --format='%(committerdate:short) %(refname:short)'
```

The most updated version can be found here  https://raw.githubusercontent.com/dilawar/Scripts/master/git_branch_sorted_by_activity.sh.