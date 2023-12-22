- Why do you need constants in Rust if you have immutable variables?
Constants must have a declared type and their values must be known
at compile time.
Constants also can be declared at module or global scope.
```rust
const MAX_VALUE: u32 = 100;
```


So you can/should use constants when value is known at compile time and/or you need a widely accessibility.


- Rust is statically types language, what does it mean?
It means that Rust must know the types of all variables at compile time.


# Ownership
Data with unknown size at compile time or a size that might change must be stored on the heap instead.

- each value in Rust has an owner
- there can only be one owner at a time
- when the owner goes out of scope, the value will be dropped

