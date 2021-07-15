---
title: Insert username and password into remote url before git push
date: 2021-07-15
tags:
    - git
    - script
    - python3
comments: true
---

I wrote a small script that is a wrapper around `git push`. It modifies the
`remote` url and insert username and password (read from `env`) from it. No need
to store ssh keys on remote server or git clone with https credentials.

For `github.com` repo, It looks for `GITHUB_USER` first, then for `GIT_USER`,
and then `USER` environment variable. Similarly it looks for `GITHUB_TOKEN`,
`GIT_TOKEN` for tokens. Fot `gitlab.com` repo, it looks for `GITLAB_USER` or
`GIT_USER` or `USER` for username, and `GITLAB_TOKEN`, `GIT_TOKEN` for password.
If the script can't determine the username and password, it raises an exception.

Then it calls `git push` on new url that has username and password. For example
`git push https://github.com/dilawar/Scripts` will become `git push
https://dilawar:mytoken@github.com/dilawar/Scritps`.

You can find the script
[here](https://raw.githubusercontent.com/dilawar/Scripts/master/%2Cgit_push).
Here is the fully reproduced one. 

```python
#!/usr/bin/env python3
# pip3 install typer --user  # only dependency

import sys
import typing as T
import subprocess
from urllib.parse import urlparse
import os

import typer

app = typer.Typer()

git = "git"


def env(key:str, default=None) -> T.Optional[str]:
    return os.environ.get(key, default)

def find_remote() -> str:
    remote = subprocess.check_output([git, "remote", "-v"], text=True)
    remote = [x for x in remote.split("\n") if "(push)" in x.strip()][0]
    remote = remote.split()[1]
    return remote


def get_user(domain) -> T.Optional[str]:
    if domain == "github.com":
        return env("GITHUB_USER", env("GIT_USER", None))
    if domain == "gitlab.com":
        return env("GITLAB_USER", env("GIT_USER", None))
    return env('GIT_USER', env('USER', None))

def get_token(domain) -> T.Optional[str]:
    if domain == "github.com":
        return env("GITHUB_TOKEN", env("GIT_TOKEN", None))
    if domain == "gitlab.com":
        return env("GITLAB_TOKEN", env("GIT_TOKEN", None))
    return env("GIT_TOKEN", None)


def find_branch() -> str:
    branches = (
        subprocess.check_output([git, "branch", "-a"], text=True).strip().split("\n")
    )
    branch = [x.strip() for x in branches if x.strip()[0] == "*"][0]
    return branch[1:].strip()  # remote leading *


def rewrite_remote(remote: str) -> str:
    o = urlparse(remote)
    assert o.scheme in ["https", "http"]
    U, P = get_user(o.netloc), get_token(o.netloc)
    assert U is not None, "Could not determine user"
    assert P is not None, "Could not determine token"
    return o.geturl().replace(f"{o.scheme}://", f"{o.scheme}://{U}:{P}@")

def git_push(url, branch):
    cmd = f"git push {url} {branch}"
    subprocess.run(cmd.split())

@app.command()
def main(remote: T.Optional[str] = None, branch: T.Optional[str] = None):
    # get the remote
    remote = find_remote() if remote is None else remote
    branch = find_branch() if branch is None else branch
    url = rewrite_remote(remote)
    git_push(url, branch)


if __name__ == "__main__":
    app()
```

### Usage

Save it as `gh_push.py` somewhere in `$PATH` and make it executable. Export
`GITHUB_USER` and `GITHUB_TOKEN` and,


```sh
$ gh_push.py
```

Its very handly when you don't want to copy your ssh keys to a remote server or
don't want to clone with credentials e.g. `git clone
https://dilawar:mytoken@github.com/dilawar/Scripts`.

