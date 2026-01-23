---
title: Some Thoughts on “Peopleware at Startup” 250e579bff89804fbfeeeeb3aeeeaf76
updated: 2026-01-14 22:54:30Z
created: 2026-01-15 04:51:09Z
---

# Some Thoughts on “Peopleware at Startup”

Published On: August 15, 2025
Author: dilawar
Status: Published
Type: Article
Tags: article, opinion

How important are people to the success of your startup? I believe the answer to be somewhere in between of “very” and “what else is there”. And so startups go hunting for rock-star, or 10x programmers.

Do you really need to hire a few 10x engineers to succeed? Of course not! One of my favorite band “Indian Ocean” has average musicians. And Afghanistan cricket team has done remarkably well. I firmly believe that a programmer who is barely 10% better at day to day chores  is at least twice or thrice a productive as normal engineer over a course of a year! I believe the reason is the non-linearity called compounding.

Our brains are not hard-wired to appreciate non-linearity. Tell a child that reducing the width and length of a house by half, makes it four time cheaper, and see her reaction. If these square-effect feels counterintuitive sometimes, imagine appreciating compounding, which is exponential in long term. To make matter worse, it feels very linear in short term, so advantages are very obvious to young and uninitiated.

You may have felt it in school. A kid who puts 5-10% extra effort every day may become twice as accomplished at the end of the year. Folks who do well at competitive exams seem to understand this very well, though they articulate it as discipline (and hard work). A fund with 5% compounding interests doubles every 14 years, with 10% every 7 years, with 15% every 5 years. A mere 10% change in day to day activity can reduce the timelines by 10 years (and *vice versa*)!

I firmly believe that many day-to-day activities related to software development have strong compounding effect on its completion or quality, or both.

## Finding force-multipliers

![image.png](../../../_resources/image-12.png)

When I was in college, Prof. H. Narayanan told me once, “You are creative, but you should also be right!” I didn’t think much of it that time. When you like working with computers, you can’t stop appreciating how hard it is to get it “right”. 

Is it possible to get hard problems right in the first or second try? Eventually yes. I mean, anyone can make good progress on anything given “enough” time with consistent effort! But if we are a startup, we don't have the luxury of spending years. I think we can get it right in first or second attempt if you do the “planning”. You could break the hard problem down to simpler problems that your team and solve with minimal supervision. If you can’t, find someone who can!

Let me put it more concretely. Let's say I can solve a class `A` problem 50% of time, class `B` 90% of time, and class `C` almost always, i.e., 99% of times. What are my chances of solving a problem if I can break it into two problems of class `A`, or four class `B`, or eight class `C` problems? 

| Class of Subproblems | Chance of Solving One | # Problems | Chance Of Solving All |
| --- | --- | --- | --- |
| A | 50 | 2 | 25% |
| B | 75 | 4 | 31% |
| C | 99 | 8 | 92% |

You have two options. Find someone who can solve the master problem with 92% chance, or find someone who can break it into eight class `C` problems. I always seek out the latter type of people when I conduct interviews. People who can partition a problem into multiple simpler problems that can be delegated to others. These folks are force multiplier (positive compounding, that is)!

This leads to a corollary that 10x engineers are not necessary. I’ve not met one yet, let alone hiring one. There is an easier way. Search for normal devs who show signs of positive compounding. These folks are usually disciplined in some way, and excellent with tools. Add a force multiplier to the team, and you are done.

None of the musicians in the band Indian Ocean is what I’d call a 10x musician, but their work is of great quality and value. Why can't start up teams be like that?

## Managing Peopleware

![image.png](../../../_resources/image%201-1.png)

A lot has been written about managing teams. You hire talented and energetic people, and they start off very well. You think everything is great but suddenly after a year or so, things are lukewarm at best. What is going on?

In case of young hires, the reason can be pretty really simple. It not always the case that management is evil. When you start a new job (or a PhD program), there is a sense of anticipation. And when you finish a hard project, there is a sense of accomplishment. At these stages, one feels good and life is good. It's the middle years — after anticipation is over and before accomplishment — that are taxing. If you don’t want to spend time motivating your young hires then just don’t hire anyone under 30s. Though, I’d recommend hiring talented young folks but ensure that they leave for greener pasture in a year or two on good terms.

For experienced hires, perhaps the reason is equally simple. Everyone has a different set of values, and what you value most may not be on the top of their list. You may value product-market-fit and customer-satisfaction, or the next fund-raise above everything else, but the developer just wants to write Rust and clean up tech-debt rather than thinking about any of that! Can you figure out how to work with your peopleware without asking them to sort their values according to your list?

Small things also matter a lot. Programming is a form of writing, and writing is primarily a solitary activity. If someone is good at it, they must have enjoyed spending a considerable time alone learning the craft. Don’t disturb them every couple of hours by sending notifications and emails. Schedule your communication effectively. These minor irritant compounds negatively on a developer psyche, much like [**"Scar Tissues Make Relationships Wear Out" — John Ousterhout**](../../../A%20Notion%20Home/0.%20Public%20Website/Blog/Scar%20Tissues%20Make%20Relationships%20Wear%20Out%20—%20John%20Ou.md).

I know some very good programmers who are good at talking, and even documentation, but I haven’t seen one yet who loves being a social-butterfly all the time, and doesn’t hate context-switching. 

## Leadership ensures ‘Everyone Going In the Same Direction’

There is much more to leadership, but I am very much content with this view. It is effective enough for managing a project, if not the whole organization! Therefore, the primary job of team leadership is communicating to ensure that everybody going in the same direction.

While I love setting up processes and workflows, I do recognize the creativity comes from people and not from processes. So there must be some channels for expressing creativity. But beware! I think creativity always conflict with that boring thing called ‘getting things done’. Being creative is nice, but “creative thinking” or thinking outside the box always leads to a different path than what you’ve charted, else it won’t be called creative.

The hardest part for leadership is to say “no” to seemingly good ideas that could lead the startup astray. I don’t feel very comfortable saying no to people for ideas, so I just don’t encourage “creative” ideas in most of the meetings. Like a young Ph.D. student, startup will do well not to indulge into “creative” things. The benefits are imaginary if any, but the cost (time spent) is real! There is time for being creative when you’ve not charted your course.

Also, I am a big fan of incremental improvements. The important meetings must be well-structured and focused, and any attempt to steal the focus should meet with a friendly frown and discouragement.

There is another indispensable aspect of leadership which is required to overcome problems discussed in [A Group is Its Own Worst Enemy by Clay Shirky](../../../undefined). If you are in this situation, then good luck. You must act decisively or give the position to someone who can. Leadership becomes a bit like using a toilet: you either shit or get off the seat!