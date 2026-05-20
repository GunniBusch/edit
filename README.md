# edit

Tiny Unix editor launcher for `$EDIT_EDITOR`, `$VISUAL`, `$EDITOR`, or `vi`.

## CLI

```sh
cargo build --release
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
