---
title: "Installing neovim on any Linux machine e.g. a shared hosting server"
date: "2020-11-08"
categories: 
  - "linux"
  - "utility"
tags: 
  - "installation"
  - "linux"
  - "neovim"
---

Once in while, I need to edit files on a shared hosting server. I have ssh access but no permission to install anything (dah! it's shared hosting.). The bare-bone `vi` doesn't cut it for me, and I am never going to use cpanel web-editor like a cave-dude!

Fortunately there is way to install neovim on sharing hosting servers. The official instructions are [here](https://github.com/neovim/neovim/wiki/Installing-Neovim#appimage-universal-linux-package).

I wrote this script to automate the process.

```bash
#!/usr/bin/env bash
(
    mkdir -p ~/.cache
    cd ~/.cache
    if [ ! -f ./nvim.appimage ]; then
        curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage 
        chmod u+x nvim.appimage
    fi

    chmod a+x nvim.appimage
    ./nvim.appimage --appimage-extract 
    mv squashfs-root ~/

    # skip this if you want your own configuration.
    if [ ! -d ~/.config/nvim ]; then
        cd ~/.config && git clone https://github.com/dilawar/nvim --recursive
    fi
    echo "Make sure ~/squashfs-root/usr/bin is in path"

    # From https://stackoverflow.com/a/28021305/1805129
    LINE='alias vim=$HOME/squashfs-root/usr/bin/nvim'
    FILE=$HOME/.bashrc
    grep -qF -- "$LINE" "$FILE" || echo "$LINE" >> "$FILE"

    source ~/.bashrc
)
```

You can get this script from [https://github.com/dilawar/Scripts/blob/master/install\_neovim\_appimage.sh](https://github.com/dilawar/Scripts/blob/master/install_neovim_appimage.sh)
