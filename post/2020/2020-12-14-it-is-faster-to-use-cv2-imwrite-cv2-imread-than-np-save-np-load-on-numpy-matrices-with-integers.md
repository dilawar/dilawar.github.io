---
title: "cv2.imwrite/cv2.imread are faster than np.save/np.load on numpy matrices with integers"
date: "2020-12-14"
categories: 
  - "benchmark"
  - "linux"
tags: 
  - "micro-benchmark"
  - "numpy"
  - "opencv"
  - "pyhton"
---

In one of the ongoing projects, I generate a lot of small matrices (~1000 entries). These are often written to the disk and read from it. I was trying to optimize this step since this step run in a REST api.

The standard way is `numpy.save` and `numpy.load` which works fine. My experience with `opencv` made me wonder if `cv2.imwrite` and `cv2.imread` can be used. The later saves the matrices to png file which works without any loss of data if you have 8 or 16 bit integer values in your matrices. I do.

Here are some benchmarks using `%timeit`.

## Only saving matrices

I generate a random matrix of 400 entries and save it to a png file using OpenCV and also using numpy. OpenCV is roughly 3.6 times faster.

```
In [8]: %timeit cv2.imwrite('/tmp/a.png', np.random.randint(0, 255, (20,20)))                                                                                     
142 µs ± 1.05 µs per loop (mean ± std. dev. of 7 runs, 10000 loops each)

In [10]: %timeit np.save('/tmp/a.npy', np.random.randint(0, 255, (20,20)))                                                                                        
517 µs ± 96.7 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

The trend continues (OpenCV is ever so slightly faster here) for matrices with 900 entries. I did not test with larger matrices.

```
In [14]: %timeit cv2.imwrite('/tmp/a.png', np.random.randint(0, 255, (30,30))); a = cv2.imread('/tmp/a.png')                                                      
215 µs ± 2.93 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [15]: %timeit np.save('/tmp/a.npy', np.random.randint(0, 255, (30,30))); np.load('/tmp/a.npy')                                                                 
791 µs ± 29.7 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

## Saving and reading matrices

Reading back into matrices is also faster with OpenCV. Following micro-benchmarks write and read a randomly generated matrix using OpenCV and numpy.

```
In [12]: %timeit cv2.imwrite('/tmp/a.png', np.random.randint(0, 255, (20,20))); a = cv2.imread('/tmp/a.png')                                                      
192 µs ± 2.1 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

In [13]: %timeit np.save('/tmp/a.npy', np.random.randint(0, 255, (20,20))); np.load('/tmp/a.npy')                                                                 
770 µs ± 23.9 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

I guess I'll be using OpenCV to save data whenever I have integer matrices.

I used Python3.6 on OpenSUSE-15.1 server.
