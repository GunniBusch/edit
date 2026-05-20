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

/// Returns `VISUAL`, `EDITOR`, or `vi`.
pub fn editor() -> OsString {
    var("VISUAL")
        .or_else(|| var("EDITOR"))
        .unwrap_or_else(|| "vi".into())
}

/// Returns `env`, `VISUAL`, `EDITOR`, or `vi`.
pub fn editor_with(env: &str) -> OsString {
    var(env).unwrap_or_else(editor)
}

/// Builds an editor command using `env` as the app-specific override.
pub fn edit_cmd_with(env: &str) -> Command {
    Command::new(editor_with(env))
}

/// Builds an editor command.
pub fn edit_cmd() -> Command {
    Command::new(editor())
}

/// Replaces the current process with the editor selected by `env`.
///
/// Returns only if the editor could not be executed.
pub fn open_with<I, S>(env: &str, files: I) -> io::Error
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    edit_cmd_with(env).args(files).exec()
}

/// Replaces the current process with the editor.
///
/// Returns only if the editor could not be executed.
pub fn open<I, S>(files: I) -> io::Error
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    edit_cmd().args(files).exec()
}

/// Starts the editor selected by `env` and returns the child process.
pub fn spawn_with<I, S>(env: &str, files: I) -> io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    edit_cmd_with(env).args(files).spawn()
}

/// Starts the editor and returns the child process.
pub fn spawn<I, S>(files: I) -> io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    edit_cmd().args(files).spawn()
}
