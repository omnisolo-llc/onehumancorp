import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
const read = path => fs.readFileSync(new URL(`../${path}`, import.meta.url), 'utf8');
test('definition routes use the actual configured store and existing strict bearer authority', () => {
  const main=read('src/server/lib.rs');
  assert.match(main,/\.merge\(api::agents::definitions::router\(/);
  const start=main.indexOf('.merge(api::agents::definitions::router(');const mount=main.slice(start,main.indexOf('http_auth_store.clone()',start)+24);
  assert.match(mount,/db::DbStore::Postgres.*DefinitionStore::Database\(persistence::AppDatabase::from_connection\(sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool\(db.pool.clone\(\)\)\)\)/s);
  assert.match(mount,/db::DbStore::Sqlite\(pool\).*DefinitionStore::Database\(persistence::AppDatabase::from_connection\(sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool\(pool.clone\(\)\)\)\)/s);
  assert.match(mount,/get_mysql_pool_if_exists[\s\S]*DefinitionStore::Unavailable/);
  assert.doesNotMatch(mount,/get_pool\(|get_sqlite_pool_if_exists|Mock/);
  const router=read('src/server/api/agents/definitions.rs');
  assert.match(router,/server_auth::strict_bearer_auth_middleware/);
  assert.match(router,/private, no-store/);
  assert.match(router,/\/api\/v1\/agents\/definitions\/operations\/\{request_id\}/);
  assert.match(router,/eq_ignore_ascii_case\("ADMIN"\)/);
  assert.match(router,/eq_ignore_ascii_case\("OWNER"\)/);
});
test('SQLite bootstraps the same explicit schema before routes; GET contains no lazy seeding', () => {
  const db=read('src/server/db.rs');assert.match(db,/raw_sql\(include_str!\("persistence\/agent_definitions_sqlite.sql"\)\)/);
  const store=read('src/server/persistence/agent_definitions.rs');
  const list=store.slice(store.indexOf('pub async fn list('));assert.doesNotMatch(list,/INSERT INTO|CREATE TABLE|Mock/);
  for(const path of ['src/server/migrations/1018_agent_definition_marketplace.sql','src/server/persistence/agent_definitions_sqlite.sql']) {
    const schema=read(path);assert.match(schema,/Senior Rust Developer/);assert.match(schema,/Technical Writer/);
    assert.doesNotMatch(schema,/INSERT INTO (?:agents|roles|workflows|agent_jobs)\b/i);
  }
});
test('inactive install has no runtime/provider dispatch and preserves real blueprint validation', () => {
  const store=read('src/server/persistence/agent_definitions.rs');
  assert.match(store,/blueprint\.validate\(\)/);assert.match(store,/blueprint\.namespace_roles/);assert.match(store,/tools: vec!\[\]/);
  assert.doesNotMatch(store,/register_agent\(|dispatch_workflow\(|hire_handler\(|reqwest::|Hub::/);
  const prepare=read('scripts/agent-definition-contract/prepare.py');
  for(const module of ['src/server/persistence/agent_definitions.rs','src/server/api/agents/definitions.rs','src/server/domain/blueprint.rs']) assert.ok(prepare.includes(module));
  assert.match(prepare, /src\/server\/persistence\/backend_neutrality_test\.sh/);
  assert.match(read('scripts/agent-definition-contract/run.sh'), /bash src\/server\/persistence\/backend_neutrality_test\.sh/);
  assert.match(prepare,/#\[path=/);assert.doesNotMatch(prepare,/disabled auth|mock auth|x-mock-auth/);
});
