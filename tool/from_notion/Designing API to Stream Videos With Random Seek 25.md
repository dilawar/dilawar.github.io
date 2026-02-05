---
title: >-
  Designing API to Stream Videos With Random Seek
  254e579bff8980aeafeac7af86fd5479
updated: 2026-01-14 22:54:34Z
created: 2026-01-15 04:51:09Z
---

# Designing API to Stream Videos With Random Seek

Published On: August 19, 2025
Author: dilawar
Status: Draft

At work, we have the following situation

- There are more than 20 cameras recording videos at 30 to 60 FPS in 1080p at various locations, but all connected to LAN.
- run real-time inference on individual video streams.
- If possible, use correlation among video stream to our advantage, e.g., a dog moving from camera 1 feed to camera 10 feed.
- Provide easy to use APIs for analysis team, e.g., allow them to seek frames from multiple camera for a given time interval, together (too advance at this stage) or separately.
- if possible, reuse the design for other stream-able data e.g., audio, sensor readings etc.

Let’s not worry about the networking speed for now. These cameras are connected with a “fast enough” router. We did some bench-marking using iperf3 ([Stress testing my router to know max transfer speed over WiFi/LAN](../../../A%20Notion%20Home/0.%20Public%20Website/Blog/Stress%20testing%20my%20router%20to%20know%20max%20transfer%20spee.md)).

## Push or pull?

Should I make cameras push videos to APIs using an endpoint I create, or should the API server poll cameras for stream? Beware whom you involve in this decision? I won’t be surprised if someone writing client vote for poll and one in charge of API recommend push since it will make their life easier! Convenience driven development is much more common-place that you’d think.

My recommendation is to start with `push` since `pull` can lead to many issues that may require much more work to get it right.

- If server goes down or misbehave, the client will know it, and it can plan for it e.g. start storing video locally till the server comes back. If the server is polling and goes offline, how’d the client know that data is not being pulled? Either client always has to keep a copy to