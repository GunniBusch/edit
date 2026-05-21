fn main() -> std::io::Result<()> {
    Err(edit::open(Some("EDIT_EDITOR"), std::env::args_os().skip(1)))
}
