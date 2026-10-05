"""Extract unchanged mounted middleware/router bodies; record every input.

Business downstream effects are recording sentinels. Signature code is imported
from its real file. Issuing is outside this active-route gate. No provider or real
secret is contacted.
"""
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rust_source import extract_item, input_paths as rust_source_inputs
import json, hashlib
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
API=ROOT/'src/server/api'
STRIPE=ROOT/'src/server/integrations/stripe'
billing=(API/'billing_webhook.rs').read_bytes().decode('utf-8')
ledger=(API/'payment_ledger.rs').read_bytes().decode('utf-8')
source='''#![allow(dead_code)]
extern crate self as server_telemetry;
pub fn record_error_signal(_: &str) {}
pub mod integrations {pub mod stripe {
'''
for name in ['webhook_signature']:
    file=STRIPE/f'{name}.rs'
    if file.exists():source+=f'#[path={json.dumps(str(file))}]pub mod {name};\n'
source+='}}\npub mod api {\n'
guard=API/'stripe_webhook_security.rs'
if guard.exists():source+=f'#[path={json.dumps(str(guard))}]pub mod stripe_webhook_security;\n'
source+='''pub mod billing_webhook {
use axum::{body::Body,extract::{Request,State},http::StatusCode,middleware::Next,response::{IntoResponse,Response}};
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
use serde_json::Value;
#[derive(Clone)] pub struct WebhookState{pub rate_limiter:Arc<NoRedis>}
pub struct NoRedis{pub calls:AtomicUsize}
impl NoRedis{pub async fn get_connection(&self)->Result<redis::aio::MultiplexedConnection,String>{self.calls.fetch_add(1,Ordering::SeqCst);Err("Redis effect sentinel".into())}}
'''+billing[billing.index('#[derive(Debug, Deserialize)]\npub struct StripeEvent'):billing.index('#[async_trait::async_trait]',billing.index('pub struct StripeEvent'))].replace('Deserialize','serde::Deserialize')+extract_item(billing, 'function', 'webhook_security_middleware')+'\n}\n'
start=ledger.index('#[derive(Deserialize)]\npub struct WebhookPayload')
end=ledger.index('#[derive(Serialize)]\npub struct BalanceResponse',start)
source+='''pub mod payment_ledger {
use axum::{Router,Json,extract::State,routing::{get,post},http::StatusCode};
use serde::Deserialize;
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
pub type AppState=Arc<AtomicUsize>;
async fn create_payment_intent()->StatusCode{StatusCode::OK}
async fn get_balance()->StatusCode{StatusCode::OK}
async fn get_safe_to_spend()->StatusCode{StatusCode::OK}
async fn process_receipt()->StatusCode{StatusCode::OK}
async fn stripe_webhook(State(effects):State<AppState>,Json(_payload):Json<WebhookPayload>)->StatusCode{effects.fetch_add(1,Ordering::SeqCst);StatusCode::OK}
'''+ledger[start:end]+extract_item(ledger, 'function', 'router')+'\n}\n}\n#[cfg(test)]#[path="test.rs"]mod tests;\n'

fixtures=(API/'billing_webhook_test.rs').read_text()
start=fixtures.index('// Public fixture signing key');end=fixtures.index('#[test]',start)
source += "\n#[cfg(test)]mod billing_fixture_contracts {\n"+fixtures[start:end]+"""
#[tokio::test] async fn existing_billing_tests_use_an_authentic_serialized_fixture() {
    let _environment = stripe_signing_environment().await;
    let value=serde_json::json!({"id":"evt_fixture","type":"fixture.unhandled","data":{"object":{}}});
    let now=chrono::Utc::now().timestamp();
    let signature=test_stripe_signature(&value,now);
    assert!(crate::integrations::stripe::webhook_signature::verify_at(&serde_json::to_vec(&value).unwrap(),&signature,TEST_STRIPE_SIGNING_SECRET.as_bytes(),now).is_ok());
}
}\n"""
(HERE/'generated.rs').write_text(source)
inputs=list(rust_source_inputs())+[ROOT/'Cargo.toml',ROOT/'src/server/common/secret_source.rs',ROOT/'src/server/integrations/mod.rs',ROOT/'Cargo.lock',STRIPE/'Cargo.toml',STRIPE/'mod.rs',STRIPE/'client.rs',STRIPE/'issuing.rs',API/'mod.rs',API/'billing_webhook.rs',API/'billing_webhook_test.rs',API/'payment_ledger.rs',ROOT/'src/server/lib.rs',HERE/'Cargo.toml',HERE/'prepare.py',HERE/'test.rs',HERE/'run.sh',HERE/'verify_lock.py',HERE/'source_contracts.py',HERE/'README.md']
inputs += [p for p in [guard,STRIPE/'webhook_signature.rs'] if p.exists()]
for directory in ['common','config','integrations/stripe','integrations/core','integrations/mercadopago','integrations/razorpay']:
    inputs += [p for p in (ROOT/'src/server'/directory).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
inputs = sorted(set(inputs))
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs},indent=2)+'\n')
print(f'Prepared Stripe exact-source gate from {len(inputs)} source inputs')
