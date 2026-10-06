"""Import exact production token handler/helper bodies for a mounted authority test.

Only connection lookup/token issuance are recording sentinels. AuthInfo is a
trusted middleware-extension fixture; this gate does not re-test JWT verification.
"""
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rust_source import extract_item
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
source = (ROOT / 'src/server/api/terminal_api.rs').read_text()
text = '''use axum::{Json,extract::State,response::IntoResponse};
use std::sync::{Arc,Mutex};
use crate::hub::Hub;
use crate::protocol as terminal_payment_identity;
static LOOKUPS:Mutex<Vec<String>>=Mutex::new(Vec::new());
struct TokenClient;
impl TokenClient {async fn create_terminal_connection_token(&self,_tenant:&str)->Result<String,String>{Ok("recording-token-only".into())}}
async fn terminal_payment_client(_pool:&sqlx::PgPool,tenant:&str)->Result<TokenClient,terminal_payment_identity::Error>{LOOKUPS.lock().unwrap().push(tenant.into());Ok(TokenClient)}
'''
for name in ['extract_tenant_id_or_error','get_terminal_connection_token_handler','terminal_json_response']:
    text += extract_item(source,'function',name)+'\n'
text += '''
#[tokio::test] async fn verified_tenant_a_with_forged_header_b_looks_up_only_a(){
 use tower::ServiceExt;
 LOOKUPS.lock().unwrap().clear();
 let pool=sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://unused:unused@127.0.0.1:1/unused").unwrap();
 let app=axum::Router::new().route("/token",axum::routing::post(get_terminal_connection_token_handler))
  .with_state(Arc::new(Hub{pool})).layer(axum::Extension(crate::orchestration::AuthInfo{org_id:"tenant_a".into(),spiffe_id:"verified_a".into(),agent_id:"actor_a".into()}));
 let response=app.oneshot(axum::http::Request::builder().method("POST").uri("/token")
  .header("authorization","Bearer verified-a-fixture")
  .header("x-spiffe-id","spiffe://omnisolo/tenant_b/agent/forged")
  .body(axum::body::Body::empty()).unwrap()).await.unwrap();
 assert_eq!(response.status(),axum::http::StatusCode::OK);
 assert_eq!(*LOOKUPS.lock().unwrap(),vec!["tenant_a"]);
}
#[tokio::test] async fn header_only_token_request_is_unauthorized_before_connection_lookup(){
 use tower::ServiceExt;
 LOOKUPS.lock().unwrap().clear();
 let pool=sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://unused:unused@127.0.0.1:1/unused").unwrap();
 let app=axum::Router::new().route("/token",axum::routing::post(get_terminal_connection_token_handler)).with_state(Arc::new(Hub{pool}));
 let response=app.oneshot(axum::http::Request::builder().method("POST").uri("/token")
  .header("x-spiffe-id","spiffe://omnisolo/tenant_b/agent/forged")
  .body(axum::body::Body::empty()).unwrap()).await.unwrap();
 assert_eq!(response.status(),axum::http::StatusCode::UNAUTHORIZED);
 assert!(LOOKUPS.lock().unwrap().is_empty());
}
'''
(HERE / 'generated_token.rs').write_text(text)

queue_source = (ROOT / 'src/server/queue.rs').read_text()
(HERE / 'generated_queue.rs').write_text('use chrono::{DateTime,Utc};\nuse async_trait::async_trait;\n' + extract_item(queue_source,'struct','Job') + '\n' + extract_item(queue_source,'trait','TaskJobHandler'))
producers = 'use serde_json::{Value,json};\n'
for path, name in [
    ('src/server/api/durable_sync.rs','offline_mutation_job'),
    ('src/server/api/terminal_offline_sync.rs','terminal_offline_job'),
    ('src/server/services/pos/service.rs','grpc_offline_job'),
    ('src/server/orchestration/hybrid_sync/daemon.rs','hybrid_offline_job'),
]:
    # Only visibility changes; exact production bodies and call sites are hashed.
    producers += extract_item((ROOT / path).read_text(),'function',name).replace('fn '+name, 'pub(crate) fn '+name, 1) + '\n'
(HERE / 'generated_producers.rs').write_text(producers)
