"""Compile exact production decision, store and queue code against disposable PostgreSQL.
Before the extraction repair, import the old HTTP function verbatim. Cache/pubsub
notifications are inert; they cannot change decision, authority or queue results.
"""
from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent; ROOT=HERE.parents[1]
paths=[]
def read(path):
 p=ROOT/path; paths.append(p); return p.read_text()
def table(source,name):
 return re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\);',source,re.S).group()
schema=[]
for file,names in [('001_initial.sql',['tenants','users','products','customers','orders']),('002_missing_tables.sql',['agent_approvals']),('078_quote_engine.sql',['invoices']),('1001_create_omni_inbox_messages_and_quotes_fix.sql',['omni_inbox_messages'])]:
 s=read('src/server/migrations/'+file)
 schema += [table(s,n) for n in names]
for name in ['143_agent_feed_items.sql','223_agent_action_requests.sql','227_agent_feed_table_and_sync.sql','060_job_queue_and_ledger.sql','1012_product_subscription_fields.sql']:
 schema.append(read('src/server/migrations/'+name))
schema += re.findall(r'ALTER TABLE invoices.*?;',read('src/server/migrations/114_invoicing_agent.sql'))
migration=ROOT/'src/server/migrations/1032_agent_feed_decisions.sql'
if migration.exists(): paths.append(migration); schema.append(migration.read_text())
(HERE/'schema.sql').write_text('\n'.join(schema))
(HERE/'token_fence.sql').write_text(read('src/server/persistence/token_revocation_fence_postgres.sql'))
initial=read('src/server/migrations/001_initial.sql')
(HERE/'products_rls.sql').write_text('\n'.join(re.findall(r'(?:ALTER TABLE products ENABLE ROW LEVEL SECURITY|CREATE POLICY tenant_isolation_products.*?);',initial)))
lines=['pub mod db { pub enum DbStore {Postgres,Sqlite(sqlx::SqlitePool)} pub struct DB {pub pool:sqlx::PgPool,pub store:DbStore} }','pub async fn invalidate_agent_feed_caches(_: &str) {}','pub fn get_redis_client()->Option<redis::Client>{None}']
for name,path in [('repository','src/server/domain/repository/agent_feed_repo.rs'),('agent_approvals','src/server/domain/agent_approvals.rs'),('omnisolo_job_queue','src/server/orchestration/queue/omnisolo_job_queue.rs'),('redis_lock','src/server/orchestration/queue/redis_lock.rs'),('agent_action_worker','src/server/workers/agent_action_worker.rs'),('catalog','src/server/domain/catalog.rs'),('incidents','src/server/domain/incidents.rs'),('action_router','src/server/domain/action_router.rs')]:
 paths.append(ROOT/path); lines.append(f'#[path={json.dumps(str(ROOT/path))}]pub mod {name};')
lines.append('macro_rules! provider_boundary {($name:ident,$($function:ident),+) => {pub mod $name {$(pub async fn $function(_tenant:&str,_payload:&serde_json::Value,_pool:&sqlx::PgPool)->Result<(),sqlx::Error>{panic!("Live provider dispatch forbidden in this contract")})+}};} provider_boundary!(quotes,handle_quote_action);provider_boundary!(inbox,handle_inbox_action);provider_boundary!(invoice,handle_invoice_action);provider_boundary!(booking,handle_booking_action,handle_booking_approval,handle_autonomous_quote_action);')
lines += ['pub mod domain {pub use crate::{agent_approvals,catalog,incidents,action_router,quotes,inbox,invoice,booking}; pub mod repository {pub use crate::repository as agent_feed_repo;} }','pub mod orchestration {pub mod queue {pub use crate::omnisolo_job_queue::{self,OmniSoloJobQueue};pub use crate::redis_lock;}}']
domain=ROOT/'src/server/domain/agent_feed_decisions.rs'
if domain.exists():
 paths.append(domain);lines.append(f'#[path={json.dumps(str(domain))}]pub mod agent_feed_decisions;')
 lines=[line.replace('pub use crate::{agent_approvals,','pub use crate::{agent_feed_decisions,agent_approvals,') for line in lines]
source=read('src/server/api/agent_feed.rs')
module=ROOT/'src/server/api/agent_feed/decisions.rs'
if module.exists():
 assert '.merge(decisions::router())' in source, 'The actual feed router must mount the production decision routes'
 mount=read('src/server/lib.rs')
 assert re.search(r'api::agent_feed::router\(\).*?Extension\(http_auth_store.clone\(\)\).*?strict_bearer_auth_middleware',mount,re.S), 'Mounted decisions require the canonical Store and strict bearer middleware'
 paths.append(module)
 lines.append(f'#[path={json.dumps(str(module))}]pub mod decisions;')
 lines.append('pub async fn mounted_decisions(pool:sqlx::PgPool,store:std::sync::Arc<server_auth::Store>)->axum::Router { decisions::router().with_state(pool).layer(axum::extract::Extension(store.clone())).route_layer(axum::middleware::from_fn_with_state(store,server_auth::strict_bearer_auth_middleware)) }')
else:
 request=re.search(r'#\[derive\(Deserialize\)\]\npub struct UpdateStateRequest \{.*?\n\}',source,re.S).group()
 start=source.index('async fn update_feed_item_state('); end=source.index('\n#[cfg(test)]',start)
 fn=source[start:end]
 body='use axum::{Json,extract::{Extension,Path,State},http::StatusCode,response::IntoResponse};use serde::Deserialize;use sqlx::PgPool;use server_common::Claims;use crate::repository::AgentFeedRepository;fn get_redis_client()->Option<redis::Client>{None}\n'+request+'\n'+fn
 lines.append('pub mod decisions {'+body+'pub fn router()->axum::Router<sqlx::PgPool>{ axum::Router::new().route("/api/v1/agent-feed/{id}",axum::routing::put(update_feed_item_state)).route("/api/v1/agent-feed/{id}/state",axum::routing::put(update_feed_item_state))}}')
 lines.append('pub async fn mounted_decisions(pool:sqlx::PgPool,store:std::sync::Arc<server_auth::Store>)->axum::Router { decisions::router().with_state(pool).layer(axum::extract::Extension(store.clone())).route_layer(axum::middleware::from_fn_with_state(store,server_auth::strict_bearer_auth_middleware)) }')
worker_guard=ROOT/'src/server/workers/agent_feed_dispatch.rs'
if worker_guard.exists():
 paths.append(worker_guard);lines.append(f'#[path={json.dumps(str(worker_guard))}]pub mod agent_feed_dispatch;')
 lines.append('pub mod workers {pub use crate::agent_feed_dispatch;}')
lines.append('#[cfg(test)]#[path="test.rs"]mod contract;')
(HERE/'generated.rs').write_text('\n'.join(lines)+'\n')
paths += [ROOT/'Cargo.lock',ROOT/'src/server/lib.rs',ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py']
for folder in ['src/server/auth','src/server/common']:
 paths += list((ROOT/folder).rglob('*.rs'))
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','source-manifest.json']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
print('Exact production decision/store/queue imports and migration source fingerprinted')
