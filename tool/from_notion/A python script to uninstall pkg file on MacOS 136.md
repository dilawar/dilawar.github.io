---
title: >-
  A python script to uninstall pkg file on MacOS
  136e579bff8980fe8eeac3084042eedb
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# A python script to uninstall pkg file on MacOS

Published On: November 7, 2024
Author: dilawar
Status: Published
Type: Note
Tags: note

[https://github.com/dilawar/Scripts/blob/master/pkg_uninstall.py](https://github.com/dilawar/Scripts/blob/master/pkg_uninstall.py)

## Usage

Pass pkg name and it will show you installed packages that matches the query and can be removed. 

```bash
dilawar@halwa ~/Scripts (master)> sudo python3 pkg_uninstall.py clamav
0: com.cisco.ClamAV.programs
1: com.cisco.ClamAV.libraries
2: com.cisco.ClamAV.documentation
Select a package to uninstall 0            
Uninstalling com.cisco.ClamAV.programs
Forgot package 'com.cisco.ClamAV.programs' on '/'.
dilawar@halwa ~/Scripts (master)> sudo python3 pkg_uninstall.py clamav
0: com.cisco.ClamAV.libraries
1: com.cisco.ClamAV.documentation
Select a package to uninstall 0
Uninstalling com.cisco.ClamAV.libraries
Forgot package 'com.cisco.ClamAV.libraries' on '/'.

```

And that’s it.