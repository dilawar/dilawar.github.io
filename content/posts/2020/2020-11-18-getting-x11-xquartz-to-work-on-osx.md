---
title: "Getting X11 (XQuartz) to work on OSX"
date: "2020-11-18"
categories: 
  - "notes"
---

Today I finally managed to get [XQuartz](https://www.xquartz.org/) to work on a mac mini. I use this machine as server to test my code for OSX. Its a horrible machine to do any development (I am an OSX noob and a terminal junkie).

Usually I login to this machine (named `strudel`) from my Linux workstation using `ssh` e.g. `ssh -Y strudel` or `ssh -Y 192.168.1.2` where 192.168.1.2 is ip address of strudel on LAN. Once a while, I need graphics.

So far, I never managed to open graphics after login. Today, I looked around and found a potential solution called XQuartz.

I installed `xquartz` following [these instructions](https://www.embird.net/sw/embird/tutorial/wine/xquartz.htm). Rebooted and logged in over ssh. Still I couldn't launch `xeyes`. I keep getting `Error: Can't open display:` error. Note that `DISPLAY` is not set.

The solution is straightforward if one reads the [XQuartz FAQ](https://www.xquartz.org/FAQs.html) carefully which I didn't :-(.

One my OSX machine `strudel`, I needed to make sure that `/etc/ssh/sshd_config` has following two lines.

```bash
 X11Forwarding yes                                                               
 X11UseLocalhost yes   
```

And that's it. You probably need to reboot after mcking these changes.
