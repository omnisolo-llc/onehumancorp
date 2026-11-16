use flate2::read::GzDecoder;
use omnisolo_builtin_agent_core::types::ToolError;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

const MAX_ARCHIVE_BYTES: usize = 64 * 1024 * 1024;
const DOWNLOAD_LOCK_COUNT: usize = 2;

static INSTALL_LOCKS: once_cell::sync::Lazy<[Arc<Mutex<()>>; DOWNLOAD_LOCK_COUNT]> =
    once_cell::sync::Lazy::new(|| [Arc::new(Mutex::new(())), Arc::new(Mutex::new(()))]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManagedBinaryKind {
    AgentBrowser,
    AgentDesktop,
}

#[derive(Clone, Debug)]
pub struct ArchiveSpec {
    pub kind: ManagedBinaryKind,
    pub version: &'static str,
    pub target_key: &'static str,
    pub url: &'static str,
    pub sha256: &'static str,
    /// Mapping from exact archive member path to installed filename inside the target directory.
    pub allowed_entries: &'static [(&'static str, &'static str)],
    pub primary_executable: &'static str,
}

impl ManagedBinaryKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::AgentBrowser => "agent-browser",
            Self::AgentDesktop => "agent-desktop",
        }
    }

    pub fn bin_env_var(self) -> &'static str {
        match self {
            Self::AgentBrowser => "OMNISOLO_AGENT_BROWSER_BIN",
            Self::AgentDesktop => "OMNISOLO_AGENT_DESKTOP_BIN",
        }
    }

    pub fn archive_env_var(self) -> &'static str {
        match self {
            Self::AgentBrowser => "OMNISOLO_AGENT_BROWSER_ARCHIVE",
            Self::AgentDesktop => "OMNISOLO_AGENT_DESKTOP_ARCHIVE",
        }
    }

    fn lock_index(self) -> usize {
        match self {
            Self::AgentBrowser => 0,
            Self::AgentDesktop => 1,
        }
    }

    fn license_notice(self) -> &'static str {
        match self {
            Self::AgentBrowser => {
                "agent-browser v0.43.0\nCopyright Vercel Labs\nLicense: Apache-2.0\nhttps://github.com/vercel-labs/agent-browser\n"
            }
            Self::AgentDesktop => {
                "agent-desktop v1.3.1\nCopyright Vercel Labs\nLicense: Apache-2.0\nhttps://github.com/vercel-labs/agent-desktop\n"
            }
        }
    }
}

pub fn current_host_target_key() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Some("linux-x64"),
        ("linux", "aarch64") => Some("linux-arm64"),
        ("macos", "aarch64") => Some("darwin-arm64"),
        ("macos", "x86_64") => Some("darwin-x64"),
        ("windows", "x86_64") => Some("win32-x64"),
        ("windows", "aarch64") => Some("win32-arm64"),
        _ => None,
    }
}

