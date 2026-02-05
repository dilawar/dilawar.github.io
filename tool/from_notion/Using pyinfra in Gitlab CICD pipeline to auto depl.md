---
title: >-
  Using pyinfra in Gitlab CICD pipeline to auto depl
  1bbe579bff8980e18cd6dc2a7e02cafe
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# Using pyinfra in Gitlab CICD pipeline to auto deploy

Published On: March 19, 2025
Author: dilawar
Status: Published
Type: Note
Tags: cicd, lang:python, note

- First, prepare ssh keys. Configure your server to accept connections from the private key e.g. you should add public key to `~/.ssh/authorized_keys` file.
- Now store the private key to a file variable https://docs.gitlab.com/ci/variables/#custom-environment-variables-of-type-file.
    
    <aside>
    💡
    
    I copy-pasted private key into a file variable called `SSH_PRIVATE_KEY` but I could not connect to the remote server. Apparently, at the time of writing, you should add an empty line after pasting your private key!!
    
    </aside>
    
- Following job is currently in production
    
    ```toml
    deploy_beta:
      image: ghcr.io/astral-sh/uv:debian
      variables:
        SERVER: beta.portal.dilawars.me
      rules:
        - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
      needs:
        - test
      tags:
        - docker-in-docker
      script:
        - echo "Bootstrapping ssh. Creating default directory and configs..."
        - mkdir -p ~/.ssh
        - echo "Checking if SSH_PRIVATE_KEY is set and available as file..."
        - if [ -f $SSH_PRIVATE_KEY ]; then echo "[SUCCESS] SSH_PRIVATE_KEY is set and exists"; else exit -1; fi
        - echo "Ensuring that we don't have too open permissions on ssh key..."
        - chmod 400 $SSH_PRIVATE_KEY
        - echo "Add remote server fingerprint to known_hots"
        - ssh-keyscan -t ed25519 -H $SERVER >> ~/.ssh/known_hosts
        - echo "Testing that we can connect to the server using ssh..."
        - ssh -i $SSH_PRIVATE_KEY ec2-user@$SERVER echo "[SUCCESS] hi from server"
        - echo "Great! Run the deploy script using pyinfra..."
        - uvx pyinfra -y -vvv @ssh/$SERVER --data ssh_key=$SSH_PRIVATE_KEY --data ssh_user=ec2-user ./ci/deploy.nightly.py
    
    ```