import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';

const root = new URL('../', import.meta.url);

test('production routers contain no fixture seeding or simulation registrations', async () => {
  for (const file of ['src/server/lib.rs', 'src/server/api/agents/approvals.rs', 'src/server/api/growth.rs']) {
    const source = await readFile(new URL(file, root), 'utf8');
    const registrations = [...source.matchAll(/\.route\(\s*"([^"]+)"/g)].map(match => match[1]);
    assert.deepEqual(registrations.filter(route => /\/(?:dev|simulate)(?:[-/]|$)/.test(route)), [], file);
  }
});

test('synthetic HTTP handler implementations are absent from production source', async () => {
  for (const file of ['src/server/lib.rs', 'src/server/api/agents/approvals.rs', 'src/server/api/growth.rs']) {
    const source = await readFile(new URL(file, root), 'utf8');
    assert.doesNotMatch(source, /async fn (?:simulate_[a-z_]+|mock_omni_inbox_handler|handle_simulate_[a-z_]+)\(/, file);
  }
});

test('Next has no dev catch-all or simulation proxy routes', async () => {
  const api = new URL('src/ui/next/src/app/api/v1/', root);
  const routes = (await readdir(api, { recursive: true })).filter(name => name.endsWith('route.ts'));
  assert.deepEqual(routes.filter(name => /(^|\/)(?:dev|simulate-[^/]+)\//.test(name)), []);
});

test('native E2E seeds only through its isolated SQL fixture path', async () => {
  const source = await readFile(new URL('scripts/native-e2e.mjs', root), 'utf8');
  assert.match(source, /\['docker',|execute\('docker', \['run'/);
  assert.match(source, /'127\.0\.0\.1:0:5432'/);
  assert.match(source, /src\/e2e\/e2e-seed\.sql/);
  assert.doesNotMatch(source, /\/api\/v1\/dev\/seed/);
});

test('dynamic approval paths reject retired simulation names before decision extraction', async () => {
  const source = await readFile(new URL('src/server/api/agents/approvals.rs', root), 'utf8');
  assert.ok(source.includes('reject_retired_fixture_paths'), 'retired names must not fall through to the real approval decision handler');
});

test('browser SQL helpers require runner-owned disposable database proof before pooling', async () => {
  const source = await readFile(new URL('src/e2e/db_utils.ts', root), 'utf8');
  assert.ok(source.includes('verifiedFixtureDatabaseUrl'), 'arbitrary DATABASE_URL must be rejected before any SQL');
  assert.ok(source.indexOf('const databaseUrl = verifiedFixtureDatabaseUrl(') < source.indexOf('new Pool('));
});

test('deployment checks reject seeding HTTP and retain real database read assertions', async () => {
  const compose = await readFile(new URL('deploy/tests/docker_compose_e2e_test.sh', root), 'utf8');
  assert.ok(compose.includes('expect_status 404 "production seed route must be absent"'));
  assert.ok(compose.includes('operational-read.sql'));
  assert.ok(compose.includes('com.docker.compose.project'));
  for (const id of ['compose-fixture-order', 'compose-fixture-inbox', 'compose-fixture-vendor']) assert.ok(compose.includes(id));
  const kind = await readFile(new URL('deploy/tests/kind_e2e_test.sh', root), 'utf8');
  assert.ok(kind.includes('retired_seed_status'));
  assert.ok(kind.includes('recorded_vendor_id'));
});

test('the retired operator seed script cannot inject demo records into an installation', async () => {
  const script = await readFile(new URL('deploy/scripts/omnisolo-seed-data.sh', root), 'utf8');
  assert.ok(script.includes('make test-e2e'));
  assert.equal(/\bcurl\b|\bpsql\b|\bsqlite3\b|OMNISOLO_ACCESS_TOKEN/.test(script), false);
});
