use std::fmt;
use std::io::Read;
use std::path::Path;

const MAX_SECRET_BYTES: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecretSourceError;

impl fmt::Display for SecretSourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid secret configuration")
    }
}

impl std::error::Error for SecretSourceError {}

fn normalize_secret(mut bytes: Vec<u8>) -> Result<Vec<u8>, SecretSourceError> {
    if bytes.ends_with(b"\r\n") {
        bytes.truncate(bytes.len() - 2);
    } else if bytes.ends_with(b"\n") {
        bytes.truncate(bytes.len() - 1);
    }
    if bytes.is_empty() || bytes.len() > MAX_SECRET_BYTES || std::str::from_utf8(&bytes).is_err() {
        return Err(SecretSourceError);
    }
    Ok(bytes)
}

fn read_secret_file(path: &Path) -> Result<Vec<u8>, SecretSourceError> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x0002_0000); // O_NOFOLLOW
    }
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x0100); // O_NOFOLLOW
    }
    #[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
    if std::fs::symlink_metadata(path)
        .map_err(|_| SecretSourceError)?
        .file_type()
        .is_symlink()
    {
        return Err(SecretSourceError);
    }

    let mut file = options.open(path).map_err(|_| SecretSourceError)?;
    let metadata = file.metadata().map_err(|_| SecretSourceError)?;
    if !metadata.is_file() || metadata.len() > (MAX_SECRET_BYTES + 2) as u64 {
        return Err(SecretSourceError);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        if metadata.permissions().mode() & 0o077 != 0 || metadata.nlink() != 1 {
            return Err(SecretSourceError);
        }
    }

    let mut bytes = Vec::with_capacity((metadata.len() as usize).min(MAX_SECRET_BYTES + 2));
    (&mut file)
        .take((MAX_SECRET_BYTES + 3) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| SecretSourceError)?;
    normalize_secret(bytes)
}

