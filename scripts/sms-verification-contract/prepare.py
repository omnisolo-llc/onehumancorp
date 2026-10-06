"""Compile exact SMS code/migrations with real selected auth and DB authority."""
from pathlib import Path
import hashlib, json, re
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
paths=[]
def read(path):
    p=ROOT/path;paths.append(p);return p.read_text()
source=read('src/server/api/sms_settings.rs')
assert source.count('#[path = "sms_settings_test.rs"]')==1
source=source.replace('#[path = "sms_settings_test.rs"]',f'#[path = {json.dumps(str(ROOT/"src/server/api/sms_settings_test.rs"))}]')
source+='\n#[cfg(test)] #[path="postgres_test.rs"] mod postgres_tests;\n#[cfg(test)] #[path="billing_test.rs"] mod billing_tests;\n'
(HERE/'generated_sms.rs').write_text(source)
lines=['#[path="generated_sms.rs"] mod sms_settings;', 'pub mod persistence { pub use crate::{capabilities,connection,entities,migration}; pub use connection::AppDatabase; }']
for name in ['capabilities','connection','entities','migration']:
    p=ROOT/f'src/server/persistence/{name}.rs';paths.append(p);lines.append(f'#[path={json.dumps(str(p))}]pub mod {name};')
billing=read('src/server/api/billing_webhook.rs')
def item(name):
    match=re.search(r'^(?:(?:pub )?async fn|pub fn|pub trait|pub struct) '+name+r'\b.*?^}',billing,re.M|re.S)
    assert match, name
    return match.group()
traits='\n'.join('#[async_trait::async_trait]\n'+item(name) for name in ['PaymentFailureNotifier','PaymentFailureMessageGenerator'])
lookup='#[derive(Debug, Clone, PartialEq, Eq)]\n'+item('PaymentFailureLookup')
functions='\n'.join(item(name) for name in ['begin_webhook_system_transaction','send_payment_failure_dunning','payment_failure_lookup','find_subscriber_for_payment_failure','mark_subscriber_past_due','process_invoice_payment_failed'])
lines.append('pub mod db {pub enum DbStore {Postgres,Sqlite(sqlx::SqlitePool)} pub struct DB {pub pool:sqlx::PgPool,pub store:DbStore}}')
lines.append('pub mod billing {use std::sync::Arc;use serde_json::Value;use crate::db::DbStore;pub struct WebhookState {pub db:Arc<crate::db::DB>}'+traits+'\n'+lookup+'\n'+functions+'}')
# Only WebhookState's unused mesh/limiter fields are omitted. Every dunning
# SQL statement, tenant lookup, write and notifier invocation is exact source.
(HERE/'generated.rs').write_text('\n'.join(lines)+'\n')
parent=read('src/server/lib.rs')
assert 'SmsService::configured(http_auth_store.clone())' in parent
assert '.merge(api::sms_settings::router(sms_service).route_layer' in parent
assert 'OTP_STORE' not in parent
assert 'let _order_sms_worker = sms_service.start_order_notifications();' in parent
initial=read('src/server/migrations/001_initial.sql')
(HERE/'core_pg.sql').write_text('\n'.join(re.search(r'CREATE TABLE IF NOT EXISTS '+name+r' \(.*?\n\);',initial,re.S).group() for name in ['tenants','users','customers','orders']))
paths += [ROOT/p for p in ['src/server/api/sms_settings_test.rs','src/server/api/order_notifications_test.rs','src/server/migrations/1043_durable_order_sms.sql','src/server/migrations/1039_sms_verification_receipts.sql','src/server/api/billing_webhook.rs','src/server/api/billing_webhook_test.rs','src/server/api/agents/webhook.rs','src/server/services/booking.rs','src/server/services/subscription/service.rs','src/server/api/mod.rs','src/server/db.rs','Cargo.lock','Cargo.toml','.github/workflows/ci.yml','scripts/focused_ci_gate.py']]
paths += list((ROOT/'src/server/persistence').glob('*.sql'))
paths += list((ROOT/'src/proto').rglob('*.proto'))
paths += [ROOT/'.cargo/config.toml',ROOT/'src/ui/next/src/lib/auth/authLimits.json']
for directory in ['auth','common','config','oidc','omnisolo','integrations/twilio','integrations/core','telemetry']:
    paths += [p for p in (ROOT/'src/server'/directory).rglob('*') if p.is_file() and (p.suffix in ['.rs','.sql'] or p.name=='Cargo.toml')]
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in ['Cargo.lock','source-manifest.json','run.log']]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
print('Exact SMS implementation, selected authority and migration inputs fingerprinted')
