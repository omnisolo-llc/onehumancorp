"""Compile production generation/admission/provider/storage with real HTTP fixtures."""
from pathlib import Path
import hashlib
import json
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
server=(ROOT/'src/server/lib.rs').read_text()
start=server.index('struct ConfiguredWorkflowInference(')
end=server.index('fn configured_workflow_execution(',start)
adapter=server[start:end]
# The factory only injects configuration into the actual provider implementation.
# It is not a fake LlmClient and never modifies provider request/response handling.
tenant_source=(ROOT/'src/agents/builtin/tenant_analysis.rs').read_text()
tenant_source='\n'.join(line for line in tenant_source.splitlines() if not line.startswith('//!'))
api=(ROOT/'src/server/builder/api.rs').read_text()
reader_source=api[api.index('fn brand_toolbox_from_record('):api.index('async fn publish_brand_toolbox_website(')]
parse_source=api[api.index('fn parse_tenant_id('):api.index('#[derive(Deserialize)]\npub struct CreateSiteRequest')]
readers="""pub mod legacy_brand_reader {
use axum::{Json,Router,extract::{State,Path,Extension},routing::get};
use sqlx::PgPool;use uuid::Uuid;use server_common::Claims;
use crate::builder::generation::BrandToolboxResponse;
use crate::db;
"""+parse_source+reader_source+"""
pub fn router<S:Clone+Send+Sync+'static>(pool:PgPool)->Router<S> {
Router::new().route("/brand_toolbox",get(list_brand_toolboxes)).route("/brand_toolbox/{toolbox_id}",get(get_brand_toolbox)).with_state(pool)
}}
"""
parts=['''#![allow(dead_code)]
extern crate self as omnisolo_builtin_agent;
#[path="../../src/agents/builtin/tools/tenant.rs"] pub mod tenant_context;
pub mod tools {pub use crate::tenant_context as tenant;}
#[path="../../src/server/workflow_execution.rs"] pub mod workflow_execution;
#[path="../../src/server/builder/generation.rs"] pub mod generation_source;
#[path="../../src/server/builder/db.rs"] pub mod db;
#[path="../../src/server/builder/publication_json.rs"] pub mod publication_json;
#[path="../../src/server/builder/publication_store.rs"] pub mod publication_store;
pub use generation_source as generation;
pub mod builder {pub use crate::generation_source as generation;pub use crate::{db,publication_json,publication_store};}
''', 'pub mod tenant_analysis {\n'+'''
#[cfg(test)] pub(crate) fn local_http_fixture(endpoint:&str)->ConfiguredTextAnalysis {
 ConfiguredTextAnalysis::from_values(|key|match key {
  "OMNISOLO_LLM_PROVIDER"=>Some("openai-compatible".into()),
  "OMNISOLO_LLM_MODEL"=>Some("owned-contract-model".into()),
  "OMNISOLO_MAX_TOKENS"=>Some("2048".into()),
  "OMNISOLO_LLM_ENDPOINT"=>Some(endpoint.into()),
  "OMNISOLO_LLM_API_KEY"=>Some("public-local-http-fixture-value".into()),
  _=>None,
 }).unwrap()
}
'''+tenant_source+'\n}\n',adapter,readers,'#[cfg(test)]#[path="test.rs"]mod tests;']
for name in ['capabilities','connection','entities','migration']:
 parts.append(f'#[path={json.dumps(str(ROOT / "src/server/persistence" / (name+".rs")))}] pub mod {name};')
parts.append('pub mod persistence { pub use crate::{capabilities,connection,entities,migration}; pub use connection::AppDatabase; }')
(HERE/'generated.rs').write_text('\n'.join(parts))
inputs=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py',ROOT/'scripts/test_focused_ci_gate.py',ROOT/'src/server/migrations/001_initial.sql',ROOT/'src/server/lib.rs',ROOT/'src/server/workflow_execution.rs',ROOT/'src/server/builder/generation.rs',ROOT/'src/server/builder/api.rs',ROOT/'src/server/builder/builder_test.rs',ROOT/'src/server/builder/db.rs',ROOT/'src/server/builder/publication_json.rs',ROOT/'src/server/builder/publication_store.rs',ROOT/'src/server/migrations/059_brand_toolboxes.sql']
for package in ['auth','common','config','harness','omnisolo','pricing','utils']:
 inputs.extend(p for p in (ROOT/'src/server'/package).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
inputs.extend(p for p in (ROOT/'src/agents/builtin').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
inputs.extend(p for p in (ROOT/'src/server/persistence').rglob('*') if p.is_file() and p.suffix in {'.rs','.sql'})
inputs.extend(p for p in (ROOT/'src/server/workflow_execution').rglob('*.rs'))
inputs.extend(p for p in HERE.iterdir() if p.is_file() and p.name not in ['generated.rs','source-manifest.json'])
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
