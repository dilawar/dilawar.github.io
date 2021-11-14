---
title: "Convert letter to image"
date: "2020-10-12"
---

TLDR: `α` -> ![](images/α-1.png)

You can download the script from [here](https://raw.githubusercontent.com/dilawar/Scripts/master/letter2img.py).

This scripts accepts a letter such as `A`, `σ`, `हिॆ` etc. By default, this script uses `DejaVuSansMono.ttf` font (use `-F` option to change it for Devnagari font).

```
usage: letter2img.py [-h] [--size SIZE] [--pen-width PEN_WIDTH] [--font FONT] LETTER

positional arguments:
  LETTER                Letter to convert

optional arguments:
  -h, --help            show this help message and exit
  --size SIZE, -s SIZE  Image size will be s by s
  --pen-width PEN_WIDTH, -p PEN_WIDTH
                        Pen width in pixel. (default auto)
  --font FONT, -F FONT  Which font to use (ttf)
```

## Examples

```
$ letter2img.py A -s 10
```

Image of `A` of size 10x10.

![](images/A.png)

```
$ letter2img.py α -s 100
```

Generates a image of `α` of size 100x100.

![](images/α-1.png)
