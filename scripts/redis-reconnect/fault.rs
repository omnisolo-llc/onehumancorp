//! Minimal loopback RESP fault fixture. It records command receipt and can lose
//! a reply after an operation, independently of the owned real-Valkey tests.
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

type RecordedCommands = Arc<Mutex<Vec<Vec<Vec<u8>>>>>;

pub struct FaultServer {
    pub url: String,
    commands: RecordedCommands,
    available: Arc<AtomicBool>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for FaultServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl FaultServer {
    pub async fn start(fault_command: &'static str, stall: bool) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("redis://{}/", listener.local_addr().unwrap());
        let commands = Arc::new(Mutex::new(Vec::new()));
        let captured = commands.clone();
        let available = Arc::new(AtomicBool::new(true));
        let accepting = available.clone();
        let used_fault = Arc::new(AtomicBool::new(false));
        let deleted = Arc::new(AtomicBool::new(false));
        let task = tokio::spawn(async move {
            let mut connections = tokio::task::JoinSet::new();
            loop {
                tokio::select! {
                    accepted = listener.accept() => {
                        let (socket, _) = accepted.unwrap();
                        if !accepting.load(Ordering::SeqCst) { continue; }
                        let commands = captured.clone();
                        let used_fault = used_fault.clone();
                        let deleted = deleted.clone();
                        connections.spawn(async move {
                            let mut socket = BufReader::new(socket);
                            while let Some(command) = read_command(&mut socket).await {
                                let name = String::from_utf8(command[0].clone()).unwrap().to_uppercase();
                                commands.lock().unwrap().push(command);
                                // Model the acknowledgement mutation before losing its reply.
                                if name == "HDEL" { deleted.store(true, Ordering::SeqCst); }
                                if name == fault_command && !used_fault.swap(true, Ordering::SeqCst) {
                                    if stall { std::future::pending::<()>().await; }
                                    break;
                                }
                                let response = match name.as_str() {
                                    "PING" => b"+PONG\r\n".to_vec(),
                                    "RPUSH" | "HDEL" | "HSET" | "DEL" => b":1\r\n".to_vec(),
                                    "GET" => b"$-1\r\n".to_vec(),
                                    "BLPOP" => b"*-1\r\n".to_vec(),
                                    "HGET" if !deleted.load(Ordering::SeqCst) => {
                                        let job = server_omnisolo::interop::QueueJob {
                                            id: "seeded-job".into(),tenant_id:"tenant-fixture".into(), ..Default::default()
                                        };
                                        let payload = prost::Message::encode_to_vec(&job);
                                        let mut bytes = format!("${}\r\n", payload.len()).into_bytes();
                                        bytes.extend(payload); bytes.extend(b"\r\n"); bytes
                                    }
                                    "HGET" => b"$-1\r\n".to_vec(),
                                    "CLIENT" | "SELECT" | "AUTH" => b"+OK\r\n".to_vec(),
                                    _ => panic!("unimplemented fixture command {name}"),
                                };
                                if socket.write_all(&response).await.is_err() { break; }
                            }
                        });
                    }
                    _ = connections.join_next(), if !connections.is_empty() => {}
                }
            }
        });
        Self {
            url,
            commands,
            available,
            task,
        }
    }
    pub fn set_available(&self, available: bool) {
        self.available.store(available, Ordering::SeqCst);
    }

    pub fn count(&self, command: &str) -> usize {
        self.commands
            .lock()
            .unwrap()
            .iter()
            .filter(|args| args[0].eq_ignore_ascii_case(command.as_bytes()))
            .count()
    }
}

async fn read_command(socket: &mut BufReader<tokio::net::TcpStream>) -> Option<Vec<Vec<u8>>> {
    let mut line = String::new();
    if socket.read_line(&mut line).await.ok()? == 0 {
        return None;
    }
    let count: usize = line.trim().strip_prefix('*')?.parse().ok()?;
    assert!(count <= 32);
    let mut args = Vec::with_capacity(count);
    for _ in 0..count {
        line.clear();
        socket.read_line(&mut line).await.ok()?;
        let size: usize = line.trim().strip_prefix('$')?.parse().ok()?;
        assert!(size <= 65536);
        let mut bytes = vec![0; size + 2];
        socket.read_exact(&mut bytes).await.ok()?;
        assert_eq!(&bytes[size..], b"\r\n");
        bytes.truncate(size);
        args.push(bytes);
    }
    Some(args)
}
