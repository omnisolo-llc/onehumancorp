//! Persist a standalone encryption key before allowing database admission.
//!
//! A successful return always contains the stored winner. Existing invalid
//! entries are errors and are never replaced with a new candidate.

use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const LOCK_TIMEOUT: Duration = Duration::from_secs(1);
const MAX_KEY_BYTES: u64 = 65_536;
static NEXT_TEMPORARY: AtomicU64 = AtomicU64::new(0);

pub(super) fn configured_key(
    value: Result<String, std::env::VarError>,
) -> io::Result<Option<String>> {
    match value {
        Ok(key) => Ok(Some(key)),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => Err(invalid(
            "Configured SQLite encryption key is not valid Unicode",
        )),
    }
}

#[cfg(all(test, unix))]
std::thread_local! {
    static FAIL_NEXT_PARENT_SYNC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub(super) fn resolve(path: &Path, candidate: impl FnOnce() -> String) -> io::Result<String> {
    let lock_deadline = Instant::now() + LOCK_TIMEOUT;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    check_parent(parent)?;
    {
        let _lock = acquire_lock(path, lock_deadline)?;
        if let Some(stored) = read_existing(path, parent)? {
            return Ok(stored);
        }
    }

    // Candidate generation can be slow. Recheck the winner after reacquiring
    // the lock; no caller may accept a just-published key before its sync ends.
    let candidate = candidate();
    if candidate.trim().is_empty() || candidate.len() as u64 > MAX_KEY_BYTES {
        return Err(invalid(
            "SQLite encryption key is empty or exceeds the supported size",
        ));
    }
    let _lock = acquire_lock(path, lock_deadline)?;
    if let Some(stored) = read_existing(path, parent)? {
        return Ok(stored);
    }
    let (temporary, mut file) = PendingKey::create(parent)?;
    file.write_all(candidate.as_bytes())?;
    file.sync_all()?;
    drop(file);
    match publish_without_replacement(&temporary.0, path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(error),
    }
    // Also synchronizes a concurrent winner, including recovery after an
    // earlier publication whose final sync could not be confirmed.
    read_existing(path, parent)?.ok_or_else(|| invalid("Published SQLite key is unavailable"))
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn check_parent(parent: &Path) -> io::Result<()> {
    let metadata = fs::symlink_metadata(parent)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(invalid("SQLite key directory must be an actual directory"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o022 != 0 {
            return Err(invalid(
                "SQLite key directory must not be writable by other users",
            ));
        }
    }
    Ok(())
}

fn secure_options(write: bool) -> io::Result<OpenOptions> {
    let mut options = OpenOptions::new();
    options.read(true).write(write);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
        #[cfg(target_os = "linux")]
        options.custom_flags(0x0002_0000); // O_NOFOLLOW
        #[cfg(target_os = "macos")]
        options.custom_flags(0x0000_0100); // O_NOFOLLOW
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Secure SQLite key files require a supported no-follow backend",
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.custom_flags(0x0020_0000); // FILE_FLAG_OPEN_REPARSE_POINT
    }
    Ok(options)
}

fn check_regular(file: &File) -> io::Result<()> {
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(invalid("SQLite key and lock entries must be regular files"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o777 != 0o600 {
            return Err(invalid(
                "SQLite key and lock files require owner-only read/write permissions",
            ));
        }
    }
    Ok(())
}

fn acquire_lock(path: &Path, deadline: Instant) -> io::Result<File> {
    let mut name = path
        .file_name()
        .ok_or_else(|| invalid("SQLite key path has no file name"))?
        .to_os_string();
    name.push(".lock");
    let lock_path = path.with_file_name(name);
    let file = secure_options(true)?
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    check_regular(&file)?;
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(file),
            Err(TryLockError::Error(error)) => return Err(error),
            Err(TryLockError::WouldBlock) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(TryLockError::WouldBlock) => {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "SQLite key publication lock is busy",
                ));
            }
        }
    }
}

