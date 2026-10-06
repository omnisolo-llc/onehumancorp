import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';

const source = readFileSync(new URL('../src/server/api/assistant.rs', import.meta.url), 'utf8');
const fixture = source.slice(source.indexOf('    async fn task_mutations_use_database()'), source.indexOf('\n#[derive(Serialize, Deserialize)]\npub struct CustomerMemorySynthesis'));

test('mounted task fixture supplies the real runtime and signed owner middleware', () => {
  assert.match(fixture, /WorkflowExecution::unavailable\(\s*auth\.clone\(\),?\s*\)/);
  assert.match(fixture, /server_auth::Store::new\(\)/);
  assert.match(fixture, /\.create_user\(/);
  assert.match(fixture, /auth\.issue_token\(&owner\)/);
  assert.match(fixture, /server_auth::strict_bearer_auth_middleware/);
  assert.match(fixture, /\.layer\(Extension\(execution\)\)/);
  assert.match(fixture, /\.header\("authorization", format!\("Bearer \{token\}"\)\)/);
  assert.doesNotMatch(fixture, /Extension\(claims\(\)\)|WorkflowExecution::configured|impl.*TextInference/);
});

test('legacy metadata mutations retain lifecycle and verify persisted effects', () => {
  assert.match(fixture, /INSERT INTO assistant_tasks/);
  assert.match(fixture, /format!\("\/legacy-tasks\/\{task_id\}"\)/);
  assert.match(fixture, /assert_eq!\(archived\["status"\], "running"\)/);
  assert.match(fixture, /assert_eq!\(archived\["archived"\], true\)/);
  assert.match(fixture, /assert_eq!\(persisted, \("Renamed Task"\.into\(\), "running"\.into\(\), 1\)\)/);
  assert.match(fixture, /assert_eq!\(deleted\["deletedTask"\]\["id"\], task_id\)/);
  assert.equal((fixture.match(/assert_eq!\(count\.0, 0\)/g) || []).length, 2);
});

test('modern task creation cannot pass through a fabricated legacy success', () => {
  assert.match(fixture, /"POST", "\/tasks", json!\(\{ "prompt": "Do something" \}\)/);
  assert.match(fixture, /\.header\("idempotency-key", Uuid::new_v4\(\)\.to_string\(\)\)/);
  assert.match(fixture, /assert_eq!\(status, StatusCode::SERVICE_UNAVAILABLE\)/);
  assert.match(fixture, /assert!\(unavailable\.get\("id"\)\.is_none\(\)\)/);
});

// Run the actual SQLite statements extracted from the fixture and production handler.
// This supplements, and does not replace, the hosted mounted Rust test.
test('production legacy SQL persists metadata and keeps foreign tenant records intact', () => {
  const result = spawnSync('python3', ['-c', String.raw`
import json, re, sqlite3, sys
source = sys.stdin.read()
fixture = source.split('async fn task_mutations_use_database()', 1)[1]
handler = source.split('async fn mutate_task(', 1)[1].split('DbStore::Postgres =>', 1)[0]
strings = lambda text: re.findall(r'"([^"\n]*)"', text)
sql = lambda prefix: next(s for s in strings(handler) if s.startswith(prefix))
connection = sqlite3.connect(':memory:')
for statement in strings(source.split('async fn test_db()', 1)[1].split('async fn request_json(', 1)[0]):
    if statement.startswith('CREATE TABLE '):
        connection.execute(statement)
seed = next(s for s in strings(fixture) if s.startswith('INSERT INTO assistant_tasks '))
connection.execute(seed, ('test-task-1',))
connection.execute(seed.replace("'tenant-real'", "'tenant-other'"), ('foreign-task',))
# A foreign owner cannot rename or archive the seeded record.
connection.execute(sql('UPDATE assistant_tasks SET archived = 1'), ('tenant-other', 'test-task-1'))
connection.execute(sql('UPDATE assistant_tasks SET title = ?'), ('Unauthorized', 'tenant-other', 'test-task-1'))
assert connection.execute('SELECT title, status, archived FROM assistant_tasks WHERE id = ?', ('test-task-1',)).fetchone() == ('Test Task', 'running', 0)
connection.execute(sql('UPDATE assistant_tasks SET archived = 1'), ('tenant-real', 'test-task-1'))
connection.execute(sql('UPDATE assistant_tasks SET title = ?'), ('Renamed Task', 'tenant-real', 'test-task-1'))
readback = next(s for s in strings(fixture) if s.startswith('SELECT title, status, archived'))
assert connection.execute(readback, ('test-task-1',)).fetchone() == ('Renamed Task', 'running', 1)
for statement in strings(handler):
    if statement.startswith('DELETE FROM assistant_'):
        connection.execute(statement, ('tenant-real', 'test-task-1'))
assert connection.execute('SELECT COUNT(*) FROM assistant_tasks WHERE id = ?', ('test-task-1',)).fetchone() == (0,)
assert connection.execute('SELECT title, status, archived FROM assistant_tasks WHERE id = ?', ('foreign-task',)).fetchone() == ('Test Task', 'running', 0)
print(json.dumps({'archive': 'persisted', 'rename': 'persisted', 'delete': 'persisted', 'lifecycle': 'preserved', 'foreign_tenant': 'unchanged'}))
`], { input: source, encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr || result.error?.message);
  assert.equal(JSON.parse(result.stdout).foreign_tenant, 'unchanged');
});
