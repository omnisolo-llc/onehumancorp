//! Process-isolated ambient-environment tests. Compiled only into test binaries.
use std::future::Future;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const CHILD: &str = "OMNISOLO_AMBIENT_TEST_CHILD";
const CANARY_KEY: &str = "UNRELATED_DEPLOYMENT_SECRET";
const CANARY_VALUE: &str = "CANARY-AMBIENT-7KQ9";
// An empty --exact selection exits zero. Require a distinct completion receipt
// so a typo in a test name can never become a passing isolation test.
const CHECK_COMPLETED_EXIT: i32 = 86;

/// Run one exact test under a known parent environment without mutating the
/// multi-threaded test process. A local mutex cannot protect other modules or
/// libraries that read the process environment, and must not span async work.
pub fn with_ambient_canary(test_name: &str, check: impl Future<Output = ()>) {
    if std::env::var(CHILD).as_deref() == Ok(test_name) {
        assert_eq!(std::env::var(CANARY_KEY).as_deref(), Ok(CANARY_VALUE));
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("isolated test runtime")
            .block_on(async {
                tokio::time::timeout(Duration::from_secs(30), check)
                    .await
                    .expect("ambient-environment test exceeded its deadline");
            });
        std::process::exit(CHECK_COMPLETED_EXIT);
    }

    let mut command = Command::new(std::env::current_exe().expect("current test executable"));
    command
        .args(["--exact", test_name, "--nocapture"])
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit());
    for key in [
        "PATH",
        "LANG",
        "LC_ALL",
        "TMPDIR",
        "TMP",
        "TEMP",
        "SystemRoot",
        "SYSTEMROOT",
        "WINDIR",
        "PATHEXT",
        "LD_LIBRARY_PATH",
        "DYLD_LIBRARY_PATH",
    ] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    let mut child = command
        .env(CHILD, test_name)
        .env(CANARY_KEY, CANARY_VALUE)
        .spawn()
        .expect("launch isolated ambient-environment test");
    let deadline = Instant::now() + Duration::from_secs(45);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                assert_eq!(
                    status.code(),
                    Some(CHECK_COMPLETED_EXIT),
                    "isolated test {test_name} failed or selected no test: {status}"
                );
                return;
            }
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            result => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("isolated test {test_name} did not complete: {result:?}");
            }
        }
    }
}
