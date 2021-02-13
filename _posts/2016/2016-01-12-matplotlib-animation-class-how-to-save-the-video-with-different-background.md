---
title: "Matplotlib Animation class, how to save the video with different background"
date: "2016-01-12"
tags: 
  - "animation-python"
  - "matplotlib"
---

This is what you have to do when saving the video:

 ani\_.save('output.avi', fps=args\_\['fps'\], savefig\_kwargs = { 'transparent' : True , 'facecolor' : 'black'  }

Argument `savefig_kwargs` is the key parameter. It is applied to `savefig` which is called every time a frame is saved into video file. Ref: http://matplotlib.org/api/animation\_api.html
