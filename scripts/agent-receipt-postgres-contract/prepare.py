"""Compile the actual portable receipt/auth persistence against owned PostgreSQL."""
from pathlib import Path
import hashlib, json, re
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
paths = {name: ROOT / f'src/server/persistence/{name}.rs' for name in ['capabilities', 'connection', 'entities', 'migration']}
paths['workflow_execution'] = ROOT / 'src/server/workflow_execution.rs'
lines = ['#![allow(dead_code)]', 'pub mod persistence { pub use crate::{capabilities,connection,entities,migration}; pub use connection::AppDatabase; }']
for name, path in paths.items():
    lines.append(f'#[path={json.dumps(str(path))}] pub mod {name};')
# Use the actual configured transport adapter for the budget/provider contract.
server=(ROOT/'src/server/lib.rs').read_text()
def block(text,name):
 start=text.index(name);opening=text.index('{',start);depth=0;quoted=escaped=False
 for i in range(opening,len(text)):
  c=text[i]
  if quoted:
   if escaped: escaped=False
   elif c=='\\': escaped=True
   elif c=='"': quoted=False
  elif c=='"':quoted=True
  elif c=='{':depth+=1
  elif c=='}':
   depth-=1
   if depth==0:return text[start:i+1]
 raise ValueError(name)
lines.append('extern crate self as omnisolo_builtin_agent;')
lines.append(f'#[path={json.dumps(str(ROOT / "src/agents/builtin/tools/tenant.rs"))}] pub mod tenant_context;')
lines.append('pub mod tools {pub use crate::tenant_context as tenant;}')
lines.append(f'#[path={json.dumps(str(ROOT / "src/agents/builtin/tenant_analysis.rs"))}] pub mod tenant_analysis;')
start=server.index('struct ConfiguredWorkflowInference(');end=server.index(');',start)+2
lines.append(server[start:end])
lines.append(block(server,'impl workflow_execution::TextInference for ConfiguredWorkflowInference'))
lines.append(block(server,'fn configured_workflow_execution('))
# Compile the exact Hub ledger selector with owned DB handles. No account or
# accounting implementation is replaced by this narrow fixture projection.
hub=(ROOT/'src/server/hub.rs').read_text()
lines.append('pub mod db { pub enum DbStore {Postgres,Sqlite(sqlx::SqlitePool)} pub struct DB {pub store:DbStore,pub pool:sqlx::PgPool} }')
lines.append('pub mod hub { pub struct TaskManager {pub db:std::sync::RwLock<Option<std::sync::Arc<crate::db::DB>>>} pub struct Hub {pub task_manager:TaskManager} impl Hub { '+block(hub,'pub fn usage_ledger(')+' } }')
lines.append(f'#[path={json.dumps(str(ROOT / "src/server/api/usage_api.rs"))}] pub mod usage_api;')

lines.append('#[cfg(test)] #[path="test.rs"] mod tests;')
(HERE / 'generated.rs').write_text('\n'.join(lines) + '\n')
initial = (ROOT/'src/server/migrations/001_initial.sql').read_text()
(HERE/'core_pg.sql').write_text('\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\);', initial, re.S).group() for name in ['tenants', 'users']))
inputs = [ROOT/'.github/workflows/ci.yml', ROOT/'scripts/focused_ci_gate.py', ROOT/'scripts/test_focused_ci_gate.py', ROOT/'scripts/agent-definition-contract/database_guard.py', ROOT/'docs/development/tenant-execution-receipts-postgres.md', ROOT/'Cargo.toml', ROOT/'Cargo.lock', ROOT/'src/server/migrations/001_initial.sql', ROOT/'src/server/migrations/1018_agent_definition_marketplace.sql', ROOT/'src/server/migrations/1022_tenant_workflow_receipts.sql', *paths.values()]
inputs += [p for p in (ROOT/'src/server/workflow_execution').rglob('*.rs')]
inputs += [ROOT/'src/server/migrations/1025_usage_accounting.sql',ROOT/'src/server/lib.rs',ROOT/'src/server/hub.rs',ROOT/'src/server/db.rs',ROOT/'src/server/api/usage_api.rs',ROOT/'scripts/agent-workflow-contract/usage-api-proxy-proof.cjs',ROOT/'scripts/agent-workflow-contract/verify_node_lock.py',ROOT/'src/ui/next/package-lock.json']
inputs += [p for p in (ROOT/'src/ui/next/src/lib/auth').glob('*') if p.is_file() and p.suffix in ('.ts','.json')]
inputs += [p for p in (ROOT/'src/agents/builtin').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
inputs += [p for p in (ROOT/'src/server/persistence').rglob('*') if p.suffix in ('.rs', '.sql')]
for name in ['auth','common','config','harness','oidc','omnisolo','telemetry','pricing','utils']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml','Cargo.lock','prepare.py','test.rs','run.sh','fetch.sh','verify_lock.py','README.md']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
