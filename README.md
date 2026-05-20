# edit

Tiny Unix editor launcher for `$EDIT_EDITOR`, `$VISUAL`, `$EDITOR`, or `vi`.

## Build

```sh
cargo build --release --bin edit
```

The binary is written to `target/release/edit`.

To also build the library artifacts:

```sh
cargo build --release --lib
```

This package emits both `libedit.rlib` and `libedit.dylib`.

To prefer dynamic Rust linking for the binary:

```sh
RUSTFLAGS="-C prefer-dynamic" cargo build --release --bin edit
```

That requires the matching Rust toolchain libraries to be available at runtime.

## CLI

```sh
edit path/to/file.txt
edit file1.txt file2.txt
```

## Library

```rust
edit::open(["file.txt"]);                       // uses VISUAL, EDITOR, vi
edit::open_with("MY_APP_EDITOR", ["file.txt"]); // app override first
edit::spawn(["file.txt"])?;                     // starts the editor
```

## License

BSD 3-Clause
