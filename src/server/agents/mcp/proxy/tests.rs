use super::client::LocalProxyClient;
use super::server::ReverseTunnelServer;
use ::server_omnisolo::mcp_proxy::mcp_reverse_tunnel_service_client::McpReverseTunnelServiceClient;
use ::server_omnisolo::mcp_proxy::mcp_reverse_tunnel_service_server::McpReverseTunnelServiceServer;
use ::server_omnisolo::mcp_proxy::{
    InvokeCommandResponse, ProxyToServer, RegisterProxyRequest, ServerToProxy, proxy_to_server,
};
use axum::body::Body;
use axum::http::{Request as HttpRequest, StatusCode};
use axum::{Router, routing::post};
use std::collections::HashMap;
use std::future::Future;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_stream::wrappers::{ReceiverStream, TcpListenerStream};
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Code, Request, Status, Streaming};
use tower::ServiceExt;

const DEADLINE: Duration = Duration::from_secs(2);
const A: &str = "spiffe://omnisolo.io/org/org-a/agent/shared";
const B: &str = "spiffe://omnisolo.io/org/org-b/agent/shared";

// Store::new otherwise reads/writes ambient signing-key state. Keep real signed
// bearer checks in a child with a test-only secret and no ambient credentials.
fn isolated(name: &str, check: impl Future<Output = ()>) {
    const CHILD: &str = "OHC_TUNNEL_TEST_CHILD";
    const COMPLETE: i32 = 86;
    let exact = format!("agents::mcp::proxy::tests::{name}");
    if std::env::var(CHILD).as_deref() == Ok(exact.as_str()) {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { timeout(Duration::from_secs(30), check).await.unwrap() });
        std::process::exit(COMPLETE);
    }
    let isolated_directory = tempfile::tempdir().unwrap();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", &exact, "--nocapture"])
        .current_dir(isolated_directory.path())
        .env_clear()
        .env(CHILD, &exact)
        .env("TEST_WORKSPACE", "reverse-tunnel-tests")
        .env("JWT_SECRET", "reverse-tunnel-test-only-signing-secret")
        .env("OMNISOLO_MULTITENANT", "true")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit());
    for key in [
        "PATH",
        "LANG",
        "LC_ALL",
        "TMPDIR",
        "LD_LIBRARY_PATH",
        "SystemRoot",
    ] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    let mut child = command.spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(40);
    loop {
        match child.try_wait().unwrap() {
            Some(status) => {
                assert_eq!(status.code(), Some(COMPLETE), "isolated {exact}: {status}");
                return;
            }
            None if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(10)),
            None => {
                child.kill().unwrap();
                child.wait().unwrap();
                panic!("isolated {exact} exceeded its deadline");
            }
        }
    }
}

struct Fixture {
    service: ReverseTunnelServer,
    channel: Channel,
    app: Router,
    tokens: HashMap<String, String>,
    task: JoinHandle<()>,
}

#[derive(Clone)]
struct PeerPolicy {
    authenticate: bool,
    standalone: bool,
}

impl tonic::service::Interceptor for PeerPolicy {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if self.authenticate {
            ::server_auth::peer_identity::authenticate_spiffe_request(
                &mut request,
                self.standalone,
            )?;
        }
        Ok(request)
    }
}

