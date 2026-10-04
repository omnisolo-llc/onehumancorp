"""Compile unchanged actual mounted staff read handlers against real SQL pools."""
from pathlib import Path
import hashlib,json,re
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
source=(ROOT/'src/server/api/staff_mesh.rs').read_text()
def between(start,end):
 assert source.count(start)==1,start
 a=source.index(start);b=source.index(end,a)
 return source[a:b]
parts=['''use axum::{Json,Router,extract::{Extension,State},response::IntoResponse,routing::get};
use serde::Serialize;
use std::sync::Arc;
use db::DB;
// Only this container is a fixture: each actual handler uses its genuine SQLx
// SQLite/PostgreSQL pool. No database query or row mapping is replaced.
pub mod db {
 pub enum DbStore {Sqlite(sqlx::SqlitePool),Postgres}
 pub struct DB {pub pool:sqlx::PgPool,pub store:DbStore}
}
''']
for name in ['StaffMember','GetStaffResponse','GetTasksResponse','GetSummariesResponse']:
 match=re.search(r'#\[derive\(Serialize\)\]\npub struct '+name+r' \{[^}]+\}',source)
 assert match,name
 parts.append(match[0])
parts.append(between('fn get_tenant_id(', 'pub async fn create_staff_handler('))
for start,end in [('get_staff_handler','get_timecard_handler'),('get_tasks_handler','update_task_handler')]:
 parts.append(between('pub async fn '+start+'(', 'pub async fn '+end+'('))
parts.append(between('pub async fn get_summaries_handler(', '#[derive(Serialize)]\npub struct GetShiftsResponse'))
parts.append('''pub fn router(database:Arc<DB>)->Router {
 Router::new().route("/staff",get(get_staff_handler)).route("/tasks",get(get_tasks_handler)).route("/summaries",get(get_summaries_handler)).with_state(database)
}
#[cfg(test)] #[path="test.rs"]mod tests;
''')
(HERE/'generated.rs').write_text('\n'.join(parts))
inputs=[ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py',ROOT/'scripts/test_focused_ci_gate.py',ROOT/'docs/development/location-record-truthfulness.md',ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'src/server/api/staff_mesh.rs',ROOT/'src/server/lib.rs',ROOT/'src/server/db.rs',ROOT/'src/server/migrations/206_staff_management.sql',ROOT/'src/server/migrations/232_order_notes_and_shift_escalations.sql',ROOT/'src/server/migrations/1027_staff_mesh_native.sql',ROOT/'src/server/db/migrations/016_staff_mesh.sql']
for package in ['common','config']:
 inputs.extend(p for p in (ROOT/'src/server'/package).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
inputs.extend(p for p in HERE.iterdir() if p.is_file() and p.name not in ['generated.rs','source-manifest.json'])
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
