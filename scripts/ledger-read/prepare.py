from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
source = (ROOT / 'src/server/api/payment_ledger.rs').read_text()

def block(text, name):
    start = text.index(name)
    opening = text.index('{', start)
    depth = 0
    quoted = escaped = False
    for index in range(opening, len(text)):
        char = text[index]
        if quoted:
            if escaped:
                escaped = False
            elif char == '\\':
                escaped = True
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char == '{':
            depth += 1
        elif char == '}':
            depth -= 1
            if depth == 0:
                return text[start:index + 1]
    raise ValueError(name)

generated = '''#![allow(dead_code)]
use axum::{Json,http::StatusCode,response::IntoResponse};
use serde::Serialize;
use sqlx::Row;
pub mod db {
    pub static POOL: std::sync::RwLock<Option<sqlx::PgPool>> = std::sync::RwLock::new(None);
    pub fn get_pool() -> sqlx::PgPool { POOL.read().unwrap().as_ref().unwrap().clone() }
    pub fn get_mysql_pool_if_exists() -> Option<sqlx::MySqlPool> { None }
'''
generated += block((ROOT / 'src/server/db.rs').read_text(), 'pub fn secure_pg_pool_options()') + '\n}\n'
for name in ['LedgerEntryResponse', 'LedgerEntriesResponse']:
    generated += '#[derive(Serialize)]\n' + block(source, 'pub struct ' + name + ' {') + '\n'
generated += block(source, 'pub async fn get_entries(')
generated += '\n#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE / 'generated.rs').write_text(generated)
inputs = [ROOT / 'Cargo.lock', ROOT / 'Cargo.toml', ROOT / 'src/server/api/payment_ledger.rs',
          ROOT / 'src/server/lib.rs', ROOT / 'src/server/db.rs', ROOT / 'src/server/migrations/080_ledger.sql',
          ROOT / 'src/ui/next/src/app/api/v1/[...path]/route.ts', ROOT / 'src/ui/next/src/lib/auth/backendTransport.ts']
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml', 'prepare.py', 'test.rs', 'run.sh', 'verify_lock.py', 'README.md', 'source_contracts.py']]
for name in ['auth', 'common', 'config', 'oidc', 'omnisolo', 'telemetry']:
    inputs += [p for p in (ROOT / 'src/server' / name).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
(HERE / 'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))}, indent=2) + '\n')
