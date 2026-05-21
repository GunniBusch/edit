# edit

Tiny Unix editor launcher for `$EDIT_EDITOR`, `$VISUAL`, `$EDITOR`, or `vi`.

## Build

```sh
cargo build --release --bin edit
```

The binary is written to `target/release/edit`.

To check the library crate:

```sh
cargo build --release --lib
```

Cargo builds the normal Rust library artifact for use by Rust crates.

To prefer dynamic Rust linking for the binary:

```sh
RUSTFLAGS="-C prefer-dynamic" cargo build --release --bin edit
```

That requires the matching Rust toolchain libraries to be available at runtime.
A Rust `dylib` is not a stable C ABI; foreign-language callers need a separate
`cdylib` API.

## CLI

```sh
edit path/to/file.txt
edit file1.txt file2.txt
```

## Library

```rust
edit::open(None, ["file.txt"]);                  // uses VISUAL, EDITOR, vi
edit::open(Some("MY_APP_EDITOR"), ["file.txt"]); // app override first
edit::spawn(None, ["file.txt"])?;                // starts the editor
```

## License

BSD 3-Clause
