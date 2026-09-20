//! Own the packaged Node server for the actual Next application. No command or
//! path comes from a web page, and remote pages do not receive shell permissions.
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::time::Duration;
use tauri::Manager;

#[derive(Default)]
pub struct NativeRuntime {
    child: Mutex<Option<Child>>,
    stopping: AtomicBool,
}

impl NativeRuntime {
    pub fn stop(&self) {
        self.stopping.store(true, Ordering::SeqCst);
        if let Ok(mut state) = self.child.lock()
            && let Some(mut child) = state.take()
        {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn backend_origin() -> Result<String, String> {
    let value = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".into());
    let url = reqwest::Url::parse(&value).map_err(|_| "BACKEND_URL must be an absolute origin")?;
    let loopback = matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"));
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
        || !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
    {
        return Err("BACKEND_URL must be an HTTPS origin or local loopback HTTP origin".into());
    }
    Ok(url.origin().ascii_serialization())
}

/// Drain a line without allocating beyond the handshake bound. Oversized log
/// lines are discarded rather than filling the child's pipe after readiness.
fn bounded_line(reader: &mut impl BufRead, line: &mut Vec<u8>) -> std::io::Result<bool> {
    line.clear();
    let mut observed = false;
    let mut oversized = false;
    loop {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            return Ok(observed);
        }
        observed = true;
        let end = buffer
            .iter()
            .position(|byte| *byte == b'\n')
            .map(|at| at + 1);
        let amount = end.unwrap_or(buffer.len());
        if !oversized {
            if line.len().saturating_add(amount) <= 16_384 {
                line.extend_from_slice(&buffer[..amount]);
            } else {
                line.clear();
                oversized = true;
            }
        }
        reader.consume(amount);
        if end.is_some() {
            return Ok(true);
        }
    }
}

fn resource_root(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    // Debug builds are not installed bundles. This compile-time path is created
    // by our packaging script; web content cannot override it.
    #[cfg(debug_assertions)]
    {
        let prepared = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("native-resources");
        if prepared.join("web/build-manifest.json").is_file() {
            return Ok(prepared);
        }
    }
    Ok(app
        .path()
        .resource_dir()
        .map_err(|error| error.to_string())?
        .join("native-runtime"))
}

fn start(app: &tauri::AppHandle) -> Result<reqwest::Url, String> {
    let resources = resource_root(app)?;
    let node = resources
        .join("bin")
        .join(if cfg!(windows) { "node.exe" } else { "node" });
    let entry = resources.join("web/src/ui/next/native-entry.cjs");
    if !node.is_file() || !entry.is_file() {
        return Err("Native web resources are missing. Rebuild with npm run desktop:build.".into());
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0").map_err(|error| error.to_string())?;
    let port = listener
        .local_addr()
        .map_err(|error| error.to_string())?
        .port();
    let launch = uuid::Uuid::new_v4().to_string();
    let expected = format!("OMNISOLO_NATIVE_READY {launch} {port}");
    let mut command = Command::new(node);
    command
        .arg(entry)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    for key in [
        "PATH",
        "HOME",
        "USERPROFILE",
        "SYSTEMROOT",
        "WINDIR",
        "TMP",
        "TEMP",
        "TMPDIR",
        "LANG",
    ] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    command
        .env("BACKEND_URL", backend_origin()?)
        .env("OMNISOLO_NATIVE_WEB_PORT", port.to_string())
        .env("OMNISOLO_NATIVE_LAUNCH_ID", &launch);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    // The child does not retry an occupied port. Its private stdout readiness
    // handshake prevents a different local process being mistaken for our app.
    drop(listener);
    let mut child = command
        .spawn()
        .map_err(|error| format!("Cannot start packaged Node: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or("Native runtime stdout unavailable")?;
    let state = app.state::<NativeRuntime>();
    {
        let mut slot = state
            .child
            .lock()
            .map_err(|_| "Native runtime state unavailable")?;
        if state.stopping.load(Ordering::SeqCst) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("Application is shutting down".into());
        }
        *slot = Some(child);
    }
    let (sender, receiver) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = Vec::new();
        loop {
            if !matches!(bounded_line(&mut reader, &mut line), Ok(true)) {
                break;
            }
            if String::from_utf8_lossy(&line).trim() == expected {
                let _ = sender.try_send(());
            }
        }
    });
    if receiver.recv_timeout(Duration::from_secs(45)).is_err() {
        state.stop();
        return Err("The local workspace did not start. Check the installed runtime and backend configuration.".into());
    }
    let mut slot = state
        .child
        .lock()
        .map_err(|_| "Native runtime state unavailable")?;
    if state.stopping.load(Ordering::SeqCst)
        || slot
            .as_mut()
            .ok_or("Native runtime stopped")?
            .try_wait()
            .map_err(|error| error.to_string())?
            .is_some()
    {
        return Err("The local workspace exited during startup".into());
    }
    reqwest::Url::parse(&format!("http://127.0.0.1:{port}/login"))
        .map_err(|error| error.to_string())
}

pub fn start_window(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let result = start(&app);
        if let Some(window) = app.get_webview_window("main") {
            match result {
                Ok(url) => {
                    if window.navigate(url).is_err() {
                        app.state::<NativeRuntime>().stop();
                    }
                }
                Err(message) => {
                    let literal = serde_json::to_string(&message)
                        .unwrap_or_else(|_| "\"Workspace startup failed\"".into());
                    let _ = window.eval(format!(
                        "document.getElementById('status').textContent={literal};"
                    ));
                }
            }
        } else {
            app.state::<NativeRuntime>().stop();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oversized_lines_are_bounded_and_following_readiness_survives() {
        let mut data = vec![b'x'; 100_000];
        data.extend_from_slice(b"\nOMNISOLO_NATIVE_READY test 1234\n");
        let mut reader = BufReader::with_capacity(257, data.as_slice());
        let mut line = Vec::new();
        assert!(bounded_line(&mut reader, &mut line).unwrap());
        assert!(line.is_empty());
        assert!(line.capacity() <= 32_768);
        assert!(bounded_line(&mut reader, &mut line).unwrap());
        assert_eq!(line, b"OMNISOLO_NATIVE_READY test 1234\n");
        assert!(!bounded_line(&mut reader, &mut line).unwrap());
    }
    #[test]
    fn unterminated_final_line_and_empty_input_are_handled() {
        let mut line = vec![];
        let mut reader = std::io::Cursor::new(b"final".as_slice());
        assert!(bounded_line(&mut reader, &mut line).unwrap());
        assert_eq!(line, b"final");
        assert!(!bounded_line(&mut reader, &mut line).unwrap());
    }
}
