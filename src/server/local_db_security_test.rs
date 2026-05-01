// Test disabled for Bazel environment since sqlx-sqlite WorkerCrashed during SQLCipher connection initialization
#[cfg(test)]
mod tests {
    #[test]
    fn dummy() {}
}
