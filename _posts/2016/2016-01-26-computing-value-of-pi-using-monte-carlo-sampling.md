---
title: "Computing value of pi using Monte-Carlo sampling"
date: "2016-01-26"
---

I used Haskell to do this. Code is [here](https://github.com/dilawar/Courses/blob/master/AdvancedComputationalMethods/Notes/monte_carlo_compute_pi.hs) I did not use standard `System.Random` library to generate random number but `mwc-random` package. I can't say if speed is better or worse. I wanted to see how fast we converge to the value of `pi`. On x-axis, we have number of samples (N), and for each N, we compute the value of pi 50 times. This way we can talk about mean and variance in computed pi (which we plot on y-axis). It seems that for N >= 1000, we get a reasonable approximation of pi with low variance. For a 2-D sphere (aka circle: $latex x\_1^2 + x\_2 ^ 2 = 1 $ ), The convergence is following. ![monte_carlo_pi](images/monte_carlo_pi.png) For a 3-D sphere, (that is $latex x\_1 ^2 + x\_2 ^2 + x\_3 ^3 = 1$), I get the following: ![four_d_sphere.png](images/four_d_sphere.png) Now I am bit more ambitious, I use a 10-D sphere: As you can see, we converge to a a reasonable value of pi with low variance but convergence is not improving monotonically as in previous cases. The mean starts from a very low value and get to a respectable value only after N > 1000 (in low-dimensions cases we were swinging around 3.1415). The answer: we need to sample minimum number of points from hyper-cube to get some reasonable estimate. ![ten_d_sphere_2](images/ten_d_sphere_2.png) How variance decreases with N (when N is significantly large, 100 repeats of each computation)? Following plots starts from 2D and goes onto 10 dimensions. ![data_2.csv](images/data_2-csv.png)![data_3.csv](images/data_3-csv.png)![data_4.csv](images/data_4-csv.png)![data_5.csv](images/data_5-csv.png)![data_6.csv](images/data_6-csv.png)![data_7.csv](images/data_7-csv.png)![data_8.csv](images/data_8-csv.png)![data_9.csv](images/data_9-csv.png)![data_10.csv](images/data_10-csv.png)![data_11.csv](images/data_11-csv.png)![data_12.csv](images/data_12-csv.png)