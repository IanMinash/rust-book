# Variables

By default, variables in rust are immutable. This makes it easier to reason through programs as one doesn't have to keep track of how or where a value changes therefore preventing bugs.

Variables are declared using the `let` keyword.

```rust
let x = 5;
println!("{}", x); // -> 5
x = 6; // Won't compile
```

Variables can be made mutable by using the `mut` keyword.

```rust
let mut x = 5;
println!("{}", x); // -> 5
x = 6;
println!("{}", x); // -> 6
```

## Constants

Unlike variables which can be made mutable, constants are always immutable. Constants are declared using the `const` keyword.
Constants must also be type annotated.

```rust
const ONE_MILLION: u32 = 1_000_000;
```

## Shadowing

Rust allows a new variable to be declared using a previous variable's name. This is known as _`shadowing`_.

```rust
let x = 5;
let x = x + 1;
```

Shadowing is especially useful when you want to convert a variable to a different data type.

```rust
let spaces = "   ";
let spaces = spaces.len();
```
