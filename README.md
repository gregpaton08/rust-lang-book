# rust-lang-book
Exercises from the [Rust book](https://doc.rust-lang.org/book/)

## Cargo

### Creating a Project

```bash
cargo new <project name>
# e.g.
cargo new my_new_project
cd my_new_project # navigate to new dir where project was created
```

This will create two files inside `my_new_project` with the following directory layout:

```bash
.
├── Cargo.toml
└── src
    └── main.rs
```

All source code should go in the `src` directory. The generated `main.rs` file contains a simple hello world program.

`Cargo.toml` will look something like this:

```toml
[package]
name = "my_new_project"
version = "0.1.0"
authors = ["Gregory Paton <myEmailAddress@internet.com>"]
edition = "2018"

[dependencies]
```

### Building and Running a Project

```bash
# create a debug build
cargo build
./target/debug/my_new_project
# build and run with a single command
cargo run

# create a release build
cargo build --release
./target/release/my_new_project
# build and run with a single command
cargo run --release
```

