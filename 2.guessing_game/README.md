# Guessing Game

To import a library, use the `use` statement:

```rust
use std::io;
```

## Variables

Use the `let` keyword to declare new variables. 
```rust
let one = 1;
```

Rust variables are immutable by default, to declare a mutable variable we use the `mut` keyword:

```rust
let mut age = 15;
age += 1;
```

## Comments
A single line comment can be created using `//`
```rust
let lame = "line"; // A lame line's comment
```

## Placeholders

We can use `{}` in `println!` to print values from our programs:
```rust
let x = 1;
let y = 2;

println!("x: {}\ty: {}", x, y);
``` 

## Adding External Libraries

We can add external crates to our project by specifying them in the `[dependencies]` section of our `Cargo.toml` file.

```toml
[dependencies]
externalib = "0.1.2" # equivalent to ^0.1.2 
```

## Updating Dependencies

We can use `cargo` to update our dependencies by running the following:
```
cargo update
``` 

This will update to dependecies in the same series. For example, `0.1.2` can be updated to `0.1.6` but not `0.2.0`.

To update to a different series, you need to edit your `Cargo.toml` directly.

## Range Expressions
Range expressions can be specified using the `start..end`.
Rust ranges are range exclusive meaning `1..100` only contains `1,2,3,...,98,99

## Comparisons

You can use the `match` expression to compare to values `x` and `y` in the following way:
```rust
match x.cmp(&y) {
    Ordering::Less => println!("x is less than y"),
    Ordering::Equal => println!("x is equal to y"),
    Ordering::Greater => println!("x is greater than y")
}
```

## Converting Strings to Numbers

A `String` type can be converted to a number type by calling the `.parse()` method on it. Before a `String` is parsed into a number all occurrences of `\t` or `\n` should be removed by using the `.trim()` method on it.


## Looping

You can enter into a loop by using the `loop` keyword. 
```rust
loop {
    // Code here runs indefinitely
}
```

A loop can be exited by using the `break` keyword.