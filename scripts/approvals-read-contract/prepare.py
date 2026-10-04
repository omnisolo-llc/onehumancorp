"""Read-only router probe: exact production read methods, no decision substitute."""
from pathlib import Path
import hashlib
import json
import re
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
source = (ROOT / 'src/server/orchestration/departments/orchestrator.rs').read_text()
methods = []
for name, next_name in [('get_pending_approvals', 'get_ledger_entries'), ('get_activity_feed', 'decide_approval')]:
    start = source.index(f'    pub async fn {name}(')
    end = source.index(f'    pub async fn {next_name}(', start)
    methods.append(source[start:end])
handler = (ROOT / 'src/server/api/agents/approvals.rs').read_text()
# Full decision integration tests stay in the root library. This deliberately
# bounded probe runs only read contracts; accidental mutation calls panic.
handler = handler.replace('#[cfg(test)]\n#[path = "approvals_readback_test.rs"]\nmod readback_tests;', '')
(HERE / 'generated_approvals.rs').write_text(handler)
parts = ['''pub mod db {
 pub enum DbStore { Postgres, Sqlite(sqlx::SqlitePool) }
 pub struct DB { pub pool:sqlx::PgPool, pub store:DbStore }
}
pub fn get_redis_client()->Option<redis::Client>{None}
''']
parts.append(f'#[path={json.dumps(str(ROOT / "src/server/utils/cache.rs"))}] pub mod cache;')
parts.append('pub mod utils {pub use crate::cache;}')
parts.append(f'#[path={json.dumps(str(ROOT / "src/server/api/fixture_boundary.rs"))}] pub mod fixture_boundary;')
parts.append('pub mod api {pub use crate::fixture_boundary;}')
parts.append('pub mod orchestration {pub mod departments {')
parts.append(f'#[path={json.dumps(str(ROOT / "src/server/orchestration/departments/types.rs"))}] pub mod types;')
parts.append('''pub mod orchestrator {
 use std::sync::Arc;
 use std::str::FromStr;
 use crate::db::{DB,DbStore};
 use super::types::{ApprovalRequest,DepartmentType,ApprovalStatus,ActionRisk};
 pub struct DepartmentOrchestrator {pub db:Arc<DB>}
 impl DepartmentOrchestrator {
 pub async fn decide_approval(&self,_id:&str,_tenant:&str,_approved:bool,_edit:Option<serde_json::Value>)->Result<(),String>{panic!("Decision execution is excluded from this read-only probe")}
 pub async fn get_ledger_entries(&self,_tenant:&str,_limit:i64)->Result<Vec<serde_json::Value>,String>{panic!("Ledger reads are excluded from this probe")}
''')
parts.extend(methods)
parts.append('}}}}')
parts.append('#[path="generated_approvals.rs"]pub mod approvals;\n#[cfg(test)]#[path="test.rs"]mod read_contract;')
(HERE / 'generated.rs').write_text('\n'.join(parts))
paths = [ROOT / p for p in ['.github/workflows/ci.yml', 'scripts/focused_ci_gate.py', 'scripts/test_focused_ci_gate.py', 'src/e2e/dashboard_audit_isolation.spec.ts', 'src/server/api/agents/approvals.rs', 'src/server/api/agents/approvals_readback_test.rs', 'src/server/api/fixture_boundary.rs', 'src/server/utils/cache.rs', 'src/server/orchestration/departments/orchestrator.rs', 'src/server/orchestration/departments/types.rs', 'src/server/lib.rs', 'Cargo.lock']]
paths.extend(p for directory in ['common', 'config'] for p in (ROOT / 'src/server' / directory).rglob('*') if p.is_file())
paths.extend(p for p in HERE.iterdir() if p.is_file() and p.name != 'source-manifest.json')
(HERE / 'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
