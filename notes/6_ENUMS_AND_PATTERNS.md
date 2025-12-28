# Chapter 6: Enums and Pattern Matching

## Defining an Enum
- `enum`: a way of saying a value is one of a possible set of values
- syntax: `enum <name> { <value1>, <value2>, ... }`; usage: `<name>::<value1>`
- each enum value can additionally store data (like tuples and structs)
- eg. `enum Message { Quit, Move{x: i32, y: i32}, Write(String, i32) }`
  - `Quit` = unit struct, `Move` = struct, `Write` = tuple struct
- similar to structs, you can `impl` methods on enums (`impl <enum_name> { }`)
- `Option<T>` enum: `Some(T)` & `None`; to use, must always handle both variants

## The `match` Control Flow Construct
- `match`: compare a value and execute code based on which pattern matches
- syntax: `match <value> { <pattern1> => <code1>, ... }`
- can also add variables in patterns that will bind to the value's states
- matches are exhaustive, meaning the arms' patterns must cover all possibilites
- the `_` (or named variable) is a catch-all pattern

## Concise Control Flow with `if let` and `let else`
- `if let`: match pattern from variable (binds pattern for inner scope)
- executes `if` or `else` statement based on whether pattern matches or not
- syntax: `if let <pattern> = <variable> { ... } else { ... }`
- `let else`: match pattern from variable (binds pattern for outer scope)
- executes `else` statement if pattern doesn't match, binds pattern otherwise
- syntax: `let <pattern> = <variable> else { <return_statement> }`
