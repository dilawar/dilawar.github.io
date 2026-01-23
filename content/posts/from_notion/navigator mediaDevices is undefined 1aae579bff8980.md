---
title: navigator mediaDevices is undefined 1aae579bff89806b9c66fb516e4f5399
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# navigator.mediaDevices is undefined

Published On: March 2, 2025
Author: dilawar
Status: Published
Type: Note
Tags: lang:js, note

If you are using internal IPs e.g. `192.168.0.300` etc for development, you are likely to encounter this error when dealing with media devices.

Assuming that you are not using [`localhost`](http://localhost) for development for a reason, you can do either of the following

1. In `Firefox` you can enable the following two options described in https://stackoverflow.com/a/66605018/1805129. Go to `about:config`
set to true `media.devices.insecure.enabled`  and `media.getusermedia.insecure.enabled`
    
    ![image.png](../../../_resources/image-14.png)
    
2. You can use service like `ngrok` or https://serveo.net/ to temporarily get https address. I learnt about these two services in the README file of https://www.npmjs.com/package/vue-qrcode-reader.