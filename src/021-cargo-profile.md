## Cargo profiles

Cargo uses different compiler options for different commands.  These options
are grouped into five *profiles*.

These commands compile **without** optimization by default:

```sh
cargo build           # uses [profile.dev]
cargo test            # uses [profile.test]
cargo doc             # uses [profile.doc]
```

These commands compile **with** optimization by default:

```sh
cargo build --release # uses [profile.release]
cargo bench           # uses [profile.bench]
```

You can customize the compiler options for each command in the [profile]
sections of the Cargo manifest (`Cargo.toml`).

**Note:** When you build a project that includes several crates as
dependencies, Cargo only uses profile sections from the manifest of the
top-level crate (the one where you are running the `cargo build` command).
When you build an application that depends on external libraries, the
application’s `Cargo.toml` (and not its dependencies’) is in full control of
the build settings.

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

By default, rustc passes each crate code to the LLVM optimizer as a single
unit.  This is good for optimization, but it also means that the optimizer
only uses a single core for each crate it is compiling.

Increasing `codegen-units` from its default value of 1 tells rustc to split
the crate into multiple units and then performing LLVM code-generation and
optimization on all units in parallel.  This can reduce build times, but it
can also make the generated code perform worse.  Because of this, it's not
usually a good idea to do this in release builds, but it might be worthwhile
for optimized debug builds:

```toml
[profile.dev]
opt-level = 1
codegen-units = 4
```

### debug

This option tells the compiler to emit information that helps debuggers and
profilers produce more human-friendly output.  This increases the size of the
compiled program, so by default it is not enabled for release.  You might want
to enable it temporarily while debugging a problem that shows up only in
release builds, or if when running release builds or benchmarks in a profiler.

```toml
[profile.release]
debug = true

[profile.bench]
debug = true
```

### debug-assertions

TODO

### panic

TODO

[profile]: http://doc.crates.io/manifest.html#the-profile-sections
