---
title: Running LanguageTool's server locally 25ce579bff89804fac81d9b17346b89b
updated: 2026-01-14 22:54:34Z
created: 2026-01-15 04:51:09Z
---

# Running LanguageTool's server locally

Published On: August 27, 2025
Author: dilawar
Status: Published
Type: Note
Tags: self-hosted, tools

I set-up LanguageTool’s server on a machine with IP `192.168.1.190`. You can set it up on your local machine. Replace the above IP with `127.0.0.1` and rest of the process remains the same.

- Use docker to deploy the server. You can also build it locally https://dev.languagetool.org/http-server.html. I am using the following script to launch the server. Note the port is `8010`.
    
    [https://github.com/dilawar/Scripts/blob/master/languagetool_server.sh](https://github.com/dilawar/Scripts/blob/master/languagetool_server.sh)
    
- Now go to the browser’s plugin and change the server address to your local server.
    
    ![image.png](../../../_resources/image-10.png)