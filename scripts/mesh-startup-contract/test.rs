use crate::transport::{MeshTransport, RedisPubSubTransport};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::io::AsyncReadExt;

const SECRET: &str = "mesh-startup-canary-never-log";

struct RedisFixture {
    process: Child,
    directory: std::path::PathBuf,
    url: String,
}
impl Drop for RedisFixture {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}
async fn redis_fixture() -> RedisFixture {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let directory = std::env::temp_dir().join(format!("ohc-mesh-startup-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    let process = Command::new("redis-server")
        .args([
            "--bind",
            "127.0.0.1",
            "--port",
            &port.to_string(),
            "--save",
            "",
            "--appendonly",
            "no",
            "--requirepass",
            "public-local-mesh-password",
            "--daemonize",
            "no",
            "--dir",
        ])
        .arg(&directory)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("official Redis fixture required");
    let fixture = RedisFixture {
        process,
        directory,
        url: format!("redis://:public-local-mesh-password@127.0.0.1:{port}/"),
    };
    let client = redis::Client::open(fixture.url.as_str()).unwrap();
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if client.get_multiplexed_tokio_connection().await.is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("owned Redis did not start");
    fixture
}

async fn silent_connection() -> (String, tokio::net::TcpListener) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    (
        format!("redis://{}", listener.local_addr().unwrap()),
        listener,
    )
}

async fn accept_handshake(listener: &tokio::net::TcpListener) -> tokio::net::TcpStream {
    let (mut peer, _) = tokio::time::timeout(Duration::from_secs(3), listener.accept())
        .await
        .unwrap()
        .unwrap();
    let mut data = [0; 1024];
    assert!(
        tokio::time::timeout(Duration::from_secs(3), peer.read(&mut data))
            .await
            .unwrap()
            .unwrap()
            > 0
    );
    peer
}

async fn assert_disconnected(mut peer: tokio::net::TcpStream) {
    let mut buffer = [0; 4096];
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            match peer.read(&mut buffer).await {
                Ok(0) => break,
                Ok(_) => continue,
                Err(error) if error.kind() == std::io::ErrorKind::ConnectionReset => break,
                Err(error) => panic!("Unexpected fixture socket error: {error}"),
            }
        }
    })
    .await
    .expect("cancelled transport left its connection open");
}

#[tokio::test]
async fn silent_redis_connection_times_out_and_closes_socket() {
    let (url, listener) = silent_connection().await;
    let mut attempt = tokio::spawn(async move { RedisPubSubTransport::new(&url).await });
    let peer = accept_handshake(&listener).await;
    let result = tokio::time::timeout(Duration::from_secs(7), &mut attempt).await;
    if result.is_err() {
        attempt.abort();
    }
    let result = result
        .expect("Redis connection exceeded bounded startup time")
        .unwrap();
    let Err(error) = result else {
        panic!("silent Redis must not initialize")
    };
    assert!(
        error.contains("timed out"),
        "timeout cause must be distinct: {error}"
    );
    assert_disconnected(peer).await;
}

#[tokio::test]
async fn cancelling_constructor_drops_pending_socket() {
    let (url, listener) = silent_connection().await;
    let attempt = tokio::spawn(async move { RedisPubSubTransport::new(&url).await });
    let peer = accept_handshake(&listener).await;
    attempt.abort();
    assert!(matches!(attempt.await, Err(error) if error.is_cancelled()));
    assert_disconnected(peer).await;
}

#[tokio::test]
async fn overall_deadline_cancels_pending_startup() {
    let (url, listener) = silent_connection().await;
    let mut startup = tokio::spawn(crate::actual_startup(Some(url), true));
    let peer = accept_handshake(&listener).await;
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(181)).await;
    let result = tokio::time::timeout(Duration::from_secs(1), &mut startup).await;
    if result.is_err() {
        startup.abort();
    }
    let result = result.expect("overall startup exceeded180seconds").unwrap();
    let Err(error) = result else {
        panic!("silent startup must fail")
    };
    assert!(error.to_string().contains("180"));
    assert!(error.to_string().contains("timed out"));
    tokio::time::resume();
    assert_disconnected(peer).await;
    assert!(
        tokio::time::timeout(Duration::from_millis(50), listener.accept())
            .await
            .is_err(),
        "retry continued after overall cancellation"
    );
}

#[tokio::test]
async fn cancelling_startup_stops_pending_connection_and_retries() {
    let (url, listener) = silent_connection().await;
    let startup = tokio::spawn(crate::actual_startup(Some(url), true));
    let peer = accept_handshake(&listener).await;
    startup.abort();
    assert!(matches!(startup.await, Err(error) if error.is_cancelled()));
    assert_disconnected(peer).await;
    assert!(
        tokio::time::timeout(Duration::from_millis(50), listener.accept())
            .await
            .is_err()
    );
}

