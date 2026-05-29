use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn resolve_runfile(path: &str) -> Option<PathBuf> {
    let candidate = PathBuf::from(path);
    if candidate.is_absolute() && candidate.is_file() {
        return Some(candidate);
    }

    if candidate.is_file() {
        return Some(candidate);
    }

    if let (Ok(runfiles_dir), Ok(workspace)) =
        (env::var("RUNFILES_DIR"), env::var("TEST_WORKSPACE"))
    {
        let runfile = Path::new(&runfiles_dir).join(workspace).join(path);
        if runfile.is_file() {
            return Some(runfile);
        }
    }

    if let Ok(runfiles_dir) = env::var("RUNFILES_DIR") {
        let runfile = Path::new(&runfiles_dir).join(path);
        if runfile.is_file() {
            return Some(runfile);
        }
    }

    None
}

fn env_usize(name: &str, default: usize) -> usize {
    env::var(name)
        .ok()
        .and_then(|raw| raw.parse::<usize>().ok())
        .unwrap_or(default)
}

fn main() -> ExitCode {
    if let Ok(status_file) = env::var("TEST_SHARD_STATUS_FILE") {
        let _ = File::create(status_file);
    }

    let binary_env = match env::var("RUST_SHARDED_TEST_BINARY") {
        Ok(path) => path,
        Err(err) => {
            eprintln!("RUST_SHARDED_TEST_BINARY is not set: {err}");
            return ExitCode::from(2);
        }
    };

    let binary = match resolve_runfile(&binary_env) {
        Some(path) => path,
        None => {
            eprintln!("Rust test binary is not in runfiles: {binary_env}");
            return ExitCode::from(2);
        }
    };

    let filters: Vec<String> = env::var("RUST_SHARDED_TEST_FILTERS")
        .unwrap_or_default()
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();

    let output = match Command::new(&binary)
        .env(
            "RUST_TEST_THREADS",
            env::var("RUST_TEST_THREADS").unwrap_or_else(|_| "1".to_owned()),
        )
        .arg("--list")
        .args(&filters)
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            eprintln!("Failed to list Rust tests from {}: {err}", binary.display());
            return ExitCode::from(2);
        }
    };

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return ExitCode::from(output.status.code().unwrap_or(1) as u8);
    }

    let tests: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.strip_suffix(": test"))
        .map(str::to_owned)
        .collect();

    if tests.is_empty() {
        return ExitCode::SUCCESS;
    }

    let total = env_usize("TEST_TOTAL_SHARDS", 1).max(1);
    let index = env_usize("TEST_SHARD_INDEX", 0);
    let selected: Vec<&str> = tests
        .iter()
        .enumerate()
        .filter_map(|(i, test)| (i % total == index).then_some(test.as_str()))
        .collect();

    if selected.is_empty() {
        return ExitCode::SUCCESS;
    }

    let status = match Command::new(&binary)
        .env(
            "RUST_TEST_THREADS",
            env::var("RUST_TEST_THREADS").unwrap_or_else(|_| "1".to_owned()),
        )
        .arg("--exact")
        .args(selected)
        .status()
    {
        Ok(status) => status,
        Err(err) => {
            eprintln!("Failed to run Rust tests from {}: {err}", binary.display());
            return ExitCode::from(2);
        }
    };

    ExitCode::from(status.code().unwrap_or(1) as u8)
}