pub fn load_optional_secret(
    value_environment_variable: &str,
    file_environment_variable: &str,
) -> Result<Option<Vec<u8>>, SecretSourceError> {
    let direct = std::env::var_os(value_environment_variable);
    let file = std::env::var_os(file_environment_variable);
    match (direct, file) {
        (None, None) => Ok(None),
        (Some(_), Some(_)) => Err(SecretSourceError),
        (Some(_), None) => std::env::var(value_environment_variable)
            .map(|value| value.into_bytes())
            .map_err(|_| SecretSourceError)
            .and_then(normalize_secret)
            .map(Some),
        (None, Some(_)) => {
            let path = std::env::var(file_environment_variable).map_err(|_| SecretSourceError)?;
            if path.is_empty() {
                return Err(SecretSourceError);
            }
            read_secret_file(Path::new(&path)).map(Some)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::load_optional_secret;
    use std::path::{Path, PathBuf};

    const VALUE_ENV: &str = "OHC_TEST_SECRET_VALUE";
    const FILE_ENV: &str = "OHC_TEST_SECRET_FILE";

    fn write_secret(contents: &[u8]) -> (tempfile::TempDir, PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("secret");
        std::fs::write(&path, contents).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        (directory, path)
    }

    fn from_file(path: &Path) -> Result<Option<Vec<u8>>, super::SecretSourceError> {
        temp_env::with_vars(
            [
                (VALUE_ENV, None::<&str>),
                (FILE_ENV, Some(path.to_str().unwrap())),
            ],
            || load_optional_secret(VALUE_ENV, FILE_ENV),
        )
    }

    #[test]
    fn direct_value_is_compatible_and_missing_is_optional() {
        temp_env::with_vars(
            [(VALUE_ENV, Some(" direct secret\t")), (FILE_ENV, None)],
            || {
                assert_eq!(
                    load_optional_secret(VALUE_ENV, FILE_ENV).unwrap(),
                    Some(b" direct secret\t".to_vec())
                );
            },
        );
        temp_env::with_vars(
            [(VALUE_ENV, None::<&str>), (FILE_ENV, None::<&str>)],
            || assert_eq!(load_optional_secret(VALUE_ENV, FILE_ENV).unwrap(), None),
        );
    }

    #[test]
    fn file_value_removes_only_one_conventional_terminal_newline() {
        for (contents, expected) in [
            (&b"file secret\n"[..], &b"file secret"[..]),
            (&b"file secret\r\n"[..], &b"file secret"[..]),
            (&b" file secret \n\n"[..], &b" file secret \n"[..]),
        ] {
            let (_directory, path) = write_secret(contents);
            assert_eq!(from_file(&path).unwrap(), Some(expected.to_vec()));
        }
    }

    #[test]
    fn direct_and_file_sources_are_mutually_exclusive() {
        let (_directory, path) = write_secret(b"file secret");
        temp_env::with_vars(
            [
                (VALUE_ENV, Some("direct secret")),
                (FILE_ENV, Some(path.to_str().unwrap())),
            ],
            || assert!(load_optional_secret(VALUE_ENV, FILE_ENV).is_err()),
        );
    }

    #[test]
    fn empty_oversized_and_non_utf8_values_are_rejected_generically() {
        let maximum_value = "x".repeat(4096);
        temp_env::with_vars(
            [(VALUE_ENV, Some(maximum_value.as_str())), (FILE_ENV, None)],
            || {
                assert_eq!(
                    load_optional_secret(VALUE_ENV, FILE_ENV).unwrap(),
                    Some(maximum_value.as_bytes().to_vec())
                );
            },
        );

        let mut maximum_file_value = vec![b'x'; 4096];
        maximum_file_value.extend_from_slice(b"\r\n");
        let (_directory, path) = write_secret(&maximum_file_value);
        assert_eq!(from_file(&path).unwrap(), Some(vec![b'x'; 4096]));

        for value in [String::new(), "x".repeat(4097), "\n".to_string()] {
            temp_env::with_vars(
                [(VALUE_ENV, Some(value.as_str())), (FILE_ENV, None)],
                || {
                    let error = load_optional_secret(VALUE_ENV, FILE_ENV).unwrap_err();
                    assert_eq!(error.to_string(), "invalid secret configuration");
                    assert!(!error.to_string().contains(VALUE_ENV));
                },
            );
        }

        for contents in [vec![], vec![b'x'; 4097], vec![0xff, 0xfe]] {
            let (_directory, path) = write_secret(&contents);
            assert!(from_file(&path).is_err());
        }
    }

    #[cfg(unix)]
    #[test]
    fn unix_rejects_non_utf8_environment_symlinks_hardlinks_and_unsafe_modes() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        use std::os::unix::fs::{PermissionsExt, symlink};

        temp_env::with_vars(
            [(VALUE_ENV, None::<&str>), (FILE_ENV, None::<&str>)],
            || {
                unsafe {
                    std::env::set_var(VALUE_ENV, OsString::from_vec(vec![0xff]));
                }
                assert!(load_optional_secret(VALUE_ENV, FILE_ENV).is_err());
                unsafe {
                    std::env::remove_var(VALUE_ENV);
                    std::env::set_var(FILE_ENV, OsString::from_vec(vec![0xff]));
                }
                assert!(load_optional_secret(VALUE_ENV, FILE_ENV).is_err());
                unsafe {
                    std::env::remove_var(FILE_ENV);
                }
            },
        );

        let (directory, path) = write_secret(b"secret");
        let symlink_path = directory.path().join("secret-link");
        symlink(&path, &symlink_path).unwrap();
        assert!(from_file(&symlink_path).is_err());

        let hardlink_path = directory.path().join("secret-hardlink");
        std::fs::hard_link(&path, &hardlink_path).unwrap();
        assert!(from_file(&path).is_err());
        std::fs::remove_file(hardlink_path).unwrap();

        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).unwrap();
        assert!(from_file(&path).is_err());
        assert!(from_file(directory.path()).is_err());
    }
}
