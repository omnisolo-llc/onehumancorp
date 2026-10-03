"""Compile the complete production search module and unchanged auth/pool helpers."""
from pathlib import Path
import hashlib
import json
import re
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
pool = (ROOT/'src/server/db.rs').read_text()
start = pool.index('pub fn secure_pg_pool_options()')
end = pool.index('\npub fn get_sqlite_pool_if_exists', start)
generated = pool[start:end] + '\n#[path="../../src/server/api/search.rs"]pub mod search;\n#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE/'generated.rs').write_text(generated)
initial = (ROOT/'src/server/migrations/001_initial.sql').read_text()
messages = (ROOT/'src/server/migrations/1001_create_omni_inbox_messages_and_quotes_fix.sql').read_text()
schema = []
for table in ['tenants','users','revoked_tokens','customers','orders']:
    schema.append(re.search(r'CREATE TABLE IF NOT EXISTS '+table+r' \(.*?\n\);', initial, re.S).group())
schema.append(re.search(r'CREATE TABLE IF NOT EXISTS omni_inbox_messages \(.*?\n\);', messages, re.S).group())
for table, source in [('customers',initial),('orders',initial),('omni_inbox_messages',messages)]:
    schema.append('ALTER TABLE '+table+' ENABLE ROW LEVEL SECURITY;')
    schema.append(re.search(r'CREATE POLICY tenant_isolation_'+table+r' ON .*?;',source,re.S).group())
(HERE/'schema.sql').write_text('\n'.join(schema)+'\n')
inputs = [ROOT/p for p in ['Cargo.lock','Cargo.toml','src/server/api/search.rs','src/server/api/mod.rs','src/server/lib.rs','src/server/db.rs','src/server/migrations/001_initial.sql','src/server/migrations/1001_create_omni_inbox_messages_and_quotes_fix.sql','scripts/focused_ci_gate.py','.github/workflows/ci.yml','src/ui/next/src/app/api/v1/[...path]/route.ts','src/ui/next/src/lib/auth/backendTransport.ts']]
inputs += [p for p in HERE.iterdir() if p.name not in ['Cargo.lock','generated.rs','schema.sql','source-manifest.json'] and p.is_file()]
for name in ['auth','common','config','oidc','omnisolo','telemetry']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
