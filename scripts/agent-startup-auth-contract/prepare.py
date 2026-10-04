"""Compile actual run_server authentication setup against non-test auth code."""
from pathlib import Path
import hashlib
import json
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
source = (ROOT/'src/server/lib.rs').read_text()
marker = '    let grpc_tls_config = grpc_tls_config_from_env(standalone)?;'
redis_marker = '\n    // Validate Redis startup before spawning workers.'
database_marker = '\n    // Initialize database'
for boundary in [marker, redis_marker, database_marker]:
    assert source.count(boundary) == 1, 'Startup extraction boundaries must be unique'
start = source.index(marker) + len(marker)
end = source.index(redis_marker)
database = source.index(database_marker)
assert start < end < database, 'Authentication must precede Redis and database startup'
# Keep the entire authentication segment, including added validations. Fail
# closed if an auth reference moves into the separately compiled Redis region.
redis_setup = source[end:database]
for auth_reference in ['builtin_agent_auth', 'omnisolo_builtin_agent::auth']:
    assert auth_reference not in redis_setup, 'Authentication moved outside the compiled auth segment'
setup = source[start:end]
assert 'let builtin_agent_auth = if standalone' in setup
assert 'omnisolo_builtin_agent::auth::auth_mode_from_env()' in setup
(HERE/'generated.rs').write_text('\n'.join([
    'extern crate self as omnisolo_builtin_agent;',
    '#[path='+json.dumps(str(ROOT/'src/agents/builtin/auth.rs'))+'] pub mod auth;',
    'pub fn actual_startup(standalone: bool) -> Result<Option<auth::AuthMode>, std::io::Error> {',
    setup, 'Ok(builtin_agent_auth)', '}',
])+'\n')
inputs = [ROOT/'src/server/lib.rs', ROOT/'src/agents/builtin/auth.rs', ROOT/'Cargo.lock',
          ROOT/'scripts/agent-startup-auth.test.mjs', ROOT/'.github/workflows/ci.yml']
inputs += [HERE/name for name in ['Cargo.toml','prepare.py','probe.rs','test.rs','run.sh','verify_lock.py','README.md']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs},indent=2)+'\n')
