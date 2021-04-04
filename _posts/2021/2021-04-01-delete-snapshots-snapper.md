---
title: Deleting snapshots using snapper (openSUSE)
status: publish
author: Dilawar Singh
layout: post
tags: []
categories: []
comments: true
---

First list the available configuration.

```shell
[dilawars@chutki ~]$ sudo snapper list-configs 
Config | Subvolume
-------+----------
root   | /        
```

List the snapshots for a given config.

```shell
[dilawars@chutki ~]$ sudo snapper --config root list
  # | Type   | Pre # | Date                            | User | Cleanup | Description           | Userdata     
----+--------+-------+---------------------------------+------+---------+-----------------------+--------------
 0  | single |       |                                 | root |         | current               |              
 1* | single |       | Thu 13 Aug 2020 12:32:08 PM IST | root |         | first root filesystem |              
 8  | pre    |       | Sun 28 Mar 2021 11:33:58 PM IST | root | number  | zypp(zypper)          | important=yes
11  | pre    |       | Mon 29 Mar 2021 02:29:30 PM IST | root | number  | zypp(zypper)          | important=yes
12  | post   |    11 | Mon 29 Mar 2021 03:13:09 PM IST | root | number  |                       | important=yes
15  | pre    |       | Wed 31 Mar 2021 04:31:40 PM IST | root | number  | zypp(zypper)          | important=no 
16  | post   |    15 | Wed 31 Mar 2021 04:32:18 PM IST | root | number  |                       | important=no 
17  | pre    |       | Thu 01 Apr 2021 12:05:31 AM IST | root | number  | zypp(zypper)          | important=no 
18  | post   |    17 | Thu 01 Apr 2021 12:13:33 AM IST | root | number  |                       | important=no 

```

Now delete.

```shell
[dilawars@chutki ~]$ sudo snapper --config root delete 15
[dilawars@chutki ~]$ sudo snapper --config root delete 16
[dilawars@chutki ~]$ sudo snapper --config root delete 17
```

_You can't delete the snapshot id with `*` on it._ In this case, it is `1*`.

## Resources

1. https://www.simplified.guide/suse/snapper-remove-snapshots
2. https://unix.stackexchange.com/questions/181621/clear-all-snapper-snapshots
