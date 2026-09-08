use std::ffi::OsStr;

use tokio::process::Command;

pub(crate) fn apply_isolated_environment<I, K, V>(command: &mut Command, explicit: I)
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    command.env_clear();
    for key in [
        "PATH",
        "LANG",
        "LC_ALL",
        "TMPDIR",
        "TEMP",
        "TMP",
        "SystemRoot",
        "WINDIR",
        "PATHEXT",
    ] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command.envs(explicit);
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, OnceLock};

    use tokio::process::Command;

    use super::apply_isolated_environment;

    fn environment_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[tokio::test]
    async fn ambient_parent_environment_is_not_inherited() {
        let _guard = environment_lock().lock().unwrap();
        // Rust 2024 makes process-wide environment mutation explicitly unsafe;
        // the lock keeps this test isolated from other environment-sensitive tests.
        unsafe {
            std::env::set_var("UNRELATED_DEPLOYMENT_SECRET", "CANARY-AMBIENT-7KQ9");
        }

        let mut command = Command::new("/bin/sh");
        command.args([
            "-c",
            "printf '%s:%s:%s' \"${UNRELATED_DEPLOYMENT_SECRET:+present}\" \"${OPENAI_API_KEY:+present}\" \"${PATH:+present}\"",
        ]);
        apply_isolated_environment(&mut command, [("OPENAI_API_KEY", "explicit-key")]);
        let output = command.output().await.unwrap();

        unsafe {
            std::env::remove_var("UNRELATED_DEPLOYMENT_SECRET");
        }
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            ":present:present"
        );
    }
}