#[tokio::test(start_paused = true)]
async fn cloud_retains_thirty_attempts_and_one_second_backoff() {
    let start = tokio::time::Instant::now();
    let result = crate::actual_startup(None, true).await;
    let Err(error) = result else {
        panic!("cloud requires a transport")
    };
    assert!(error.to_string().contains("30"));
    assert_eq!(start.elapsed(), Duration::from_secs(29));
}

#[tokio::test(start_paused = true)]
async fn standalone_retains_immediate_in_process_fallback() {
    let start = tokio::time::Instant::now();
    let transport = crate::actual_startup(None, false).await.unwrap();
    transport
        .register_presence("standalone-test", "ready", 30)
        .await
        .unwrap();
    assert_eq!(
        transport.get_active_agents().await.unwrap(),
        vec![("standalone-test".into(), "ready".into())]
    );
    assert_eq!(start.elapsed(), Duration::ZERO);
}

#[tokio::test]
async fn malformed_redis_url_is_redacted() {
    let result = RedisPubSubTransport::new(&format!("https://user:{SECRET}@127.0.0.1/")).await;
    let Err(error) = result else {
        panic!("malformed URL must fail")
    };
    assert!(!error.contains(SECRET));
    assert!(error.contains("Invalid Redis mesh configuration"));
}

#[tokio::test]
async fn server_authentication_failure_is_redacted() {
    let fixture = redis_fixture().await;
    let result =
        RedisPubSubTransport::new(&fixture.url.replace("public-local-mesh-password", SECRET)).await;
    let Err(error) = result else {
        panic!("wrong password must fail")
    };
    assert!(!error.contains(SECRET));
    assert_eq!(error, "Redis mesh service is unavailable");
}

#[tokio::test]
async fn real_redis_transport_keeps_presence_locks_and_pubsub() {
    let fixture = redis_fixture().await;
    let transport = crate::actual_startup(Some(fixture.url.clone()), true)
        .await
        .unwrap();
    transport
        .register_presence("mesh-owned-agent", "ready", 30)
        .await
        .unwrap();
    assert_eq!(
        transport.get_active_agents().await.unwrap(),
        vec![("mesh-owned-agent".into(), "ready".into())]
    );
    assert!(
        transport
            .acquire_lock("mesh-owned-resource", "owner1", 30)
            .await
            .unwrap()
    );
    assert!(
        !transport
            .acquire_lock("mesh-owned-resource", "owner2", 30)
            .await
            .unwrap()
    );
    transport
        .release_lock("mesh-owned-resource", "owner1")
        .await
        .unwrap();
    assert!(
        transport
            .acquire_lock("mesh-owned-resource", "owner2", 30)
            .await
            .unwrap()
    );
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let cancel = transport
        .subscribe(
            "mesh-owned-topic",
            Box::new(move |message| {
                tx.send(message).unwrap();
            }),
        )
        .await
        .unwrap();
    let message = crate::transport::Message {
        agent_id: "mesh-owned-agent".into(),
        action: "regression".into(),
        status: "ready".into(),
        payload: b"public-test".to_vec(),
        msg_id: uuid::Uuid::new_v4().to_string(),
    };
    transport
        .publish("mesh-owned-topic", message.clone())
        .await
        .unwrap();
    assert_eq!(
        tokio::time::timeout(Duration::from_secs(2), rx.recv())
            .await
            .unwrap()
            .unwrap(),
        message
    );
    cancel();
}

#[tokio::test]
async fn healthy_constructor_succeeds_after_cancelled_attempt() {
    let (url, listener) = silent_connection().await;
    let pending = tokio::spawn(async move { RedisPubSubTransport::new(&url).await });
    let peer = accept_handshake(&listener).await;
    pending.abort();
    assert!(matches!(pending.await, Err(error) if error.is_cancelled()));
    assert_disconnected(peer).await;
    let fixture = redis_fixture().await;
    let healthy = RedisPubSubTransport::new(&fixture.url).await.unwrap();
    healthy
        .register_presence("after-cancellation", "ready", 30)
        .await
        .unwrap();
    assert_eq!(healthy.get_active_agents().await.unwrap().len(), 1);
}

#[tokio::test]
async fn healthy_retry_succeeds_after_one_refused_connection() {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    let fixture = redis_fixture().await;
    let parsed = reqwest::Url::parse(&fixture.url).unwrap();
    let upstream = format!("127.0.0.1:{}", parsed.port().unwrap());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!(
        "redis://:public-local-mesh-password@{}/",
        listener.local_addr().unwrap()
    );
    let accepted = Arc::new(AtomicUsize::new(0));
    let count = accepted.clone();
    let proxy = tokio::spawn(async move {
        let mut forwarding = tokio::task::JoinSet::new();
        loop {
            tokio::select! {
                incoming = listener.accept() => {
                    let (mut downstream,_) = incoming.unwrap();
                    if count.fetch_add(1,Ordering::SeqCst)==0 {drop(downstream);continue;}
                    let upstream = upstream.clone();
                    forwarding.spawn(async move {
                        let mut upstream = tokio::net::TcpStream::connect(upstream).await.unwrap();
                        let _ = tokio::io::copy_bidirectional(&mut downstream,&mut upstream).await;
                    });
                }
                Some(_) = forwarding.join_next(), if !forwarding.is_empty() => {}
            }
        }
    });
    let result = tokio::time::timeout(
        Duration::from_secs(8),
        crate::actual_startup(Some(url), true),
    )
    .await;
    let transport = result.unwrap().unwrap();
    transport
        .register_presence("retry-recovered", "ready", 30)
        .await
        .unwrap();
    assert_eq!(
        transport.get_active_agents().await.unwrap(),
        vec![("retry-recovered".into(), "ready".into())]
    );
    assert_eq!(accepted.load(Ordering::SeqCst), 2);
    drop(transport);
    proxy.abort();
    assert!(proxy.await.unwrap_err().is_cancelled());
}

