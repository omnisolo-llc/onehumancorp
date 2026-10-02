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
lines.append('#[cfg(test)] #[path="test.rs"] mod tests;')
(HERE / 'generated.rs').write_text('\n'.join(lines) + '\n')
initial = (ROOT/'src/server/migrations/001_initial.sql').read_text()
(HERE/'core_pg.sql').write_text('\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\);', initial, re.S).group() for name in ['tenants', 'users']))
inputs = [ROOT/'.github/workflows/ci.yml', ROOT/'scripts/focused_ci_gate.py', ROOT/'scripts/test_focused_ci_gate.py', ROOT/'scripts/agent-definition-contract/database_guard.py', ROOT/'docs/development/tenant-execution-receipts-postgres.md', ROOT/'Cargo.toml', ROOT/'Cargo.lock', ROOT/'src/server/migrations/001_initial.sql', ROOT/'src/server/migrations/1018_agent_definition_marketplace.sql', ROOT/'src/server/migrations/1022_tenant_workflow_receipts.sql', *paths.values()]
inputs += [p for p in (ROOT/'src/server/workflow_execution').rglob('*.rs')]
inputs += [p for p in (ROOT/'src/server/persistence').rglob('*') if p.suffix in ('.rs', '.sql')]
for name in ['auth','common','config','oidc','omnisolo','telemetry','pricing','utils']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml','Cargo.lock','prepare.py','test.rs','run.sh','fetch.sh','verify_lock.py','README.md']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
