---
title: "In Python3, converting slice to range"
date: "2020-10-08"
tags: 
  - "python"
  - "slice-to-range"
---

If you want to override `[]` operator in Python, you may need to handle `slice` as well. For example in the following `__getindex__` method which implements `[]`, argument `key` could be a single `int`, for example, `a[1]` or `a[-3]`; or it could be a `slice` e.g., `a[1:4]` is a slice.

```python
def __getitem__(self, key): 
    x = [1,2,3,4,5,6]  # fixed data
    if isinstance(key, slice): 
        return [x[i] for i in slice2range(slice, len(x)]    
    return x[key]
```

Ideally, I\\'d have loved if `range(key)` would automatically turn/cast a `slice` to the equivalent `range`. See this [question on SO](https://stackoverflow.com/questions/13855288/turn-slice-into-range).

The simplest solution, IMO, is the following.

```python
def slice2range(slice, N): 
    return range(slice.start or 0, slice.stop or N, slice.step or 1) 
```

You can use it inline.

```python
def __getitem__(self, key)
    x = [1,2,3,4,5,6] 
    if isinstance(key, slice): 
        # return a list of values. 
        return [x[i] for i in range(key.start or 0, key.stop or len(x), key.step or 1)] 
    return x[key] # return a value 
```
