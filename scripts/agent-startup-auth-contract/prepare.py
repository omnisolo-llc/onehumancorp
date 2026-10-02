"""Compile actual run_server authentication setup against non-test auth code."""
from pathlib import Path
import hashlib
import json
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
source = (ROOT/'src/server/lib.rs').read_text()
marker = '    let grpc_tls_config = grpc_tls_config_from_env(standalone)?;'
assert source.count(marker) == 1
start = source.index(marker) + len(marker)
end = source.index('\n    // Initialize database', start)
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
