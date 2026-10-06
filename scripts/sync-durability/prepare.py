"""Compile exact production handlers/helpers without the monolithic server crate.

The existing route tests run too; only external cache/mesh plumbing is stubbed. Bearer
validation is the real server_auth crate, including PostgreSQL revocation reads.
"""
from pathlib import Path
import hashlib, re, json
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
API=ROOT/'src/server/api'

def balanced(source, start):
    # Source blocks passed here contain no unmatched braces in Rust strings.
    # Braces inside strings must be excluded for source-bound extraction.
    at=source.index('{',start); depth=0; quoted=False; escaped=False
    for i in range(at,len(source)):
        c=source[i]
        if quoted:
            if escaped: escaped=False
            elif c=='\\': escaped=True
            elif c=='"': quoted=False
            continue
        if c=='"': quoted=True
        elif c=='{': depth+=1
        elif c=='}':
            depth-=1
            if depth==0:return i+1
    raise ValueError('unbalanced source block')

offline=(API/'offline_sync.rs').read_text()
offline=offline.replace('include!("offline_sync_route_test.rs")',f'include!({json.dumps(str(API/"offline_sync_route_test.rs"))})')
offline=offline.replace('#[path = "durable_sync.rs"]',f'#[path = {json.dumps(str(API/"durable_sync.rs"))}]')
terminal=(API/'terminal_api.rs').read_text()
a=terminal.index('#[derive(serde::Deserialize)]\npub struct PosOfflineTransaction')
b=terminal.index('#[derive(serde::Deserialize)]\npub struct EdgeLedgerTransaction',a)
types=terminal[a:b]
handler_start=terminal.index('pub async fn sync_offline_transactions_handler(')
handler=terminal[handler_start:balanced(terminal,handler_start)]
source='''#![allow(dead_code)]
extern crate self as omnisolo_builtin_agent;
pub use server_auth as auth;
pub mod mesh { pub mod transport { #[async_trait::async_trait] pub trait MeshTransport: Send + Sync { async fn publish(&self, topic:&str, event:server_omnisolo::orchestration::TeammateMeshEvent)->Result<(),String>; } #[derive(Default)] pub struct InProcessTransport; impl InProcessTransport {pub fn new()->Self{Self}} #[async_trait::async_trait] impl MeshTransport for InProcessTransport {async fn publish(&self,_topic:&str,_event:server_omnisolo::orchestration::TeammateMeshEvent)->Result<(),String>{Ok(())}} } }
pub mod utils {pub mod cache {pub use server_utils::cache::HybridCache;} pub mod edge_caching_middleware { pub fn get_cdn_cache()->crate::builder::edge::Cache {crate::builder::edge::Cache} }}
pub fn get_redis_client()->Option<redis::Client>{None}
pub mod builder { pub mod edge {
    pub struct Cache;
    pub fn get_edge_cache()->Cache{Cache}
    impl Cache {pub async fn invalidate_by_tag(&self,_tag:&str){}}
}}
pub mod db {
    static POOL:std::sync::RwLock<Option<sqlx::PgPool>>=std::sync::RwLock::new(None);
    pub static POS_READ_LOCK:tokio::sync::Mutex<()>=tokio::sync::Mutex::const_new(());
    pub fn get_pool()->sqlx::PgPool{POOL.read().unwrap().as_ref().expect("test pool configured").clone()}
    pub fn set_pool(pool:sqlx::PgPool){*POOL.write().unwrap()=Some(pool);}
    pub fn get_mysql_pool_if_exists()->Option<sqlx::MySqlPool>{None}
}
pub struct Hub;
pub mod offline_sync {\n'''+offline+'\n}\npub mod terminal_api {\nuse crate::Hub;\nuse axum::{Json,extract::State,response::IntoResponse};\nuse std::sync::Arc;\nuse tracing::info;\n'+types+'\n'+f'#[path = {json.dumps(str(API/"terminal_offline_sync.rs"))}]\nmod offline_sync;\n'+handler+'\n}\n'
source+=f'\n#[path={json.dumps(str(API / "sync_transaction.rs"))}]\nmod sync_transaction;\n'
source+=f'\npub mod api {{pub mod field_ops {{#[path={json.dumps(str(API/"field_ops/records.rs"))}] pub mod records;}}}}\n'
pos=(API/'pos.rs').read_text()
def pos_function(name):
    match=re.search(r'(?:pub )?(?:async )?fn '+name+r'\(',pos)
    if not match:raise ValueError('missing POS function '+name)
    return pos[match.start():balanced(pos,match.start())]
source+='\npub mod pos_read {use crate::Hub;use crate::utils::cache::HybridCache;use axum::{Json,extract::{Extension,State},response::IntoResponse};use serde_json::{Value,json};use sqlx::Row;use std::sync::{Arc,OnceLock};\n'
source+=re.search(r'pub static POS_ORDERS_CACHE:[^\n]+',pos)[0]+'\n'
source+=re.search(r'const POS_ORDERS_SQL:[^\n]+',pos)[0]+'\n'
for name in ['pos_tenant','fetch_pos_orders','get_orders_handler','get_inventory_handler']:
    source+=pos_function(name)+'\n'
source+='pub fn router(hub:Arc<Hub>)->axum::Router{axum::Router::new().route("/api/v1/pos/orders",axum::routing::get(get_orders_handler)).route("/api/v1/pos/inventory",axum::routing::get(get_inventory_handler)).with_state(hub)}\n'
source+='#[cfg(test)] #[tokio::test] '+pos_function('pos_orders_keep_customer_identity_and_notes_tenant_scoped')+'\n}\n'
source+='\n#[cfg(test)]\n#[path="mounted_test.rs"]\nmod mounted_test;\n'
(HERE/'generated.rs').write_text(source)
inputs=[ROOT/'Cargo.lock',ROOT/'src/server/lib.rs',HERE/'README.md',HERE/'source_contract_test.py',API/'pos.rs',ROOT/'src/server/utils/cache.rs',HERE/'prepare.py',HERE/'Cargo.toml',API/'mod.rs',API/'sync_transaction.rs',API/'offline_sync.rs',API/'offline_sync_route_test.rs',HERE/'mounted_test.rs',API/'terminal_api.rs',API/'terminal_offline_authority.rs',*API.glob('durable_sync*'),API/'durable_appointment_sync.rs',API/'field_ops/records.rs',*API.glob('terminal_offline_sync*'),ROOT/'src/server/migrations/236_pos_offline_request_identity.sql',ROOT/'src/server/migrations/234_sync_durable_receipts.sql']
manifest={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs if p.is_file()}
(HERE/'source-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(f'Prepared exact-source harness from {len(manifest)} source inputs')
