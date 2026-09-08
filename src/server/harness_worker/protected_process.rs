//! Protect the worker's upstream credentials from same-UID child processes.

#[cfg(target_os = "linux")]
pub(crate) fn protect_credentials() -> std::io::Result<()> {
    unsafe extern "C" {
        fn prctl(option: std::os::raw::c_int, ...) -> std::os::raw::c_int;
    }
    // PR_SET_DUMPABLE prevents same-UID children from reading /proc/<worker>/
    // environ or memory through ptrace. Worker containers also drop all caps.
    const PR_SET_DUMPABLE: std::os::raw::c_int = 4;
    // SAFETY: prctl takes integer arguments here and retains no pointers.
    if unsafe { prctl(PR_SET_DUMPABLE, 0_usize, 0_usize, 0_usize, 0_usize) } == -1 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn protect_credentials() -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "protected provider workers require the Linux process isolation contract",
    ))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn child_cannot_read_worker_environment_through_proc() {
        const CHILD: &str = "OMNISOLO_PROTECTED_PROCESS_TEST";
        if std::env::var_os(CHILD).is_some() {
            protect_credentials().unwrap();
            let probe = Command::new("python3")
                .args([
                    "-c",
                    "import os; open('/proc/%d/environ' % os.getppid(), 'rb').read()",
                ])
                .output()
                .unwrap();
            assert!(
                !probe.status.success(),
                "child read protected worker environment"
            );
            assert!(String::from_utf8_lossy(&probe.stderr).contains("PermissionError"));
            return;
        }
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "child_cannot_read_worker_environment_through_proc",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .env("OPENAI_API_KEY", "protected-parent-key-canary")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
