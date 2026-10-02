"""Compile actual authenticated workflow/worker/provider paths with isolated storage."""
from pathlib import Path
import hashlib
import json
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
server = (ROOT / 'src/server/lib.rs').read_text()
hire = (ROOT / 'src/server/api/agents/hire.rs').read_text()
hub_source = (ROOT / 'src/server/hub.rs').read_text()

def block(text, name):
    start = text.index(name)
    opening = text.index('{', start)
    depth = 0
    quoted = escaped = False
    for index in range(opening, len(text)):
        char = text[index]
        if quoted:
            if escaped: escaped = False
            elif char == '\\': escaped = True
            elif char == '"': quoted = False
        elif char == '"': quoted = True
        elif char == '{': depth += 1
        elif char == '}':
            depth -= 1
            if depth == 0: return text[start:index + 1]
    raise ValueError(name)

parts = ['''#![allow(dead_code)]
use axum::{Json,extract::{State,FromRequest},http::StatusCode,response::IntoResponse};
use serde::{Deserialize,Serialize};
use std::sync::{Arc,RwLock};
use hub::Hub;
extern crate self as omnisolo_builtin_agent;
#[path="../../src/agents/builtin/tools/tenant.rs"] pub mod tenant_context;
pub mod tools { pub use crate::tenant_context as tenant; }
#[path="../../src/agents/builtin/tenant_analysis.rs"] pub mod tenant_analysis;
// The observed provider boundary below records input; all receipt, admission,
// worker and auth code is the exact production implementation on real SQLite.
struct ObservedDispatch { task:String }
static DISPATCHES: RwLock<Vec<ObservedDispatch>> = RwLock::new(Vec::new());
#[path="../../src/server/workflow_execution.rs"] pub mod workflow_execution;
pub fn workflow_agent_binary() -> String { "local-nondispatching-fixture".into() }
pub mod hub {
    use std::sync::Arc;
    use server_omnisolo::orchestration::Agent;
    #[derive(Default)]
    pub struct Hub { agents: tokio::sync::RwLock<std::collections::HashMap<String, Agent>> }
    impl Hub {
        async fn invalidate_agent_cache(&self) {}
        pub async fn fire_agent(&self, id: &str) { self.agents.write().await.remove(id); }
        /* HUB_LIFECYCLE_METHODS */
        pub async fn register_agent(&self, agent: Agent) { self.agents.write().await.insert(agent.id.clone(), agent); }
        pub async fn get_agents(&self) -> Arc<Vec<Agent>> { Arc::new(self.agents.read().await.values().cloned().collect()) }
        pub async fn get_agents_by_org(&self, id: &str) -> Vec<Agent> {
            self.agents.read().await.values().filter(|a| a.organization_id == id).cloned().collect()
        }
    }
}
''']
for text, name, derive in [
    (server, 'pub struct WorkflowRecord {', '#[derive(Clone,serde::Serialize)]'),
    (server, 'struct CreateWorkflowRequest {', '#[derive(serde::Deserialize)]'),
    (hire, 'pub struct HireAgentRequest {', '#[derive(Deserialize,Debug)]'),
    (hire, 'pub struct HireAgentResponse {', '#[derive(Serialize,Debug)]'),
]:
    start=text.index(name)
    attributes=text[text.rfind('#[derive',0,start):start]
    parts += [attributes+block(text,name)]
parts.append(block(server,'impl From<workflow_execution::receipts::Receipt> for WorkflowRecord'))
for text, name in [
    (server, 'pub fn workflow_agent_task('),
    (server, 'async fn list_workflows_handler('),
    (server, 'async fn create_workflow_handler('),
    (server, 'async fn workflow_receipt_handler('),
    (server, 'async fn workflow_request_receipt_handler('),
    (server, 'async fn cancel_workflow_handler('),
    (hire, 'async fn hire_handler('),
    (hire, 'async fn execution_policy_handler('),
    (hire, 'async fn list_agents_handler('),
]: parts.append(block(text, name))
parts[0] = parts[0].replace('/* HUB_LIFECYCLE_METHODS */', block(hub_source, 'pub async fn get_agent(') + '\n' + block(hub_source, 'pub async fn update_agent_status('))
start=server.index('struct ConfiguredWorkflowInference(')
end=server.index(');',start)+2
parts.append(server[start:end])
parts.append(block(server,'impl workflow_execution::TextInference for ConfiguredWorkflowInference'))
parts.append(block(server,'fn configured_workflow_execution('))
parts.append(block(server,'pub(crate) struct RegisteredWorkflowAgent'))
parts.append(block(server,'impl workflow_execution::RegistrationLease for RegisteredWorkflowAgent'))
# Preserve actual route ordering: these outer layers precede the separate raw
# RPC merge. Copy the complete raw handler/constructor; replace terminal agent,
# provider and tool implementations only with fail-fast/recording boundaries.
rpc_source = (ROOT/'src/agents/builtin/json_rpc_server.rs').read_text().split('#[cfg(test)]', 1)[0]
parts.append((HERE/'rpc_boundary.rs.in').read_text())
parts.append('pub mod json_rpc_server {\n'+rpc_source+'\n}')
parts.append(block(server, 'async fn protected_bearer_auth_middleware('))
parts.append('const AGENT_RPC_REQUEST_LIMIT_BYTES: usize = 1_048_576;\nconst AGENT_RPC_RESPONSE_LIMIT_BYTES: usize = 2_097_152;')
for name in ['fn agent_rpc_available(', 'fn extend_agent_rpc_body(', 'async fn read_limited_agent_rpc_body(', 'fn allowed_agent_rpc_method(', 'fn agent_rpc_url(', 'async fn proxy_agent_rpc_handler(']:
    parts.append(block(server, name))
