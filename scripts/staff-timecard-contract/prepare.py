"""Bind complete clock modules and exact mounted/schema fragments to this gate.

The tiny DB container and reduced staff router are structural adapters only.
No clock persistence, canonical authorization, receipt, or commit body is copied,
rewritten, stubbed, or reimplemented. Fragment offsets are UTF-8 byte offsets.
"""
from pathlib import Path
import hashlib
import json
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
FRAGMENTS = []
INPUTS = set()


def read(path):
    source = ROOT / path
    INPUTS.add(source)
    return source.read_text()


def record(path, start, end, label):
    text = read(path)
    value = text[start:end]
    FRAGMENTS.append({
        'label': label,
        'path': path,
        'start_byte': len(text[:start].encode()),
        'end_byte_exclusive': len(text[:end].encode()),
        'start_line': text[:start].count('\n') + 1,
        'end_line_exclusive': text[:end].count('\n') + 1,
        'sha256': hashlib.sha256(value.encode()).hexdigest(),
        'literal': value,
    })
    return value


def unique(path, pattern, label, flags=0):
    matches = list(re.finditer(pattern, read(path), flags))
    if len(matches) != 1:
        raise ValueError(f'{label}: expected one exact production fragment, found {len(matches)}')
    match = matches[0]
    return record(path, match.start(), match.end(), label)


def between(path, start_marker, end_marker, label):
    text = read(path)
    if text.count(start_marker) != 1:
        raise ValueError(f'{label}: missing or ambiguous start marker')
    start = text.index(start_marker)
    end = text.index(end_marker, start + len(start_marker))
    return record(path, start, end, label)


def top_level_item(path, marker, label):
    text = read(path)
    if text.count(marker) != 1:
        raise ValueError(f'{label}: missing or ambiguous item marker')
    start = text.index(marker)
    # All selected items are top-level; nested closing braces are indented.
    end = text.index('\n}', start) + len('\n}')
    return record(path, start, end, label)


def whole_module(path, name, visibility='pub'):
    text = read(path)
    record(path, 0, len(text), f'whole module: {name}')
    return f'#[path={json.dumps(str(ROOT / path))}]{visibility} mod {name};'


