---
title: Natural Numbers d911c6c26690446d8f7ba593bf9e7a52
updated: 2026-01-14 22:58:28Z
created: 2026-01-15 04:51:09Z
---

# Natural Numbers

Published On: July 18, 2010
Author: dilawar
Status: Published
Type: Article
Tags: article

Using natural numbers is easy; we all learned how to do so in school. However, defining natural numbers and the operations we perform on them (e.g. multiplication or addition) is more challenging. Terrence Tao's book provides an insightful discussion of these topics; I recommend reading Chapters 2 and 3 [1]. Here we discuss one of the methods to define natural numbers.

### [**Defining Natural Numbers : Peano Axioms](http://en.wikipedia.org/wiki/Peano_axioms)**

These axioms are the standard way to define natural numbers (Can you build your own set of axioms? This will be a nice exercise. Please let me know). Let's first solve our first problem. To use natural numbers, we have to write them down. How to write down the natural numbers? We use standard figures of 0, 1, 2, 3, etc. to represent them, but we would not take this for granted. But to start, we need some raw materials. This raw material is the minimum required to build our structure, called natural numbers. For example, if we wish to build a wall, we need bricks. If we are bold enough, we can start from clay and make the bricks by ourselves. Well, I'd like to have superpowers so that I can make silica by myself. All needed were fundamental particles. One can again ask whether I can start from more fundamental. This is where our imagination restricts us.

Anyway, to build our structure of natural numbers, we start with two things. Let's assume that someone has given them to us, one is the number 0 (zero) and one is the increment operator `++` and we collect all the natural numbers in a set (a mathematical basket) of Natural numbers represented by $\mathbb{N}$.

**Operator `++`:** If we apply this operator to any of the natural numbers, we get the **next** natural number. So `0++` is our next natural number (`1` is a more concise way to write this number). The next in line will be `(0++)++` (or `2`) and `((0++)++)++` (`3`) and so on. Now we bring in these axioms known as Peano Axioms.

<aside>
💡 **Axiom 1.** *0 is a natural number.*

</aside>

No, you can not ask any question, you mortal earthling!

<aside>
💡 **Axiom 2.** *if n is a natural number then n++ is a natural number.*

</aside>

We start from 0 and apply this axiom once. We get 0++, also known as 1, in this big empty world. Apply this axiom again and we get (0++)++, also known as 2. Now, if someone asks you to prove that 5 is a natural number, then invoke axiom 1 and apply axiom 2 five times and do not forget to laugh at his face [wink, wink]. These two axioms seem to be enough to build natural numbers.

But the world is not filled with ordinary people only: there are mathematicians too, and these mathematicians do not like each other. Consider two mathematicians, Superman and Lax Luthor. Superman gave these two axioms and thought that he saved the day. But the next day, Lax Luthor came up with his evil argument. He argues (fortunately, anti-Superman Lax Luthor only argues) that as soon as he goes up to some number, say 7, the next time Superman invokes axiom 2, he'll wrap back to 0, i.e. 7++ = 0. What are you going to do about it, pretty face? These two axioms still hold, but our natural number set $\mathbb{N}$ will contain numbers from this series: 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2....

<aside>
💡 **Axiom 3 :** *0 is not the successor of any natural number, i.e. there is no number n such that n++ = 0.*

</aside>

Lax now argues, "Okay, fine! 7++ is not equal to 0. Let's say 7++ is now equal to 1 or 2 or 3, etc. This will not contradict any of Superman's axioms. If 7++ = 2, then my system is 0 1 2 3 4 5 6 7 2 3 4 5 6 7 2 3 4 5 6 ....

<aside>
💡 **Axiom 4:** *Different natural numbers have different successors; i.e., if n, m are two different natural numbers than n++ and m++ are also different natural numbers. EQUIVALENTLY if m++ = n++ then we must have m = n.*

</aside>

Now, what does Lax Luthor have up his sleeve? All natural numbers are distinct; 7++ cannot be equal to 2 because there is another number, 1++, such that 1++ = 2. According to this axiom, 7++ and 1++ must be different.

But Luthor isn't done yet. He is not considered one of the top-ranked supervillains for nothing. He now argues that 0.5, 1.5, 2.5, etc. are also natural numbers, introducing "rogue elements."

It is now Superman's primary duty to keep these rogue elements away from mathematics. One can argue that one can never get 0.5, 1.5, etc. if they start building natural numbers from 0 using these axioms. But how can Superman prove this? How does he know when 0.5 will never appear in $\mathbb{N}$? Superman can keep constructing natural numbers until the end of time, and Lax will keep saying, "You have not exhausted all possibilities. Keep it coming!"

Superman has proposed the *Lakshman Rekha* to keep rogue elements out of $\mathbb{N}$. He claims that it is impossible to obtain 0.5 in the set of natural numbers because it lies between 0 and 1 and any number between 0 and 1 can be produced. However, it is difficult for the Mathematical League (not the creepy Justice League) to quantify Superman's statement that "0.5 lies between 0 and 1". Is there any proof to back up this claim?

<aside>
💡 **Axiom 5:** (Principle of Mathematical Induction) Let $P(n)$ be any property pertaining to the natural number $n$. Suppose that $P(0)$ is true, and suppose that when $P(n)$ is true, $P(n+1)$ is also true. Then $P(n)$ is true for all natural numbers $n$.

</aside>

What does Superman mean when he says that "property" is hard to say at this point? For example, $P(n)$ might be "n is prime", "n is odd", or "n solves a given equation".

Now, $P(n)$ is such that $P(0)$ is true; and whenever $P(n)$ is true, $P(n++)$ is also true. Since $P(0)$ is true, $P(0++)$, or $P(1)$, is true. Since $P(1)$ is true, so is $P(1++)$, or $P(2)$, and so on. If this fails with $P(0.5)$, then $0.5$ is not a natural number. We can use a simple proof given in [2] to keep the rogue elements $0.5$, $1.5$, etc. out of $\mathbb{N}$.

***Proof:*** We will need integers without defining them. We borrow the definition of integers. Let's say P(n) = n "is not a half integer", i.e. an integer plus 0.5. Then P(0) is true. And if P(n) is true, then P(n++) is also true. Thus, this axiom asserts that P(n) is true for all natural numbers n, i.e. no natural number can be a half integer; and P(0.5) fails this test. [2]

According to Analysis I by Terence Tao (pp. 20), the axiom in question should be referred to as an "axiom schema" rather than an axiom, as it refers not only to variables, but also to properties. This axiom schema is a template for producing an infinite number of axioms, rather than being a single axiom in its own right.

### End notes

It is hard to believe that there are some people who do not have an idea of natural numbers. People use different names and figures to speak and write natural numbers. It is believed that most animals have some idea or sense of natural numbers. For example, a bird can be seen in distress if one of her eggs is stolen, indicating that she knows how to count. Language helps us to represent these numbers. However, there is a [study published in Science in August 16, 2004](http://www.nature.com/news/2004/040816/full/news040816-10.html) which contradicts this view. Apparently, their counting system consists of three quantities: one, few, and many. Go ahead and read this story published in Nature: [http://www.nature.com/news/2004/040816/full/news040816-10.html](http://www.nature.com/news/2004/040816/full/news040816-10.html).

## References

1. [**Analysis I**](http://terrytao.wordpress.com/books/analysis-i/), by [*Terrence Tao.*](http://terrytao.wordpress.com/)