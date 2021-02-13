---
title: "argmax in pure Python"
date: "2020-11-12"
categories: 
  - "notes"
tags: 
  - "argmax"
  - "python"
---

Python does not have an inbuilt `argmax` function. `numpy` does.

Here is one implementation.

```python
def argmax(ls : list) -> int:
    _m, _mi = -math.inf, 0   # requires `import math` at top
    for i, v in enumerate(ls):
        if v > _m:
            _m = v
            _mi = i
    return _mi
```

There is also a one-liner which I often use: `max(zip(ls, range(len(ls))))[1]` where `ls` is the input list. To my surprise, the one liner is slower than `argmax` defined above (Python3.9 on openSUSE-Tumbleweed/Intel).

```
In [1]: a = [1,2,3,9,5,6,3,2]                                                                                                                                     
In [2]: max(zip(a, range(len(a))))[1]                                                                                                                                                         
Out[2]: 3

In [4]: %timeit max(zip(a, range(len(a))))                                                                                                                        
1.34 µs ± 28.7 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)                                                                                                                                 

In [7]: %timeit argmax(a)                                                                                                                                         
837 ns ± 8.65 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)
```

On large list,

```
In [11]: a = [random.randint(0, 111111) for x in range(1000)]                                                                                                     

In [12]: %timeit argmax(a)                                                                                                                                        
67.7 µs ± 446 ns per loop (mean ± std. dev. of 7 runs, 10000 loops each)

In [13]: %timeit max(zip(a, range(len(a))))                                                                                                                       
86.1 µs ± 739 ns per loop (mean ± std. dev. of 7 runs, 10000 loops each)
```