api_start=server.index('        .route(\n            "/api/v1/rpc",')
api_end=server.index('.with_state(mesh_transport)',api_start)+len('.with_state(mesh_transport)')
api_chain=server[api_start:api_end]
raw_start=server.index('.merge(legacy_agent_rpc_router(')
for name in ['fn protect_internal_ingress<S>(', 'async fn legacy_agent_rpc_unavailable(', 'fn legacy_agent_rpc_router(']:
    parts.append(block(server, name))
raw_end=server.index('        .merge(meta_webhook_router)',raw_start)
raw_chain=server[raw_start:raw_end].strip()
# No later global layer surrounds this merge in the real app construction.
app_end=server.index('.fallback(api_not_found_handler);',raw_end)
assert '.layer(' not in server[raw_end:app_end]
assert '.route_layer(' not in server[raw_end:app_end]
parts.append('fn mounted_rpc_boundary(http_auth_store: std::sync::Arc<server_auth::Store>) -> axum::Router {\n    let rate_limiter = std::sync::Arc::new(server_pricing::rate_limit::RedisRateLimiter::new(redis::Client::open("redis://127.0.0.1:1").unwrap()));\n    #[derive(Clone)]\n    struct MeshTransportFixture;\n    let mesh_transport = MeshTransportFixture;\n    axum::Router::new()\n'+api_chain+'\n'+raw_chain+'\n}')
for name in ['capabilities','connection','entities','migration']:
    parts.append(f'#[path={json.dumps(str(ROOT / "src/server/persistence" / (name+".rs")))}] pub mod {name};')
parts.append('pub mod persistence { pub use crate::{capabilities,connection,entities,migration}; pub use connection::AppDatabase; }')
# Compile exact provider transports and their wire tests, reusing the canonical
# circuit-breaker dependency without changing Tokio feature closure.
llm=(ROOT/'src/agents/builtin/llm/mod.rs').read_text()
wire=['pub use omnisolo_builtin_agent_llm::{LlmClient,minify_chat_request,circuit_breaker};']
wire.append(next(line for line in llm.splitlines() if line.startswith('pub const MAX_PROVIDER_RESPONSE_BYTES')))
wire.append(block(llm,'pub(crate) async fn read_provider_json'))
for name in ['anthropic','openai','ollama']:
 wire.append(f'#[path={json.dumps(str(ROOT / "src/agents/builtin/llm" / (name+".rs")))}] pub mod {name};')
parts.append('pub mod llm_wire_contract {\n'+'\n'.join(wire)+'\n}')
parts += ['#[cfg(test)]#[path="test.rs"]mod tests;']
(HERE/'generated.rs').write_text('\n\n'.join(parts)+'\n')
inputs = [ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py',ROOT/'scripts/test_focused_ci_gate.py',ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'src/server/lib.rs',ROOT/'src/server/api/agents/hire.rs',ROOT/'src/server/workflow_execution.rs',ROOT/'src/server/hub.rs']
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml','prepare.py','test.rs','run.sh','README.md','verify_lock.py','rpc_boundary.rs.in','test_receipt_schema.py','proxy-readback-proof.cjs','package.json','package-lock.json','verify_node_lock.py']]
inputs += [p for p in (ROOT/'src/ui/next/src/lib/auth').glob('*') if p.is_file() and p.suffix in ('.ts','.json')]
inputs += [ROOT/'src/ui/next/package.json',ROOT/'src/ui/next/package-lock.json',ROOT/'package-lock.json']
inputs += [p for p in (ROOT/'src/proto').rglob('*.proto')]
inputs += [p for p in (ROOT/'src/server/workflow_execution').rglob('*.rs')]
inputs += [p for p in (ROOT/'src/server/persistence').rglob('*') if p.suffix in ['.rs','.sql']]
inputs += [p for p in (ROOT/'src/server/migrations').glob('1020_*.sql')]
for name in ['auth','common','config','harness','oidc','omnisolo','telemetry','pricing','utils']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
inputs += [p for p in (ROOT/'src/agents/builtin').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