pub fn archive_spec_for_target(kind: ManagedBinaryKind, target_key: &str) -> Option<ArchiveSpec> {
    match (kind, target_key) {
        // Pinned agent-browser v0.43.0 registry tarballs (Apache-2.0)
        (ManagedBinaryKind::AgentBrowser, "linux-x64") => Some(ArchiveSpec {
            kind,
            version: "0.43.0",
            target_key: "linux-x64",
            url: "https://registry.npmjs.org/@agent-browser/cli-linux-x64-gnu/-/cli-linux-x64-gnu-0.43.0.tgz",
            sha256: "aa00b14ee8294edfeb1bc01bc6418cf4b7e50d75b046a9ab497c10384207383f",
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        }),
        (ManagedBinaryKind::AgentBrowser, "linux-arm64") => Some(ArchiveSpec {
            kind,
            version: "0.43.0",
            target_key: "linux-arm64",
            url: "https://registry.npmjs.org/@agent-browser/cli-linux-arm64-gnu/-/cli-linux-arm64-gnu-0.43.0.tgz",
            sha256: "465823648aef4a94d20bc2ae91beffdef8f68d6750a795dddbc2d13928209435",
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        }),
        (ManagedBinaryKind::AgentBrowser, "darwin-arm64") => Some(ArchiveSpec {
            kind,
            version: "0.43.0",
            target_key: "darwin-arm64",
            url: "https://registry.npmjs.org/@agent-browser/cli-darwin-arm64/-/cli-darwin-arm64-0.43.0.tgz",
            sha256: "c8b9457c850956a40dc5a4895505fbf3f222e31b2ad3eec9ce6b43bd04842fc2",
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        }),
        (ManagedBinaryKind::AgentBrowser, "darwin-x64") => Some(ArchiveSpec {
            kind,
            version: "0.43.0",
            target_key: "darwin-x64",
            url: "https://registry.npmjs.org/@agent-browser/cli-darwin-x64/-/cli-darwin-x64-0.43.0.tgz",
            sha256: "0ff552ff465b35c121dd208103b165d1d30db470b2b201e58e9a7b2fc78928d8",
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        }),
        (ManagedBinaryKind::AgentBrowser, "win32-x64" | "win32-arm64") => Some(ArchiveSpec {
            kind,
            version: "0.43.0",
            target_key: "win32-x64",
            url: "https://registry.npmjs.org/@agent-browser/cli-win32-x64-msvc/-/cli-win32-x64-msvc-0.43.0.tgz",
            sha256: "6a5a02516281d619d05534d8817989481c405d8c242ea94029737ed1c703e5b3",
            allowed_entries: &[("package/bin/agent-browser.exe", "agent-browser.exe")],
            primary_executable: "agent-browser.exe",
        }),

        // Pinned agent-desktop v1.3.1 release archives (Apache-2.0)
        (ManagedBinaryKind::AgentDesktop, "linux-x64") => Some(ArchiveSpec {
            kind,
            version: "1.3.1",
            target_key: "linux-x64",
            url: "https://github.com/vercel-labs/agent-desktop/releases/download/v1.3.1/agent-desktop-v1.3.1-x86_64-unknown-linux-gnu.tar.gz",
            sha256: "ca77b6b75034eeb12e537dd8c0a4f2a3424119253f38b7e6bd79ba0d2fef1264",
            allowed_entries: &[
                ("agent-desktop", "agent-desktop"),
                (
                    "agent-desktop-linux-clipboard-owner",
                    "agent-desktop-linux-clipboard-owner",
                ),
            ],
            primary_executable: "agent-desktop",
        }),
        (ManagedBinaryKind::AgentDesktop, "linux-arm64") => Some(ArchiveSpec {
            kind,
            version: "1.3.1",
            target_key: "linux-arm64",
            url: "https://github.com/vercel-labs/agent-desktop/releases/download/v1.3.1/agent-desktop-v1.3.1-aarch64-unknown-linux-gnu.tar.gz",
            sha256: "438990a703fbeb19775596c341c3701435e80adbfa1b2f64b82ff17567d5573b",
            allowed_entries: &[
                ("agent-desktop", "agent-desktop"),
                (
                    "agent-desktop-linux-clipboard-owner",
                    "agent-desktop-linux-clipboard-owner",
                ),
            ],
            primary_executable: "agent-desktop",
        }),
        (ManagedBinaryKind::AgentDesktop, "darwin-arm64") => Some(ArchiveSpec {
            kind,
            version: "1.3.1",
            target_key: "darwin-arm64",
            url: "https://github.com/vercel-labs/agent-desktop/releases/download/v1.3.1/agent-desktop-v1.3.1-aarch64-apple-darwin.tar.gz",
            sha256: "9b7b7cfc74892eab46c2c85d6cb7dc4863bf8dab0a222cfcfb8ae4604c69c86c",
            allowed_entries: &[
                ("agent-desktop", "agent-desktop"),
                ("agent-desktop-macos-helper", "agent-desktop-macos-helper"),
            ],
            primary_executable: "agent-desktop",
        }),
        (ManagedBinaryKind::AgentDesktop, "darwin-x64") => Some(ArchiveSpec {
            kind,
            version: "1.3.1",
            target_key: "darwin-x64",
            url: "https://github.com/vercel-labs/agent-desktop/releases/download/v1.3.1/agent-desktop-v1.3.1-x86_64-apple-darwin.tar.gz",
            sha256: "469da10cfb68928c58caff88b92b30526e0ca895ac06bc86b7f533b1d9b309b3",
            allowed_entries: &[
                ("agent-desktop", "agent-desktop"),
                ("agent-desktop-macos-helper", "agent-desktop-macos-helper"),
            ],
            primary_executable: "agent-desktop",
        }),
        (ManagedBinaryKind::AgentDesktop, "win32-x64") => Some(ArchiveSpec {
            kind,
            version: "1.3.1",
            target_key: "win32-x64",
            url: "https://github.com/vercel-labs/agent-desktop/releases/download/v1.3.1/agent-desktop-v1.3.1-x86_64-pc-windows-msvc.tar.gz",
            sha256: "6ee4b1e078f4e78792e79f16e32e4a78b9919464aa3c55629b7e27ed53a08c13",
            allowed_entries: &[("agent-desktop.exe", "agent-desktop.exe")],
            primary_executable: "agent-desktop.exe",
        }),
        (ManagedBinaryKind::AgentDesktop, "win32-arm64") => Some(ArchiveSpec {
            kind,
            version: "1.3.1",
            target_key: "win32-arm64",
            url: "https://github.com/vercel-labs/agent-desktop/releases/download/v1.3.1/agent-desktop-v1.3.1-aarch64-pc-windows-msvc.tar.gz",
            sha256: "805f1f61890f9983fceaf38cda94a37a049c379435d65607db5cedd22c8ef97b",
            allowed_entries: &[("agent-desktop.exe", "agent-desktop.exe")],
            primary_executable: "agent-desktop.exe",
        }),
        _ => None,
    }
}

