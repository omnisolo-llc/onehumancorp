from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
source=(ROOT/'src/server/migrations/001_initial.sql').read_text()
tables=['tenants','users','products','customers','orders','order_items']
schema='\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\);',source,re.S).group() for name in tables)
db_source=(ROOT/'src/server/db.rs').read_text()
sqlite='\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\s{20}\);',db_source,re.S).group() for name in ['orders','delivery_tasks'])
(HERE/'sqlite_schema.sql').write_text(sqlite+'\n')
(HERE/'schema.sql').write_text(schema+'\n'+(ROOT/'src/server/migrations/102_delivery_task_provider_tracking.sql').read_text()+'\n'+(ROOT/'src/server/migrations/1029_delivery_provider_bindings.sql').read_text())
modules={'fulfillment':'src/server/api/fulfillment.rs','shipping':'src/server/api/shipping.rs'}
lines=['pub use server_common as common;','pub mod api { pub use crate::{fulfillment,shipping}; }','pub mod integrations { pub use server_integrations_shippo as shippo; }','pub mod db { pub enum DbStore { Postgres,Sqlite(sqlx::SqlitePool) } pub struct DB { pub pool:sqlx::PgPool,pub store:DbStore } }']
for name,path in modules.items(): lines.append(f'#[path={json.dumps(str(ROOT/path))}]pub mod {name};')
parent=(ROOT/'src/server/lib.rs').read_text()
start=parent.index('        .nest(\n            "/api/v1/fulfillment",')
end=parent.index('        .nest("/api/v1/staff"',start)
mount=parent[start:end]
shipping_start=parent.index('        .nest(\n            "/api/v1/shipping",')
shipping_end=parent.index('        .nest("/api/v1/checkout"',shipping_start)
mount+=parent[shipping_start:shipping_end]
helper_start=parent.index('async fn protected_bearer_auth_middleware(')
helper=parent[helper_start:parent.index('\n}\n',helper_start)+3]
tenant_source=(ROOT/'src/server/utils/tenant_middleware.rs').read_text()
bypass_start=tenant_source.index('pub fn is_auth_bypass_path(')
bypass=tenant_source[bypass_start:tenant_source.index('\n}\n',bypass_start)+3]
lines.append('extern crate self as server_utils; pub mod tenant_middleware {'+bypass+'}'+helper)
global_start=parent.index('        .route_layer(axum::middleware::from_fn_with_state(\n            http_auth_store.clone(),\n            protected_bearer_auth_middleware,',end)
global_end=parent.index('        .with_state(mesh_transport)',global_start)
global_auth=parent[global_start:global_end]
public_marker='        .nest(\n            "/api/v1/fulfillment",\n            api::fulfillment::webhook_router(db.pool.clone()),\n        )'
public_start=parent.find(public_marker,global_end)
assert public_start>=0, 'Provider webhook mount must be outside global owner middleware'
public_mount=parent[public_start:public_start+len(public_marker)]
if public_start>=0:
 assert end<parent.index('tenant_middleware::tenant_middleware',end)<global_start<global_end<public_start
shipping_setup=re.search(r'let shipping_access\s*=\s*.*?;',parent,re.S).group()
lines.append('pub async fn actual_parent_mount(db:std::sync::Arc<db::DB>,http_auth_store:std::sync::Arc<server_auth::Store>)->axum::Router {'+shipping_setup+' axum::Router::new()'+mount+global_auth+public_mount+'}')
startup='crate::api::fulfillment::storage::ensure_sqlite_schema(sqlite_pool).await?;'
assert startup in db_source, 'SQLite bootstrap must invoke the real shipping schema upgrade'
lines.append('pub async fn actual_sqlite_shipping_startup(sqlite_pool:&sqlx::SqlitePool)->Result<(),sqlx::Error>{'+startup+'Ok(())}')
lines.append('#[cfg(test)]#[path="test.rs"]mod contract;')
(HERE/'generated.rs').write_text('\n'.join(lines)+'\n')
paths=[ROOT/p for p in [*modules.values(),'src/server/lib.rs','src/server/utils/tenant_middleware.rs','src/server/db.rs','Cargo.toml','Cargo.lock','.github/workflows/ci.yml','scripts/focused_ci_gate.py','src/server/migrations/001_initial.sql','src/server/migrations/102_delivery_task_provider_tracking.sql','src/server/migrations/1029_delivery_provider_bindings.sql']]
for folder in ['src/server/api/fulfillment','src/server/api/shipping','src/server/integrations/shippo','src/server/common','src/server/config','src/server/auth','src/server/oidc','src/server/omnisolo','src/server/telemetry','src/server/integrations/core','src/server/integrations/omnichannel']:
 paths += [p for p in (ROOT/folder).rglob('*') if p.is_file() and (p.suffix in ('.rs','.sql') or p.name=='Cargo.toml')]
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','source-manifest.json']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
print('Imported complete production shipping/fulfillment modules and recorded exact source inputs; DB adapter supplies only fields, never persistence behavior.')
