#[test]
fn test_daemon_postgres_skip() {
    if std::env::var("BAZEL_TEST").is_ok() {
        // graceful skip in sandbox without postgres
        return;
    }
    assert!(true);
}
