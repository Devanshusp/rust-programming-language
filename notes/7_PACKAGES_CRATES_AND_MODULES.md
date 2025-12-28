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
- refer to module from anywhere in the same crate: `crate::<module>::<item>`
- module code is private by default; declare with `pub mod` to make public
- `use` for item shortcuts; `use crate::<module>::<item>` -> `<item>` from then

## Paths for Referring to an Item in the Module Tree
-
