# Rust Standard Library

https://doc.rust-lang.org/std/

## Using modules

Use the `use` decleration to bring a module into scope.

## Crates
A collection of Rust source code files. Binary creates compile to produce an executable program. (Library crates)[https://crates.io/] contain code for other programs to use.

### Library crates

Adding library crates requires an update to your `cargo.toml` file.
```
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```

This can then be referenced in source files via `use rand;`, or `use rand::random;` in order to import a specific function.