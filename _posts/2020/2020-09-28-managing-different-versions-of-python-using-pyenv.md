---
title: "Managing multiple versions of Python using pyenv"
date: "2020-09-28"
tags: 
  - "pyenv"
  - "python"
---

I recently discovered [pyenv](https://github.com/pyenv/pyenv). I think it's cool. It lets me setting up Python version easily on my Linux box.

It is _not_ written in Python and it is not installable using `pip`. Executing `pip install pyenv` in terminal didn't work for me.

I used the system package manager to install it: `sudo zypper install pyenv` (on OpenSUSE).

Here is `$ tldr pyenv` for it.

 **pyenv**

  **Switch between multiple versions of Python easily.**
 **More information: https://github.com/pyenv/pyenv.**

  - List all available commands:
    pyenv commands

  - List all Python versions under the ${PYENV\_ROOT}/versions directory:
    pyenv versions

  - Install a Python version under the ${PYENV\_ROOT}/versions directory:
    pyenv install 2.7.10

  - Uninstall a Python version under the ${PYENV\_ROOT}/versions directory:
    pyenv uninstall 2.7.10

  - Set Python version to be used globally in the current machine:
    pyenv global 2.7.10

  - Set Python version to be used in the current directory and all directories below it:
    pyenv local 2.7.10

I used `$ pyenv install 3.7.2` to install version 3.7.2 on my system. Leap 15.2 has 3.6 as default which does not have good `typing` support.

A detailed tutorial is available here [https://realpython.com/intro-to-pyenv/](https://realpython.com/intro-to-pyenv/) .
