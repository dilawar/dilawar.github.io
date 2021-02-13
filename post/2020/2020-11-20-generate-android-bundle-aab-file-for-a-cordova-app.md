---
title: "Generate Android Bundle (.aab file) for a Cordova app"
date: "2020-11-20"
categories: 
  - "notes"
  - "web"
tags: 
  - "android"
  - "android-bundle"
  - "cordova"
---

Cordova 8.1+ supports generating Android Bundle (`.aab` file). Usually cordova generates an `.apk` file for your app. Playstore often warns about `.apk` file about it's bloated size.

To generate an `.aab` file, add `--packageType=bundle` option to your `cordova build` command.

```
cordova build android --release -- --packageType=bundle
```

## Signed bundles

Here is the command I use to generate a singed bundle.

```
cordova build android --release  -- --keystore=~/keystore/dilawar.jks --storePassword=$(KEYSTORE_PASSWORD)  --alias=dilawar --password=$(KEYSTORE_PASSWORD)  --packageType=bundle --webpack.mode=production
```

# The old way to generate bundle

> See the discussion [here](https://github.com/apache/cordova-android/issues/610). And feature request here [https://github.com/apache/cordova-android/issues/729](https://github.com/apache/cordova-android/issues/729) .

These instructions were posted here [https://github.com/apache/cordova-android/issues/610#issuecomment-499114935](https://github.com/apache/cordova-android/issues/610#issuecomment-499114935). They worked for me.

1. Launch Android Studio
2. Go To Import Project (Eclipse ADT, Gradle, etc).
3. Select Android platform directory in your project (/platforms/android).
4. Wait for finish the Sync
5. Go to Build > Generate Sign Bundle
6. Complete sign data
7. Upload de .aab file generated (in path /platforms/android/outputs/

I did not find any significant reduction in the app size when using the `.aab` format (signed bundle). My app was reduced from 6.8MB to 6.5MB :neutral\_face:
