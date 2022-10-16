# Cargo: Package Management for Rust
---

### What is Cargo?

Cargo is the build system and package manager for Rust. It allows you to build your Rust projects, download the Rust packages you depend on, and build those packages. Cargo is distributed with Rust itself, so getting started with Cargo is as easy as installing Rust from [the official page](https://www.rust-lang.org/downloads.html).

### Creating a project with Cargo

To create a new Rust project, we can run `cargo new`:

    $ cargo new hello_cargo
         Created binary (application) `hello_cargo` project


This will create a new directory called `hello_cargo` with the following contents:

    $ tree hello_cargo
    hello_cargo
    ├── Cargo.toml
    └── src
        └── main.rs

    1 directory, 2 files


The `Cargo.toml` file is the manifest of your project. It contains information about your project such as name, version, authors, and dependencies. The `src` directory contains your project source code. The `main.rs` file is the main source file of your project, which is the default name for a binary crate.

### Building a project with Cargo

To build a project with Cargo, we can run `cargo build`:

    $ cargo build
       Compiling hello_cargo v0.1.0 (file:///home/kevin/code/rust/hello_cargo)
        Finished debug [unoptimized + debuginfo] target(s) in 0.25 secs


This will compile your project and output the compiled binary to `target/debug/hello_cargo`. You can run the binary with `./target/debug/hello_cargo`:

    $ ./target/debug/hello_cargo
    Hello, world!


### Building for release

Cargo has support for three different build profiles: `dev`, `release`, and `test`. The default profile is `dev`, which is optimized for fast compilation and includes debug symbols. The `release` profile is optimized for fast execution and does not include debug symbols. The `test` profile is used when running tests.

To build your project with optimizations for fast execution, we can run `cargo build --release`:

    $ cargo build --release
       Compiling hello_cargo v0.1.0 (file:///home/kevin/code/rust/hello_cargo)
        Finished release [optimized] target(s) in 0.25 secs


This will compile your project with optimizations and output the compiled binary to `target/release/hello_cargo`. You can run the binary with `./target/release/hello_cargo`:


    $ ./target/release/hello_cargo
    Hello, world!


### Running a project with Cargo

To run a project with Cargo, we can run `cargo run`:

    $ cargo run
       Compiling hello_cargo v0.1.0 (file:///home/kevin/code/rust/hello_cargo)
        Finished debug [unoptimized + debuginfo] target(s) in 0.25 secs
         Running `target/debug/hello_cargo`
    Hello, world!


This will compile your project if it is not already compiled, and then run the compiled binary.