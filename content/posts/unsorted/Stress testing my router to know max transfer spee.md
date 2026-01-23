---
title: >-
  Stress testing my router to know max transfer spee
  24de579bff8980f68511debcd2075dae
updated: 2026-01-14 22:54:30Z
created: 2026-01-15 04:51:09Z
---

# Stress testing my router to know max transfer speed over WiFi/LAN

Published On: August 12, 2025
Author: dilawar
Status: Published
Type: Note
Tags: note

https://www.speedtest.net/ is great tool for learning about your internet connection speed. 

If you are in a local environment, how do you figure out available bandwidth? Use cases: copying large files from one internal machine to another internal machine? Or running a local streaming service. 

You can usehttps://iperf.fr/. You need two machines capable or running iperf3.

To test WiFi speed, at least one of connected via Wi-Fi. Both can also be connected by WiFi (worst case). My recommendation is to test in both cases.

- Install `iperf3` on both machines `A` and `B`.
- Run `iperf3 -s` on machine `A` or use `iperf3 -s -D` to keep it running as daemon. Handy if you plan to test periodically.
- Run `iperf3 -c <ip_of_A>` on machine B.

## Sample output

Here is sample output at my home when both devices are connected by WiFi.

On a machine with IP `191.168.1.142`, I run `iperf3` in server mode. 

```bash
$ iperf3 -s -D
```

Then on second machine

```bash
$ iperf3 -c 192.168.1.142
Connecting to host 192.168.1.142, port 5201
[  5] local 192.168.1.131 port 38888 connected to 192.168.1.142 port 5201
[ ID] Interval           Transfer     Bitrate         Retr  Cwnd
[  5]   0.00-1.00   sec  12.8 MBytes   107 Mbits/sec    0    124 KBytes       
[  5]   1.00-2.00   sec  13.2 MBytes   111 Mbits/sec    0    124 KBytes       
[  5]   2.00-3.00   sec  13.1 MBytes   110 Mbits/sec    0    124 KBytes       
[  5]   3.00-4.00   sec  13.6 MBytes   114 Mbits/sec    0    124 KBytes       
[  5]   4.00-5.00   sec  13.0 MBytes   109 Mbits/sec    0    124 KBytes       
[  5]   5.00-6.00   sec  12.6 MBytes   106 Mbits/sec    0    124 KBytes       
[  5]   6.00-7.00   sec  11.2 MBytes  94.4 Mbits/sec    0    124 KBytes       
[  5]   7.00-8.00   sec  10.4 MBytes  87.0 Mbits/sec    0    124 KBytes       
[  5]   8.00-9.00   sec  13.0 MBytes   109 Mbits/sec    0    124 KBytes       
[  5]   9.00-10.00  sec  13.1 MBytes   110 Mbits/sec    0    124 KBytes       
- - - - - - - - - - - - - - - - - - - - - - - - -
[ ID] Interval           Transfer     Bitrate         Retr
[  5]   0.00-10.00  sec   126 MBytes   106 Mbits/sec    0            sender
[  5]   0.00-10.01  sec   126 MBytes   105 Mbits/sec                  receiver

iperf Done.
```

Here we go. I have some idea how fast I can transfer data between two computers on my local network.