fn read_existing(path: &Path, parent: &Path) -> io::Result<Option<String>> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
        Ok(metadata) if !metadata.is_file() || metadata.file_type().is_symlink() => {
            return Err(invalid("Existing SQLite key is not a regular file"));
        }
        Ok(_) => {}
    }
    let mut file = secure_options(true)?.open(path)?;
    check_regular(&file)?;
    let mut stored = String::new();
    (&mut file)
        .take(MAX_KEY_BYTES + 1)
        .read_to_string(&mut stored)?;
    if stored.len() as u64 > MAX_KEY_BYTES || stored.trim().is_empty() {
        return Err(invalid(
            "Existing SQLite key is empty or exceeds the supported size",
        ));
    }
    file.sync_all()?;
    sync_parent(parent)?;
    Ok(Some(stored.trim().to_owned()))
}

struct PendingKey(PathBuf);

impl PendingKey {
    fn create(parent: &Path) -> io::Result<(Self, File)> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_nanos();
        for _ in 0..32 {
            let sequence = NEXT_TEMPORARY.fetch_add(1, Ordering::Relaxed);
            // No key material appears in a file name. create_new prevents
            // collisions or pre-existing links from replacing any entry.
            let path = parent.join(format!(
                ".sqlite-key-{}-{timestamp}-{sequence}.pending",
                std::process::id()
            ));
            match secure_options(true)?.create_new(true).open(&path) {
                Ok(file) => return Ok((Self(path), file)),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "SQLite key temporary file collisions",
        ))
    }
}

impl Drop for PendingKey {
    fn drop(&mut self) {
        // Only this instance's temporary name is removed. The published key and
        // stable lock inode remain intact, including after a failed final sync.
        let _ = fs::remove_file(&self.0);
    }
}

#[cfg(unix)]
fn publish_without_replacement(temporary: &Path, destination: &Path) -> io::Result<()> {
    fs::hard_link(temporary, destination)
}

#[cfg(unix)]
fn sync_parent(parent: &Path) -> io::Result<()> {
    let directory = secure_options(false)?.open(parent)?;
    if !directory.metadata()?.is_dir() {
        return Err(invalid("SQLite key directory changed during publication"));
    }
    #[cfg(test)]
    if FAIL_NEXT_PARENT_SYNC.with(|failure| failure.replace(false)) {
        return Err(io::Error::other("Injected SQLite directory sync failure"));
    }
    directory.sync_all()
}

