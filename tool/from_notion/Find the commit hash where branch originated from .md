---
title: >-
  Find the commit hash where branch originated from
  26de579bff898098ac0ff3d227416f30
updated: 2026-01-14 22:54:40Z
created: 2026-01-15 04:51:09Z
---

# Find the commit hash where branch originated from

Author: dilawar
Status: Published
Type: Article
Tags: note

```bash
$ git merge-base main $(git branch --show-current)
9cfaf7ec74a4d76a9cf1a2078aa65b0607368dc1
```

The following command, however, returned a different commit

```bash
git reflog show --no-abbrev $(git branch --show-current) | grep "branch: Created from" | awk '{print $1;}'
```