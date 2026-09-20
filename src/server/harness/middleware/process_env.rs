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
    use tokio::process::Command;

    use super::apply_isolated_environment;

    #[test]
    fn ambient_parent_environment_is_not_inherited() {
        crate::ambient_test::with_ambient_canary(
            "middleware::process_env::tests::ambient_parent_environment_is_not_inherited",
            async {
                let mut command = Command::new("/bin/sh");
                command.args([
            "-c",
            "printf '%s:%s:%s' \"${UNRELATED_DEPLOYMENT_SECRET:+present}\" \"${OPENAI_API_KEY:+present}\" \"${PATH:+present}\"",
        ]);
                apply_isolated_environment(&mut command, [("OPENAI_API_KEY", "explicit-key")]);
                let output = command.output().await.unwrap();

                assert!(output.status.success());
                assert_eq!(
                    String::from_utf8(output.stdout).unwrap(),
                    ":present:present"
                );
            },
        );
    }
}
