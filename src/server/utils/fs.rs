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
pub fn write_file_atomic<P: AsRef<Path>>(filename: P, data: &[u8], mode: u32) -> io::Result<()> {
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
    
    let tmp_name = dir.join(format!("{}.{}.tmp", base_name_str, random_suffix));
    
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    options.mode(mode);

    let mut file = options.open(&tmp_name)?;

    file.write_all(data)?;
    file.sync_all()?;
    drop(file); // Close file

    if let Err(e) = fs::rename(&tmp_name, filename) {
        let _ = fs::remove_file(&tmp_name); // Try to clean up
        return Err(e);
    }

    Ok(())
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
        let filename = format!("{}/test_atomic_write_{}.txt", std::env::temp_dir().to_string_lossy(), random_suffix);
        let data = b"hello world";
        let mode = 0o644;

        write_file_atomic(&filename, data, mode).unwrap();

        let mut file = fs::File::open(&filename).unwrap();
        let mut content = Vec::new();
        file.read_to_end(&mut content).unwrap();
        assert_eq!(content, data);

        #[cfg(unix)]
        {
            let metadata = fs::metadata(&filename).unwrap();
            let perm = metadata.permissions();
            assert_eq!(perm.mode() & 0o777, mode);
        }

        fs::remove_file(&filename).unwrap();
    }
}
