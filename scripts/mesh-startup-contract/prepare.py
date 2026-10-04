"""Compile the exact production mesh transports and server startup wiring."""
from pathlib import Path
import hashlib
import json
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
server = (ROOT/'src/server/lib.rs').read_text()
transport = (ROOT/'src/agents/builtin/mesh/transport.rs').read_text()
# Exclude legacy tests with ambient service assumptions, not production code.
transport = transport[:transport.index('\n#[cfg(test)]\nmod tests {')]
(HERE/'transport.rs').write_text(transport)
start_marker = '    let is_cloud = !crate::is_standalone_runtime();'
end_marker = '\n    // Initialize Handoff Manager'
assert server.count(start_marker) == server.count(end_marker) == 1
start = server.index(start_marker) + len(start_marker)
startup = server[start:server.index(end_marker, start)]
source = '\n'.join([
    '#![allow(dead_code)]',
    'extern crate self as omnisolo_builtin_agent;',
    'pub mod transport;',
    '#[path='+json.dumps(str(ROOT/'src/agents/builtin/proto.rs'))+'] pub mod proto;',
    'pub mod mesh {pub use crate::transport;}',
    'pub async fn actual_startup(redis_url:Option<String>,is_cloud:bool) -> Result<std::sync::Arc<dyn transport::MeshTransport>,Box<dyn std::error::Error+Send+Sync>> {',
    startup, 'Ok(mesh_transport)', '}',
    '#[cfg(test)]#[path="test.rs"]mod tests;',
])+'\n'
(HERE/'generated.rs').write_text(source)
inputs = [ROOT/p for p in ['Cargo.toml','Cargo.lock','src/server/lib.rs','src/agents/builtin/proto.rs','src/agents/builtin/mesh/transport.rs','.github/workflows/ci.yml','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py']]
for directory in ['src/server/omnisolo', 'src/proto']:
    inputs += [p for p in (ROOT/directory).rglob('*') if p.is_file() and p.suffix in {'.rs','.toml','.proto'}]
inputs += [p for p in HERE.iterdir() if p.is_file() and p.name not in {'Cargo.lock','source-manifest.json'}]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
print('Exact mesh transport module and startup retry call fingerprinted')
