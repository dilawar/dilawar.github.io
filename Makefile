JEKYLL_VERSION := 3.8

JEKYLL := docker run --rm \
	--volume="$(PWD):/srv/jekyll" \
	--volume="$(PWD)/vendor/bundle:/usr/local/bundle" \
	-p 4000:4000 \
	-it jekyll/jekyll:$(JEKYLL_VERSION) jekyll

all :
	$(JEKYLL) serve
