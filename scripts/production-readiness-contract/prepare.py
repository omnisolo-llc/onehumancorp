"""Compile the exact mounted production routes, without substitute handlers."""
import hashlib
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
ROUTES = [
    '/api/v1/dashboard', '/api/v1/costs', '/api/v1/approvals/request',
    '/api/v1/approvals/decide', '/api/v1/handoffs', '/api/v1/skills/import',
    '/api/v1/snapshots/create',
]
source = (ROOT / 'src/server/lib.rs').read_text()
growth = (ROOT / 'src/server/api/growth.rs').read_text().split('\n#[cfg(test)]', 1)[0]
GROWTH_ROUTES = ['/trial-extension/claim', '/time-savings', '/campaign/generate-win-back']
assert '.nest("/api/v1/growth", api::growth::router(' in source, 'growth router must remain mounted'

def route_source(route, source=source):
    needle = '"' + route + '",'
    assert source.count(needle) == 1, f'exactly one mounted route required: {route}'
    position = source.index(needle)
    start = source.rfind('.route(', 0, position)
    depth = 0
    quoted = escaped = False
    for index in range(start + len('.route'), len(source)):
        char = source[index]
        if quoted:
            if escaped:
                escaped = False
            elif char == '\\':
                escaped = True
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char == '(':
            depth += 1
        elif char == ')':
            depth -= 1
            if depth == 0:
                return source[start:index + 1]
    raise ValueError(route)

helper = ROOT / 'src/server/api/production_readiness.rs'
parts = ['use axum::{Json, response::IntoResponse, routing::{get, post}};', 'use serde::{Deserialize, Serialize};']
for item in ['GenerateWinBackRequest', 'GenerateWinBackResponse']:
    definition = re.search(r'^#\[derive\([^\n]+\)\]\npub struct ' + item + r' \{.*?^\}', growth, re.M | re.S)
    assert definition, f'actual production declaration required: {item}'
    parts.append(definition.group())
handler = re.search(r'^async fn handle_generate_win_back\(.*?^\}', growth, re.M | re.S)
assert handler, 'actual production template handler required'
parts.append(handler.group())
if helper.exists():
    parts += [f'#[path={json.dumps(str(helper))}] pub mod production_readiness;',
              'pub mod api { pub use crate::production_readiness; }']
parts += ['pub fn actual_routes() -> axum::Router { axum::Router::new()',
          *[route_source(route) for route in ROUTES],
          '.nest("/api/v1/growth", axum::Router::new()',
          *[route_source(route, growth) for route in GROWTH_ROUTES], ')', '}',
          '#[cfg(test)] #[path="test.rs"] mod tests;']
(HERE / 'generated.rs').write_text('\n'.join(parts) + '\n')
inputs = [ROOT / 'src/server/lib.rs', ROOT / 'src/server/api/growth.rs', ROOT / 'Cargo.lock', HERE / 'Cargo.toml',
          HERE / 'prepare.py', HERE / 'test.rs', HERE / 'run.sh',
          HERE / 'verify_lock.py', HERE / 'README.md',
          ROOT / 'src/server/api/mod.rs', ROOT / 'scripts/production-readiness.test.mjs',
          ROOT / '.github/workflows/ci.yml']
if helper.exists():
    inputs.append(helper)
(HERE / 'source-manifest.json').write_text(json.dumps({
    str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
    for path in inputs
}, indent=2) + '\n')
print(f'Imported {len(ROUTES) + len(GROWTH_ROUTES)} exact mounted production routes')
