"""Compile exact dispatch and real provider adapter sources; fingerprint all wiring."""
from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent; ROOT=HERE.parents[1]
source=ROOT/'src/server/orchestration/departments/message_delivery.rs'
manual=ROOT/'src/server/orchestration/departments/manual_inbox.rs'
(HERE/'generated_manual_inbox.rs').write_text(manual.read_text()+'\n#[cfg(test)] #[path="manual_test.rs"] mod tests;\n')
(HERE/'generated_delivery.rs').write_text(source.read_text().replace('#[path = "manual_inbox.rs"]', '#[path = "generated_manual_inbox.rs"]')+'\n#[cfg(test)] #[path="test.rs"] mod tests;\n#[cfg(test)] #[path="postgres_test.rs"] mod postgres_tests;\n')
lines=['#![allow(dead_code)]','pub mod db {pub enum DbStore {Postgres,Sqlite(sqlx::SqlitePool)} pub struct DB {pub pool:sqlx::PgPool,pub store:DbStore}}']
lines.append('pub mod integrations { pub use server_integrations_twilio as twilio; pub mod meta {')
for name in ['client','provider']:
    path=ROOT/f'src/server/integrations/meta/{name}.rs';lines.append(f'#[path={json.dumps(str(path))}] pub mod {name};')
lines += ['}}','#[path="generated_delivery.rs"] pub mod message_delivery;']
(HERE/'generated.rs').write_text('\n'.join(lines)+'\n')
paths=[source,manual,ROOT/'src/server/lib.rs',ROOT/'src/server/migrations/1045_manual_inbox_requests.sql',ROOT/'src/server/persistence/manual_inbox_requests_sqlite.sql',ROOT/'scripts/manual-inbox-receipts.test.mjs',ROOT/'scripts/agent-feed-decision-contract/prepare.py',ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py',ROOT/'scripts/department-delivery-receipts.test.mjs',ROOT/'src/server/migrations/1044_department_message_delivery_receipts.sql',ROOT/'src/server/persistence/department_message_delivery_sqlite.sql',ROOT/'src/server/persistence/migration.rs',ROOT/'src/server/db.rs',ROOT/'src/server/domain/inbox.rs',ROOT/'src/server/workers/agent_action_worker.rs',ROOT/'src/server/workers/message_triage_worker.rs']
paths += [ROOT/f'src/server/orchestration/departments/{name}.rs' for name in ['mod','orchestrator','customer_success_agent']]
paths += [ROOT/f'src/ui/next/src/app/{name}/page.tsx' for name in ['inbox','dashboard']]
paths += [ROOT/f'src/ui/next/src/lib/{name}' for name in ['messageDeliveryStatus.ts','messageDeliveryStatus.test.ts']]
paths += list((ROOT/'src/ui/next/src/app/api/v1/ui/omni_inbox').rglob('*.ts'))
paths += list((ROOT/'src/ui/next/src/app/inbox').glob('*.test.tsx'))
paths += list((ROOT/'src/ui/next/src/lib').glob('inboxManualReceipt*'))
for name in ['meta','twilio','core']:
    paths += [p for p in (ROOT/f'src/server/integrations/{name}').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
paths += [p for p in HERE.iterdir() if p.is_file() and not p.name.startswith('generated') and p.name not in ['source-manifest.json','Cargo.lock','run.log']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
print('Exact department provider, dispatch, migration, producer and consumer sources fingerprinted')
