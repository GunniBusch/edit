//! Open files in the user's configured editor.

use std::{
    env,
    ffi::{OsStr, OsString},
    io,
    os::unix::process::CommandExt,
    process::{Child, Command},
};

fn var(name: &str) -> Option<OsString> {
    env::var_os(name).filter(|v| !v.is_empty())
}

/// Returns `EDIT_EDITOR`, `VISUAL`, `EDITOR`, or `vi`.
pub fn editor() -> OsString {
    var("EDIT_EDITOR")
        .or_else(|| var("VISUAL"))
        .or_else(|| var("EDITOR"))
        .unwrap_or_else(|| "vi".into())
}

/// Builds an editor command.
pub fn edit_cmd() -> Command {
    Command::new(self::editor())
}

/// Replaces the current process with the editor.
///
/// Returns only if the editor could not be executed.
pub fn open<I, S>(files: I) -> io::Error
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    self::edit_cmd().args(files).exec()
}

/// Starts the editor and returns the child process.
pub fn spawn<I, S>(files: I) -> io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    edit_cmd().args(files).spawn()
}
