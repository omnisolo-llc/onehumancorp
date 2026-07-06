use std::fs;
use std::io::{self, Write};
use std::path::Path;
use rand::Rng;
use rand::distributions::Alphanumeric;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(all(unix, test))]
use std::os::unix::fs::PermissionsExt;

/// WriteFileAtomic writes data to a file atomically by writing to a temporary file first
/// and then renaming it to the final path. This prevents file corruption on process crash.
pub fn write_file_atomic<P: AsRef<Path>>(filename: P, data: &[u8], _mode: u32) -> io::Result<()> {
    let filename = filename.as_ref();
    let dir = filename.parent().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?;
    
    fs::create_dir_all(dir)?;

    let base_name = filename.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?;
    let base_name_str = base_name.to_string_lossy();
    
    let random_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    
    let mut tmp_name = std::env::temp_dir();
    tmp_name.push("ohc-atomic-writes");
    let _ = fs::create_dir_all(&tmp_name);
    tmp_name.push(format!("{}.{}.tmp", base_name_str, random_suffix));
    
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    options.mode(_mode);

    let mut file = match options.open(&tmp_name) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

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
        if e.raw_os_error() == Some(18) { // EXDEV
            if let Err(e2) = fs::copy(&tmp_name, filename) {
                let _ = fs::remove_file(&tmp_name);
                return Err(e2);
            }
            let _ = fs::remove_file(&tmp_name);
            return Ok(());
        }
        let _ = fs::remove_file(&tmp_name); // Try to clean up
        return Err(e);
    }

    Ok(())
}

pub fn cleanup_stale_temp_files() {
    let tmp_dir = std::env::temp_dir();

    // Clean up .tmp files created by ohc-atomic-writes
    let mut atomic_tmp = tmp_dir.clone();
    atomic_tmp.push("ohc-atomic-writes");
    if let Ok(entries) = std::fs::read_dir(&atomic_tmp) {
        let now = std::time::SystemTime::now();
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if let Ok(modified) = meta.modified() {
                    if let Ok(duration) = now.duration_since(modified) {
                        if duration.as_secs() > 3600 {
                            let _ = std::fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }
    }

    // Clean up .tmp_py_*.py, *.tmp.rs, and general *.tmp files in /tmp created by agents
    if let Ok(entries) = std::fs::read_dir(&tmp_dir) {
        let now = std::time::SystemTime::now();
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let mut should_delete = false;

            if file_name.starts_with(".tmp_py_") && file_name.ends_with(".py") {
                should_delete = true;
            } else if file_name.ends_with(".tmp.rs") {
                should_delete = true;
            } else if file_name.ends_with(".tmp") {
                should_delete = true;
            } else if file_name.ends_with(".log") && file_name.starts_with("test_") {
                 should_delete = true;
            }

            if should_delete {
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        if let Ok(duration) = now.duration_since(modified) {
                            if duration.as_secs() > 3600 {
                                let _ = std::fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }

    // Clean up .tmp files created by agents/builtin/json_store.rs
    let ohc_runtime_dir = std::env::var("OHC_RUNTIME_DIR").unwrap_or_else(|_| ".ohc/runtime".to_string());
    let memory_dir = std::path::PathBuf::from(ohc_runtime_dir).join("memory");
    if let Ok(entries) = std::fs::read_dir(&memory_dir) {
        let now = std::time::SystemTime::now();
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "tmp" {
                    if let Ok(meta) = entry.metadata() {
                        if let Ok(modified) = meta.modified() {
                            if let Ok(duration) = now.duration_since(modified) {
                                if duration.as_secs() > 3600 {
                                    let _ = std::fs::remove_file(entry.path());
                                }
                            }
                        }
                    }
                }
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
        let filename = std::env::temp_dir().join(format!("test_atomic_write_{}.txt", random_suffix));
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
    fn test_cleanup_stale_temp_files() {
        let tmp_dir = std::env::temp_dir();
        let atomic_tmp = tmp_dir.join("ohc-atomic-writes");
        let _ = std::fs::create_dir_all(&atomic_tmp);

        let fresh_atomic = atomic_tmp.join("fresh.tmp");
        std::fs::write(&fresh_atomic, b"fresh").unwrap();

        let fresh_rs = tmp_dir.join("fresh.tmp.rs");
        std::fs::write(&fresh_rs, b"fresh_rs").unwrap();

        super::cleanup_stale_temp_files();

        assert!(fresh_atomic.exists(), "fresh atomic file should be kept");
        assert!(fresh_rs.exists(), "fresh rs file should be kept");

        // Clean up
        let _ = std::fs::remove_file(fresh_atomic);
        let _ = std::fs::remove_file(fresh_rs);
    }
}
