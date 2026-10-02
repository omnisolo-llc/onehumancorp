from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
db_source=(ROOT/'src/server/db.rs').read_text()
active=re.search(r'static POSTGRES_MIGRATOR[^;]+sqlx::migrate!\("([^"]+)"\)',db_source)
if not active or (ROOT/active[1]).resolve()!=ROOT/'src/server/migrations':raise SystemExit('Widget fixtures must use the actual active PostgreSQL migration source')
source=(ROOT/'src/server/lib.rs').read_text();start=source.index('    let widget_router =');end=source.index('    let oauth_callback_router:',start)
mount=source[start:end]
core_sqlite='\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\s{20}\);',db_source,re.S).group() for name in ['tenants','users','revoked_tokens'])
(HERE/'auth_sqlite.sql').write_text(core_sqlite+'\n')
pool_start=db_source.index('pub fn secure_pg_pool_options()')
pool_end=db_source.index('\npub fn get_sqlite_pool_if_exists()',pool_start)
pool_helper=db_source[pool_start:pool_end]
assert '.nest("/api/widget", api::widget::router(' in mount
assert '.merge(widget_router)' in source
modules={'widget':'src/server/api/widget/mod.rs','omnichannel_repo':'src/server/domain/repository/omnichannel_repo.rs','chat_models':'src/server/services/chat/models.rs'}
lines=['pub mod services { pub mod chat { pub use crate::chat_models as models; } }','pub mod domain { pub mod repository { pub use crate::omnichannel_repo; } }','pub mod api { pub use crate::widget; }','pub mod db { pub enum DbStore { Postgres,Sqlite(sqlx::SqlitePool) } pub struct DB { pub pool:sqlx::PgPool,pub store:DbStore }'+pool_helper+'}']
for name,path in modules.items():lines.append(f'#[path={json.dumps(str(ROOT/path))}]pub mod {name};')
lines.append('pub fn actual_parent_mount(db:std::sync::Arc<db::DB>,http_auth_store:std::sync::Arc<server_auth::Store>)->axum::Router { let _=&http_auth_store;\n'+mount+'\naxum::Router::new().merge(widget_router)\n}')
lines.append('#[cfg(test)]#[path="test.rs"]mod tests;')
generated='\n'.join(lines)+'\n';(HERE/'generated.rs').write_text(generated)
paths=[ROOT/p for p in [*modules.values(),'src/server/api/widget/chat.rs','src/server/lib.rs','src/server/db.rs','Cargo.toml','Cargo.lock','src/server/migrations/233_chat_omnichannel.sql','src/server/migrations/1021_chat_sender_identity_text.sql','src/server/services/chat/service.rs']]
paths += [p for name in ['auth','common','config','oidc','omnisolo','telemetry'] for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
paths += list((ROOT/'src/proto').rglob('*.proto'))
paths += list((HERE/'compatibility').glob('*.sql'))
paths += [p for p in [ROOT/'src/server/migrations/1009_native_omnichannel_chat.sql'] if p.is_file()]
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in ['generated.rs','Cargo.lock','source-manifest.json','auth_sqlite.sql']]
manifest={'inputs':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},'actual_pool_helper_sha256':hashlib.sha256(pool_helper.encode()).hexdigest(),'actual_parent_mount_sha256':hashlib.sha256(mount.encode()).hexdigest(),'generated_sha256':hashlib.sha256(generated.encode()).hexdigest()}
(HERE/'source-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Imported whole real widget router/handlers/repository and exact parent mount')
