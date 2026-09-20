use rand::Rng;
use rand::distributions::Alphanumeric;
use std::fs;
use std::io::{self, Write};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(all(unix, test))]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// WriteFileAtomic writes data to a file atomically by writing to a temporary file first
/// and then renaming it to the final path. This prevents file corruption on process crash.
pub fn write_file_atomic<P: AsRef<Path>>(filename: P, data: &[u8], _mode: u32) -> io::Result<()> {
    let filename = filename.as_ref();
    let dir = filename
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    fs::create_dir_all(dir)?;

    let base_name = filename
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?;
    let base_name_str = base_name.to_string_lossy();

    let random_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    // Renaming is atomic only within one filesystem. Stage next to the
    // destination, never in a global temp directory with a copy fallback.
    let tmp_name = dir.join(format!(
        ".omnisolo-atomic-{base_name_str}.{random_suffix}.tmp"
    ));

    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    options.mode(_mode);

    let mut file = options.open(&tmp_name)?;

    if let Err(e) = file.write_all(data) {
        drop(file);
        let _ = fs::remove_file(&tmp_name);
        return Err(e);
    }

    if let Err(e) = file.sync_all() {
        drop(file);
        let _ = fs::remove_file(&tmp_name);
        return Err(e);
    }
    drop(file); // Close file

    if let Err(e) = fs::rename(&tmp_name, filename) {
        let _ = fs::remove_file(&tmp_name); // Try to clean up
        return Err(e);
    }

    Ok(())
}

pub fn cleanup_stale_temp_files() {
    let runtime =
        std::env::var("OMNISOLO_RUNTIME_DIR").unwrap_or_else(|_| ".omnisolo/runtime".to_string());
    cleanup_owned_temp_files(&std::env::temp_dir(), Path::new(&runtime));
}

fn cleanup_owned_temp_files(temp: &Path, runtime: &Path) {
    // Only application-owned namespaces are eligible. A *.tmp/*.log suffix in
    // the shared host temp directory is not evidence that this app owns it.
    for directory in [temp.join("omnisolo-atomic-writes"), runtime.join("memory")] {
        if !fs::symlink_metadata(&directory).is_ok_and(|meta| meta.is_dir()) {
            continue; // Never follow a substituted directory symlink.
        }
        let Ok(entries) = fs::read_dir(&directory) else {
            continue;
        };
        let now = std::time::SystemTime::now();
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|ext| ext == "tmp")
                && let Ok(meta) = fs::symlink_metadata(entry.path())
                && meta.is_file()
                && let Ok(modified) = meta.modified()
                && let Ok(age) = now.duration_since(modified)
                && age.as_secs() > 3600
            {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_write_file_atomic() {
        let random_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        let filename =
            std::env::temp_dir().join(format!("test_atomic_write_{}.txt", random_suffix));
        let data = b"hello world";
        let _mode = 0o644;

        write_file_atomic(&filename, data, _mode).unwrap();

        let mut file = fs::File::open(&filename).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();
        assert_eq!(content, data);

        #[cfg(unix)]
        {
            let metadata = fs::metadata(&filename).unwrap();
            let perm = metadata.permissions();
            assert_eq!(perm.mode() & 0o777, _mode);
        }

        fs::remove_file(&filename).unwrap();
    }

    #[test]
    fn failed_atomic_replace_preserves_destination_and_removes_staging() {
        let root = std::env::temp_dir().join(format!("ohc-atomic-{}", rand::random::<u64>()));
        fs::create_dir_all(root.join("destination")).unwrap();
        fs::write(root.join("destination/keep"), b"original").unwrap();
        assert!(write_file_atomic(root.join("destination"), b"replacement", 0o600).is_err());
        assert_eq!(
            fs::read(root.join("destination/keep")).unwrap(),
            b"original"
        );
        assert_eq!(
            fs::read_dir(&root).unwrap().count(),
            1,
            "staging file leaked"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cleanup_preserves_foreign_stale_files_and_only_removes_owned_stale_temps() {
        let root = std::env::temp_dir().join(format!("ohc-cleanup-{}", rand::random::<u64>()));
        fs::create_dir_all(root.join("omnisolo-atomic-writes")).unwrap();
        fs::create_dir_all(root.join("runtime/memory")).unwrap();
        let old = std::time::SystemTime::now() - std::time::Duration::from_secs(7200);
        let names = [
            "foreign.tmp",
            "test_foreign.log",
            "foreign.tmp.rs",
            ".tmp_py_foreign.py",
            "omnisolo-atomic-writes/owned.tmp",
            "runtime/memory/owned.tmp",
            "runtime/memory/keep.json",
        ];
        for name in names {
            let mut file = fs::File::create(root.join(name)).unwrap();
            file.write_all(b"preserve unless owned temporary").unwrap();
            file.set_modified(old).unwrap();
        }
        fs::write(root.join("runtime/memory/fresh.tmp"), b"fresh").unwrap();
        cleanup_owned_temp_files(&root, &root.join("runtime"));
        for name in &names[..4] {
            assert!(root.join(name).exists(), "deleted foreign file {name}");
        }
        assert!(!root.join(names[4]).exists());
        assert!(!root.join(names[5]).exists());
        assert!(root.join(names[6]).exists());
        assert!(root.join("runtime/memory/fresh.tmp").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn cleanup_does_not_follow_directory_or_file_symlinks() {
        use std::os::unix::fs::symlink;
        let root =
            std::env::temp_dir().join(format!("ohc-cleanup-links-{}", rand::random::<u64>()));
        fs::create_dir_all(root.join("foreign")).unwrap();
        fs::create_dir_all(root.join("runtime/memory")).unwrap();
        let file = fs::File::create(root.join("foreign/keep.tmp")).unwrap();
        file.set_modified(std::time::SystemTime::now() - std::time::Duration::from_secs(7200))
            .unwrap();
        symlink(root.join("foreign"), root.join("omnisolo-atomic-writes")).unwrap();
        symlink(
            root.join("foreign/keep.tmp"),
            root.join("runtime/memory/link.tmp"),
        )
        .unwrap();
        cleanup_owned_temp_files(&root, &root.join("runtime"));
        assert!(root.join("foreign/keep.tmp").exists());
        assert!(
            fs::symlink_metadata(root.join("runtime/memory/link.tmp"))
                .unwrap()
                .file_type()
                .is_symlink()
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn test_cleanup_stale_temp_files() {
        let tmp_dir = std::env::temp_dir().join(format!("ohc-fresh-{}", rand::random::<u64>()));
        let atomic_tmp = tmp_dir.join("omnisolo-atomic-writes");
        let _ = std::fs::create_dir_all(&atomic_tmp);

        let fresh_atomic = atomic_tmp.join("fresh.tmp");
        std::fs::write(&fresh_atomic, b"fresh").unwrap();

        let fresh_rs = tmp_dir.join("fresh.tmp.rs");
        std::fs::write(&fresh_rs, b"fresh_rs").unwrap();

        cleanup_owned_temp_files(&tmp_dir, &tmp_dir.join("runtime"));

        assert!(fresh_atomic.exists(), "fresh atomic file should be kept");
        assert!(fresh_rs.exists(), "fresh rs file should be kept");

        fs::remove_dir_all(tmp_dir).unwrap();
    }
}