fn default_cache_root() -> PathBuf {
    if let Ok(explicit) = std::env::var("OMNISOLO_TOOL_CACHE_DIR")
        && !explicit.trim().is_empty()
    {
        return PathBuf::from(explicit);
    }
    if let Ok(xdg) = std::env::var("XDG_CACHE_HOME")
        && !xdg.trim().is_empty()
    {
        return PathBuf::from(xdg).join("omnisolo").join("tools");
    }
    if let Ok(home) = std::env::var("HOME")
        && !home.trim().is_empty()
    {
        return PathBuf::from(home)
            .join(".cache")
            .join("omnisolo")
            .join("tools");
    }
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA")
        && !local_app_data.trim().is_empty()
    {
        return PathBuf::from(local_app_data).join("OmniSolo").join("tools");
    }
    std::env::temp_dir().join("omnisolo-tool-cache")
}

fn normalize_archive_entry_path(path: &Path) -> Result<String, ToolError> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(seg) => {
                let s = seg.to_str().ok_or_else(|| {
                    ToolError::Unexpected("archive entry path is not valid UTF-8".to_string())
                })?;
                parts.push(s);
            }
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(ToolError::Unexpected(
                    "archive entry contains forbidden path traversal or root prefix".to_string(),
                ));
            }
        }
    }
    if parts.is_empty() {
        return Err(ToolError::Unexpected(
            "archive entry path is empty".to_string(),
        ));
    }
    Ok(parts.join("/"))
}

