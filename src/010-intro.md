# Introduction

This book is about writing fast, efficient code in the [Rust] programming
language.

## Who this book is for

This book is for intermediate or “advanced beginner” Rust programmers who want
to start learning about how to optimize the performance of their code.

I will assume that you already know the basics of Rust programming. If you
don't, you should start [The Rust Programming Language][trpl] first.

Aside from that, no specific knowledge is required.  If you are are already
familiar with common optimization techniques in other programming languages,
you will learn how to apply many of the same techniques in Rust.  If not, I
hope this will be an excellent place to learn these techniques for the first
time.

## What is optimization?

Optimizing a program means making it do the same thing with fewer resources.
These “resources” can be a variety of things, including:

* Time
* CPU cycles
* Memory (RAM)
* Storage (disk space)
* Power (electricity)
* Network bandwidth

This book is mostly about making code faster (optimizing for time), but along
the way it will also discuss some of these other areas.  Optimizing a program
in one dimension often improves it in other dimensions too.  For example,
making the code or data smaller can save time as well as space: Smaller data
takes less time to transfer between disk, memory, and CPU; it is also more
likely to fit into the CPU's fast caches.

But sometimes, efficiency in one resource comes at the cost of inefficiency in
another.  For example, storing previously-computed results in a cache can make
a program faster, but it also requires more RAM or disk space.  Using multiple
threads running in parallel might make a program finish faster but also
consume more total CPU cycles (which could otherwise be used by other
processes running on the same system).

Because of these trade-offs, it's important to know *why* you are optimizing
your code.  Consuming fewer resources is usually not a goal for its own sake;
instead it is often a means to an end.  The real objective might be one of these:

* Latency: How long the program takes to respond to input or events. For
  example, a video game may need to render 60 frames per second, which means
  that each frame must be finished about 16 milliseconds after the previous
  one.  Or a web service might want to minimize the time that users spend
  waiting for its responses to their requests.  In these cases, the programmer
  may decide to consume all available CPU/memory/disk/power as long as it
  helps reduce latency.

* Capacity: How much hardware is required to run the program. If you are
  developing software for an embedded device which has limited battery,
  memory, and processor speed, you'll need to make sure that your program's
  requirements don't exceed the available hardware.  Or if you are running
  a network service that handles thousands of requests per second, reducing
  the CPU and memory used for each request might save you money, by allowing a
  smaller fleet of servers to handle the same number of requests.

* (TODO: others?)

It's important to know what you are really trying to optimize so you can
choose the right measuments and benchmarks to guide your work.

## What this book isn't

I called this book *Writing Fast Rust Programs* because it's for people who
are trying to do just that.  However, it can’t teach you everything you need to
reach that goal.  Mostly it will teach about the performance impact of
specific Rust language and library features, so you can avoid needless
overhead from using them the wrong way.  (*Not Writing Terribly Slow Rust
Programs* might be a more accurate but less catchy title.)

If you want to write the *most efficient program in the world* to solve a
given problem, you'll need more than just a solid understanding of your
language and libraries. You'll also need a deep understanding of the problem
domain, and of the software and hardware platform where your program runs.

For example, writing a high-performance 3D game may require knowledge of:

* Physics simulation
* AI algorithms
* GPU hardware and drivers
* Network protocols

…and many other topics that I don’t have the time or expertise to discuss in
this book. I hope you will find some useful tools here, and combine them with
ones you find elsewhere to form a complete toolbox.

[Rust]: https://www.rust-lang.org/
[trpl]: https://doc.rust-lang.org/book/
