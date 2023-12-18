- Why do you need constants in Rust if you have immutable variables?
Constants must have a declared type and their values must be known
at compile time.
Constants also can be declared at module or global scope.
```rust
const MAX_VALUE: u32 = 100;
```


So you can/should use constants when value is known at compile time and/or you need a widely accessibility.