pub fn verify_and_extract_archive(
    archive_bytes: &[u8],
    spec: &ArchiveSpec,
    install_dir: &Path,
) -> Result<PathBuf, ToolError> {
    if archive_bytes.is_empty() || archive_bytes.len() > MAX_ARCHIVE_BYTES {
        return Err(ToolError::LlmRecoverable(format!(
            "{}: archive size {} bytes is outside valid bounds",
            spec.kind.name(),
            archive_bytes.len()
        )));
    }

    let actual_sha256 = format!("{:x}", Sha256::digest(archive_bytes));
    if actual_sha256 != spec.sha256 {
        return Err(ToolError::Unexpected(format!(
            "{}: archive SHA-256 mismatch (expected {}, got {})",
            spec.kind.name(),
            spec.sha256,
            actual_sha256
        )));
    }

    let parent = install_dir.parent().ok_or_else(|| {
        ToolError::Unexpected("install directory must have a parent path".to_string())
    })?;
    std::fs::create_dir_all(parent).map_err(|e| {
        ToolError::Unexpected(format!(
            "{}: failed to create cache parent {}: {e}",
            spec.kind.name(),
            parent.display()
        ))
    })?;

    let staging = parent.join(format!(
        ".staging-{}-{}-{}",
        spec.kind.name(),
        spec.version,
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&staging).map_err(|e| {
        ToolError::Unexpected(format!(
            "{}: failed to create staging directory: {e}",
            spec.kind.name()
        ))
    })?;

    let result = extract_into_staging(archive_bytes, spec, &staging, install_dir);
    if result.is_err() {
        let _ = std::fs::remove_dir_all(&staging);
    }
    result
}

fn extract_into_staging(
    archive_bytes: &[u8],
    spec: &ArchiveSpec,
    staging: &Path,
    install_dir: &Path,
) -> Result<PathBuf, ToolError> {
    let allowed: BTreeMap<&str, &str> = spec.allowed_entries.iter().copied().collect();
    let mut extracted_hashes = BTreeMap::new();

    let gz = GzDecoder::new(archive_bytes);
    let mut archive = tar::Archive::new(gz);

    for entry_res in archive.entries().map_err(|e| {
        ToolError::Unexpected(format!("{}: corrupt tar stream: {e}", spec.kind.name()))
    })? {
        let mut entry = entry_res.map_err(|e| {
            ToolError::Unexpected(format!("{}: unreadable tar entry: {e}", spec.kind.name()))
        })?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(ToolError::Unexpected(format!(
                "{}: refusing archive containing symlink or hardlink",
                spec.kind.name()
            )));
        }
        if entry_type.is_dir() {
            continue;
        }
        if !entry_type.is_file() {
            return Err(ToolError::Unexpected(format!(
                "{}: refusing unsupported tar entry kind",
                spec.kind.name()
            )));
        }
        let raw_path = entry
            .path()
            .map_err(|e| {
                ToolError::Unexpected(format!("{}: invalid entry path: {e}", spec.kind.name()))
            })?
            .into_owned();
        let normalized = normalize_archive_entry_path(&raw_path)?;

        let Some(&dest_filename) = allowed.get(normalized.as_str()) else {
            // npm tarballs also contain package/package.json and package/README.md; ignore only benign metadata in package/
            if spec.kind == ManagedBinaryKind::AgentBrowser
                && matches!(
                    normalized.as_str(),
                    "package/package.json" | "package/README.md"
                )
            {
                continue;
            }
            return Err(ToolError::Unexpected(format!(
                "{}: unexpected archive entry '{}'",
                spec.kind.name(),
                normalized
            )));
        };

        let mut buf = Vec::new();
        entry.read_to_end(&mut buf).map_err(|e| {
            ToolError::Unexpected(format!(
                "{}: failed reading entry '{}': {e}",
                spec.kind.name(),
                normalized
            ))
        })?;
        if buf.is_empty() || buf.len() > MAX_ARCHIVE_BYTES {
            return Err(ToolError::Unexpected(format!(
                "{}: extracted binary '{}' has invalid size {}",
                spec.kind.name(),
                dest_filename,
                buf.len()
            )));
        }

        let file_hash = format!("{:x}", Sha256::digest(&buf));
        let out_path = staging.join(dest_filename);
        std::fs::write(&out_path, &buf).map_err(|e| {
            ToolError::Unexpected(format!(
                "{}: failed writing '{}': {e}",
                spec.kind.name(),
                dest_filename
            ))
        })?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(0o755)).map_err(
                |e| {
                    ToolError::Unexpected(format!(
                        "{}: failed setting executable bit on '{}': {e}",
                        spec.kind.name(),
                        dest_filename
                    ))
                },
            )?;
        }
        extracted_hashes.insert(dest_filename.to_string(), file_hash);
    }

    for &(_, expected_file) in spec.allowed_entries {
        if !extracted_hashes.contains_key(expected_file) {
            return Err(ToolError::Unexpected(format!(
                "{}: archive missing required binary '{}'",
                spec.kind.name(),
                expected_file
            )));
        }
    }

    std::fs::write(staging.join("LICENSE.txt"), spec.kind.license_notice()).map_err(|e| {
        ToolError::Unexpected(format!(
            "{}: failed writing LICENSE.txt: {e}",
            spec.kind.name()
        ))
    })?;

    let receipt = serde_json::json!({
        "tool": spec.kind.name(),
        "version": spec.version,
        "target": spec.target_key,
        "archive_sha256": spec.sha256,
        "files": extracted_hashes,
    });
    std::fs::write(
        staging.join(".verified-receipt.json"),
        serde_json::to_vec_pretty(&receipt).unwrap_or_default(),
    )
    .map_err(|e| {
        ToolError::Unexpected(format!("{}: failed writing receipt: {e}", spec.kind.name()))
    })?;

    if install_dir.exists() {
        let _ = std::fs::remove_dir_all(install_dir);
    }
    std::fs::rename(staging, install_dir).map_err(|e| {
        ToolError::Unexpected(format!(
            "{}: failed moving verified tool into {}: {e}",
            spec.kind.name(),
            install_dir.display()
        ))
    })?;

    Ok(install_dir.join(spec.primary_executable))
}

