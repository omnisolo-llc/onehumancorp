"""Compile complete production link-bio items against real auth and PostgreSQL."""
from pathlib import Path
import hashlib,json,re
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
source=(ROOT/'src/server/api/growth.rs').read_text()
items=[]; exact={}
for name,pattern in [
    ('is_supported_bio_url', r"^fn is_supported_bio_url\(.*?^\}\n"),
    *[(n,rf"^#\[derive\([^\n]+\)\]\npub struct {n} \{{.*?^\}}\n") for n in ['LinkItem','LinkInBioConfig','SetLinkInBioConfigReq']],
    *[(n,rf"^pub async fn {n}\(.*?^\}}\n") for n in ['growth_auth_fallback_middleware','handle_get_link_in_bio','handle_post_link_in_bio']],
]:
    matches=list(re.finditer(pattern,source,re.M|re.S))
    if len(matches)!=1:raise RuntimeError(f'Expected one complete production item: {name}')
    text=matches[0].group();items.append(text);exact[name]=hashlib.sha256(text.encode()).hexdigest()
head='#![allow(dead_code)]\nuse axum::{extract::Request,middleware::Next};\n#[derive(Clone)] pub struct GrowthState {pool:sqlx::PgPool}\n'
generated=head+'\n'.join(items)+'\n#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE/'generated.rs').write_text(generated)
initial=(ROOT/'src/server/migrations/001_initial.sql').read_text()
schema=[re.search(r'CREATE TABLE IF NOT EXISTS '+n+r' \(.*?\n\);',initial,re.S).group() for n in ['tenants','users','revoked_tokens']]
schema.append((ROOT/'src/server/migrations/005_agent_kv_store.sql').read_text())
(HERE/'schema.sql').write_text('\n'.join(schema)+'\n')
paths=['Cargo.lock','scripts/focused_ci_gate.py','.github/workflows/ci.yml','src/server/api/growth.rs','src/server/lib.rs','src/server/utils/tenant_middleware.rs','src/server/migrations/001_initial.sql','src/server/migrations/005_agent_kv_store.sql']
paths += [str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','generated.rs','schema.sql','source-manifest.json']]
for name in ['auth','common','config','oidc','omnisolo','telemetry']:
 paths += [str(p.relative_to(ROOT)) for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
manifest={'inputs':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sorted(set(paths))},'exact_items':exact,'generated_sha256':hashlib.sha256(generated.encode()).hexdigest()}
(HERE/'source-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Prepared seven complete production items with real authentication and canonical schemas')
