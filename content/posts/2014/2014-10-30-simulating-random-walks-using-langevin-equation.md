---
title: "Simulating Random Walks using Langevin Equation"
date: "2014-10-30"
categories: 
  - "algorithms"
  - "biological-systems"
tags: 
  - "langevin-equation"
  - "python"
  - "random-walk"
---

Random walks (Brownian motions), in addition to their theoretical potency (describes macro-scale behavior of gas starting with micro-scale description), also describes behavior of many processes in nature. A few of them; genetic network, protein expression caused by mRNA,  have been described/predicted well using stochasticity. Moreover they are ideal noise sources cause by thermal fluctuation and often found in natural processes. To get the solid foundation in this subject, see the classic "Stochatic Processes in Physics and Chemistry" by Van Kampen, and also "handbook of stochastic methods" by Gandiner. Random walks (in fact any stochastic process) can be described by Fokker Planck equation: it describes how probability density function evolves over time. An equivalent is Master Equation which are much easier to visualize and solve (using Gillespie algorithm, a variant of Markov method). Master equation can describe "almost" all of the chemistry. In fact, Einstein built his theory of Brownian motion by writing down a variant of Fokker Planck equation. After Einstein published his work, a Frenchmen Paul Langevian discovered the same theory using a totally different approach; which is "infinitely simpler" than Einstein approach. I'd highly recommend to read the paper which does not require more than intermediate mathematics.  [http://scitation.aip.org/content/aapt/journal/ajp/65/11/10.1119/1.18725](http://scitation.aip.org/content/aapt/journal/ajp/65/11/10.1119/1.18725) In this note, I'd present a Python recipe to solve Langevian equation to simulate random walk. The Langevian approach is computationally extremely cheap and works extremely well in practice. Of course there is a cost involve. Fokker-Planck gives you "exact" mean and variance (if you can solve it), you need to produce many Langevian trajectories to see mean and variance converging to fixed values. But for simulating biological and physical processes, you don't worry too much about overall mean and variance. One trajectory is good enough for introducing noise sources. Langevian equation looks something like the following. $latex dx = −f(x) dt+\\alpha g(x) \\sqrt{dt}$ where α is normally distributed with mean 0 and variance 1. Forget f(x) and g(x), the out-of-place thing about these equation is square root of dt on the right-hand side. This led to fractional calculus, and stochastic differential equations. For the sake of "web and coding", problem statement and python recipe which simulates this equation can be found [here](https://github.com/dilawar/Courses/tree/master/RandomnessInBiology_Monsoon_2014/Assignment04). 5 model trajectories of Random walk in 1D generated by this equation are attached with this note. Many others can be generated using the script solve.py. Mean is as usual; and standard deviation relates with diffusion coefficient. ![](https://groups.google.com/group/wncc_iitb/attach/f2f408b7329836e7/plot_1dt_0initv_5.png?part=0.1&authuser=0) ​ PS: File solve.py implements the assignment. To produce these trajectories run the following command: **     $ python solve.py 1** To get more about "Randomness in Biology" visit [http://courses.ncbs.res.in/mod/resource/view.php?id=374](http://courses.ncbs.res.in/mod/resource/view.php?id=374) (login as guest) NOTES:

1. Google for: Sriram Ramaswamy's Resonance article on Einstein's derivation. Nice readable article. Do check the Langevin original paper linked above.
2. It is instructive for those who are interested in Control theory to read about biological processes: "How cell control its size" and other macro-molecules inside it in such. Biology is unexplored gold-mine for Control theory. Also its worth thinking how noise is minimized in cellular processes.