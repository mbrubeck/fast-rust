# Configuring the Rust Compiler

Most Rust code depends on compiler optimization for good performance.  This is
true of many languages but it can be especially severe in Rust, which is
specifically designed to use safe high-level abstractions that can be
optimized into efficient low-level code.  Optimized Rust code very often runs
10 to 50 times faster than the same code compiled without optimization.

The compiler's optimization passes can take a long time to run.  Building a
Rust project with optimization enabled often takes more than twice as long as
an unoptimized build.  So the Rust toolchain makes it easy to do unoptimized
builds during development (when fast build time is important) and optimized
builds only when for measuring performance or deploying to production.

## Cargo profiles

Cargo uses different compiler options for different commands.  You can
customize thes options the [profile] sections of the `Cargo.toml` file.  There
are five profiles.

These profiles have optimization disabled by default:

```sh
cargo build # uses the `dev` profile
cargo test  # uses the `test` profile
cargo doc   # uses the `doc` profile
```

These profiles have optimization enabled by default:

```sh
cargo build --release # uses the `release` profile
cargo bench           # uses the `bench` profile
```

### opt-level



[profile]: http://doc.crates.io/manifest.html#the-profile-sections
