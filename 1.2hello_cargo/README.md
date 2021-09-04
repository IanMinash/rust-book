# Hello, Cargo!

## Creating a Project

To create a project with Cargo run the following:

```
cargo new project_name
```

## Building and Running

To compile executables for your project while using Cargo, run the following command:

```
cargo build
```

By default, `cargo build` will build an unoptimized executable

To build and run your executable with 1 command, run the following command:

```
cargo run
```

Cargo only recompiles your project if the source code has changed.

It is also possible to check whether your project can be compiled by using the following command:

```
cargo check
```

### Specifying a Release Version

To build an optimized release version of your executable, you can use the `--release` flag:

```
cargo build --release
```

The same flag can be specified when using the `cargo run` command