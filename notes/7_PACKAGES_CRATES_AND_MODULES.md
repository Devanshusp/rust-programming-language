# Chapter 7: Packages, Crates, and Modules

## Packages & Crates
- `crate`: smallest amount of code the Rust compiler considers at a time
- `binary crate`: programs you compile to an executable (all have `fn main`)
- `library crate`: define functionality to be shared (don't have `fn main`)
- `crate root`: source file that compiler starts from and makes root module
- `package`: bundle of one or more crates that provide a set of functionality
- can contain at most one `binary crate` and multiple `library crates`

## Control Scope & Privacy with Modules
- when compiling a crate, the compiler first looks in the crate root file
- root: `src/libs.rs` for a library crate and `src/main.rs` for a binary crate
- in the crate root file, you can declare new modules (`mod <name>`)
- new modules declared inline (` {...};`), `src/<name>.rs`, or `src/name/mod.rs`
- you can declare `submodules` in modules the same way are modules
- refer to module from anywhere in the same crate: `crate::<module(s)>::<item>`
- module code is private by default; declare with `pub mod` to make public
- `use` for item shortcuts; `use crate::<module>::<item>` -> `<item>` from then

## Paths for Referring to an Item in the Module Tree
- `absolute path` is a full path starting from a crate root; starts with `crate`
- `relative path` starts at current module; uses `self`, `super`, or identifier
- see [rust api guide](https://rust-lang.github.io/api-guidelines/about.html)
- structs are private by default; make them public using `pub`
- each struct field will remain private by default; make them public using `pub`
- enums are private by default; use `pub` to make (all its values) public

## Bringing Paths into Scope with the `use` Keyword
- if module is defined in different scope (or file), `use` can bring it in scope
- idiomatically, specify before fn name; this is to know where fn is from
- rename path using `as`: `use std::io::Result as IoResult` -> `IoResult<()>`
- re-export names by making them public using `pub use <path>`
- add external packages to `Cargo.toml`, then add `use` line to bring in scope
- can use nested path to clean `use` lists: `use std::{cmp::Ordering, io};`
- can use `self` in nested path: `use std:io::{self, Write};`
- can use `*` to bring all public items into scope: `use std::collections::*;`

## Seperating Moduls into Different Files
- extract modules into files outside crate roots: `src/lib.rs` & `src/main.rs`
- do this by creating `<parent>/<module_name>.rs`
- parent is `src/` if module declared in a crate root file
- otherwise parent is a directory named as the file it was declared in
- there is also a `<parent>/<module_name>/mod.rs` style (older, discouraged)
