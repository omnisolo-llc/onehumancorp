use std::path::Path;

/// A configured replica ID takes precedence; otherwise persist one per installation.
pub(super) fn mesh_node_id(explicit: Option<&str>, directory: &Path) -> Result<String, String> {
    use std::io::{Read, Write};
    fn validated(value: &str) -> Result<String, String> {
        let value = value.trim();
        if value.is_empty()
            || value.len() > 128
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-.:".contains(&b))
        {
            return Err("mesh node identity must contain 1-128 letters, digits, dots, colons, underscores or hyphens".into());
        }
        Ok(value.to_owned())
    }
    if let Some(value) = explicit {
        return validated(value);
    }
    let path = directory.join("node-id");
    let read = || -> Result<String, String> {
        let mut value = String::new();
        std::fs::File::open(&path)
            .map_err(|e| format!("read mesh identity: {e}"))?
            .take(130)
            .read_to_string(&mut value)
            .map_err(|e| format!("read mesh identity: {e}"))?;
        validated(&value)
    };
    if path
        .try_exists()
        .map_err(|e| format!("inspect mesh identity: {e}"))?
    {
        return read();
    }
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder
        .create(directory)
        .map_err(|e| format!("create mesh identity directory: {e}"))?;
    let id = uuid::Uuid::new_v4().to_string();
    let mut temporary = tempfile::NamedTempFile::new_in(directory)
        .map_err(|e| format!("create mesh identity: {e}"))?;
    temporary
        .write_all(id.as_bytes())
        .map_err(|e| format!("write mesh identity: {e}"))?;
    temporary
        .as_file()
        .sync_all()
        .map_err(|e| format!("persist mesh identity: {e}"))?;
    match temporary.persist_noclobber(&path) {
        Ok(_) => Ok(id),
        Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => read(),
        Err(error) => Err(format!("publish mesh identity: {}", error.error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_survives_restart_and_is_distinct_between_installations() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let id = mesh_node_id(None, first.path()).unwrap();
        assert_eq!(id, mesh_node_id(None, first.path()).unwrap());
        assert_ne!(id, mesh_node_id(None, second.path()).unwrap());
    }
    #[test]
    fn concurrent_initialization_keeps_one_identity() {
        let dir = tempfile::tempdir().unwrap();
        let ids: Vec<_> = std::thread::scope(|scope| {
            let threads: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| mesh_node_id(None, dir.path()).unwrap()))
                .collect();
            threads.into_iter().map(|t| t.join().unwrap()).collect()
        });
        assert!(ids.iter().all(|id| id == &ids[0]));
    }
    #[test]
    fn explicit_identity_does_not_touch_local_storage() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("unused");
        assert_eq!(
            mesh_node_id(Some("replica-1"), &missing).unwrap(),
            "replica-1"
        );
        assert!(!missing.exists());
        assert!(mesh_node_id(Some("  "), &missing).is_err());
    }
    #[test]
    fn malformed_persisted_identity_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("node-id"), "").unwrap();
        assert!(mesh_node_id(None, dir.path()).is_err());
    }
}
