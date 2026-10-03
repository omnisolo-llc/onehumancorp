from pathlib import Path
import hashlib
import json
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
def function(path, start, end):
    source = (ROOT / path).read_text()
    return source[source.index(start):source.index(end, source.index(start))]
def module(path, name):
    return f'#[path={json.dumps(str(ROOT/path))}] pub mod {name};\n'
source = '#![allow(dead_code)]\npub use server_config as config;\n'
source += function('src/server/lib.rs', 'pub fn is_standalone_runtime()', '\nimpl From<workflow_execution::receipts::Receipt>')
source += module('src/server/redis_pool.rs', 'redis_pool')
source += 'pub mod api { pub mod mesh_handler { use axum::http::HeaderMap;\n'
source += function('src/server/api/mesh_handler.rs', 'pub fn check_spiffe_auth(', '\n/// Helper method')
source += '}\n'
for name in ['ws_compression', 'ws_batch', 'sync']:
    source += module(f'src/server/api/{name}.rs', name)
source += '}\n'
(HERE/'generated.rs').write_text(source)
paths = ['Cargo.lock', 'Cargo.toml', 'src/server/lib.rs', 'src/server/redis_pool.rs', 'src/server/api/mesh_handler.rs', 'src/server/api/sync.rs', 'src/server/api/sync_gateway.rs', 'src/server/api/ws_compression.rs', 'src/server/api/ws_batch.rs', 'scripts/ignored_rust_gate.py', 'scripts/test_ignored_rust_gate.py']
inputs = [ROOT/p for p in paths]
for directory in ['src/server/config', 'src/server/auth', 'src/server/omnisolo', 'src/server/common', 'src/server/oidc', 'src/proto']:
    inputs += [p for p in (ROOT/directory).rglob('*') if p.is_file() and p.suffix in {'.rs','.toml','.proto'}]
inputs += [p for p in HERE.iterdir() if p.is_file() and p.name not in {'Cargo.lock','generated.rs','source-manifest.json'}]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
print('Prepared whole legacy sync/Redis modules and exact config/auth helper slices')
