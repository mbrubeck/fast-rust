# Configuring the Rust Compiler

Most Rust code depends on compiler optimization for good performance.  This is
true of many languages but it can be especially severe in Rust, which is
specifically designed to use safe high-level abstractions that can be
optimized into efficient low-level code.  **Optimized Rust code often runs 10
to 50 times faster than the same code compiled without optimization.**

Building a Rust project with optimization enabled often takes more than twice
as long as an unoptimized build.  Optimized code can also be harder to debug
in a debugger.  So the Rust toolchain makes it easy to use unoptimized builds
during development (when rapid compilation and easy debugging are important)
and optimized builds when measuring performance or deploying to production.

## Building optimized code

To build a Cargo project with optimization enabled, use the `--release` flag:

```sh
cargo build --release
```

Cargo will place the compiled output in the `target/release` directory, rather
than the `target/debug` directory used for non-release builds.

You can use the `--release` flag again to run an optimized binary:

```sh
cargo run --release
```

This automatically compiles the program with optimization enabled (if it isn’t
already compiled) and then runs it.

## Cargo profiles

Cargo uses different compiler options for different commands.  These options
are grouped into five *profiles*.

These commands compile **without** optimization by default:

```sh
cargo build # uses `[profile.dev]`
cargo test  # uses `[profile.test]`
cargo doc   # uses `[profile.doc]`
```

These commands compile **with** optimization by default:

```sh
cargo build --release # uses `[profile.release]`
cargo bench           # uses `[profile.bench]`
```

You can customize the compiler options for each command in the [profile]
sections of the Cargo manifest (`Cargo.toml`).

**Note:** When you build a project that includes several crates as
depenencies, Cargo only uses the `[profile.*]` sections from the manifest of
the top-level crate (the one where you are running the `cargo build` command).
When you build an application that depends on external libraries, the
application's `Cargo.toml` is in full control of the build settings.

### opt-level

This controls which types of optimization the compiler performs.

`opt-level = 0` disables optimization.  This is the default for non-release
builds and for the `test` profile.

`opt-level = 1` performs a limited amount of optimization, but disables
optimization passes that are most likely to interfere with debugging.  If
you want your development builds or unit tests to run faster, and don't mind
them taking a little longer to compile, it can be useful to bump the `dev` or
`test` profiles up to this level:

```toml
[profile.dev]
opt-level = 1

[profile.test]
opt-level = 1
```

`opt-level = 2` and `opt-level = 3` optimize the code for speed.  `3` is the
default for release builds, and optimizes the most aggressively for speed,
possibly at the cost of generating larger code and taking more time.

TODO: More detailed differences between `2` and `3`.  Does `3` always generate
faster code than `2`?

`opt-level = "s"` and `opt-level = "z"` optimize for code size.  `"z"` is more
aggressive at reducing the size of the compiled code, at the cost of
generating slower code.  These are currently [experimental options][gh35784]
available in nightly builds.  Their names might change in future versons.

[gh35784]: https://github.com/rust-lang/rust/issues/35784

### lto

Link-time optimization (LTO) is a feature that lets the optimizer run on the
entire program at once (including all its external crates), instead of just
one crate at a time.  This can produce better-optimized code, and is
especially good at reducing the size of the compiled program by eliminating
unused code.  However, it can take a very long time and use a huge amount of
memory during compilation, so it is disabled by default.

Set `lto = true` to enable LTO for Cargo profile.  For example, you can add
these lines to `Cargo.toml` to enable LTO for both release builds and
benchmarks:

```toml
[profile.release]
lto = true

[profile.bench]
lto = true
```

### codegen-units

TODO

### debug

TODO

### debug-assertions

TODO

### panic

TODO

## rustc flags

The Rust compiler has many options that don't have corresponding keys in the
`Cargo.toml` file.  When you are building a Cargo project, you can pass extra
options by setting the `RUSTFLAGS` environment variable or setting the
`build.rustflags` or `target.<triple>.rustflags` key in a
[.cargo/config](config) file.

Like the `[profile.*]` sections in the manifest, `.cargo/config` files from
dependencies are ignored.  Only the top-level crate's config file is used.

### target-cpu and target-feature

By default, Rust will build code that runs on a wide range of processors.
This means that it can't use all the instructions that are available on newer
processors.  In particular this affects the code generated by [vectorized]
loops.

If you are building and running a program on a single computer and don't
whether it is portable to computers with different processors, you can pass
`-C target-cpu=native` to tell the compiler to use all the processor features
available on the computer where it is currently running.  Add these lines to a
`.cargo/config` file in your project's directory or any of its parent
directories:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

If you want to use specific processor features, but not necessarily all the
features of your current computer, you can instead use the `-C
target-feature` option to add or remove features.  For example, add this to
`.cargo/config` to make 32-bit Linux builds use [SSE3] instructions:

```toml
[target.i686-unknown-linux-gnu]
rustflags = ["-C", "target-feature=+sse3"]
```

This will generate code that might not run on older processors like the
Pentium 4, which doesn't have support the SSE3 extension.

You can run these commands for more information about the `target-cpu` and
`target-feature` options:

```sh
rustc -C target-cpu=help
rustc -C target-feature=help
```

[profile]: http://doc.crates.io/manifest.html#the-profile-sections
[config]: http://doc.crates.io/config.html
[vectorized]: https://en.wikipedia.org/wiki/Automatic_vectorization
[SSE3]: https://en.wikipedia.org/wiki/SSE3
