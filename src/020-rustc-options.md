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
sections of the `Cargo.toml` file.

### opt-level

TODO

### opt-level

TODO

### codegen-units

TODO

### debug

TODO

### debug-assertions

TODO

### panic

TODO


[profile]: http://doc.crates.io/manifest.html#the-profile-sections
