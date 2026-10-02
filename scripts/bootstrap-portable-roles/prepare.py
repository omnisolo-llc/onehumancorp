"""Compile the complete production setup module with actual portable auth storage."""
from pathlib import Path
import json,hashlib,re
ROOT=Path(__file__).resolve().parents[2]; HERE=Path(__file__).resolve().parent
source=(ROOT/'src/server/api/setup.rs').read_text()
production,tests=source.split('\n#[cfg(test)]\nmod tests {',1)
assert 'async fn bootstrap_postgres(' in production and 'async fn bootstrap_sqlite(' in production
(HERE/'setup.rs').write_text(production+'\n')
db=(ROOT/'src/server/db.rs').read_text(); start=db.index('pub fn secure_pg_pool_options()'); end=db.index('\npub fn get_sqlite_pool_if_exists',start)
(HERE/'generated.rs').write_text('''pub use server_auth as auth;
pub mod db {
    pub enum DbStore { Postgres, Sqlite(sqlx::SqlitePool) }
    pub struct DB { pub pool: sqlx::PgPool, pub store: DbStore }
'''+db[start:end]+'''\n}
#[path="setup.rs"]pub mod setup;
#[cfg(test)]#[path="test.rs"]mod tests;
''')
initial=(ROOT/'src/server/migrations/001_initial.sql').read_text()
schema=[]
for table in ['tenants','users','revoked_tokens']:
 schema.append(re.search(r'CREATE TABLE IF NOT EXISTS '+table+r' \(.*?\n\);',initial,re.S).group())
(HERE/'postgres-schema.sql').write_text('\n'.join(schema)+'\n')
inputs=[ROOT/p for p in ['Cargo.toml','Cargo.lock','src/server/api/setup.rs','src/server/db.rs','src/server/lib.rs','src/server/persistence/migration.rs','src/server/migrations/001_initial.sql','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py','.github/workflows/ci.yml','deploy/tests/kind_e2e_test.sh']]
inputs += [p for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','generated.rs','setup.rs','postgres-schema.sql','source-manifest.json']]
for name in ['auth','common','config','oidc','omnisolo','telemetry']:
 inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