fn verify_cached_installation(install_dir: &Path, spec: &ArchiveSpec) -> Option<PathBuf> {
    let receipt_path = install_dir.join(".verified-receipt.json");
    let receipt_bytes = std::fs::read(receipt_path).ok()?;
    let receipt: serde_json::Value = serde_json::from_slice(&receipt_bytes).ok()?;
    if receipt.get("archive_sha256")?.as_str()? != spec.sha256
        || receipt.get("version")?.as_str()? != spec.version
    {
        return None;
    }
    let files = receipt.get("files")?.as_object()?;
    for &(_, filename) in spec.allowed_entries {
        let expected_hash = files.get(filename)?.as_str()?;
        let path = install_dir.join(filename);
        let bytes = std::fs::read(&path).ok()?;
        let actual_hash = format!("{:x}", Sha256::digest(&bytes));
        if actual_hash != expected_hash {
            return None;
        }
    }
    let primary = install_dir.join(spec.primary_executable);
    primary.is_file().then_some(primary)
}

/// Resolves a managed binary, checking explicit env override first, then verified cache,
/// and finally performing an on-demand HTTPS download verified against the pinned SHA-256 digest.
pub async fn ensure_managed_binary(kind: ManagedBinaryKind) -> Result<PathBuf, ToolError> {
    ensure_managed_binary_with_cache(kind, &default_cache_root()).await
}

pub async fn ensure_managed_binary_with_cache(
    kind: ManagedBinaryKind,
    cache_root: &Path,
) -> Result<PathBuf, ToolError> {
    if let Ok(explicit) = std::env::var(kind.bin_env_var())
        && !explicit.trim().is_empty()
    {
        let path = PathBuf::from(explicit.trim());
        if path.is_file() {
            return Ok(path);
        }
        return Err(ToolError::LlmRecoverable(format!(
            "{} points to missing executable '{}'",
            kind.bin_env_var(),
            path.display()
        )));
    }

    let target_key = current_host_target_key().ok_or_else(|| {
        ToolError::LlmRecoverable(format!(
            "{}: unsupported host platform {}-{}",
            kind.name(),
            std::env::consts::OS,
            std::env::consts::ARCH
        ))
    })?;
    let spec = archive_spec_for_target(kind, target_key).ok_or_else(|| {
        ToolError::LlmRecoverable(format!(
            "{}: no pinned binary archive for target {}",
            kind.name(),
            target_key
        ))
    })?;

    ensure_managed_binary_with_spec(&spec, cache_root).await
}

pub async fn ensure_managed_binary_with_spec(
    spec: &ArchiveSpec,
    cache_root: &Path,
) -> Result<PathBuf, ToolError> {
    let kind = spec.kind;
    let install_dir = cache_root.join(format!(
        "{}-v{}-{}",
        kind.name(),
        spec.version,
        spec.target_key
    ));
    if let Some(verified) = verify_cached_installation(&install_dir, spec) {
        return Ok(verified);
    }

    let _guard = INSTALL_LOCKS[kind.lock_index()].lock().await;
    if let Some(verified) = verify_cached_installation(&install_dir, spec) {
        return Ok(verified);
    }

    let archive_bytes = if let Ok(local_archive) = std::env::var(kind.archive_env_var())
        && !local_archive.trim().is_empty()
    {
        tokio::fs::read(local_archive.trim()).await.map_err(|e| {
            ToolError::LlmRecoverable(format!(
                "{}: failed to read {}: {e}",
                kind.name(),
                kind.archive_env_var()
            ))
        })?
    } else {
        download_archive(spec).await?
    };

    let spec_owned = spec.clone();
    tokio::task::spawn_blocking(move || {
        verify_and_extract_archive(&archive_bytes, &spec_owned, &install_dir)
    })
    .await
    .map_err(|e| ToolError::Unexpected(format!("{}: installer task panicked: {e}", kind.name())))?
}

