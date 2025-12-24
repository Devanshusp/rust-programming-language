# The Rust Programming Language

- the `main` function is always executed first in Rust
- use `cargo` to manage (build, run, check, & release) projects

## Cargo

Cargo is Rust's build system and package manager.

Cargo expects all source files to live inside the `src` directory. 
The top-level project directory is for the README, license, configs, etc.

Cargo Commands:
- create a new project using `cargo new <project name>`
- create a Cargo.toml in an existing project using `cargo init`
- build project: `cargo build` (executable in target/debug)
- build project (release): `cargo build --release` (exec. in target/release)
- build & run project: `cargo run`
- check if code can compile: `cargo check`

The Cargo.toml file has the following sections:
- [package]
  - project's configurations (name, rust version, edition)
- [dependencies]
  - list of project's dependencies 
  
The Cargo.lock file is generated on build and does the following:
- records exact versions of every dependency in your project
- ensures reproducible builds

## Common Commands

| Action | Command |
|--------|---------|
| rust version | `rustc --version`|
| cargo version | `cargo --version`|
| compile file | `rustc <file>`|
| create new project | `cargo new <project name>` |
| create Cargo.toml | `cargo init` |
| build project | `cargo build` |
| build project (release) | `cargo build --release` |
| build project & run | `cargo run` |
| check if code compiles | `cargo check` |
| update rust | `rustup update`|
| uninstall rust | `rustup self uninstall`|
