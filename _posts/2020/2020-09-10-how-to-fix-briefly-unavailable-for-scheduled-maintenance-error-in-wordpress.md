---
title: "How to Fix or Get Rid of \"Briefly Unavailable for Scheduled Maintenance\" message in WordPress"
date: "2020-09-10"
categories: 
  - "notes"
tags: 
  - "short-note"
  - "wordpres"
---

**Reason**: During the update of a plugin or a theme, you refreshed the page, moved to another page, or the upgrade failed due to some other reason.

WordPress creates .maintenace file in the top-level directory before the start of the update process. And removes it if the upgrade is successful.

**Solution:**You need to delete the .maintenace file.
