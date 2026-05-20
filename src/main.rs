fn main() -> std::io::Result<()> {
    Err(edit::open(std::env::args_os().skip(1)))
}
