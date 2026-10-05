"""Compile exact cash/billing handlers and inventory service with real PG/Redis."""
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rust_source import extract_item, input_paths as rust_source_inputs
import hashlib
import json
import os
import subprocess
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def source_file(path):
    baseline=os.environ.get('OHC_CASH_BASELINE')
    return subprocess.check_output(['git','show',f'{baseline}:{path}'],cwd=ROOT).decode('utf-8') if baseline else (ROOT/path).read_bytes().decode('utf-8')
terminal=source_file('src/server/api/terminal_api.rs')
billing=source_file('src/server/api/billing_api.rs')
inventory=source_file('src/server/services/inventory/service.rs').split('#[cfg(test)]\nmod tests')[0]
(HERE/'generated_inventory.rs').write_text(inventory)
source='''#![allow(dead_code)]
extern crate self as server_auth;
extern crate self as server_omnisolo;
use axum::{Json,extract::State,http::{HeaderMap,StatusCode},response::IntoResponse};
use std::sync::Arc;
pub static POOL:std::sync::RwLock<Option<sqlx::PgPool>>=std::sync::RwLock::new(None);
pub mod db {
 pub fn get_pool()->sqlx::PgPool{crate::POOL.read().unwrap().as_ref().unwrap().clone()}
 pub fn get_sqlite_pool_if_exists()->Option<sqlx::SqlitePool>{None}
}
pub mod common {pub use server_common::auth_utils;}
pub mod integrations {pub use crate::stripe;}
pub mod orchestration {
 #[derive(Clone)]pub struct AuthInfo {pub org_id:String,pub spiffe_id:String,pub agent_id:String}
 pub struct MeshEvent {pub event_id:String,pub topic:String,pub payload:Vec<u8>,pub timestamp:i64}
 pub mod departments {pub mod types {
 #[derive(serde::Serialize)]pub struct DepartmentEvent {pub id:String,pub tenant_id:String,pub event_type:String,pub payload:serde_json::Value}
 }}
}
pub fn parse_spiffe_id(_value:&str)->Result<(String,String),String>{Err("probe only accepts signed extension authority".into())}
pub mod hub {
 pub struct Tracker {pub stripe_client:Option<crate::stripe::client::StripeClient>}
 pub struct Hub {pub pool:sqlx::PgPool,pub tracker:Tracker,pub redis:redis::Client,pub events:std::sync::Mutex<Vec<crate::orchestration::MeshEvent>>}
 impl Hub {
  pub fn tracker(&self)->&Tracker{&self.tracker}
  pub fn redis_client(&self)->Option<redis::Client>{Some(self.redis.clone())}
  pub async fn publish_mesh_event(&self,event:crate::orchestration::MeshEvent)->Result<(),String>{self.events.lock().unwrap().push(event);Ok(())}
 }
}
use hub::Hub;
#[path="generated_inventory.rs"]pub mod inventory;
pub mod services {pub mod inventory {pub use crate::inventory::*;}}
'''
client=source_file('src/server/integrations/stripe/client.rs')
source+='pub mod stripe {pub mod client {\n'+extract_item(client, 'struct', 'StripeClient')+'\nimpl StripeClient {\n'
for name in ['new', 'require_api_key', 'api_base', 'create_checkout_session']:
    source+=extract_item(client, 'function', name, impl_type='StripeClient')+'\n'
source+='}}\n'
for module in ['routing','safe_checkout']:
    source+=f'#[path={json.dumps(str(ROOT / "src/server/integrations/stripe" / (module+".rs")))}]pub mod {module};\n'
source+='}\n'
for name in ['ReserveInventoryRequest','CommitInventoryRequest']:
    source+=extract_item(terminal, 'struct', name)+'\n'
for name in ['reserve_inventory_handler','commit_inventory_handler']:
    source+=extract_item(terminal, 'function', name)+'\n'
if 'pub async fn read_cash_receipt_handler(' in terminal:
    source+=extract_item(terminal, 'function', 'read_cash_receipt_handler')+'\n'
    source+='pub const HAS_READBACK:bool=true;\n'
else:
    source+='pub const HAS_READBACK:bool=false;\nasync fn read_cash_receipt_handler()->StatusCode{StatusCode::NOT_FOUND}\n'
if 'pub items:' in terminal:
    source+=f'#[path={json.dumps(str(ROOT/"src/server/api/terminal_cash_receipts.rs"))}]mod cash_receipts;\n'
for name, adapter in [('CreateCheckoutSessionRequest', 'serde::Serialize'), ('CreateCheckoutSessionResponse', 'serde::Deserialize')]:
    # Preserve production attributes, adding only the existing test serialization adapter.
    source+=f'#[derive({adapter})]\n'+extract_item(billing, 'struct', name)+'\n'
for name in ['validated_checkout_quantity', 'validated_subscription_interval', 'begin_billing_tenant_transaction', 'create_checkout_session_handler']:
    source+=extract_item(billing, 'function', name)+'\n'
source+='#[cfg(test)]#[path="test.rs"]mod cash_contract;\n'
(HERE/'generated.rs').write_text(source)
paths=list(rust_source_inputs())+[ROOT/'Cargo.lock',ROOT/'src/server/api/terminal_api.rs',ROOT/'src/server/api/billing_api.rs',ROOT/'src/server/services/inventory/service.rs',ROOT/'src/server/services/inventory/mod.rs']
paths += [ROOT / path for path in ['.github/workflows/ci.yml', 'scripts/focused_ci_gate.py', 'scripts/test_focused_ci_gate.py', 'scripts/agent-feed-decision-contract/database_guard.py']]
paths += [p for p in (ROOT/'src/server/migrations').glob('*.sql')]
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in ['source-manifest.json','Cargo.lock']]
for directory in ['common','config','integrations/stripe','integrations/core','integrations/mercadopago','integrations/razorpay']:
    paths += [p for p in (ROOT/'src/server'/directory).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
if (ROOT/'src/server/api/terminal_cash_receipts.rs').exists():paths.append(ROOT/'src/server/api/terminal_cash_receipts.rs')
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(paths))},indent=2)+'\n')
