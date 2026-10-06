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
for file,names in [('001_initial.sql',['tenants','users','products','customers','orders']),('002_missing_tables.sql',['agent_approvals']),('078_quote_engine.sql',['invoices']),('1001_create_omni_inbox_messages_and_quotes_fix.sql',['omni_inbox_messages']),('118_incident_resolution.sql',['incidents'])]:
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
pool_source=read('src/server/db.rs')
secure_pool=pool_source[pool_source.index('pub fn secure_pg_pool_options()'):pool_source.index('pub fn get_sqlite_pool_if_exists()')]
lines=['pub mod db { pub enum DbStore {Postgres,Sqlite(sqlx::SqlitePool)} pub struct DB {pub pool:sqlx::PgPool,pub store:DbStore} '+secure_pool+' }','pub async fn invalidate_agent_feed_caches(_: &str) {}','pub fn get_redis_client()->Option<redis::Client>{None}']
for file,modules in [('src/server/domain/mod.rs',['catalog','agent_feed_decisions']),('src/server/workers/mod.rs',['agent_action_worker','agent_feed_dispatch','agent_catalog_dispatch'])]:
 source=read(file)
 for module in modules: assert re.search(r'^pub mod '+module+r';$',source,re.M), f'Production registration missing: {module}'
startup=read('src/server/lib.rs')
start=startup.index('    if legacy_sqlx_background_enabled {\n        let agent_action_worker =')
brace=startup.index('{',start);depth=1;end=brace+1
while depth:
 if startup[end]=='{':depth+=1
 elif startup[end]=='}':depth-=1
 end+=1
worker_start=startup[start:end]
assert startup.index('let http_auth_store =') < start
assert '.with_authority(http_auth_store.as_ref())' in worker_start
assert 'agent_action_worker.start()' in worker_start
assert startup.count('AgentActionWorker::new(')==1, 'Do not start an unbound duplicate worker'
lines.append('pub async fn configured_worker_start(db: &db::DB, http_auth_store: std::sync::Arc<server_auth::Store>, legacy_sqlx_background_enabled: bool) {'+worker_start+'}')
for name,path in [('repository','src/server/domain/repository/agent_feed_repo.rs'),('agent_approvals','src/server/domain/agent_approvals.rs'),('omnisolo_job_queue','src/server/orchestration/queue/omnisolo_job_queue.rs'),('redis_lock','src/server/orchestration/queue/redis_lock.rs'),('agent_action_worker','src/server/workers/agent_action_worker.rs'),('agent_catalog_dispatch','src/server/workers/agent_catalog_dispatch.rs'),('catalog','src/server/domain/catalog.rs'),('incidents','src/server/domain/incidents.rs'),('action_router','src/server/domain/action_router.rs')]:
 paths.append(ROOT/path); lines.append(f'#[path={json.dumps(str(ROOT/path))}]pub mod {name};')
lines.append('macro_rules! provider_boundary {($name:ident,$($function:ident),+) => {pub mod $name {$(pub async fn $function(_tenant:&str,_payload:&serde_json::Value,_pool:&sqlx::PgPool)->Result<(),sqlx::Error>{panic!("Live provider dispatch forbidden in this contract")})+}};} provider_boundary!(quotes,handle_quote_action);provider_boundary!(inbox,handle_inbox_action);provider_boundary!(invoice,handle_invoice_action);provider_boundary!(booking,handle_booking_action,handle_booking_approval,handle_autonomous_quote_action);')
lines += ['pub mod domain {pub use crate::{agent_approvals,catalog,incidents,action_router,quotes,inbox,invoice,booking}; pub mod repository {pub use crate::repository as agent_feed_repo;} }','pub mod orchestration {pub mod queue {pub use crate::omnisolo_job_queue::{self,OmniSoloJobQueue};pub use crate::redis_lock;}}']
domain=ROOT/'src/server/domain/agent_feed_decisions.rs'
if domain.exists():
 paths.append(domain);lines.append(f'#[path={json.dumps(str(domain))}]pub mod agent_feed_decisions;')
 lines=[line.replace('pub use crate::{agent_approvals,','pub use crate::{agent_feed_decisions,agent_approvals,') for line in lines]
source=read('src/server/api/agent_feed.rs')
# Compile exact existing presentation closures separately from canonical rows.
# These are pure DTO projections, never replacement persistence or authority.
def braced_item(text,start):
 brace=text.index('{',start);depth=1;end=brace+1
 while depth:
  if text[end]=='{':depth+=1
  elif text[end]=='}':depth-=1
  end+=1
 return text[start:end]
mobile_start=source.index('pub struct MobileAgentFeedItem {')
mobile_start=source.rfind('#[derive',0,mobile_start)
mobile_type=braced_item(source,mobile_start)
lines.append('pub mod api {pub mod agent_feed {use serde::{Serialize,Deserialize};use chrono::{DateTime,Utc};'+mobile_type+'}}')
row_type='crate::repository::AgentFeedItem'
mobile_type='crate::api::agent_feed::MobileAgentFeedItem'
def projection(name,text,marker,input_type,output_type,expected):
 matches=list(re.finditer(re.escape(marker),text))
 assert len(matches)==expected, f'Actual {name} projection inventory changed'
 for i,match in enumerate(matches):
  closure_start=match.start()+len(marker)
  if '|item|' in marker: closure_start=text.index('|item|',match.start())
  closure=braced_item(text,closure_start)
  imports='use crate::api::agent_feed::MobileAgentFeedItem;' if name=='api_mobile_projection' else ''
  lines.append(f'pub fn {name}_{i}(item:{input_type})->{output_type} {{'+imports+f'let project:fn({input_type})->{output_type}='+closure+'; project(item)}')
projection('api_mobile_projection',source,'.map(|item| MobileAgentFeedItem ',row_type,mobile_type,1)
triage=read('src/server/api/work_triage.rs')
projection('triage_mobile_projection',triage,'items.into_iter().map(|item| crate::api::agent_feed::MobileAgentFeedItem ',row_type,mobile_type,2)
projection('triage_mobile_response',triage,'let items: Vec<serde_json::Value> = mob_resp.items.into_iter().map(',mobile_type,'serde_json::Value',2)
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
 lines.append('pub mod workers {pub use crate::{agent_action_worker,agent_feed_dispatch,agent_catalog_dispatch};}')
lines.append('#[cfg(test)]#[path="test.rs"]mod contract;')
from prepare_legacy import extend_legacy_contract
extend_legacy_contract(ROOT, HERE, paths, lines, read)
(HERE/'generated.rs').write_text('\n'.join(lines)+'\n')
paths += [ROOT/'Cargo.lock',ROOT/'src/server/lib.rs',ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py']
for folder in ['src/server/auth','src/server/common']:
 paths += list((ROOT/folder).rglob('*.rs'))
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','source-manifest.json']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
print('Exact production decision/store/queue imports and migration source fingerprinted')
