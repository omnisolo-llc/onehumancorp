"""Compile actual appointment reads with canonical schema and real auth."""
from pathlib import Path
import hashlib
import json
import re
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
module = ROOT/'src/server/api/field_ops/appointments.rs'
if not module.is_file():
    raise SystemExit('The actual appointment read module is required')
generated = '#[path="../../src/server/api/field_ops/appointments.rs"]pub mod appointments;\n#[cfg(test)]use appointments::router as app;\n'
parent_source=(ROOT/'src/server/api/field_ops.rs').read_text()
def rust_item(text,start_marker):
    start=text.index(start_marker);opening=text.index('{',start);depth=0;quoted=False;escaped=False
    for index in range(opening,len(text)):
        char=text[index]
        if quoted:
            if escaped:escaped=False
            elif char=='\\':escaped=True
            elif char=='"':quoted=False
        elif char=='"':quoted=True
        elif char=='{':depth+=1
        elif char=='}':
            depth-=1
            if depth==0:return text[start:index+1]
    raise ValueError(start_marker)
parent_router=rust_item(parent_source,"pub fn router<S:")
parent_state=rust_item(parent_source,"pub struct FieldOpsState")
generated += '''
// Only unrelated mutation leaves and their unused mesh dependency are fixtures.
// The complete parent router body and protected GET module are production source.
extern crate self as omnisolo_builtin_agent;
pub mod mesh {pub mod transport {pub trait MeshTransport:Send+Sync {} impl MeshTransport for () {}}}
pub mod mounted_parent {
use crate::appointments;use axum::{Router,extract::State,http::StatusCode};use sqlx::PgPool;use std::sync::Arc;
''' + parent_state + '''
async fn update_appointment(State(_state):State<Arc<FieldOpsState>>)->StatusCode{StatusCode::IM_A_TEAPOT}
async fn optimize_route()->StatusCode{StatusCode::IM_A_TEAPOT}
async fn running_late()->StatusCode{StatusCode::IM_A_TEAPOT}
''' + parent_router + '\n}\n'
generated += '#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE/'generated.rs').write_text(generated)
initial = (ROOT/'src/server/migrations/001_initial.sql').read_text()
field = (ROOT/'src/server/migrations/162_field_ops_appointments.sql').read_text()
locations = (ROOT/'src/server/migrations/222_field_ops_and_global_commerce.sql').read_text()
schema = []
for table in ['tenants','users','revoked_tokens','customers']:
    schema.append(re.search(r'CREATE TABLE IF NOT EXISTS '+table+r' \(.*?\n\);',initial,re.S).group())
schema.append('ALTER TABLE customers ENABLE ROW LEVEL SECURITY;')
schema.append(re.search(r'CREATE POLICY tenant_isolation_customers ON .*?;',initial,re.S).group())
schema.extend([field, locations])
(HERE/'schema.sql').write_text('\n'.join(schema)+'\n')
paths = ['.github/workflows/ci.yml','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py','Cargo.lock','Cargo.toml','src/server/api/field_ops.rs','src/server/lib.rs','src/server/migrations/001_initial.sql','src/server/migrations/162_field_ops_appointments.sql','src/server/migrations/222_field_ops_and_global_commerce.sql']
if module.exists():
    paths.append(str(module.relative_to(ROOT)))
paths += [str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','generated.rs','schema.sql','source-manifest.json']]
for name in ['auth','common','config','oidc','omnisolo','telemetry']:
    paths += [str(p.relative_to(ROOT)) for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({'inputs':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sorted(set(paths))},'generated_sha256':hashlib.sha256(generated.encode()).hexdigest(),'schema_sha256':hashlib.sha256((HERE/'schema.sql').read_bytes()).hexdigest()},indent=2)+'\n')
