"""Import complete mounted field handlers; only process-wide mesh/Hub are adapters."""
from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
modules={'field_ops':'src/server/api/field_ops.rs','field_service_routing':'src/server/api/field_service_routing.rs','cache':'src/server/utils/cache.rs','offline_sync':'src/server/api/offline_sync.rs','sync_transaction':'src/server/api/sync_transaction.rs','terminal_offline_authority':'src/server/api/terminal_offline_authority.rs'}
parts=['''extern crate self as omnisolo_builtin_agent;
extern crate self as server_utils;
pub mod common { pub use server_common::auth_utils; }
pub mod auth { pub use server_auth::*; }
pub mod proto { pub use server_omnisolo::orchestration as hub; }
pub mod mesh { pub mod transport {
 #[async_trait::async_trait] pub trait MeshTransport:Send+Sync {
  async fn publish(&self,topic:&str,event:server_omnisolo::orchestration::TeammateMeshEvent)->Result<(),String>;
 }
 #[derive(Default)] pub struct InProcessTransport {pub events:std::sync::Mutex<Vec<(String,server_omnisolo::orchestration::TeammateMeshEvent)>>}
 impl InProcessTransport { pub fn new()->Self {Self::default()} }
 #[async_trait::async_trait] impl MeshTransport for InProcessTransport {
  async fn publish(&self,topic:&str,event:server_omnisolo::orchestration::TeammateMeshEvent)->Result<(),String>{self.events.lock().unwrap().push((topic.into(),event));Ok(())}
 }
}
}
pub fn get_redis_client()->Option<redis::Client>{None}
pub mod builder {pub mod edge {pub struct Cache;pub fn get_edge_cache()->Cache{Cache}impl Cache {pub async fn invalidate_by_tag(&self,_tag:&str){}}}}
pub mod utils {pub mod edge_caching_middleware {pub fn get_cdn_cache()->crate::builder::edge::Cache{crate::builder::edge::Cache}}}
pub mod db {pub struct DB {pub pool:sqlx::PgPool} impl DB {pub fn postgres_pool(&self)->Option<&sqlx::PgPool>{Some(&self.pool)}}}
pub mod hub {
 #[derive(Default)]pub struct Hub;
 impl Hub {
  pub fn redis_client(&self)->Option<redis::Client>{None}
  pub async fn publish_teammate_event(&self,_topic:String,_event:server_omnisolo::orchestration::TeammateMeshEvent)->Result<(),String>{Ok(())}
 }
}
''']
for name,path in modules.items():
 target=ROOT/path
 if name=='offline_sync':
  text=target.read_text().replace('include!("offline_sync_route_test.rs")',f'include!({json.dumps(str(ROOT/"src/server/api/offline_sync_route_test.rs"))})').replace('#[path = "durable_sync.rs"]',f'#[path={json.dumps(str(ROOT/"src/server/api/durable_sync.rs"))}]')
  target=HERE/'generated_offline_sync.rs';target.write_text(text)
 if name=='field_ops':
  text=target.read_text()
  for child in (ROOT/'src/server/api/field_ops').glob('*.rs'):
   text=re.sub(r'(?m)^(pub(?:\(crate\))? )?mod '+child.stem+r';',lambda m:f'#[path={json.dumps(str(child))}] '+m.group(0),text)
  target=HERE/'generated_field_ops.rs';target.write_text(text)
 parts.append(f'#[path={json.dumps(str(target))}] pub mod {name};')
parts.append('pub mod api { pub use crate::{field_ops,field_service_routing,offline_sync,terminal_offline_authority}; }')
source=(ROOT/'src/server/lib.rs').read_text()
field_setup=re.search(r'let field_ops_pool\s*=\s*.*?;',source,re.S).group()
field_setup+='\n'+re.search(r'let sync_events_state\s*=\s*.*?;',source,re.S).group()
field_setup+='\n'+re.search(r'let sync_write_state\s*=\s*.*?;',source,re.S).group()
mounts=[]
for prefix in ['/api/v1/field-ops','/api/v1/field-service-routing','/api/v1/sync/events','/api/v1/sync/offline','/api/v1/sync/operation-intents']:
 line=next(line.strip() for line in source.splitlines() if f'"{prefix}"' in line)
 mounts.append(line)
parts.append('''pub async fn actual_mount(db:std::sync::Arc<db::DB>,http_auth_store:std::sync::Arc<server_auth::Store>)->axum::Router {
 let mesh_transport:std::sync::Arc<dyn mesh::transport::MeshTransport>=std::sync::Arc::new(mesh::transport::InProcessTransport::new());
 let hub=std::sync::Arc::new(hub::Hub);
 '''+field_setup+'''
 axum::Router::new()'''+''.join(mounts)+'\n}\n#[cfg(test)] #[path="test.rs"]mod tests;')
(HERE/'generated.rs').write_text('\n'.join(parts)+'\n')
initial=(ROOT/'src/server/migrations/001_initial.sql').read_text()
schema=[re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\);',initial,re.S).group() for name in ['tenants','users','customers']]
schema.extend((ROOT/'src/server/migrations'/name).read_text() for name in ['162_field_ops_appointments.sql','222_field_ops_and_global_commerce.sql','226_applied_client_mutations.sql'])
missing=(ROOT/'src/server/migrations/002_missing_tables.sql').read_text()
schema.append(re.search(r'CREATE TABLE IF NOT EXISTS department_tasks \(.*?\n\);',missing,re.S).group())
receipt=ROOT/'src/server/migrations/1030_field_mutation_receipts.sql'
if receipt.exists():schema.append(receipt.read_text())
sync=(ROOT/'src/server/api/durable_sync_test_schema.sql').read_text()
for name in ['sync_events','sync_conflict_queue','products','orders','inventory_levels','ohc_job_queue']:
 schema.append(re.search(r'CREATE TABLE '+name+r' \(.*?;',sync,re.S).group())
schema.append((ROOT/'src/server/migrations/234_sync_durable_receipts.sql').read_text())
(HERE/'schema.sql').write_text('\n'.join(schema)+'\n')

paths=[ROOT/p for p in ['.github/workflows/ci.yml','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py',*modules.values(),'src/server/api/field_ops/appointments.rs','src/server/lib.rs','Cargo.toml','Cargo.lock']]
paths.extend((ROOT/'src/server/api/field_ops').glob('*.rs'))
paths.extend((ROOT/'src/server/api').glob('*sync*'))
paths.append(ROOT/'src/server/migrations/234_sync_durable_receipts.sql')
paths.extend((ROOT/'src/server/migrations').glob('*field*.sql'))
paths.extend(p for name in ['auth','common','config','oidc','omnisolo','telemetry'] for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
paths.extend(p for p in HERE.iterdir() if p.is_file() and p.name not in ['source-manifest.json','Cargo.lock'])
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