#[cfg(windows)]
fn publish_without_replacement(temporary: &Path, destination: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "MoveFileExW"]
        fn move_file_ex_w(existing: *const u16, new: *const u16, flags: u32) -> i32;
    }
    fn path_wide(path: &Path) -> io::Result<Vec<u16>> {
        let mut encoded: Vec<_> = path.as_os_str().encode_wide().collect();
        if encoded.contains(&0) {
            return Err(invalid("SQLite key path contains a null character"));
        }
        encoded.push(0);
        Ok(encoded)
    }
    let existing = path_wide(temporary)?;
    let new = path_wide(destination)?;
    // MOVEFILE_WRITE_THROUGH; deliberately exclude REPLACE_EXISTING and
    // COPY_ALLOWED. Both names belong to the same directory. Buffers are valid
    // null-terminated UTF-16 for the complete synchronous call.
    let result = unsafe { move_file_ex_w(existing.as_ptr(), new.as_ptr(), 0x8) };
    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn sync_parent(_parent: &Path) -> io::Result<()> {
    // New directory-entry persistence is provided by WRITE_THROUGH while the
    // stable lock remains held. read_existing additionally syncs the key file.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super as subject;
    use std::{
        fs,
        io::Write,
        path::PathBuf,
        sync::{
            Arc, Barrier,
            atomic::{AtomicU64, Ordering},
        },
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    const PUBLIC_A: &str = "public-disposable-fixture-key-a";
    const PUBLIC_B: &str = "public-disposable-fixture-key-b";

    #[cfg(unix)]
    #[test]
    fn invalid_unicode_environment_key_is_rejected_without_exposing_its_bytes() {
        use std::os::unix::ffi::OsStringExt;
        assert_eq!(
            subject::configured_key(Ok(PUBLIC_A.to_owned())).unwrap(),
            Some(PUBLIC_A.to_owned())
        );
        assert!(
            subject::configured_key(Err(std::env::VarError::NotPresent))
                .unwrap()
                .is_none()
        );
        let mut bytes = PUBLIC_A.as_bytes().to_vec();
        bytes.push(0xff);
        let invalid = std::env::VarError::NotUnicode(std::ffi::OsString::from_vec(bytes));
        let error = subject::configured_key(Err(invalid)).unwrap_err();
        let context = format!("{error:?} {error}");
        assert!(
            !context.contains(PUBLIC_A),
            "configured key bytes must never enter diagnostics"
        );
    }

    struct OwnedDirectory(PathBuf);
    impl OwnedDirectory {
        fn new() -> Self {
            let root = std::env::var_os("OHC_KEY_TEST_ROOT")
                .map(PathBuf::from)
                .unwrap_or_else(std::env::temp_dir);
            let path = root.join(format!(
                "ohc-key-fixture-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for OwnedDirectory {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
    fn owned_file(path: &std::path::Path, contents: &[u8]) {
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        options.open(path).unwrap().write_all(contents).unwrap();
    }

    #[test]
    fn missing_destination_parent_does_not_admit_an_unstored_key() {
        let directory = OwnedDirectory::new();
        let key = directory.0.join("missing").join("key");
        assert!(subject::resolve(&key, || PUBLIC_A.to_owned()).is_err());
        assert!(!key.exists());
    }

    #[test]
    fn an_existing_empty_key_is_rejected_without_replacement() {
        let directory = OwnedDirectory::new();
        let key = directory.0.join("key");
        owned_file(&key, b"");
        assert!(subject::resolve(&key, || PUBLIC_A.to_owned()).is_err());
        assert_eq!(fs::read(&key).unwrap(), b"");
    }

    #[test]
    fn an_existing_nonregular_key_is_rejected_without_replacement() {
        let directory = OwnedDirectory::new();
        let key = directory.0.join("key");
        fs::create_dir(&key).unwrap();
        assert!(subject::resolve(&key, || PUBLIC_A.to_owned()).is_err());
        assert!(key.is_dir());
    }

    #[cfg(unix)]
    #[test]
    fn an_existing_symlink_key_is_rejected_without_touching_its_target() {
        let directory = OwnedDirectory::new();
        let actual = directory.0.join("actual");
        let key = directory.0.join("key");
        owned_file(&actual, PUBLIC_A.as_bytes());
        std::os::unix::fs::symlink(&actual, &key).unwrap();
        assert!(subject::resolve(&key, || PUBLIC_B.to_owned()).is_err());
        assert_eq!(fs::read_to_string(&actual).unwrap(), PUBLIC_A);
        assert!(fs::symlink_metadata(&key).unwrap().file_type().is_symlink());
    }

    #[test]
    fn valid_existing_key_is_reused_without_modification() {
        let directory = OwnedDirectory::new();
        let key = directory.0.join("key");
        owned_file(&key, PUBLIC_A.as_bytes());
        let before = fs::metadata(&key).unwrap().modified().unwrap();
        assert_eq!(
            subject::resolve(&key, || panic!(
                "existing key must not generate a candidate"
            ))
            .unwrap(),
            PUBLIC_A
        );
        assert_eq!(fs::read_to_string(&key).unwrap(), PUBLIC_A);
        assert_eq!(fs::metadata(&key).unwrap().modified().unwrap(), before);
    }

    #[test]
    fn concurrent_creators_return_the_single_persisted_winner() {
        let directory = OwnedDirectory::new();
        let key = directory.0.join("key");
        let barrier = Arc::new(Barrier::new(2));
        let threads: Vec<_> = [PUBLIC_A, PUBLIC_B]
            .into_iter()
            .map(|candidate| {
                let key = key.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    subject::resolve(&key, || {
                        barrier.wait();
                        candidate.to_owned()
                    })
                })
            })
            .collect();
        let results: Vec<_> = threads
            .into_iter()
            .map(|thread| thread.join().unwrap().unwrap())
            .collect();
        let stored = fs::read_to_string(&key).unwrap();
        assert!([PUBLIC_A, PUBLIC_B].contains(&stored.as_str()));
        assert_eq!(results, [stored.clone(), stored]);
    }

    #[test]
    fn a_real_held_publication_lock_times_out_without_creating_a_key() {
        let directory = OwnedDirectory::new();
        let key = directory.0.join("key");
        let lock =
            subject::acquire_lock(&key, std::time::Instant::now() + subject::LOCK_TIMEOUT).unwrap();
        let start = std::time::Instant::now();
        let error = subject::resolve(&key, || {
            panic!("blocked admission must not generate a candidate")
        })
        .unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::TimedOut);
        assert!(start.elapsed() < std::time::Duration::from_secs(2));
        assert!(!key.exists());
        drop(lock);
        assert_eq!(
            subject::resolve(&key, || PUBLIC_A.to_owned()).unwrap(),
            PUBLIC_A
        );
    }

    #[cfg(unix)]
    #[test]
    fn unsafe_existing_key_permissions_are_rejected_without_chmod_or_clobber() {
        use std::os::unix::fs::PermissionsExt;
        let directory = OwnedDirectory::new();
        let key = directory.0.join("key");
        owned_file(&key, PUBLIC_A.as_bytes());
        fs::set_permissions(&key, fs::Permissions::from_mode(0o640)).unwrap();
        assert!(subject::resolve(&key, || PUBLIC_B.to_owned()).is_err());
        assert_eq!(fs::read_to_string(&key).unwrap(), PUBLIC_A);
        assert_eq!(
            fs::metadata(&key).unwrap().permissions().mode() & 0o777,
            0o640
        );
    }

    #[cfg(unix)]
    #[test]
    fn unconfirmed_final_sync_keeps_the_published_key_and_redacts_error_context() {
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                subject::FAIL_NEXT_PARENT_SYNC.with(|failure| failure.set(false));
            }
        }
        let directory = OwnedDirectory::new();
        let key = directory.0.join("key");
        let _reset = Reset;
        subject::FAIL_NEXT_PARENT_SYNC.with(|failure| failure.set(true));
        let error = subject::resolve(&key, || PUBLIC_A.to_owned()).unwrap_err();
        assert!(error.to_string().contains("sync failure"));
        let context = format!("{error:?} {error}");
        assert!(!context.contains(PUBLIC_A));
        assert!(!context.contains(PUBLIC_B));
        assert_eq!(fs::read_to_string(&key).unwrap(), PUBLIC_A);
        let mut names: Vec<_> = fs::read_dir(&directory.0)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        names.sort();
        assert_eq!(
            names,
            [
                std::ffi::OsString::from("key"),
                std::ffi::OsString::from("key.lock")
            ]
        );
        assert_eq!(
            subject::resolve(&key, || panic!(
                "an unconfirmed publication must reuse its persisted winner"
            ))
            .unwrap(),
            PUBLIC_A
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn actual_write_failure_does_not_admit_a_key() {
        if std::env::var_os("OHC_KEY_WRITE_FAILURE_CHILD").is_some() {
            let directory = OwnedDirectory::new();
            let key = directory.0.join("key");
            assert!(subject::resolve(&key, || PUBLIC_A.to_owned()).is_err());
            assert!(!key.exists());
            return;
        }
        let module = module_path!().split_once("::").unwrap().1;
        let test_name = format!("{module}::actual_write_failure_does_not_admit_a_key");
        for limit in [0, 8] {
            let output = std::process::Command::new("python3")
            .arg("-c")
            .arg("import os,resource,signal,sys; limit=int(sys.argv[1]); resource.setrlimit(resource.RLIMIT_FSIZE,(limit,limit)); signal.signal(signal.SIGXFSZ,signal.SIG_IGN); os.execv(sys.argv[2],sys.argv[2:])")
            .arg(limit.to_string())
            .arg(std::env::current_exe().unwrap())
            .args(["--exact", &test_name, "--nocapture"])
            .env("OHC_KEY_WRITE_FAILURE_CHILD", "1")
            .output().unwrap();
            assert!(
                output.status.success()
                    && String::from_utf8_lossy(&output.stdout)
                        .contains("test result: ok. 1 passed; 0 failed;"),
                "actual kernel write failure at limit {limit} must execute the one fallible key case: {} {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
