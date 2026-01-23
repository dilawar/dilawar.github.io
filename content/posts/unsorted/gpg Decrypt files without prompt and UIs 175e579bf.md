---
title: gpg Decrypt files without prompt and UIs 175e579bff8980669b64fdb0309df8d6
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# gpg: Decrypt files without prompt and UIs

Published On: January 8, 2025
Author: dilawar
Status: Published
Type: Note
Tags: cicd, gpg, note, terminal

For example, decrypting `.env.gpg` in CI pipeline in a bare minimal shell.

```bash
gpg --pinentry-mode=loopback --passphrase $GPG_PASSPHRASE -q --batch --yes --output .env .env.gpg
```