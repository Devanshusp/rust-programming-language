# Chapter 4: Understanding Ownership

## What is Ownership
- Others manage memory with garbage collection or explicit allocate/free
- Rust manages memory through ownership: rules program must follow to compile
- This reduces the overhead of garbage collection and potential memory errors
- Ownership Rules:
  1. Each value in Rust has an owner
  2. There can only be one owner at a time
  3. When the owner goes out of scope, the value is 'dropped'
- Most values on the stack 'cloned' by default since there is no overhead
- Values on the heap are 'moved' by default, since 'cloning' adds overhead
- `move`: change ownership of a value 
- `clone`: create a deep copy of a value without impacting old value / owner
- `drop`: clean memory for that value (auto-call when value out of scope)
- When assigning a value (from the heap) to another variable, it changes owners
- When passing a value (from the heap) to function, it changes owners
- When returning a value (from the heap), it changes owners
- When a value is out of scope, it is dropped automatically

## References & Borrowing
- When we don't want to change ownership of a value, we can `borrow` it
- Values borrowed using references (`&<var>` immutable; `&mut <var>` mutable)
- There can be multiple immutable references, but only one mutable reference
- We also can't have a mutable reference while we have an immutable reference
- Scope of references start on declaration and end on last use
- Reference Rules:
  1. At any time: can have one mutable reference or unlimited mutable references
  2. References must always be valid

## The Slice Type
- `slice`: borrowed view into contiguous memory
- syntax: `&s[a..b]` -> yields `&[T]` or `&str` (end index exclusive)
- stores a pointer and length
- enforces bounds safety and lifetime safety at compile time
- avoids copies; mutation controlled by borrow rules
