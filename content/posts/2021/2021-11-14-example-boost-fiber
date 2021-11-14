---
title: An Example of Boost Fiber
author: Dilawar Singh
categories: [concurrency]
tags: [c++, concurrency,boost]
---

A fiber is _just a thread implemented in user space_.

Fibers are easier to reason about and have advantages such as much cheaper
context switching.  Fibers are very well suited for handling concurrent IO
operations. In such situations a processor mostly wait for the data to become
available and threads usually have pretty big context switching cost. So
multiple fibers running in a single thread is an effective solution.

It is also much easier to reason about concurrency with fibers. Watch this
great talk by Nat Goldman on  [Youtube](https://youtu.be/e-NUmyBou8Q).

Here is a simple program I wrote to explore fibers. You can find the full
example here
https://github.com/dilawar/playground/blob/a5fba9f21ff121249c71bff75ee8964c1016aa3f/BOOST/fiber.cpp.

The program has two functions: `print_a` prints `a` and `print_b` prints `b`
and then launches a thread that prints `B` (in detached mode).

```cpp
void print_a()
{
    cout << "a";
    boost::this_fiber::yield();
}

void print_b()
{
    cout << "b";
    std::thread j([]() { printf("B"); });
    j.detach();
    boost::this_fiber::yield();
}
```

Following is the _main_ function. We created a shared variable `i` initialized 
to 0. We use this a global state. We create two `detach`ed fibers. First one 
keeps calling `print_a`  till `i < 20`. Similarly, the second one loops on `print_b` till `i < 20`.  Both increment `i` by 1. When `i = 20`, both fibers should be able to `join`. 

```cpp
int main()
{
    int i = 0;
    boost::fibers::fiber([&]() {
        do {
            print_a();
            i++;
        }
        while (i < 20);
    }).detach();

    boost::fibers::fiber([&]() {
        do {
            i++;
            print_b();
        } while (i < 20);
    }).detach();

    printf("X");
    return 0;
}
```

Let’s guess the output of this program. It is most likely to be the same as if
`std::thread`s were used instead of fiber.

`X` is printed first? **Yes**. Note that `detach()` is called on each fibers so
neither of their functions are called. They are put in the background. Control
passes to the fiber manager at `return 0;` when it asks the fibers to `join`.
In fact, you can put more computations after the `printf("X");` statement and
it would be computed before any fiber is called.

As soon as we try to return from the `main`, fiber manager is asked to `join`
the fibers. The first fiber *awakes*, `a` is printed and the fiber `yield`s the
control to the manager. Fiber manager then wakes up the second fiber (who was
waiting in the queue) that prints `b` and also launched a thread in the
background that prints `B`. We can not be sure if `B` will be printed
immediately after the `b` (it is a `std::thread`).  `print_b` yields the cotrol and
goes to sleep . The fiber manager wakes up first fiber again that calls
`print_a` again and `a` is printed and so on. Note that `i` is also incremented
every time either of the fibers are called.

When `i` hits 20, both fibers terminates and `joined` and the main function `return 0;`.

So we have `print_a` called 10 times and `print_b` is also called 10 times. In the output, we should have 10 `a`s, 10 `b`s and 10 `B`s. `B` may not strictly follow `b` but `b` must come after the `a`.

Here are few runs of the program. Note that the location of `B` is not deterministic.

- `XababBabBabBababBBabBabBabBabBB`
- `XababBabBabBabBabBabBabaBbBabBB`
- `XababBabBabBabBabBabBabBabBabBB`
- `XababBabBabBabBabBabBabBabBabBB`
- `XababBabBabBBababBabBabBabBabBB`
- `XababBabBabBabBabBabBabBabBabBB`
- `XababBabBabBababBBabBabBababBBB`
- `XababBabBabBababBBabBabBabBabBB`
- `XababBabBabBababBBabBabBabBabBB`
- `XababBabBabBabBabBababBBabBabBB`
- `XababBabBabBabBabBabBabBabBabBB`

## References

1.  A great talk by Nat on Boost fibers https://youtu.be/e-NUmyBou8Q
