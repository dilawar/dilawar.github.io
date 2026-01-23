---
title: >-
  Using cargo install in docker and remove temporary
  ce714197a7104b3aa4e41e6d320aeff7
updated: 2026-01-14 22:58:28Z
created: 2026-01-15 04:51:09Z
---

# Using cargo install in docker and remove temporary files

Published On: November 12, 2024
Author: dilawar
Status: Published
Type: Note
Tags: note

We are going to use `cargo install mdbook`  and make sure to remove the temporary files generated during the process. It will reduce the image size

```docker
RUN CARGO_TARGET_DIR=/tmp/cargo cargo install mdbook --root /usr && rm -rf /tmp/cargo
RUN mdbook --version
```