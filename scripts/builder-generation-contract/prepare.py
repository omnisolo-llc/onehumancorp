"""Compile production generation/admission/provider/storage with real HTTP fixtures."""
from pathlib import Path
import hashlib
import json
import re
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
# Compile the production None-backend router and its actual storage-independent
# handlers; an optional pool selector alone must not hide a route/state error.
def api_item(prefix):
 match=re.search(r'(?ms)^'+re.escape(prefix)+r'.*?^}\n',api)
 assert match, prefix
 return match.group()
non_pg_router="""pub mod non_pg_builder {
use axum::{Json,Router,extract::Extension,middleware::{self,Next},response::Response,routing::post};
use server_common::Claims;use serde::Deserialize;use serde_json::Value;use uuid::Uuid;
"""+''.join(api_item(prefix) for prefix in [
 'fn default_builder_tenant_id(', 'async fn ensure_builder_claims(',
 'pub fn storage_independent_router<', 'async fn unavailable_builder_storage(',
 '#[derive(Deserialize)]\npub struct AutoSeoRequest', 'async fn auto_seo(',
])+'}\n'
# Evaluate the actual production builder mount's pool expression in the owned
# database fixture. This catches a correctly tested handler mounted on a second
# pool in run_server, even when both pools have equal connection URLs.
builder_marker='crate::builder::api::router('
builder_start=server.index(builder_marker,server.index('pub async fn run_server()'))+len(builder_marker)
builder_end=builder_start
builder_depth=1
while builder_depth:
 char=server[builder_end]
 if char=='(': builder_depth+=1
 elif char==')': builder_depth-=1
 builder_end+=1
builder_pool_expression=server[builder_start:builder_end-1]
# Project only the actual legacy DB handle fields/backend guard needed at this
# boundary; database operations still use the real SQLx pools and production code.
legacy_db=(ROOT/'src/server/db.rs').read_text()
def db_item(prefix):
 match=re.search(r'(?ms)^'+re.escape(prefix)+r'.*?^}\n',legacy_db)
 assert match,prefix
 return match.group()
postgres_method=re.search(r'(?ms)^    pub fn postgres_pool\(.*?^    }\n',legacy_db)
startup_db='pub mod startup_db {use sqlx::{PgPool,SqlitePool,MySqlPool};use std::sync::OnceLock;\n'
startup_db+='static GLOBAL_MYSQL_POOL:OnceLock<MySqlPool>=OnceLock::new();\n'
startup_db+=db_item('pub enum DbStore {')+db_item('pub struct DB {')
startup_db+=db_item('fn database_url_from_environment()')
startup_db+='pub fn configured_url()->Result<Option<String>,server_common::secret_source::SecretSourceError>{database_url_from_environment()}\n'
if postgres_method: startup_db+='impl DB {\n'+postgres_method.group()+'}\n'
startup_db+='}\n'
startup_pool_proof='\n#[cfg(test)] async fn application_builder_pool(pool:sqlx::PgPool, auth_database:&crate::persistence::AppDatabase)->Option<sqlx::PgPool> {\n application_builder_pool_from_store(startup_db::DB {pool,store:startup_db::DbStore::Postgres},auth_database).await\n}\n#[cfg(test)] async fn application_builder_pool_from_store(db:startup_db::DB, auth_database:&crate::persistence::AppDatabase)->Option<sqlx::PgPool> {\n let _=(&db,auth_database);\n fn optional<T:Into<Option<sqlx::PgPool>>>(pool:T)->Option<sqlx::PgPool> {pool.into()}\n'+ 'optional('+builder_pool_expression+')\n}\n'
commands=(ROOT/'src/server/persistence/commands.rs').read_text()
environment_functions=commands[commands.index('pub fn database_url_from_environment()'):commands.index('#[derive(Clone, Copy, Debug')]
startup_commands='pub mod startup_commands {use crate::persistence::{AppDatabase,DatabaseUrl};type CommandResult<T=()> = Result<T,Box<dyn std::error::Error>>;\n'+environment_functions+'}\n'
startup_selection=re.search(r'(?ms)^    let portable_database = .*?;(?=\n    let catalog_repository)',server)
assert startup_selection, 'actual portable startup selection'
startup_pool_proof+='\n#[cfg(test)] async fn application_startup_database(db:startup_db::DB, standalone:bool)->Result<Option<std::sync::Arc<crate::persistence::AppDatabase>>,Box<dyn std::error::Error>> {\nlet _=&db;\n'+startup_selection.group().replace('db::DbStore::','startup_db::DbStore::')+'\nOk(portable_database)\n}\n'
cipher_start=legacy_db.index('            let canonical_connection =')
cipher_end=legacy_db.index('\n\n            Ok(DB {',cipher_start)
startup_pool_proof+='\nasync fn legacy_startup_cipher_check(sqlite_pool:sqlx::SqlitePool)->Result<(),Box<dyn std::error::Error>> {\n'+legacy_db[cipher_start:cipher_end]+'\nOk(())\n}\n'

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
'''+tenant_source+'\n}\n',adapter,readers,non_pg_router,startup_db,startup_commands,startup_pool_proof,'#[cfg(test)]#[path="test.rs"]mod tests;']
# Preserve actual parent visibility/re-exports. Publishing a synthetic connection
# alias here previously hid an E0603 in the real legacy startup caller.
persistence_parent=(ROOT/'src/server/persistence/mod.rs').read_text()
parts.append('pub mod persistence {')
for name in ['capabilities','connection','entities','migration']:
 declaration=re.search(r'(?m)^(?:pub(?:\([^)]*\))? )?mod '+name+r';$',persistence_parent)
 assert declaration, 'actual persistence module declaration: '+name
 parts.append(f'#[path={json.dumps(str(ROOT / "src/server/persistence" / (name+".rs")))}] '+declaration.group())
for line in persistence_parent.splitlines():
 if re.match(r'^pub(?:\([^)]*\))? use (?:connection|capabilities)::',line):
  parts.append(line)
parts.append('pub use crate::startup_commands as commands; }')
(HERE/'generated.rs').write_text('\n'.join(parts))
inputs=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'.github/workflows/ci.yml',ROOT/'scripts/focused_ci_gate.py',ROOT/'scripts/test_focused_ci_gate.py',ROOT/'src/server/migrations/001_initial.sql',ROOT/'src/server/migrations/009_builder.sql',ROOT/'src/server/migrations/1019_site_publication_receipts.sql',ROOT/'src/server/db.rs',ROOT/'src/server/migrations/1018_agent_definition_marketplace.sql',ROOT/'src/server/lib.rs',ROOT/'src/server/workflow_execution.rs',ROOT/'src/server/builder/generation.rs',ROOT/'src/server/builder/api.rs',ROOT/'src/server/builder/builder_test.rs',ROOT/'src/server/builder/db.rs',ROOT/'src/server/builder/publication_json.rs',ROOT/'src/server/builder/publication_store.rs',ROOT/'src/server/migrations/059_brand_toolboxes.sql']
for package in ['auth','common','config','harness','omnisolo','pricing','utils']:
 inputs.extend(p for p in (ROOT/'src/server'/package).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
inputs.extend(p for p in (ROOT/'src/agents/builtin').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
inputs.extend(p for p in (ROOT/'src/server/persistence').rglob('*') if p.is_file() and p.suffix in {'.rs','.sql'})
inputs.extend(p for p in (ROOT/'src/server/workflow_execution').rglob('*.rs'))
inputs.extend(p for p in HERE.iterdir() if p.is_file() and p.name not in ['generated.rs','source-manifest.json'])
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
