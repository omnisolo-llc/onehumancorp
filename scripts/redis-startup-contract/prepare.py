"""Compile exact production queue and startup statements without the monolithic server."""
from pathlib import Path
import hashlib
import json
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
server = (ROOT/'src/server/lib.rs').read_text()
queue = (ROOT/'src/server/queue.rs').read_text()
# The legacy queue tests opportunistically access ambient PostgreSQL. Compile the
# whole production module, with this gate's explicit owned-service tests only.
queue = queue[:queue.index('\n#[cfg(test)]\nmod tests {')]
(HERE/'queue.rs').write_text(queue)
start = server.index('    let omnisolo_job_queue: std::sync::Arc<dyn crate::queue::TaskQueue> =')
end = server.index(';\n', server.index('\n        }', start)) + 2
selection = server[start:end]
preflight = ''
if '    // Validate Redis startup before spawning workers.' in server:
    start = server.index('    // Validate Redis startup before spawning workers.')
    end = server.index('\n    // Initialize database', start)
    preflight = server[start:end]
source = '''#![allow(dead_code)]
pub mod queue;
pub mod db {
    pub enum DbStore {Postgres, Sqlite(sqlx::SqlitePool)}
    pub struct DB {pub pool:sqlx::PgPool, pub store:DbStore}
}
pub async fn initialize(db:&db::DB, standalone:bool)
 -> Result<std::sync::Arc<dyn queue::TaskQueue>, Box<dyn std::error::Error+Send+Sync>> {
    let is_standalone = standalone;
    let _ = is_standalone;
'''+preflight+'\n    '+ ('let _ = rate_limit_redis_client;' if preflight else '') +'\n'+selection+'\n    Ok(omnisolo_job_queue)\n}\n'
agent = (ROOT/'src/agents/builtin/agent.rs').read_text()
start = agent.index('pub fn agent_task_timeout()')
helper = agent[start:agent.index('\n}', start)+2]
source += 'extern crate self as omnisolo_builtin_agent; pub mod agent { '+helper+' }\n'
source += 'pub use server_config as config;\n'
start = server.index('pub fn is_standalone_runtime()')
source += server[start:server.index('\nimpl From<workflow_execution::receipts::Receipt>', start)]
source += f'#[path={json.dumps(str(ROOT/"src/server/redis_pool.rs"))}] pub mod redis_pool;\n'
source += '#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE/'generated.rs').write_text(source)
paths = [ROOT/p for p in ['.github/workflows/ci.yml','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py','Cargo.toml','Cargo.lock','src/server/lib.rs','src/server/queue.rs','src/server/redis_pool.rs','src/agents/builtin/agent.rs']]
for folder in ['src/server/common', 'src/server/omnisolo', 'src/server/telemetry', 'src/server/config', 'src/proto']:
    paths += [p for p in (ROOT/folder).rglob('*') if p.is_file() and p.suffix in {'.rs','.toml','.proto'}]
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in {'Cargo.lock','source-manifest.json'}]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
print('Prepared exact production queue and bootstrap selection; source fingerprinted')
