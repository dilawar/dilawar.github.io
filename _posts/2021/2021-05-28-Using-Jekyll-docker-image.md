---
title: Using the Jekyll docker image
date: 2021-05-30
comments: true
---

I use Jekyll to build this site. Recently, I switched to Manjaro that comes with ruby-3.0 by
default, I could install Jekyll successfully using `pacman` but could not run it.

```
$ jekyll --help
/usr/lib/ruby/3.0.0/rubygems.rb:281:in `find_spec_for_exe': can't find gem jekyll (>= 0.a) with executable jekyll (Gem::GemNotFoundException)
	from /usr/lib/ruby/3.0.0/rubygems.rb:300:in `activate_bin_path'
	from /usr/bin/jekyll:23:in `<main>'
```

Honestly, I don't like to fight with installation process. And I know very little about ruby. So I
turned to docker which I know little.

TLDR: Here is my `Makefile`. I run `make` in the Github repository to build and test this site.

```make
JEKYLL_VERSION := 3.8

JEKYLL := docker run --rm \
	--volume="$(PWD):/srv/jekyll" \
	--volume="$(PWD)/vendor/bundle:/usr/local/bundle" \
	-p 4000:4000 \
	-it jekyll/jekyll:$(JEKYLL_VERSION) jekyll

all :
	$(JEKYLL) serve

```

The core command is the following. Note the small tweak since `bash` and `make` has different syntax
for expanding variable: `$(FOO)` in Makefile is equivalent to `$FOO` or `${FOO}` in bash.

```bash
$ docker run --rm \
	--volume="$PWD:/srv/jekyll" \
	--volume="$PWD/vendor/bundle:/usr/local/bundle" \
	-p 4000:4000 \
	-it jekyll/jekyll:3.8 jekyll serve
```

Option `-p 4000:4000` exposes port 4000 of docker container to localhost. Without this option,
`http://localhost:4000` in your browser will not show any content.  Option
`--volume="$(PWD)/vendor/bundle:/usr/local/bundle"` to make sure that gems installed during the run
process are not lost.