def prepare():
    staff = 'src/server/api/staff_mesh.rs'
    parent = 'src/server/lib.rs'
    db = 'src/server/db.rs'
    fixture_path = 'src/server/api/staff_timecards_test/fixture.rs'
    modules = 'src/server/api/mod.rs'

    unique(modules, r'^pub mod staff_timecards;$', 'production clock module registration', re.M)
    unique(modules, r'^#\[cfg\(test\)\]\nmod staff_timecards_test;$', 'native clock test registration', re.M)
    handler_import = unique(staff, r'^use crate::api::staff_timecards::\{\s*sync_timecard_handler,\s*timecard_receipt_handler,?\s*\};$', 'real POST handler import', re.M)
    response = top_level_item(staff, '#[derive(Serialize)]\npub struct GetTimecardResponse', 'GET response DTO')
    tenant = between(staff, 'fn get_tenant_id(', 'pub async fn create_staff_handler(', 'GET signed tenant helper')
    get_handler = between(staff, 'pub async fn get_timecard_handler(', 'pub async fn create_task_handler(', 'whole mounted GET handler')
    route = unique(staff, r'        \.route\(\s*"/timecard",\s*post\(sync_timecard_handler\)\.get\(get_timecard_handler\),\s*\)', 'timecard method route')
    receipt_route = unique(staff, r'        \.route\(\s*"/timecard/receipts/\{id\}",\s*axum::routing::get\(timecard_receipt_handler\),\s*\)', 'receipt recovery method route')
    router_signature = unique(staff, r"pub fn router<S: Clone \+ Send \+ Sync \+ 'static>\(db: Arc<DB>\) -> Router<S> \{", 'staff router signature')
    router_state = unique(staff, r'^        \.with_state\(db\)$', 'staff router state', re.M)
    secure_pool = top_level_item(db, 'pub fn secure_pg_pool_options()', 'real pooled tenant reset hooks')

    access = unique(parent, r'let timecard_access\s*=\s*api::staff_timecards::TimecardAccess::configured\(\s*&db,\s*http_auth_store.clone\(\),?\s*\)\s*\.await;', 'canonical access configuration')
    mount = unique(parent, r'\.nest\(\s*"/api/v1/staff",\s*api::staff_mesh::router\(db.clone\(\)\)\s*\.layer\(axum::Extension\(timecard_access\)\),?\s*\)', 'actual staff parent mount')
    auth_layer = unique(parent, r'        \.route_layer\(axum::middleware::from_fn_with_state\(\s*http_auth_store.clone\(\),\s*protected_bearer_auth_middleware,\s*\)\)', 'actual global protected bearer layer')
    if not read(parent).index(access) < read(parent).index(mount) < read(parent).index(auth_layer):
        raise ValueError('Staff mount must remain inside the protected bearer scope')
    protected = top_level_item(parent, 'async fn protected_bearer_auth_middleware(', 'actual bearer middleware')
    bypass = top_level_item('src/server/utils/tenant_middleware.rs', 'pub fn is_auth_bypass_path(', 'actual authentication bypass allowlist')

    schema = []
    for table in ['ohc_staff_member', 'ohc_timecard_event']:
        schema.append(unique(db, rf'CREATE TABLE IF NOT EXISTS {table} \(.*?\n                    \);', f'actual SQLite {table} bootstrap', re.S))
    sqlite_upgrade = unique(db, r'ensure_sqlite_column\(\s*sqlite_pool,\s*"ohc_timecard_event",\s*"request_identity",\s*"TEXT",?\s*\)\s*\.await\?;', 'actual SQLite receipt upgrade')
    upgrade_helper = top_level_item(db, 'async fn ensure_sqlite_column(', 'whole SQLite additive column helper')
    fixture = read(fixture_path)
    for dependency in ['include_str!("../../db.rs")', '1027_staff_mesh_native.sql', '1035_staff_timecard_receipts.sql', 'TimecardAccess::configured']:
        if dependency not in fixture:
            raise ValueError(f'Clock fixture must consume actual production dependency: {dependency}')
    if 'const SQLITE_BUSINESS:' in fixture:
        raise ValueError('Clock fixture must extract actual SQLite bootstrap, not maintain a parallel schema')
    for migration in ['1027_staff_mesh_native.sql', '1035_staff_timecard_receipts.sql']:
        path = 'src/server/migrations/' + migration
        text = read(path)
        record(path, 0, len(text), f'actual PostgreSQL migration: {migration}')

    lines = [
        'pub mod db { use sqlx::{PgPool,SqlitePool,Row};',
        'pub enum DbStore { Postgres, Sqlite(SqlitePool) }',
        'pub struct DB { pub pool:PgPool, pub store:DbStore }',
        secure_pool,
        upgrade_helper,
        'pub async fn actual_sqlite_clock_upgrade(sqlite_pool:&SqlitePool)->Result<(),sqlx::Error>{' + sqlite_upgrade + 'Ok(())}',
        '}',
        'pub mod api {',
        whole_module('src/server/api/staff_timecards.rs', 'staff_timecards'),
        whole_module('src/server/api/sync_transaction.rs', 'sync_transaction', 'pub(crate)'),
        'pub mod staff_mesh { use crate::db::DB; use axum::{Json,Router,extract::{Extension,State},response::IntoResponse,routing::post}; use serde::Serialize; use std::sync::Arc;',
        handler_import, response, tenant, get_handler,
        router_signature + 'Router::new()' + route + receipt_route + router_state + '}',
        '}',
        '#[cfg(test)]' + whole_module('src/server/api/staff_timecards_test.rs', 'staff_timecards_test', 'pub(crate)'),
        '}',
        'extern crate self as server_utils; pub mod tenant_middleware {' + bypass + '}',
        protected,
        'pub async fn actual_parent_timecard_app(db:std::sync::Arc<db::DB>,http_auth_store:std::sync::Arc<server_auth::Store>)->axum::Router {' + access + 'axum::Router::new()' + mount + auth_layer + '}',
        'pub const ACTUAL_SQLITE_BUSINESS:&str=' + json.dumps('\n'.join(schema)) + ';',
        '#[cfg(test)]#[path="schema_test.rs"]mod schema_contract;',
    ]
    (HERE / 'generated.rs').write_text('\n'.join(lines) + '\n')
    INPUTS.update(ROOT / path for path in [
        'Cargo.toml', 'Cargo.lock', 'Makefile', '.github/workflows/ci.yml',
        'scripts/focused_ci_gate.py', 'scripts/test_focused_ci_gate.py',
        'scripts/agent-feed-decision-contract/database_guard.py',
        'scripts/staff-read-contract/prepare.py',
    ])
    # These are the real local path dependencies in the preserved lock graph.
    for directory in ['auth', 'common', 'config', 'telemetry', 'oidc', 'omnisolo',
                      'integrations/core', 'integrations/omnichannel', 'integrations/shippo']:
        INPUTS.update(path for path in (ROOT / 'src/server' / directory).rglob('*')
                      if path.is_file() and (path.suffix in ('.rs', '.sql') or path.name == 'Cargo.toml'))
    INPUTS.update(path for path in HERE.iterdir()
                  if path.is_file() and path.name != 'source-manifest.json')
    proof = {
        'format_version': 1,
        'input_hashes': {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
                         for path in sorted(INPUTS)},
        'production_fragments': FRAGMENTS,
        'adapters': 'DB fields and reduced staff router only. Whole POST module, commit helper, real canonical auth crates and native tests are compiled unchanged. Exact GET, route, parent access/mount/auth and SQLite upgrade fragments are compiled unchanged.',
    }
    (HERE / 'source-manifest.json').write_text(json.dumps(proof, indent=2) + '\n')
    print('Bound whole runtime clock modules, exact route/access/auth/schema fragments and native tests')


if __name__ == '__main__':
    prepare()
