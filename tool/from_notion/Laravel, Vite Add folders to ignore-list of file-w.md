---
title: >-
  Laravel, Vite Add folders to ignore-list of file-w
  153e579bff89800daaaeef1684f1f6b0
updated: 2026-01-14 22:54:14Z
created: 2026-01-15 04:51:09Z
---

# Laravel, Vite | Add folders to ignore-list of file-watcher

Published On: December 5, 2024
Author: dilawar
Status: Published
Type: TIL
Tags: lang:js, lang:php

Launching Laravel app with Vite failed today. 

It is because there are too many files in the app directory and watcher that looks for changes and signal server to reload could not watch so many files due to upper limit.

```jsx
[vite] 
[vite]   ➜  APP_URL: http://localhost
[server]   2024-12-05 13:56:31 / ................................................ ~ 1s
[vite] node:internal/fs/watchers:247
[vite]     const error = new UVException({
[vite]                   ^
[vite] 
[vite] Error: ENOSPC: System limit for number of file watchers reached, watch '/home/dilawar/Work/SUBCOM/binhound/app/storage/cvelistV5.git/cves/2013
/1xxx/CVE-2013-1934.json'
[vite]     at FSWatcher.<computed> (node:internal/fs/watchers:247:19)
[vite]     at Object.watch (node:fs:2491:36)
[vite]     at createFsWatchInstance (file:///home/dilawar/Work/SUBCOM/binhound/app/node_modules/vite/dist/node/chunks/dep-CB_7IfJ-.js:42779:17)
[vite]     at setFsWatchListener (file:///home/dilawar/Work/SUBCOM/binhound/app/node_modules/vite/dist/node/chunks/dep-CB_7IfJ-.js:42826:15)
[vite]     at NodeFsHandler._watchWithNodeFs (file:///home/dilawar/Work/SUBCOM/binhound/app/node_modules/vite/dist/node/chunks/dep-CB_7IfJ-.js:42981:
14)
[vite]     at NodeFsHandler._handleFile (file:///home/dilawar/Work/SUBCOM/binhound/app/node_modules/vite/dist/node/chunks/dep-CB_7IfJ-.js:43045:23)
[vite]     at NodeFsHandler._addToNodeFs (file:///home/dilawar/Work/SUBCOM/binhound/app/node_modules/vite/dist/node/chunks/dep-CB_7IfJ-.js:43287:21)
[vite] Emitted 'error' event on FSWatcher instance at:
[vite]     at FSWatcher._handleError (file:///home/dilawar/Work/SUBCOM/binhound/app/node_modules/vite/dist/node/chunks/dep-CB_7IfJ-.js:44480:10)
[vite]     at NodeFsHandler._addToNodeFs (file:///home/dilawar/Work/SUBCOM/binhound/app/node_modules/vite/dist/node/chunks/dep-CB_7IfJ-.js:43295:18) 
{
[vite]   errno: -28,
[vite]   syscall: 'watch',
[vite]   code: 'ENOSPC',
[vite]   path: '/home/dilawar/Work/SUBCOM/binhound/app/storage/cvelistV5.git/cves/2013/1xxx/CVE-2013-1934.json',
[vite]   filename: '/home/dilawar/Work/SUBCOM/binhound/app/storage/cvelistV5.git/cves/2013/1xxx/CVE-2013-1934.json'
[vite] }
[vite] 
[vite] Node.js v20.15.1
[vite] npm run dev-rasmalai exited with code 1

```

The solution is somehow add folders with temp data to ignore list.  https://stackoverflow.com/questions/77355876/vite-not-respecting-server-watch-ignored-option-laravel/77355912#77355912 

```jsx
import { defineConfig } from 'vite';                                                                                                                 
import laravel from 'laravel-vite-plugin';                                                                                                           
                                                                                                                                                     
export default defineConfig({                                                                                                                        
    plugins: [                                                                                                                                       
        laravel({                                                                                                                                    
            input: [                                                                                                                                 
                'resources/sass/app.scss',                                                                                                           
                'resources/js/app.js',                                                                                                               
            ],                                                                                                                                       
            refresh: true,                                                                                                                           
        }),                                                                                                                                          
    ],                                                                                                                                               
    // Thanks https://stackoverflow.com/a/77355912/1805129                                                                                           
    server: {                                                                                                                                        
        watch: {                                                                                                                                     
            ignored: [ '**/node_modules/**', '**/storage/**'],                                                                                             
        },                                                                                                                                           
    }                                                                                                                                                
});                                                                                                                                                  
```