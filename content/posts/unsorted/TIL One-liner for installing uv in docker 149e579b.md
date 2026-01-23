---
title: TIL One-liner for installing uv in docker 149e579bff89807a85feebd0ec2db807
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# TIL: One-liner for installing uv in docker

Published On: November 25, 2024
Author: dilawar
Status: Published
Type: TIL
Tags: lang:python

One can install [#uv](https://fosstodon.org/tags/uv) in container using the following one-liner

`COPY --from=ghcr.io/astral-sh/uv:latest /uv /bin/uv`

Thanks https://simonwillison.net/2024/Nov/24/async-django/#atom-everything