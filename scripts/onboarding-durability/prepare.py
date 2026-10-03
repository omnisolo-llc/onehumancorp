from pathlib import Path
import json, hashlib
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
s=(ROOT/'src/server/services/onboarding/onboarding_agent.rs').read_text()
def block(name):
    a=s.index(name); at=s.index('{',a); depth=0; quote=False; esc=False
    for i in range(at,len(s)):
        c=s[i]
        if quote:
            if esc: esc=False
            elif c=='\\':esc=True
            elif c=='"':quote=False
            continue
        if c=='"':quote=True
        elif c=='{':depth+=1
        elif c=='}':
            depth-=1
            if not depth:return s[a:i+1]
    raise ValueError(name)
# Match longer type first: avoid IntakeProduct prefix choosing IntakeProductVariant.
types='\n'.join('#[derive(Clone,serde::Serialize,serde::Deserialize)]\n'+block('pub struct '+n+' {') for n in ['IntakeProduct','IntakeProductVariant','IntakeData','ChatMessage','ChatResponse'])
methods=['pub async fn process_intake','pub async fn process_chat','pub async fn start_onboarding_for_identity','async fn start_onboarding_internal','async fn generate_initial_products','pub async fn save_onboarding_state','pub async fn save_onboarding_system_state','async fn save_onboarding_state_internal','pub async fn prepare_onboarding_for_identity','pub async fn prepared_state','pub async fn launch_preparation','async fn invalidate_onboarding_cache','fn catalog_product','fn default_catalog','pub async fn get_onboarding_state']
head='''#![allow(dead_code)]
extern crate self as omnisolo_builtin_agent;
pub mod mesh{pub mod transport{pub trait MeshTransport:Send+Sync{} #[derive(Default)] pub struct InProcessTransport; impl InProcessTransport{pub fn new()->Self{Self}} impl MeshTransport for InProcessTransport{}}}
use serde_json::json;
use server_omnisolo::orchestration::{StartOnboardingRequest,StartOnboardingResponse};
pub use server_common as common;
pub mod db {pub enum DbStore{Postgres} pub struct DB{pub pool:sqlx::PgPool,pub store:DbStore}}
pub mod hub {pub struct Hub{pub pool:sqlx::PgPool,pub events:tokio::sync::Mutex<Vec<server_omnisolo::orchestration::TeammateMeshEvent>>} impl Hub{pub fn new(_tx:tokio::sync::mpsc::Sender<()>,pool:sqlx::PgPool)->Self{Self{pool,events:Default::default()}} pub fn redis_client(&self)->Option<redis::Client>{None} pub async fn publish_teammate_event(&self,_topic:String,event:server_omnisolo::orchestration::TeammateMeshEvent)->Result<(),String>{self.events.lock().await.push(event);Ok(())}}}
pub mod telemetry {pub fn track_onboarding_step(_t:&str,_s:&str,_m:u64){}}
pub mod services{pub mod onboarding{pub use crate::preparation; pub mod onboarding_agent{pub(crate) use crate::valid_draft_state; pub use crate::{OnboardingAgent,IntakeData,ChatMessage,ChatResponse};} pub mod provisioner{pub fn check_environment(_cloud:bool)->Result<(),String>{Err("disabled in isolated tests".into())}pub fn provision_environment(_cloud:bool)->Result<(),String>{Err("disabled in isolated tests".into())}}}pub mod dashboard{pub mod service{pub static ONBOARDING_STATE_CACHE:std::sync::OnceLock<server_utils::cache::HybridCache<server_omnisolo::app::GetOnboardingStateResponse>>=std::sync::OnceLock::new();}}}
use server_utils::cache::HybridCache;
pub static ONBOARDING_STATE_AGENT_CACHE:std::sync::OnceLock<HybridCache<serde_json::Value>>=std::sync::OnceLock::new();
// No model request belongs in this focused crate. The real missing-provider
// branches execute; any accidental configured-provider call fails the test.
pub struct MinimaxClient;
impl MinimaxClient{pub async fn reason(&self,_prompt:&str)->Result<String,String>{panic!("provider execution forbidden in onboarding durability tests")}}
#[derive(Clone)] pub struct OnboardingAgent{db:std::sync::Arc<db::DB>,hub:std::sync::Arc<hub::Hub>,minimax:Option<std::sync::Arc<MinimaxClient>>}
'''
a=s.index('const USER_ONBOARDING_STATE_FIELDS'); b=s.index('#[derive(Debug, Serialize, Deserialize, Clone)]\npub struct IntakeData',a)
generated=head+f'#[path={json.dumps(str(ROOT/"src/server/services/onboarding/preparation.rs"))}]\npub mod preparation;\n'+s[a:b]+types+'\nimpl OnboardingAgent {\n'+'\n'.join(block(n).replace('super::preparation::','crate::preparation::') for n in methods)+'\n pub fn new(db:std::sync::Arc<db::DB>,hub:std::sync::Arc<hub::Hub>)->Self{Self{db,hub,minimax:None}} \n}\n'+block('pub fn onboarding_feature_state')+f'\n#[path={json.dumps(str(ROOT/"src/server/api/onboarding/mod.rs"))}]pub mod onboarding_api;\n'+'\n#[cfg(test)]\n#[path="test.rs"]mod tests;\n'


