---
title: >-
  Using zypper inside Dockerfile in non-interactive 
  93f3a84187f74e769bbaa6282c897dcc
updated: 2026-01-14 22:55:26Z
created: 2026-01-15 04:51:09Z
---

# Using zypper inside Dockerfile in non-interactive mode (without user prompts)

Published On: April 8, 2023
Author: dilawar
Status: Published
Type: Note
Tags: note, tips

`zypper` is not the most container-friendly package manager out there. There is a `--non-interactive` global option that silences the prompts and assumes suitable defaults, but it doesn't always work, for example, when you add a repository and import the keys. See more here 

[How to use zypper in bash scripts for someone coming from apt-get?](https://unix.stackexchange.com/questions/82016/how-to-use-zypper-in-bash-scripts-for-someone-coming-from-apt-get)

Following works for me and I use this pattern in all of my openSUSE based containers.

```docker
FROM opensuse/leap:15.4

RUN zypper --non-interactive --quiet addrepo --refresh  \
    https://download.opensuse.org/repositories/devel:languages:rust/15.4/devel:languages:rust.repo
RUN zypper --non-interactive --gpg-auto-import-keys refresh
RUN zypper --non-interactive install \
    --allow-vendor-change \
    osc obs-service-cargo_vendor obs-service-recompress obs-service-tar_scm \
    && zypper clean --all
```

This is based on the following answer.

[How to use zypper in bash scripts for someone coming from apt-get?](https://unix.stackexchange.com/a/438220/5362)