---
title: "Fix    \"unknown script 'context.lua' or 'mtx-context.lua'\""
date: "2016-02-20"
tags: 
  - "context"
  - "latex"
---

- \`$ locate mtx-context.lua\` in shell (\`sudo updatedb\`  may be required to update the `mlocate` database first). If this file is found, then you probably need to export \`TEXMF\` variable.
- In my case, all I had to do: \`$ export TEXMF=/usr/share/texmf\` on my OpenSUSE Leap box. I installed \`sudo zypper install texlive-context\`.

Other solutions are here: http://tex.stackexchange.com/questions/53892/texlive-2011-context-problem