service=s[s.index('#[cfg(test)]\nmod tests {'):]
a=service.index('    async fn setup_test_db()');b=service.index('    async fn authenticated_test_identity',a)
service=service[:a]+"    async fn setup_test_db() -> Option<Arc<DB>> { Some(crate::tests::setup().await.db) }\n\n"+service[b:]
a=service.index('    static TEST_MIGRATIONS:');b=service.index('    #[test]',a)
service=service[:a]+service[b:]
service=service.replace('mod tests {','mod service_regressions {',1)
a=s.index('fn repair_truncated_json(');b=s.index('\nimpl OnboardingAgent {',a)
generated += s[a:b]+service

auth=(ROOT/'src/server/auth/http.rs').read_text()
a=auth.index('    fn identity_request('); b=auth.index('    #[tokio::test]\n    async fn login_rejects_non_json',a)
auth_tests=auth[a:b]
auth_tests=auth_tests.replace('&store.secret','std::env::var("JWT_SECRET").unwrap().as_bytes()')
auth_tests=auth_tests.replace('store.users.write().unwrap().insert(user.id.clone(), user);','store.update_user(&user.id,None,None,Some(false),user.organization_id.as_deref().unwrap()).await.unwrap();')
generated += """
#[cfg(test)] mod session_identity_regressions {
use axum::{Router,body::{Body,to_bytes},http::{Request,StatusCode,header},response::Response};
use chrono::Utc;
use server_auth::{Store,User};
use std::sync::Arc;
use tower::ServiceExt;
async fn app_with_user()->(Router,Arc<Store>,User){let a=crate::tests::setup().await;let (_,store)=crate::tests::mounted(&a);let app=server_auth::http::router(store.clone()).unwrap();(app,store,crate::tests::auth_user("ADMIN"))}
""" + auth_tests + "\n}\n"

generated += '\n#[cfg(test)]\ninclude!("catalog_test_adapter.rs");\n'
(HERE/'generated.rs').write_text(generated)

inputs=[ROOT/'Cargo.lock',ROOT/'src/server/lib.rs',ROOT/'src/server/auth/http.rs',ROOT/'src/server/api/onboarding/mod.rs',ROOT/'src/server/services/onboarding/onboarding_agent.rs',ROOT/'src/server/services/onboarding/preparation.rs',ROOT/'src/server/services/onboarding/mod.rs',ROOT/'src/server/migrations/235_onboarding_preparation_receipt.sql',HERE/'test.rs',HERE/'catalog_test_adapter.rs',HERE/'prepare.py',HERE/'Cargo.toml']
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs},indent=2)+'\n')
print('Prepared exact production onboarding methods with a test-only catalog adapter')
