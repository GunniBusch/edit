fn main() -> std::io::Result<()> {
    Err(edit::open_with("EDIT_EDITOR", std::env::args_os().skip(1)))
}
