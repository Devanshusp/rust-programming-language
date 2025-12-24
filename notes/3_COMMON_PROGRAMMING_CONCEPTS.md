# Chapter 3: Common Programming Concepts

## Variables & Mutability
- Variables created using `let`
- Variables are immutable by default
- Variables are mutable using `mut` keyword
- Variables can shadow: new variable with same name can use old value
- Variables naming convention: all_lower_with_underscore
- Constants created using `const`
- Constants are immutable (always)
- Constants require type annotation
- Constants naming convention: ALL_UPPER_WITH_UNDERSCORE

## Data Types
- Rust is statically typed (must know types of all variables at compile time)
- Scalar types represent a single value
- Signed integer created using: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Unsigned integer created using: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Float created using: `f32`, `f64`
- Boolean created using: `bool`
- Character created using: `char`
- Compound types group multiple values into one type
- Tuple created using: `tup: (<type>, ...)`
- Tuples are fixed length and can store multiple types
- Access tuple elements: `tuple.index`
- An empty tuple is a unit: `()`
- Array created using: `[<type>; <length>]` or `[<element>; <length>]`
- Arrays are fixed length and only store a single type
- Access array elements: `array[index]`

## Functions
- functions defined by `fn` keyword and snake_case name
- function parameters must include types: `(<param>: <type>)`
- statements evaluate and do not return a value: `let x = 5;`
- expressions evaluate and return a value: `x + 1` (note missing semi-colon)
- specify functions return values by adding this to declaration: `-> <type>`
- return values from a function using expressions
- functions without return expressions return a unit: `-> ()`

## Comments
- comments are ignored by the compiler
- to add a comment use double forward slashes: `// this is a comment`

## Control Flow
- `if <condition>` executes a block once if condition is true
- `else` executes a block once if the if-condition is false
- `else if <condition>` expression allows chaining multiple conditions
- `loop` allows execution of a block over and over again
- we can `continue` or `break` out of loops
- we can assign loops names: `'<name> loop:` (must start with single quote)
- specify loop to continue or break by name: `break '<name>`
- `while <condition>` loops until the condition is no longer true
- `for <element> in <compound>` is the safest way to loop over collections