#[tokio::test]
async fn nats_remains_first_choice_and_failure_falls_back_without_leaking() {
    const CHILD: &str = "OHC_MESH_NATS_CANARY_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_ansi(false)
            .try_init();
        let fixture = redis_fixture().await;
        let unrelated = tokio::spawn(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            tracing::warn!("mesh-concurrent-security-event");
        });
        let transport = crate::actual_startup(Some(fixture.url.clone()), true)
            .await
            .unwrap();
        transport
            .register_presence("after-nats-rejection", "ready", 30)
            .await
            .unwrap();
        unrelated.await.unwrap();
        let direct = RedisPubSubTransport::new(&fixture.url).await.unwrap();
        assert_eq!(
            direct.get_active_agents().await.unwrap(),
            vec![("after-nats-rejection".into(), "ready".into())]
        );
        return;
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    listener.set_nonblocking(true).unwrap();
    let fixture = std::thread::spawn(move || {
        use std::io::{Read, Write};
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(3)))
                        .unwrap();
                    stream.write_all(b"INFO {\"server_id\":\"owned-fixture\",\"server_name\":\"owned-fixture\",\"version\":\"2.10.0\",\"proto\":1,\"host\":\"127.0.0.1\",\"port\":4222,\"auth_required\":true,\"headers\":true,\"max_payload\":1048576}\r\n").unwrap();
                    let mut data = Vec::new();
                    while !data.windows(6).any(|chunk| chunk == b"PING\r\n") {
                        let mut chunk = [0; 1024];
                        let size = stream.read(&mut chunk).unwrap();
                        assert!(size > 0, "NATS closed before CONNECT and PING");
                        data.extend_from_slice(&chunk[..size]);
                        assert!(data.len() < 8192, "unexpected NATS handshake size");
                    }
                    assert!(String::from_utf8_lossy(&data).contains("CONNECT "));
                    // Hold the actual NATS handshake pending while another task logs.
                    std::thread::sleep(Duration::from_millis(200));
                    stream
                        .write_all(format!("-ERR '{SECRET}'\r\n").as_bytes())
                        .unwrap();
                    return;
                }
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        && std::time::Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(5))
                }
                result => panic!("NATS was not selected before Redis: {result:?}"),
            }
        }
    });
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "tests::nats_remains_first_choice_and_failure_falls_back_without_leaking",
            "--nocapture",
        ])
        .env(CHILD, "1")
        .env("NATS_URL", format!("nats://user:{SECRET}@{address}"))
        .output()
        .unwrap();
    fixture.join().unwrap();
    let log = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !log.contains(SECRET),
        "NATS provider error leaked the credential canary"
    );
    assert!(output.status.success(), "{log}");
    let concurrent = log
        .find("mesh-concurrent-security-event")
        .expect("unrelated security log was suppressed");
    let boundary = log
        .find("NATS mesh transport is unavailable")
        .expect("safe failure diagnostic missing");
    assert!(
        concurrent < boundary,
        "unrelated log must occur while the handshake is pending"
    );
    assert!(log.contains("test result: ok. 1 passed; 0 failed;"));
}

#[tokio::test]
async fn standalone_retains_optional_fallback_after_redis_timeout() {
    let (url, listener) = silent_connection().await;
    let mut startup = tokio::spawn(crate::actual_startup(Some(url), false));
    let peer = accept_handshake(&listener).await;
    tokio::time::pause();
    tokio::time::advance(Duration::from_secs(5)).await;
    let result = tokio::time::timeout(Duration::from_secs(1), &mut startup).await;
    if result.is_err() {
        startup.abort();
    }
    let transport = result
        .expect("standalone optional Redis must time out")
        .unwrap()
        .unwrap();
    transport
        .register_presence("standalone-after-timeout", "ready", 30)
        .await
        .unwrap();
    assert_eq!(transport.get_active_agents().await.unwrap().len(), 1);
    tokio::time::resume();
    assert_disconnected(peer).await;
    assert!(
        tokio::time::timeout(Duration::from_millis(50), listener.accept())
            .await
            .is_err(),
        "standalone must not retry optional Redis"
    );
}
