---
title: Using Jekyll docker image to build your site
date: 2021-05-30
comments: true
---

__Update__: I no longer use Jekyll. I use [Hugo](https://gohugo.io) now.

I usa [Jekyll](https://jekyllrb.com/) to build this site. Recently, I switched
to Manjaro Linux which has ruby-3.0 by default, I could install Jekyll
successfully using `pacman` but could not run it.

```sh
$ jekyll --help
/usr/lib/ruby/3.0.0/rubygems.rb:281:in `find_spec_for_exe': can't find gem jekyll (>= 0.a) with executable jekyll (Gem::GemNotFoundException)
	from /usr/lib/ruby/3.0.0/rubygems.rb:300:in `activate_bin_path'
	from /usr/bin/jekyll:23:in `<main>'
```

I don't like to fight with installation process. And also I know very little
about ruby. So I turned to docker which I have used in the past.

TLDR: Here is my `Makefile`. I execute the command `make` in the Github
repository of this site to build and test this site.

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

If you are familiar with makefiles, you would notice the core command. You can
directly run this command in bash. Note the small tweak since `bash` and `make`
has different syntax for expanding variable: `$(FOO)` in Makefile is equivalent
to `$FOO` or `${FOO}` in bash.

```sh
$ docker run --rm \
	--volume="$PWD:/srv/jekyll" \
	--volume="$PWD/vendor/bundle:/usr/local/bundle" \
	-p 4000:4000 \
	-it jekyll/jekyll:3.8 jekyll serve
```

Option `-p 4000:4000` binds port 4000 of docker container to localhost:4000.
Without this option, `http://localhost:4000` in your browser will not show any
content.  Option `--volume="$(PWD)/vendor/bundle:/usr/local/bundle"` to make
sure that gems installed during the run process are not lost.
