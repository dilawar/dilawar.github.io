---
title: >-
  Storing environment variables for CICD pipelines
  831d0bb304dc416fbcab15e868e3067a
updated: 2026-01-14 22:55:26Z
created: 2026-01-15 04:51:09Z
---

# Storing environment variables for CICD pipelines

Published On: March 27, 2024
Author: dilawar
Status: Published
Type: Note
Tags: devops

Instead of storing environment variables in Gitlab, we encrypt the `.env` file using `gpg` and commit the encrypted file to the repository. Let’s say that we used key stored in environment variable `KEY` , encrypting is easy `gpg -c .env`. It will prompt for passphrase and create an encrypted file `.env.gpg`. 

To decrypt in the pipeline,

```bash
gpg --yes --batch --passphrase-file <(echo $KEY) -d ./.env.gpg  > .env
```

I am usually nervous talking about security since there are many ways things can go wrong. But here is my take on the pros of this approach.

- The environment remains part of the code and can be changed easily by the developer.
- You can run the pipelines locally as well without downloading many environment variables and files from [Gitlab.](http://Gitlab.YOu)
- It will be easy to migrate to a bit more secure setup e.g. using password vaults and their cli.

Cons

- One secret to rule them all! Keep it secure please.