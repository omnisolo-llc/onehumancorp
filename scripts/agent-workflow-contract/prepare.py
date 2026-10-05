"""Compile actual authenticated workflow/worker/provider paths with isolated storage."""
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from rust_source import extract_item, input_paths as rust_source_inputs
import hashlib
import json
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
server = (ROOT / 'src/server/lib.rs').read_bytes().decode('utf-8')
hire = (ROOT / 'src/server/api/agents/hire.rs').read_bytes().decode('utf-8')
hub_source = (ROOT / 'src/server/hub.rs').read_bytes().decode('utf-8')


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
for text, name in [
    (server, 'WorkflowRecord'),
    (server, 'CreateWorkflowRequest'),
    (hire, 'HireAgentRequest'),
    (hire, 'HireAgentResponse'),
]:
    parts.append(extract_item(text, 'struct', name))
parts.append(extract_item(server, 'impl', 'WorkflowRecord', impl_trait='From<workflow_execution::receipts::Receipt>'))
for text, name in [
    (server, 'workflow_agent_task'),
    (server, 'list_workflows_handler'),
    (server, 'create_workflow_handler'),
    (server, 'workflow_receipt_handler'),
    (server, 'workflow_request_receipt_handler'),
    (server, 'cancel_workflow_handler'),
    (hire, 'hire_handler'),
    (hire, 'execution_policy_handler'),
    (hire, 'list_agents_handler'),
]: parts.append(extract_item(text, 'function', name))
parts[0] = parts[0].replace('/* HUB_LIFECYCLE_METHODS */', extract_item(hub_source, 'function', 'get_agent', impl_type='Hub') + '\n' + extract_item(hub_source, 'function', 'update_agent_status', impl_type='Hub'))
parts.append(extract_item(server, 'struct', 'ConfiguredWorkflowInference'))
parts.append(extract_item(server, 'impl', 'ConfiguredWorkflowInference', impl_trait='workflow_execution::TextInference'))
parts.append(extract_item(server, 'function', 'configured_workflow_execution'))
parts.append(extract_item(server, 'struct', 'RegisteredWorkflowAgent'))
parts.append(extract_item(server, 'impl', 'RegisteredWorkflowAgent', impl_trait='workflow_execution::RegistrationLease'))
# Preserve actual route ordering: these outer layers precede the separate raw
# RPC merge. Copy the complete raw handler/constructor; replace terminal agent,
# provider and tool implementations only with fail-fast/recording boundaries.
rpc_source = (ROOT/'src/agents/builtin/json_rpc_server.rs').read_text().split('#[cfg(test)]', 1)[0]
parts.append((HERE/'rpc_boundary.rs.in').read_text())
parts.append('pub mod json_rpc_server {\n'+rpc_source+'\n}')
parts.append(extract_item(server, 'function', 'protected_bearer_auth_middleware'))
parts.append('const AGENT_RPC_REQUEST_LIMIT_BYTES: usize = 1_048_576;\nconst AGENT_RPC_RESPONSE_LIMIT_BYTES: usize = 2_097_152;')
for name in ['agent_rpc_available', 'extend_agent_rpc_body', 'read_limited_agent_rpc_body', 'allowed_agent_rpc_method', 'agent_rpc_url', 'proxy_agent_rpc_handler']:
    parts.append(extract_item(server, 'function', name))
api_start=server.index('        .route(\n            "/api/v1/rpc",')
api_end=server.index('.with_state(mesh_transport)',api_start)+len('.with_state(mesh_transport)')
api_chain=server[api_start:api_end]
raw_start=server.index('.merge(legacy_agent_rpc_router(')
for name in ['protect_internal_ingress', 'legacy_agent_rpc_unavailable', 'legacy_agent_rpc_router']:
    parts.append(extract_item(server, 'function', name))
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
llm=(ROOT/'src/agents/builtin/llm/mod.rs').read_bytes().decode('utf-8')
wire=['pub use omnisolo_builtin_agent_llm::{LlmClient,minify_chat_request,circuit_breaker};']
wire.append(next(line for line in llm.splitlines() if line.startswith('pub const MAX_PROVIDER_RESPONSE_BYTES')))
wire.append(extract_item(llm, 'function', 'read_provider_json'))
for name in ['anthropic','openai','ollama']:
 wire.append(f'#[path={json.dumps(str(ROOT / "src/agents/builtin/llm" / (name+".rs")))}] pub mod {name};')
wire.append('#[cfg(test)]\n#[path='+json.dumps(str(ROOT/'src/agents/builtin/llm/structured_output_test.rs'))+']\nmod structured_output_test;')
parts.append('pub mod llm_wire_contract {\n'+'\n'.join(wire)+'\n}')
parts += ['#[cfg(test)]#[path="test.rs"]mod tests;']
(HERE/'generated.rs').write_text('\n\n'.join(parts)+'\n')
inputs = list(rust_source_inputs()) + [ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py',ROOT/'scripts/test_focused_ci_gate.py',ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'src/server/lib.rs',ROOT/'src/server/api/agents/hire.rs',ROOT/'src/server/workflow_execution.rs',ROOT/'src/server/hub.rs']
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml','prepare.py','test.rs','run.sh','README.md','verify_lock.py','rpc_boundary.rs.in','test_receipt_schema.py','proxy-readback-proof.cjs','package.json','package-lock.json','verify_node_lock.py','test_usage_schema.py']]
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
