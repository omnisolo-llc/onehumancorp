"""Compile complete production milestone read against canonical auth and tables."""
from pathlib import Path
import hashlib, json, re
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
source = (ROOT/'src/server/api/growth.rs').read_text()
items = []; exact = {}
for name, pattern in [
    *[(n, rf'^#\[derive\([^\n]+\)\]\npub struct {n} \{{.*?^\}}\n') for n in ['MilestoneQuery', 'MilestoneResponse']],
    ('handle_get_milestone', r'^async fn handle_get_milestone\(.*?^\}\n'),
    ('growth_auth_fallback_middleware', r'^pub async fn growth_auth_fallback_middleware\(.*?^\}\n'),
]:
    matches = list(re.finditer(pattern, source, re.M|re.S))
    if len(matches) != 1: raise RuntimeError(f'Expected one complete production item: {name}')
    text = matches[0].group(); items.append(text); exact[name] = hashlib.sha256(text.encode()).hexdigest()
generated = '#![allow(dead_code)]\nuse axum::{Extension,Json,extract::Request,middleware::Next,response::IntoResponse};\nuse serde::{Deserialize,Serialize};\n#[derive(Clone)]pub struct GrowthState{pool:sqlx::PgPool}\n' + '\n'.join(items) + '\n#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE/'generated.rs').write_text(generated)
initial = (ROOT/'src/server/migrations/001_initial.sql').read_text()
schema = [re.search(r'CREATE TABLE IF NOT EXISTS '+n+r' \(.*?\n\);', initial, re.S).group() for n in ['tenants','users','revoked_tokens','customers','orders']]
schema += ['ALTER TABLE orders ENABLE ROW LEVEL SECURITY;', re.search(r'CREATE POLICY tenant_isolation_orders ON .*?;', initial, re.S).group()]
schema.append((ROOT/'src/server/migrations/017_business_milestones.sql').read_text())
currency = (ROOT/'src/server/migrations/167_multi_currency.sql').read_text()
schema += re.findall(r'^ALTER TABLE orders .*?;', currency, re.M)
(HERE/'schema.sql').write_text('\n'.join(schema)+'\n')
paths = ['Cargo.lock','Cargo.toml','.github/workflows/ci.yml','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py','src/server/api/growth.rs','src/server/lib.rs','src/server/utils/tenant_middleware.rs','src/server/migrations/001_initial.sql','src/server/migrations/017_business_milestones.sql','src/server/migrations/167_multi_currency.sql']
paths += [str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','generated.rs','schema.sql','source-manifest.json']]
for name in ['auth','common','config','oidc','omnisolo','telemetry']:
    paths += [str(p.relative_to(ROOT)) for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({'inputs':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sorted(set(paths))}, 'exact_items':exact, 'generated_sha256':hashlib.sha256(generated.encode()).hexdigest(), 'schema_sha256':hashlib.sha256((HERE/'schema.sql').read_bytes()).hexdigest()}, indent=2)+'\n')
