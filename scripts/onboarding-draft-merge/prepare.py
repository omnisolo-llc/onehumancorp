from pathlib import Path
import json,hashlib
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
s=(ROOT/'src/server/services/onboarding/onboarding_agent.rs').read_text()
def block(name):
    a=s.index(name);at=s.index('{',a);depth=0;quoted=False;escape=False
    for i in range(at,len(s)):
        c=s[i]
        if quoted:
            if escape:escape=False
            elif c=='\\':escape=True
            elif c=='"':quoted=False
        elif c=='"':quoted=True
        elif c=='{':depth+=1
        elif c=='}':
            depth-=1
            if depth==0:return s[a:i+1]
    raise ValueError(name)
source='''#![allow(dead_code)]
extern crate self as server_utils;
extern crate self as server_omnisolo;
pub use server_common as common;
use serde_json::json;
use std::sync::{Arc,OnceLock};
pub static INVALIDATIONS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
pub mod cache { pub struct HybridCache<T>(std::marker::PhantomData<T>); impl<T> HybridCache<T>{pub fn new(_:Option<()>)->Self{Self(std::marker::PhantomData)}pub async fn invalidate(&self,_key:&str){crate::INVALIDATIONS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);}}}
use cache::HybridCache;
pub mod app {pub struct GetOnboardingStateResponse;}
pub mod services{pub mod dashboard{pub mod service{pub static ONBOARDING_STATE_CACHE:std::sync::OnceLock<crate::cache::HybridCache<crate::app::GetOnboardingStateResponse>>=std::sync::OnceLock::new();}}}
pub static ONBOARDING_STATE_AGENT_CACHE:OnceLock<HybridCache<serde_json::Value>>=OnceLock::new();
pub struct Hub{pub pool:sqlx::PgPool}impl Hub{pub fn redis_client(&self)->Option<()>{None}}
pub struct OnboardingAgent{pub hub:Arc<Hub>}
'''
a=s.index('const USER_ONBOARDING_STATE_FIELDS');b=s.index('#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct IntakeData',a)
source+=s[a:b]+'\nimpl OnboardingAgent{\n'
for name in ['pub async fn save_onboarding_state(', 'pub async fn save_onboarding_system_state(', 'async fn save_onboarding_state_internal(']:source+=block(name)+'\n'
source+='}\n#[cfg(test)]#[path="test.rs"]mod tests;\n'
(HERE/'generated.rs').write_text(source)
inputs=[ROOT/'Cargo.lock',ROOT/'src/server/services/onboarding/onboarding_agent.rs',HERE/'prepare.py',HERE/'test.rs',HERE/'Cargo.toml',HERE/'run.sh',HERE/'README.md',HERE/'verify_lock.py',ROOT/'Cargo.toml',ROOT/'src/server/api/onboarding/mod.rs',ROOT/'src/server/migrations/002_missing_tables.sql',ROOT/'src/server/migrations/235_onboarding_preparation_receipt.sql']
for directory in ['common','config']:
    inputs += [p for p in (ROOT/'src/server'/directory).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
inputs=sorted(set(inputs))
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs},indent=2)+'\n')
