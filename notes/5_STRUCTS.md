# Chapter 5: Using Structs to Structure Related Data

## Defining and Instantiating Structs
- `struct`: hold multiple related named values of various types
- syntax: `struct <name> { <var_name>: <type>, ... }`
- use dot notation to get specific values from structs: `<name>.<var_name>`
- `tuple struct`: hold multiple related unnamed values of various types
- syntax: `struct <name>(<type>, ...)`
- must name the type of tuple struct when destructuring (unlike regular tuples)
- `unit-like struct`: struct without fields (useful for traits without data)
- syntax: `struct <name>`

## An Example Program Using Structs
- implement Debug trait to struct by adding `#[derive(Debug)]` above struct def
- use `dbg!(<var>)` or `println!("{<var>:?}")` macros to print debug values
- adding a hash will pretty-print: `{<var>:#?}`

## Methods
- a method is a function, but defined in context of a struct (+ enum or trait)
- syntax: `impl <struct_name> { fn <fn_name>(<signature>) {...} }`
- signature: `self` (own), `&self` (borrow), or `&mut self` (borrow mutably)
- constructor is a method without self in signature (`<struct_name>::<fn_name>`)
