# Benchmarking Rust Code

## General advice about benchmarking

TODO

## Simple time and memory measurement with `time`

If you have a simple program that performs a task and then exits, you can use
the `time` command (part of most Unix-style command-line environments) to
measure its overall time and CPU usage:

```
$ cargo build --release
$ time ./target/release/my_program

real    0m0.028s
user    0m0.024s
sys     0m0.000s
```

This tells me that my program ran for 28 ms.  While running, it used 24 ms of
CPU time in user mode, and 0 ms of CPU time in kernel mode ("sys").  The CPU
time can be less than the elapsed time if the program spends part of its time
sleeping, waiting for I/O, or otherwise blocked.  It can also be higher than
the elapsed time if the program uses multiple CPU cores in parallel.

Note that I ran the binary directly using its path inside the `target`
directory, instead of using the `cargo run` command.  This avoids measuring
resources spent by Cargo rather than my own program.

On some systems the `time` program installed at `/usr/bin/time` provides more
information than the command built in to your shell.  For example, on Debian
GNU/Linux you can do this to run the GNU `time` program:

```
$ sudo apt-get install time
$ /usr/bin/time ./target/release/my_program

0.00user 0.00system 0:00.00elapsed 88%CPU (0avgtext+0avgdata 6304maxresident)k
0inputs+32outputs (0major+1146minor)pagefaults 0swaps
```

In addition to the CPU time and elapsed time, this shows that the program's
peak memory usage ([resident set size][rss]) was 6304 KB, and also some
information about I/O and memory events.  See the [GNU time manual] for more
information.

(TODO: Instructions for Windows?)

## Benchmark tests

Rust has a built-in benchmarking system that can provide detailed and
information about the speed of arbitrary pieces of Rust code.  Read the
[Benchmark Tests][bench] chapter of the official Rust documentation to learn
how to use this feature.

### Compatibility with stable Rust

Benchmark tests are currently an unstable feature, available only on the
Rust nightly channel.  If you want your project to work with non-nightly
versions of Rust, there are two options:

1. Place your benchmark tests in a separate file or files in the `benches`
   subdirectory of your project.  These files will be compiled only when
   running the `cargo bench` command.  Users on the stable channel will get an
   error if they run this command, but they can still build and run the other
   code in your project.

   `benches/*.rs` files will be compiled as separate crates, and need to
   import your library using an `extern crate` line, just like [integration
   tests][testdir].  Because of this, they can only access your library's
   public items.

2. Add a dependency on the [rustc-test] crate, which is a copy of the unstable
   `test` crate modified to work with stable Rust.  This can be useful if you
   specifically need to measure performance of your code when built with a
   stable release of Rust.

### Benchmarks and inlining

TODO

[rss]: https://en.wikipedia.org/wiki/Resident_set_size
[man time]: http://man7.org/linux/man-pages/man1/time.1.html
[bench]: https://doc.rust-lang.org/book/benchmark-tests.html
[rustc-test]: https://crates.io/crates/rustc-test
[testdir]: https://doc.rust-lang.org/book/testing.html#the-tests-directory
