from pathlib import Path
import hashlib, json
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
source = (ROOT / "src/server/db.rs").read_text()
start = source.index("pub fn secure_pg_pool_options()")
opening = source.index("{", start)
depth = 0
for end in range(opening, len(source)):
    if source[end] == "{": depth += 1
    if source[end] == "}": depth -= 1
    if depth == 0: break
pool_options = source[start:end+1]
generated = """pub mod common { pub use server_common::*; }
pub mod db {
    pub enum DbStore { Postgres, Sqlite(sqlx::SqlitePool) }
    pub struct DB { pub pool: sqlx::PgPool, pub store: DbStore }
    pub fn get_pool() -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://unused@127.0.0.1:1/unused").unwrap()
    }
    pub async fn create_sqlite_pool_for_test() -> sqlx::SqlitePool {
        sqlx::sqlite::SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap()
    }
""" + pool_options + "\n}\n"
generated += '#[path=' + json.dumps(str(ROOT / 'src/server/api/booking/create_service.rs')) + '] pub mod create_service;\n'
generated += '#[cfg(test)] #[path="test.rs"] mod tests;\n'
(HERE / 'generated.rs').write_text(generated)
inputs=[ROOT/'.github/workflows/ci.yml', ROOT/'scripts/focused_ci_gate.py', ROOT/'scripts/test_focused_ci_gate.py', ROOT/'src/e2e/playwright/autonomous_booking_quote.spec.ts', ROOT/'Cargo.lock', ROOT/'src/server/db.rs', ROOT/'src/server/api/booking/create_service.rs', ROOT/'src/server/migrations/001_initial.sql', ROOT/'src/server/migrations/008_data_model_architecture.sql', ROOT/'src/server/migrations/1000_final_tenant_isolation_rls.sql']
inputs += [p for p in HERE.iterdir() if p.suffix in ['.py','.rs','.toml','.sh','.md'] and p.name != 'generated.rs']
for name in ['auth','common','config','oidc','omnisolo','telemetry']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