impl Fixture {
    async fn start(authenticate: bool, standalone: bool) -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(5))
            .connect_lazy("postgres://invalid:invalid@127.0.0.1:1/test")
            .unwrap();
        let service = ReverseTunnelServer::new(Arc::new(pool));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let grpc = McpReverseTunnelServiceServer::with_interceptor(
            service.clone(),
            PeerPolicy {
                authenticate,
                standalone,
            },
        );
        let task = tokio::spawn(async move {
            Server::builder()
                .add_service(grpc)
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .unwrap();
        });
        let channel = Endpoint::from_shared(format!("http://{address}"))
            .unwrap()
            .connect()
            .await
            .unwrap();
        let store = Arc::new(::server_auth::Store::new());
        let mut tokens = HashMap::new();
        for org in ["org-a", "org-b"] {
            let user = store
                .create_user(
                    format!("owner-{org}"),
                    format!("owner-{org}@example.test"),
                    "local-test-password".to_owned(),
                    vec!["owner".to_owned()],
                    org.to_owned(),
                )
                .await
                .unwrap();
            tokens.insert(org.to_owned(), store.issue_token(&user).unwrap());
        }
        let app = crate::protect_internal_ingress(
            Router::new()
                .route(
                    "/api/v1/relay/webhook/{agent_id}",
                    post(crate::api::mcp_webhook::handle_relay_webhook),
                )
                .with_state(service.clone()),
            store,
        );
        Self {
            service,
            channel,
            app,
            tokens,
            task,
        }
    }

    async fn register(
        &self,
        authenticated: Option<&str>,
        claimed: &str,
    ) -> Result<(mpsc::Sender<ProxyToServer>, Streaming<ServerToProxy>), Status> {
        self.connect(authenticated, registration(claimed)).await
    }

    async fn connect(
        &self,
        authenticated: Option<&str>,
        first: ProxyToServer,
    ) -> Result<(mpsc::Sender<ProxyToServer>, Streaming<ServerToProxy>), Status> {
        let (sender, receiver) = mpsc::channel(8);
        sender.send(first).await.unwrap();
        let mut request = Request::new(ReceiverStream::new(receiver));
        if let Some(identity) = authenticated {
            request
                .metadata_mut()
                .insert("x-spiffe-id", identity.parse().unwrap());
        }
        let response = timeout(
            DEADLINE,
            McpReverseTunnelServiceClient::new(self.channel.clone()).establish_tunnel(request),
        )
        .await
        .expect("registration did not finish")?;
        Ok((sender, response.into_inner()))
    }

    async fn relay(&self, tenant: Option<&str>, payload: &str) -> axum::response::Response {
        let mut request = HttpRequest::builder()
            .method("POST")
            .uri("/api/v1/relay/webhook/shared")
            .header("x-tenant-id", "org-b")
            .header("x-org-id", "org-b");
        if let Some(tenant) = tenant {
            request = request.header("authorization", format!("Bearer {}", self.tokens[tenant]));
        }
        self.app
            .clone()
            .oneshot(request.body(Body::from(payload.to_owned())).unwrap())
            .await
            .unwrap()
    }

    async fn registered_count(&self, expected: usize) {
        timeout(DEADLINE, async {
            while self.service.connections.len() != expected {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("unexpected live registration count");
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn registration(identity: &str) -> ProxyToServer {
    ProxyToServer {
        request_id: "register".to_owned(),
        payload: Some(proxy_to_server::Payload::Register(RegisterProxyRequest {
            spiffe_id: identity.to_owned(),
            supported_tools: vec!["webhook_forward".to_owned()],
        })),
    }
}

async fn received(stream: &mut Streaming<ServerToProxy>, expected: &str) {
    let message = timeout(DEADLINE, stream.message())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let Some(::server_omnisolo::mcp_proxy::server_to_proxy::Payload::InvokeRequest(command)) =
        message.payload
    else {
        panic!("forwarded message had no invocation");
    };
    assert_eq!(command.tool_id, "webhook_forward");
    assert_eq!(command.params, expected);
}

#[test]
fn registration_requires_verified_extension_even_with_identity_metadata() {
    isolated(
        "registration_requires_verified_extension_even_with_identity_metadata",
        async {
            let fixture = Fixture::start(false, true).await;
            let error = fixture
                .register(Some(A), A)
                .await
                .expect_err("unverified registration succeeded");
            assert_eq!(error.code(), Code::Unauthenticated);
            assert!(fixture.service.connections.is_empty());
            assert_eq!(
                fixture.relay(Some("org-a"), "denied").await.status(),
                StatusCode::NOT_FOUND
            );
        },
    );
}

#[test]
fn standalone_metadata_is_validated_and_cloud_metadata_cannot_replace_a_certificate() {
    isolated(
        "standalone_metadata_is_validated_and_cloud_metadata_cannot_replace_a_certificate",
        async {
            let standalone = Fixture::start(true, true).await;
            for identity in [None, Some("spiffe://test")] {
                assert_eq!(
                    standalone.register(identity, A).await.unwrap_err().code(),
                    Code::Unauthenticated
                );
            }
            assert!(standalone.service.connections.is_empty());
            let cloud = Fixture::start(true, false).await;
            assert_eq!(
                cloud.register(Some(A), A).await.unwrap_err().code(),
                Code::Unauthenticated
            );
            assert!(cloud.service.connections.is_empty());
        },
    );
}

#[test]
fn claimed_identity_cannot_replace_verified_registration_identity() {
    isolated(
        "claimed_identity_cannot_replace_verified_registration_identity",
        async {
            let fixture = Fixture::start(true, true).await;
            let error = fixture
                .register(Some(A), B)
                .await
                .expect_err("cross-tenant claim succeeded");
            assert_eq!(error.code(), Code::PermissionDenied);
            assert!(fixture.service.connections.is_empty());
            assert_eq!(
                fixture.relay(Some("org-b"), "denied").await.status(),
                StatusCode::NOT_FOUND
            );
        },
    );
}

#[test]
fn registration_is_required_before_the_rpc_reports_success() {
    isolated(
        "registration_is_required_before_the_rpc_reports_success",
        async {
            let fixture = Fixture::start(true, true).await;
            let response = ProxyToServer {
                request_id: "unregistered".to_owned(),
                payload: Some(proxy_to_server::Payload::InvokeResponse(
                    InvokeCommandResponse::default(),
                )),
            };
            assert_eq!(
                fixture.connect(Some(A), response).await.unwrap_err().code(),
                Code::FailedPrecondition
            );
            assert!(fixture.service.connections.is_empty());
        },
    );
}

#[test]
fn signed_relay_tenant_routes_same_agent_ids_independently() {
    isolated(
        "signed_relay_tenant_routes_same_agent_ids_independently",
        async {
            let fixture = Fixture::start(true, true).await;
            let (_a_sender, mut a_stream) = fixture.register(Some(A), A).await.unwrap();
            fixture.registered_count(1).await;
            let (_b_sender, mut b_stream) = fixture.register(Some(B), B).await.unwrap();
            fixture.registered_count(2).await;
            assert_eq!(
                fixture.relay(None, "unauthenticated").await.status(),
                StatusCode::UNAUTHORIZED
            );
            assert_eq!(
                fixture.relay(Some("org-a"), "only A").await.status(),
                StatusCode::OK
            );
            received(&mut a_stream, "only A").await;
            assert_eq!(
                fixture.relay(Some("org-b"), "only B").await.status(),
                StatusCode::OK
            );
            received(&mut b_stream, "only B").await;
            assert!(
                timeout(Duration::from_millis(50), a_stream.message())
                    .await
                    .is_err()
            );
            assert!(
                timeout(Duration::from_millis(50), b_stream.message())
                    .await
                    .is_err()
            );
        },
    );
}

#[test]
fn signed_relay_cannot_reach_another_tenants_only_connection() {
    isolated(
        "signed_relay_cannot_reach_another_tenants_only_connection",
        async {
            let fixture = Fixture::start(true, true).await;
            let (_sender, mut stream) = fixture.register(Some(B), B).await.unwrap();
            fixture.registered_count(1).await;
            assert_eq!(
                fixture.relay(Some("org-a"), "foreign").await.status(),
                StatusCode::NOT_FOUND
            );
            assert!(
                timeout(Duration::from_millis(50), stream.message())
                    .await
                    .is_err()
            );
        },
    );
}

#[test]
fn old_disconnect_does_not_remove_a_replacement_connection() {
    isolated(
        "old_disconnect_does_not_remove_a_replacement_connection",
        async {
            let fixture = Fixture::start(true, true).await;
            let (old_sender, mut old_stream) = fixture.register(Some(A), A).await.unwrap();
            fixture.registered_count(1).await;
            let (_new_sender, mut new_stream) = fixture.register(Some(A), A).await.unwrap();
            assert_eq!(
                fixture
                    .relay(Some("org-a"), "replacement ready")
                    .await
                    .status(),
                StatusCode::OK
            );
            received(&mut new_stream, "replacement ready").await;
            drop(old_sender);
            assert!(
                timeout(DEADLINE, old_stream.message())
                    .await
                    .unwrap()
                    .unwrap()
                    .is_none()
            );
            drop(old_stream);
            assert_eq!(
                fixture
                    .relay(Some("org-a"), "after old disconnect")
                    .await
                    .status(),
                StatusCode::OK
            );
            received(&mut new_stream, "after old disconnect").await;
        },
    );
}

#[test]
fn repeated_registration_terminates_its_stream_without_evicting_replacement() {
    isolated(
        "repeated_registration_terminates_its_stream_without_evicting_replacement",
        async {
            let fixture = Fixture::start(true, true).await;
            let (old_sender, mut old_stream) = fixture.register(Some(A), A).await.unwrap();
            fixture.registered_count(1).await;
            let (_new_sender, mut new_stream) = fixture.register(Some(A), A).await.unwrap();
            assert_eq!(
                fixture
                    .relay(Some("org-a"), "replacement ready")
                    .await
                    .status(),
                StatusCode::OK
            );
            received(&mut new_stream, "replacement ready").await;
            old_sender.send(registration(A)).await.unwrap();
            let error = timeout(DEADLINE, old_stream.message())
                .await
                .unwrap()
                .unwrap_err();
            assert_eq!(error.code(), Code::FailedPrecondition);
            assert_eq!(
                fixture
                    .relay(Some("org-a"), "still replaced")
                    .await
                    .status(),
                StatusCode::OK
            );
            received(&mut new_stream, "still replaced").await;
        },
    );
}

#[test]
fn local_client_uses_the_existing_standalone_identity_policy() {
    isolated(
        "local_client_uses_the_existing_standalone_identity_policy",
        async {
            let fixture = Fixture::start(true, true).await;
            let grpc_client = McpReverseTunnelServiceClient::new(fixture.channel.clone());
            let mut client = LocalProxyClient::new_with_channel(grpc_client, A.to_owned());
            timeout(DEADLINE, client.start())
                .await
                .unwrap()
                .expect("authenticated client start failed");
            fixture.registered_count(1).await;
        },
    );
}
