---
title: "Declarative languages and biological systems"
date: "2013-10-08"
---

A program written in declarative paradigm does not specify "how to do it", at best, it tells the computer "what to do". The spice-netlist is one such example, here we tell ngspice or some other spice (circuit simulator), what to compute (like .OP, .DC, .AC etc.). To be sure, this distinction is rather based on arguments which are nothing but "hand-waving", but too many people use these terms to ignore them. For example, FIFO (First In First Out) is an algorithm, but people often use it as if it is an object (Queue).

There is no thick-line which strictly separates languages according to these categories. Following example which computes factorial can be classified  "somewhat declarative" (written in functional programming language haskell). Here, we are not exactly saying how to "really" compute a factorial of a natural number.

\[code language="erlang"\] fact 0 = 0 fact 1 = 1 fact n = n \* fact (n -1) \[/code\]

Following will not be called declarative at all.

\[code language="C"\] unsigned long factorial(unsigned n) { if(n == 0) return 0; if(n == 1) return 1; unsigned long result n; for(unsigned i = n-1; i &amp;gt; 0; i--)     result \*= i; return result; } \[/code\]

(None of these snippets is tested).

I am not sure if to be "declarative", a program must lack  control flow. Or if there is a consistent theoretical category into which a program must fit.

Till people with neat theoretical mind fix the confusion for us, we can look at where and how to use declarative paradigm. There are many and quick search will reveal some. Let me share one on which currently I have to work (not enjoying it too much) i.e. modeling large scale neural networks.

A VLSI engineer is trained not to be frightened with numbers of components. But unlike CMOS devices, two neurons even when they belong to same class much like two humans, do not behave in the same way. Fortunately their behavior is not without a pattern, so attempts are being made to model them (as a real biological network) for a computer to answer few questions.

A lot of work has already been done in the area of neuronal modeling. People have build equivalent "models" of neurons; these models have been simulated with good success. These models are stored in some format and there are now many such formats. Efforts are on the way to standardize them.

It is agreed that some declarative paradigm will be most suitable for biologists for they might not prefer to learn a language to tell the computer "how to do it". They would rather prefer on "what to do". Engineers find it hard to learn a second language; many avoid going thoroughly the first one.

To store the models one often wants a standard format so that models can be reused by others. Astronomers have been using HDF5 format to store complex data (hierarchical patterns). Now there is XML technology all around. XML looks dirty to human eyes but a computer can easily read them. So why not describe "what to do" in XML. Imagine, rewriting a ngspice script in XML format and what they wish to do will be clear. The major difference however remains, two neurons belonging to same class do not behave like two capacitors or two transistors.  Much depends on how they are connected and who is doing what near them; and what is happening at different scales. A release of certain chemicals or presence of certain ions can not only change the values of component in equivalent circuit model, it can also change the circuit model itself. The speed of computation not only depends on how fast you can solve electrical circuits, but also how phenomenon at different timescales interact. Often simulators are optimized for certain type of computation and developing "jack of all trade" simulator would be an ambitious project.

Modeling phenomenon happening at different scales (time or space) is called multiscale-modeling, ubiquitous in biological systems. How changes in "one scale"changes the behaviour in other scale? This is answered/hinted at by experimentalists. They also prepare models to capture behaviour. And how to describe it is usually done by, well, programmers. Now the problem is how to describe a model for multiscale-modelings; if not using some data-base (XML)?

\--

Dilawar
