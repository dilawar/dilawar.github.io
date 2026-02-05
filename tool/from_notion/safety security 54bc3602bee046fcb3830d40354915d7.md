---
title: safety security 54bc3602bee046fcb3830d40354915d7
updated: 2026-01-14 22:55:24Z
created: 2026-01-15 04:51:09Z
---

# safety > security

Published On: May 25, 2024
Author: dilawar
Status: Published
Type: Article
Tags: opinion

The advertisements during this IPL made me realize that *safety* can not be ensured by striving for *security* only. Safety is a superset of security. You can be inside a totally *secur*e and trusted environment, yet harmful messages (or advertisements) land in your lap. Direct or surrogate messages to consume paan-masala or bet on cricket originate from trusted people, likes of Sunil Gavaskar and Shahrukh Khan. 

Similarly, your endpoint may be very secure yet remain unsafe. A malicious PDF or email can land in your inbox from a trusted source over a very secure channel protected by TLS or VPNs or QUIC channels and still constitute a safety hazard. 

Safety often requires users to exercise critical thinking and good judgment. Your people should be taught how to detect scams, and they must *know* how to detect scams. 

> Scam messages often appear to come from someone you know, if your contact list has been compromised. They almost always carry an element of urgency—for instance, “Help, I have lost my wallet and passport and need funds!” or “I don’t have time to get a gift for my friend. Can you send a gift card?” Some of these messages used to play on greed and now they trade on so many people’s willingness to help a friend. Some of the worst scams prey on people grieving lost friends or family or who want to help during disasters. [[1](https://cacm.acm.org/opinion/on-the-difference-between-security-and-safety/)]
> 

***But what do you do when your endpoint has no user?!*** How do you teach your endpoint to exercise critical thinking and sound judgment? Similar messaging tactics will fool your endpoint into running unsafe code. If you can turn your endpoint *immutable,* then you don’t have this problem, but what about endpoints that must remain mutable to function?

It is a tricky problem to solve. Zero-trust is a strategy in the right direction (though it means a lot of different things to different people). Nonetheless, any incremental progress toward a solution will always be a positive ROI.

First, securing the network is not enough. You *need* an agent on the device to stop malicious behavior or force the user to make a judgment call. Second, you can’t do the “data to decision” step in the cloud—it will be too late! We at Subconscious Compute are taking steps towards a *good solution—*a kernel agent called [Shepherd](https://shepherd.watch) (*trust its instinct*).