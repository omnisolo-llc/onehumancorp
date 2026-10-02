"""Import entire production store/router/blueprint modules; never dispatch agents."""
from pathlib import Path
import json, hashlib, re
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
paths = {
    'agent_definitions': ROOT / 'src/server/persistence/agent_definitions.rs',
    'definitions_api': ROOT / 'src/server/api/agents/definitions.rs',
    'blueprint': ROOT / 'src/server/domain/blueprint.rs',
    **{name: ROOT / f'src/server/persistence/{name}.rs' for name in ['capabilities','connection','entities','migration']},
}
lines = ['pub mod persistence { pub use crate::{agent_definitions,capabilities,connection,entities,migration}; pub use connection::AppDatabase; }', 'pub mod domain { pub use crate::blueprint; }']
for module, path in paths.items():
    if not path.is_file():
        raise SystemExit(f'Missing required production module: {path.relative_to(ROOT)}')
    lines.append(f'#[path={json.dumps(str(path))}] pub mod {module};')
db_source=(ROOT/'src/server/db.rs').read_text()
initial=(ROOT/'src/server/migrations/001_initial.sql').read_text()
(HERE/'core_pg.sql').write_text('\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\);',initial,re.S).group() for name in ['tenants','users']))
(HERE/'core_sqlite.sql').write_text('\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\s{20}\);',db_source,re.S).group() for name in ['tenants','users']))
start=db_source.index('pub fn secure_pg_pool_options()')
end=db_source.index('\npub fn get_sqlite_pool_if_exists()',start)
lines.append('pub use server_auth as auth; pub mod db { pub enum DbStore { Postgres, Sqlite(sqlx::SqlitePool) } pub struct DB { pub pool:sqlx::PgPool,pub store:DbStore }'+db_source[start:end]+'}')
setup=(ROOT/'src/server/api/setup.rs').read_text().split('\n#[cfg(test)]\nmod tests {',1)[0]
(HERE/'setup_source.rs').write_text(setup+'\n')
growth=(ROOT/'src/e2e/growth_owner.ts').read_text()
growth_sql=re.search(r'`(WITH source_owner AS \(.*?RETURNING user_id, tenant_id)`',growth,re.S).group(1)
(HERE/'growth_owner.sql').write_text(growth_sql+'\n')
lines.append('#[path="setup_source.rs"] pub mod setup;')
commands=(ROOT/'src/server/persistence/commands.rs').read_text()
bootstrap=re.search(r'^pub async fn bootstrap_admin\(.*?^\}',commands,re.M|re.S).group()
lines.append('pub mod command_bootstrap { use crate::persistence::{AppDatabase,entities,migration}; use chrono::Utc; use sea_orm::{ActiveModelTrait,ColumnTrait,EntityTrait,QueryFilter,Set}; type CommandResult<T=()> = Result<T,Box<dyn std::error::Error>>;'+bootstrap+'}')
native=(ROOT/'src/server/persistence_commands_test.rs').read_text()
helper=re.search(r'^async fn insert_user\(.*?^\}',native,re.M|re.S).group()
test=re.search(r'^#\[tokio::test\]\nasync fn migration_backfills_portable_roles_from_existing_json_users\(.*?^\}',native,re.M|re.S).group()
lines.append('#[cfg(test)] mod original_role_migration { use crate::persistence::{AppDatabase,entities,migration}; use chrono::Utc; use sea_orm::{ActiveModelTrait,ConnectionTrait,Set,Statement};'+helper+test+'}')
lines.append('#[cfg(test)] #[path="test.rs"] mod tests;')
(HERE / 'generated.rs').write_text('\n'.join(lines) + '\n')
inputs = list(paths.values()) + [ROOT/'Cargo.toml', ROOT/'Cargo.lock', ROOT/'.github/workflows/ci.yml', ROOT/'scripts/focused_ci_gate.py', ROOT/'scripts/test_focused_ci_gate.py', ROOT/'src/server/persistence_commands_test.rs', ROOT/'src/server/persistence/commands.rs', ROOT/'src/server/api/setup.rs', ROOT/'src/e2e/growth_owner.ts', ROOT/'src/server/migrations/110_trial_extension_claim.sql', ROOT/'src/server/db.rs', ROOT/'src/server/lib.rs', ROOT/'src/server/migrations/001_initial.sql', ROOT/'src/server/persistence/mod.rs', ROOT/'src/server/api/agents/mod.rs', ROOT/'scripts/agent-definition-wiring.test.mjs', ROOT/'scripts/agent-definition-authority/test_sqlite.py', ROOT/'src/server/migrations/1018_agent_definition_marketplace.sql', ROOT/'src/server/persistence/agent_definitions_sqlite.sql', ROOT/'src/server/persistence/agent_definition_authority_pg.sql', ROOT/'src/server/persistence/agent_definition_authority_sqlite.sql']
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml','prepare.py','run.sh','verify_lock.py','test.rs','authority_test.rs','lifecycle_test.rs','writer_test.rs','database_guard.py','test_database_guard.py','README.md']]
for name in ['auth', 'common', 'config', 'oidc', 'omnisolo', 'telemetry']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
inputs += list((ROOT/'src/proto').rglob('*.proto'))
inputs += [p for p in [ROOT/'.cargo/config.toml', ROOT/'src/ui/next/src/lib/auth/authLimits.json'] if p.is_file()]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
