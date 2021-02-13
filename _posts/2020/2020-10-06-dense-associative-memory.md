---
title: "Dense Associative Memory"
date: "2020-10-06"
categories: 
  - "biological-systems"
tags: 
  - "dense-associative-memory"
  - "hopfield-network"
---

[John Hopfield](https://www.youtube.com/watch?v=DKyzcbNr8WE) in 1982 proposed a neat idea. It was a network that could store memories and recall them when presented with a partial cue. There are many tutorials out there on Hopfield network (my personal favourite is [https://neuronaldynamics.epfl.ch/online/Ch17.S1.html](https://www.google.com/url?q=https%3A%2F%2Fneuronaldynamics.epfl.ch%2Fonline%2FCh17.S1.html&sa=D&sntz=1&usg=AFQjCNGkguSyzf2wqjGnE43Vb2ZgK-bAFg), the [original paper](https://www.google.com/url?q=https%3A%2F%2Fwww.pnas.org%2Fcontent%2F79%2F8%2F2554&sa=D&sntz=1&usg=AFQjCNH9Hx14XDoV7ImQ5QRiGlapmqyQag) is a great read as well.).

These networks are highly studied. Despite their limitations and non-biological nature, they are still very popular among neuroscientits for their simplicity and tractability. Their storage capacity is barely enough: a network with N neurons stores roughly 0.14N memories (still linear though).

This note is about recent progress on these networks proposed by [Krotov and Hopfield in 2016](https://www.google.com/url?q=https%3A%2F%2Farxiv.org%2Fabs%2F1606.01164&sa=D&sntz=1&usg=AFQjCNH3UjDPbTBmt30tPHiAjmysi92lfw) which has shown great promise in practice e.g. [Hopfield Networks is All You Need](https://www.google.com/url?q=https%3A%2F%2Farxiv.org%2Fabs%2F2008.02217&sa=D&sntz=1&usg=AFQjCNFz8IpDC5_hxJpKkf-Be76e47Mb9g) (vis-à-vis [Attention is all you need](https://www.google.com/url?q=https%3A%2F%2Fpapers.nips.cc%2Fpaper%2F7181-attention-is-all-you-need.pdf&sa=D&sntz=1&usg=AFQjCNH56HZZSxCt5XFFNJ9vJ66EUsHXNA)).

There are two major feats achieved in this work.

- Dense networks can store many more memories than the number of neurons using rectified polynomials (ee note: a polynomial signal generator and a diode in series) in the update rule. Probably because higher order polynomials can easily tease apart two patterns which looks tightly correlated to the traditional update rule. These are [very non-biological](https://www.google.com/url?q=https%3A%2F%2Farxiv.org%2Fabs%2F2008.06996&sa=D&sntz=1&usg=AFQjCNFDp-7In65kJP6viqs9cBTiJct89A)!
- They pointed out a connection or a duality between recurrent networks of Hopfield type and feedforward networks. Specifically, they work out how the activation function of feedforward network is related to update rule in the Hopfield network.

**A sidenote**

Long time ago, a Russian psychologist wrote a book [The Mind of a Mnemonist](https://books.google.co.in/books/about/The_Mind_of_a_Mnemonist.html?id=HTsSszl2ogcC&redir_esc=y) about a person who couldn't forget anything. He could recall all the details from any meeting years ago almost flawlessly. The cost of this seemingly endless memory capacity was the inability to generalize or to see patterns. For him there is no difference between these two lists of numbers: \[1,2,3,4,5,6,7...,9\] and \[9,1,2,4,7,8..\]. If he was a deer, he would never be able to learn that "all tigers are dangerous"; only that particular tiger with that particular stripe patterns who attacked him on that particular day is dangerous!

The point being that there is a trade off between the ability to generalize (**features** learning) and ability to remember examples (**prototype** learning). Animals with higher cognitive facilities such as mice learn prototypes first, then they quickly learn the features. Flies, on the other hand, learn prototypes but can't generalize well or at all. Today's neural network, AFAIK, seems to take the reverse direction: learn features first, and when overtrained learns prototypes.

[https://github.com/dilawar/algorithms/tree/master/MemoryNetwork/DenseAssociativeNetworks](https://www.google.com/url?q=https%3A%2F%2Fgithub.com%2Fdilawar%2Falgorithms%2Ftree%2Fmaster%2FMemoryNetwork%2FDenseAssociativeNetworks&sa=D&sntz=1&usg=AFQjCNEYb598UlkM0LKhmaTDzNemPHN9fw) has a Python3 implementation for this paper for the XOR function.