async fn download_archive(spec: &ArchiveSpec) -> Result<Vec<u8>, ToolError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| {
            ToolError::Unexpected(format!(
                "{}: failed to construct HTTP client: {e}",
                spec.kind.name()
            ))
        })?;

    let mut response = client
        .get(spec.url)
        .header("User-Agent", "OmniSolo-ToolInstaller/1.0")
        .send()
        .await
        .map_err(|e| {
            ToolError::LlmRecoverable(format!(
                "{}: on-demand download from {} failed: {e}",
                spec.kind.name(),
                spec.url
            ))
        })?;

    if !response.status().is_success() {
        return Err(ToolError::LlmRecoverable(format!(
            "{}: on-demand download returned HTTP {}",
            spec.kind.name(),
            response.status()
        )));
    }

    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|e| {
        ToolError::LlmRecoverable(format!(
            "{}: failed reading download stream: {e}",
            spec.kind.name()
        ))
    })? {
        if chunk.len() > MAX_ARCHIVE_BYTES.saturating_sub(bytes.len()) {
            return Err(ToolError::LlmRecoverable(format!(
                "{}: download exceeded 64 MiB safety limit",
                spec.kind.name()
            )));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::GzEncoder;

    fn build_gzip_tar(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut tar_buf = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut tar_buf);
            for (path, data) in entries {
                let mut header = tar::Header::new_gnu();
                header.set_size(data.len() as u64);
                header.set_mode(0o755);
                header.set_cksum();
                builder.append_data(&mut header, *path, *data).unwrap();
            }
            builder.finish().unwrap();
        }
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        std::io::Write::write_all(&mut encoder, &tar_buf).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn extracts_verified_archive_and_reuses_tamper_checked_cache() {
        let archive = build_gzip_tar(&[
            ("package/bin/agent-browser", b"#!/bin/sh\necho ok\n"),
            ("package/package.json", b"{\"version\":\"0.43.0\"}"),
        ]);
        let sha256: &'static str =
            Box::leak(format!("{:x}", Sha256::digest(&archive)).into_boxed_str());
        let spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentBrowser,
            version: "0.43.0",
            target_key: "linux-x64",
            url: "https://example.invalid/agent-browser.tgz",
            sha256,
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        };

        let dir = tempfile::tempdir().unwrap();
        let install_dir = dir.path().join("agent-browser-v0.43.0-linux-x64");
        let exe = verify_and_extract_archive(&archive, &spec, &install_dir).unwrap();
        assert!(exe.is_file());
        assert!(install_dir.join("LICENSE.txt").is_file());
        assert_eq!(
            verify_cached_installation(&install_dir, &spec),
            Some(exe.clone())
        );

        // Tampering with the extracted binary invalidates the cached receipt.
        std::fs::write(&exe, b"tampered").unwrap();
        assert!(verify_cached_installation(&install_dir, &spec).is_none());
    }

    #[test]
    fn rejects_checksum_mismatch_and_unexpected_entries() {
        let archive = build_gzip_tar(&[("package/bin/agent-browser", b"binary")]);
        let bad_spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentBrowser,
            version: "0.43.0",
            target_key: "linux-x64",
            url: "https://example.invalid/agent-browser.tgz",
            sha256: "0000000000000000000000000000000000000000000000000000000000000000",
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        };
        let dir = tempfile::tempdir().unwrap();
        assert!(verify_and_extract_archive(&archive, &bad_spec, &dir.path().join("bad")).is_err());

        let unexpected =
            build_gzip_tar(&[("agent-desktop", b"bin"), ("evil-extra-binary", b"payload")]);
        let sha256: &'static str =
            Box::leak(format!("{:x}", Sha256::digest(&unexpected)).into_boxed_str());
        let desktop_spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentDesktop,
            version: "1.3.1",
            target_key: "linux-x64",
            url: "https://example.invalid/agent-desktop.tar.gz",
            sha256,
            allowed_entries: &[("agent-desktop", "agent-desktop")],
            primary_executable: "agent-desktop",
        };
        assert!(
            verify_and_extract_archive(&unexpected, &desktop_spec, &dir.path().join("extra"))
                .is_err()
        );
    }

    #[test]
    fn all_supported_targets_have_valid_sha256_specs() {
        for target in [
            "linux-x64",
            "linux-arm64",
            "darwin-arm64",
            "darwin-x64",
            "win32-x64",
        ] {
            let browser = archive_spec_for_target(ManagedBinaryKind::AgentBrowser, target).unwrap();
            assert_eq!(browser.sha256.len(), 64);
            let desktop = archive_spec_for_target(ManagedBinaryKind::AgentDesktop, target).unwrap();
            assert_eq!(desktop.sha256.len(), 64);
        }
    }

    #[test]
    fn rejects_path_traversal_and_symlinks_in_archives() {
        let mut tar_buf = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut tar_buf);
            let mut header = tar::Header::new_gnu();
            header.set_entry_type(tar::EntryType::Symlink);
            header.set_size(0);
            header.set_mode(0o777);
            builder
                .append_link(&mut header, "agent-desktop", "/etc/passwd")
                .unwrap();
            builder.finish().unwrap();
        }
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        std::io::Write::write_all(&mut encoder, &tar_buf).unwrap();
        let symlink_archive = encoder.finish().unwrap();
        let sha256: &'static str =
            Box::leak(format!("{:x}", Sha256::digest(&symlink_archive)).into_boxed_str());
        let spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentDesktop,
            version: "1.3.1",
            target_key: "linux-x64",
            url: "https://example.invalid/agent-desktop.tar.gz",
            sha256,
            allowed_entries: &[("agent-desktop", "agent-desktop")],
            primary_executable: "agent-desktop",
        };
        let dir = tempfile::tempdir().unwrap();
        assert!(
            verify_and_extract_archive(&symlink_archive, &spec, &dir.path().join("sym")).is_err()
        );

        // Missing required entry in archive
        let missing = build_gzip_tar(&[("package/package.json", b"{}")]);
        let missing_sha: &'static str =
            Box::leak(format!("{:x}", Sha256::digest(&missing)).into_boxed_str());
        let missing_spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentBrowser,
            version: "0.43.0",
            target_key: "linux-x64",
            url: "https://example.invalid/agent-browser.tgz",
            sha256: missing_sha,
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        };
        assert!(
            verify_and_extract_archive(&missing, &missing_spec, &dir.path().join("missing"))
                .is_err()
        );

        // Traversal path inside archive
        assert!(normalize_archive_entry_path(Path::new("../evil")).is_err());
        assert!(normalize_archive_entry_path(Path::new("/abs/evil")).is_err());
    }

    #[tokio::test]
    async fn ensure_managed_binary_supports_bin_override_offline_archive_and_http_download() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let dir = tempfile::tempdir().unwrap();
        let cache_dir = dir.path().join("cache");

        // 1. Missing OMNISOLO_AGENT_BROWSER_BIN fails closed
        temp_env::async_with_vars(
            [
                (
                    "OMNISOLO_AGENT_BROWSER_BIN",
                    Some(dir.path().join("does-not-exist").to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_BROWSER_ARCHIVE", None::<&str>),
            ],
            async {
                let err =
                    ensure_managed_binary_with_cache(ManagedBinaryKind::AgentBrowser, &cache_dir)
                        .await
                        .unwrap_err();
                assert!(err.to_string().contains("points to missing executable"));
            },
        )
        .await;

        // 2. Existing OMNISOLO_AGENT_DESKTOP_BIN returns immediately
        let explicit_desktop = dir.path().join("custom-agent-desktop");
        std::fs::write(&explicit_desktop, b"#!/bin/sh\n").unwrap();
        temp_env::async_with_vars(
            [
                (
                    "OMNISOLO_AGENT_DESKTOP_BIN",
                    Some(explicit_desktop.to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_DESKTOP_ARCHIVE", None::<&str>),
            ],
            async {
                let got =
                    ensure_managed_binary_with_cache(ManagedBinaryKind::AgentDesktop, &cache_dir)
                        .await
                        .unwrap();
                assert_eq!(got, explicit_desktop);
            },
        )
        .await;

        // 3. Offline OMNISOLO_AGENT_DESKTOP_ARCHIVE installs and caches receipt
        let desktop_archive_bytes = build_gzip_tar(&[
            ("agent-desktop", b"#!/bin/sh\necho desktop\n"),
            ("agent-desktop-linux-clipboard-owner", b"helper"),
        ]);
        let desktop_archive_path = dir.path().join("desktop.tar.gz");
        std::fs::write(&desktop_archive_path, &desktop_archive_bytes).unwrap();
        let desktop_sha: &'static str =
            Box::leak(format!("{:x}", Sha256::digest(&desktop_archive_bytes)).into_boxed_str());
        let local_spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentDesktop,
            version: "1.3.1-test",
            target_key: "linux-x64",
            url: "http://127.0.0.1:9/unreachable",
            sha256: desktop_sha,
            allowed_entries: &[
                ("agent-desktop", "agent-desktop"),
                (
                    "agent-desktop-linux-clipboard-owner",
                    "agent-desktop-linux-clipboard-owner",
                ),
            ],
            primary_executable: "agent-desktop",
        };
        temp_env::async_with_vars(
            [(
                "OMNISOLO_AGENT_DESKTOP_ARCHIVE",
                Some(desktop_archive_path.to_str().unwrap()),
            )],
            async {
                let exe = ensure_managed_binary_with_spec(&local_spec, &cache_dir)
                    .await
                    .unwrap();
                assert!(exe.is_file());
                assert!(
                    exe.with_file_name("agent-desktop-linux-clipboard-owner")
                        .is_file()
                );
            },
        )
        .await;
        // Cached installation is reused even without archive env var or network.
        temp_env::async_with_vars([("OMNISOLO_AGENT_DESKTOP_ARCHIVE", None::<&str>)], async {
            let cached = ensure_managed_binary_with_spec(&local_spec, &cache_dir)
                .await
                .unwrap();
            assert!(cached.is_file());
        })
        .await;

        // 4. Loopback HTTP on-demand download + SHA-256 verification + 404 handling
        let browser_archive_bytes = build_gzip_tar(&[
            ("package/bin/agent-browser", b"#!/bin/sh\necho browser\n"),
            ("package/package.json", b"{}"),
        ]);
        let browser_sha: &'static str =
            Box::leak(format!("{:x}", Sha256::digest(&browser_archive_bytes)).into_boxed_str());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let payload = browser_archive_bytes.clone();
        let server = tokio::spawn(async move {
            for idx in 0..2 {
                let (mut sock, _) = listener.accept().await.unwrap();
                let mut req = [0u8; 1024];
                let _ = sock.read(&mut req).await;
                if idx == 0 {
                    let header = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        payload.len()
                    );
                    sock.write_all(header.as_bytes()).await.unwrap();
                    sock.write_all(&payload).await.unwrap();
                } else {
                    sock.write_all(
                        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    )
                    .await
                    .unwrap();
                }
            }
        });

        let ok_url: &'static str =
            Box::leak(format!("http://{addr}/agent-browser.tgz").into_boxed_str());
        let http_spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentBrowser,
            version: "0.43.0-http",
            target_key: "linux-x64",
            url: ok_url,
            sha256: browser_sha,
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        };
        let fail_spec = ArchiveSpec {
            kind: ManagedBinaryKind::AgentBrowser,
            version: "0.43.0-http-404",
            target_key: "linux-x64",
            url: ok_url,
            sha256: browser_sha,
            allowed_entries: &[("package/bin/agent-browser", "agent-browser")],
            primary_executable: "agent-browser",
        };

        temp_env::async_with_vars([("OMNISOLO_AGENT_BROWSER_ARCHIVE", None::<&str>)], async {
            let downloaded = ensure_managed_binary_with_spec(&http_spec, &cache_dir)
                .await
                .unwrap();
            assert!(downloaded.is_file());

            let http_err = ensure_managed_binary_with_spec(&fail_spec, &cache_dir)
                .await
                .unwrap_err();
            assert!(http_err.to_string().contains("404"));
        })
        .await;
        server.await.unwrap();
    }
}
