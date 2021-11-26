---
title: Put default arguments on function declaration
tags: [C++,note]
date: 2021-11-26
commentable: true
---

Default parameter values arguably are better suited on the declaration since that 
is the only thing that a caller always sees. 

You *can* have the argument on the definition as well but if you put it on
the definition then only those who see the definition would be able to use 
the default value ([https://stackoverflow.com/questions/1142209/what-if-the-default-parameter-value-is-defined-in-code-not-visible-at-the-call-s](https://stackoverflow.com/questions/1142209/what-if-the-default-parameter-value-is-defined-in-code-not-visible-at-the-call-s)). 

🚩 **You can not have default values on both declaration and definition.** 
I don't know why this is the case. My guess is that if default values are different
in one place then they will be issues. 

Following is a good idea.

```cpp
// Declaration
void foo(int x = 3.14, const char* y = "pi");

// Definition.
void foo(int x /*= 3.14 */, const char* y /*= "pi" */)
{
   ...
}
```
