---
title: A math tutor using NextJs tRPC and OpenAI 148e579bff8980e0af92ecef7b5081cf
updated: 2026-01-14 22:54:12Z
created: 2026-01-15 04:51:09Z
---

# A math tutor using NextJs/tRPC and OpenAI

Published On: November 24, 2024
Author: dilawar
Status: Published
Type: Note
Tags: app, lang:js

Last week, I spend some time learning react framework. I did a small project where you can upload screenshot of your math problem and get step-by-step solution using OpenAI. The project was not my own idea. 

[https://github.com/dilawar/math-tutor-openai](https://github.com/dilawar/math-tutor-openai)

I used NextJS, Typescript and tRPC for this project — this stack was suggested. In the past, I have used JavaScript but do not have great understanding of it. I am yet to read a decent book such a https://eloquentjavascript.net/.

React was totally new to me. Though I am familiar with underlying concepts or reacting programming since I’ve used Vue before. I have no opinion about nextjs, it is popular among React people so I just used it. I still don’t know how I feel about React. The Vue documentation is certainly better than React/NextJS. 

https://trpc.io/ is certainly very interesting and I see its appeal. tRPC has the benefit of using the same language for both server and client side. At the time of writing his blog post, it didn’t have good support for uploading form-data (the form-data API is in beta which I didn’t try). Laravel + PHP8 is my go to stack for backend but I guess it is not popular with cool kids. PHP5 has a well deserved bad reputation which is no longer true for PHP8 but bad reputations are not easily lost.

The dependency management with `npm` is worse than Python’s ecosystem. I hope it gets better. Rust ecosystem and `cargo` has spoiled me so bad and sometimes I feel like why do I even bother with any other language!

Here is screenshot of the app. You’d need OpenAI key to run this app.

![](https://github-production-user-asset-6210df.s3.amazonaws.com/895681/387906092-f742c5e3-96e8-4121-8625-f13382bd40c2.png?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAVCODYLSA53PQK4ZA%2F20241124%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Date=20241124T085131Z&X-Amz-Expires=300&X-Amz-Signature=9955335e05a6c2e5b68f634fa136799c032e366ef869832ad6234c3c7e6ea120&X-Amz-SignedHeaders=host)