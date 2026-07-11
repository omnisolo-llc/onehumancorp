use ohc_builtin_agent_core::types::ToolError;
use std::path::{Component, Path, PathBuf};

fn path_error(action: &str, requested: &str, message: impl std::fmt::Display) -> ToolError {
    ToolError::LlmRecoverable(format!("{action}: path {requested:?}: {message}"))
}

fn relative_path(action: &str, requested: &str) -> Result<PathBuf, ToolError> {
    let mut relative = PathBuf::new();
    for component in Path::new(requested).components() {
        match component {
            Component::Normal(part) => relative.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(path_error(
                    action,
                    requested,
                    "absolute paths and parent traversal are not allowed",
                ));
            }
        }
    }
    if relative.as_os_str().is_empty() {
        return Err(path_error(action, requested, "path is empty"));
    }
    Ok(relative)
}

async fn canonical_root(action: &str, root: &Path) -> Result<PathBuf, ToolError> {
    tokio::fs::canonicalize(root)
        .await
        .map_err(|error| path_error(action, &root.display().to_string(), error))
}

fn require_within_root(
    action: &str,
    requested: &str,
    root: &Path,
    target: &Path,
) -> Result<(), ToolError> {
    if target.starts_with(root) {
        Ok(())
    } else {
        Err(path_error(action, requested, "path escapes workspace root"))
    }
}

/// Captures and canonicalizes the workspace root once at tool construction.
pub fn configured_root(working_dir: Option<PathBuf>) -> Result<PathBuf, ToolError> {
    let root = match working_dir {
        Some(root) => root,
        None => std::env::current_dir().map_err(|error| path_error("workspace", ".", error))?,
    };
    std::fs::canonicalize(&root)
        .map_err(|error| path_error("workspace", &root.display().to_string(), error))
}

/// Resolves an existing path and rejects traversal or symlink escapes.
pub async fn existing(root: &Path, requested: &str) -> Result<PathBuf, ToolError> {
    let relative = relative_path("read", requested)?;
    let root = canonical_root("read", root).await?;
    let target = tokio::fs::canonicalize(root.join(relative))
        .await
        .map_err(|error| path_error("read", requested, error))?;
    require_within_root("read", requested, &root, &target)?;
    Ok(target)
}

/// Resolves a write target through its nearest existing parent without allowing
/// absolute paths, parent traversal, or symlinked-parent escapes.
pub async fn for_write(root: &Path, requested: &str) -> Result<PathBuf, ToolError> {
    let relative = relative_path("write", requested)?;
    let file_name = relative
        .file_name()
        .ok_or_else(|| path_error("write", requested, "path has no file name"))?
        .to_owned();
    let root = canonical_root("write", root).await?;
    let requested_parent = relative.parent().unwrap_or_else(|| Path::new(""));
    let lexical_parent = root.join(requested_parent);
    let mut existing_parent = lexical_parent.as_path();

    loop {
        match tokio::fs::symlink_metadata(existing_parent).await {
            Ok(_) => break,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                existing_parent = existing_parent
                    .parent()
                    .ok_or_else(|| path_error("write", requested, "path has no existing parent"))?;
            }
            Err(error) => return Err(path_error("write", requested, error)),
        }
    }

    let canonical_parent = tokio::fs::canonicalize(existing_parent)
        .await
        .map_err(|error| path_error("write", requested, error))?;
    require_within_root("write", requested, &root, &canonical_parent)?;
    let remaining_parent = lexical_parent
        .strip_prefix(existing_parent)
        .map_err(|error| path_error("write", requested, error))?;

    Ok(canonical_parent.join(remaining_parent).join(file_name))
}

#[cfg(test)]
mod tests {
    use super::{existing, for_write};
    use tempfile::tempdir;

    #[tokio::test]
    async fn workspace_path_rejects_parent_and_absolute_paths() {
        let root = tempdir().unwrap();

        assert!(existing(root.path(), "../outside").await.is_err());
        assert!(for_write(root.path(), "../outside").await.is_err());
        assert!(existing(root.path(), "/etc/passwd").await.is_err());
        assert!(for_write(root.path(), "/tmp/outside").await.is_err());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn workspace_path_rejects_existing_symlink_to_outside() {
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        let outside_file = outside.path().join("secret.txt");
        tokio::fs::write(&outside_file, "secret").await.unwrap();
        std::os::unix::fs::symlink(&outside_file, root.path().join("escape.txt")).unwrap();

        assert!(existing(root.path(), "escape.txt").await.is_err());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn workspace_path_rejects_write_through_symlinked_parent() {
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        std::os::unix::fs::symlink(outside.path(), root.path().join("escape")).unwrap();

        assert!(for_write(root.path(), "escape/new.txt").await.is_err());
    }

    #[tokio::test]
    async fn workspace_path_accepts_normal_nested_paths() {
        let root = tempdir().unwrap();
        let nested = root.path().join("nested");
        tokio::fs::create_dir(&nested).await.unwrap();
        let file = nested.join("file.txt");
        tokio::fs::write(&file, "ok").await.unwrap();

        assert_eq!(
            existing(root.path(), "nested/file.txt").await.unwrap(),
            file
        );
        assert_eq!(
            for_write(root.path(), "nested/new.txt").await.unwrap(),
            nested.join("new.txt")
        );
    }
}
