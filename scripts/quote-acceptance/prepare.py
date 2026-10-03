from pathlib import Path
import hashlib,json
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
s=(ROOT/'src/server/api/quotes.rs').read_text()
m=(ROOT/'src/server/domain/repository/models.rs').read_text()
def block(source,name):
    a=source.index(name); at=source.index('{',a); depth=0; quoted=False; escape=False
    for i in range(at,len(source)):
        c=source[i]
        if quoted:
            if escape:escape=False
            elif c=='\\':escape=True
            elif c=='"':quoted=False
        elif c=='"':quoted=True
        elif c=='{':depth+=1
        elif c=='}':
            depth-=1
            if depth==0:return source[a:i+1]
    raise ValueError(name)
source='''
#![allow(dead_code)]
use axum::{Json,extract::{Extension,Path,Query,State},http::StatusCode,response::IntoResponse};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Serialize,Deserialize};
use chrono::{DateTime,Utc};
use std::sync::atomic::AtomicUsize;
pub static PROVIDER_CALLS:AtomicUsize=AtomicUsize::new(0);
pub static PROVIDER_UNKNOWN:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
pub static PROVIDER_BLOCKED:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
pub static OPERATIONS:std::sync::Mutex<Vec<String>>=std::sync::Mutex::new(vec![]);
pub mod integrations{pub use server_integrations_stripe as stripe;}
pub mod orchestration{pub mod queue{pub mod redis_lock{
pub struct RedisLock;
impl RedisLock {
 pub fn new(_url:&str)->Result<Self,String>{Err("unrelated booking lock is not used in quote acceptance tests".into())}
 pub async fn acquire_lock(&self,_tenant:&str,_kind:&str,_id:&str,_seconds:u64)->Result<(),String>{unreachable!()}
}
}}}
'''
for name in ['Quote','QuoteLineItem']:
    source+='#[derive(Debug,Clone,Serialize,Deserialize,sqlx::FromRow)]\n'+block(m,'pub struct '+name+' {')+'\n'
for line in s.splitlines():
    if line.startswith('const QUOTE_'):source+=line+'\n'
source+='#[derive(Debug,Clone)]struct TenantAuthority(String);\n'+block(s,'impl TenantAuthority')+'\n'
for name in ['QuoteResponse','QuoteQuery','UpdateQuoteRequest','QuoteLineItemRequest','AcceptQuoteRequest']:
    source+='#[derive(Serialize,Deserialize)]\n'+block(s,('pub struct ' if name!='AcceptQuoteRequest' else 'struct ')+name+' {')+'\n'
source+=f'#[path={json.dumps(str(ROOT/"src/server/api/quote_acceptance.rs"))}]mod quote_acceptance;\n'
for name in ['validate_line_item_references','lock_owned_quote','accept_quote','get_quote','approve_quote','update_quote']:
    source+=block(s,'async fn '+name+'(')+'\n'
source+='#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE/'generated.rs').write_text(source)
inputs=[ROOT/'Cargo.lock',ROOT/'src/server/api/quotes.rs',ROOT/'src/server/domain/repository/models.rs',HERE/'prepare.py',HERE/'test.rs',HERE/'Cargo.toml',HERE/'run.sh',HERE/'source_contracts.py',HERE/'README.md',HERE/'verify_lock.py',ROOT/'Cargo.toml',ROOT/'src/server/lib.rs',ROOT/'src/server/utils/tenant_middleware.rs',ROOT/'src/server/api/quote_acceptance.rs',ROOT/'src/server/migrations/1017_quote_acceptance_receipt.sql']
for directory in ['common','config','integrations/stripe','integrations/core','integrations/mercadopago','integrations/razorpay']:
    inputs += [p for p in (ROOT/'src/server'/directory).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
for name in ['078_quote_engine.sql','107_quote_checkout_url.sql','114_invoicing_agent.sql','166_invoice_quote_link.sql','229_quotes_and_builder_parity.sql','1001_create_omni_inbox_messages_and_quotes_fix.sql','1002_add_created_updated_at_to_quotes.sql','1013_invoice_runtime_contract.sql','1016_quote_line_items_parity.sql']:
    inputs.append(ROOT/'src/server/migrations'/name)
inputs=sorted(set(inputs))
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs},indent=2)+'\n')
