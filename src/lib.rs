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

/// Returns `env`, `VISUAL`, `EDITOR`, or `vi`.
pub fn editor(editor_env: Option<&str>) -> OsString {
    editor_env
        .and_then(var)
        .or_else(|| var("VISUAL"))
        .or_else(|| var("EDITOR"))
        .unwrap_or_else(|| "vi".into())
}

/// Builds an editor command.
/// Use the env parameter to use a custom editor env too
pub fn edit_cmd(editor_env: Option<&str>) -> Command {
    Command::new(editor(editor_env))
}

/// Replaces the current process with the editor selected by `env`.
///
/// Returns only if the editor could not be executed.
pub fn open<I, S>(editor_env: Option<&str>, files: I) -> io::Error
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    edit_cmd(editor_env).args(files).exec()
}

/// Starts the editor selected by `env` and returns the child process.
pub fn spawn<I, S>(editor_env: Option<&str>, files: I) -> io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    edit_cmd(editor_env).args(files).spawn()
}
