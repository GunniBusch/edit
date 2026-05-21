use std::{env, ffi::OsString};

struct Vars([(&'static str, Option<OsString>); 4]);

impl Drop for Vars {
    fn drop(&mut self) {
        for (key, value) in &self.0 {
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
        }
    }
}

#[test]
fn picks_editor_in_order() {
    let _vars = Vars(["EDIT_EDITOR", "APP_EDITOR", "VISUAL", "EDITOR"].map(|key| (key, env::var_os(key))));
    for key in ["EDIT_EDITOR", "APP_EDITOR", "VISUAL", "EDITOR"] {
        env::remove_var(key);
    }

    assert_eq!(edit::editor(None), "vi");
    env::set_var("EDITOR", "ed");
    assert_eq!(edit::editor(None), "ed");
    env::set_var("VISUAL", "vim");
    assert_eq!(edit::editor(None), "vim");
    env::set_var("EDIT_EDITOR", "");
    assert_eq!(edit::editor(Some("EDIT_EDITOR")), "vim");
    env::set_var("EDIT_EDITOR", "zed");
    assert_eq!(edit::editor(None), "vim");
    assert_eq!(edit::editor(Some("EDIT_EDITOR")), "zed");
    env::set_var("APP_EDITOR", "nano");
    assert_eq!(edit::editor(Some("APP_EDITOR")), "nano");
}
