import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const server = await readFile(new URL('../src/server/lib.rs', import.meta.url), 'utf8');
const unavailableRoutes = [
  ['/api/v1/dashboard', 'organization_dashboard'],
  ['/api/v1/costs', 'cost_summary'],
  ['/api/v1/approvals/request', 'approval_request'],
  ['/api/v1/approvals/decide', 'approval_decision'],
  ['/api/v1/handoffs', 'handoff_creation'],
  ['/api/v1/skills/import', 'skill_import'],
  ['/api/v1/snapshots/create', 'snapshot_creation'],
];

function mountedRoute(path) {
  const quotedPath = JSON.stringify(path);
  const position = server.indexOf(`${quotedPath},`);
  assert.notEqual(position, -1, `the endpoint remains mounted: ${path}`);
  assert.equal(server.indexOf(`${quotedPath},`, position + 1), -1, `one endpoint: ${path}`);
  const start = server.lastIndexOf('.route(', position);
  let depth = 0;
  let quoted = false;
  let escaped = false;
  for (let index = start + '.route'.length; index < server.length; index += 1) {
    const char = server[index];
    if (quoted) {
      if (escaped) escaped = false;
      else if (char === '\\') escaped = true;
      else if (char === '"') quoted = false;
    } else if (char === '"') quoted = true;
    else if (char === '(') depth += 1;
    else if (char === ')' && --depth === 0) return server.slice(start, index + 1);
  }
  assert.fail(`unterminated route: ${path}`);
}

for (const [path, capability] of unavailableRoutes) {
  test(`${path} cannot expose its former fabricated business receipt`, () => {
    const route = mountedRoute(path);
    assert.ok(route.includes(`api::production_readiness::unavailable("${capability}")`),
      'Use the explicit unavailable response until an authenticated durable implementation replaces this gate and its tests.');
    assert.doesNotMatch(route, /e2e-org|approval-e2e|handoff-e2e|skill-e2e|snapshot-e2e|totalCostUSD/);
  });
}

test('the required Rust CI lane executes the real mounted readiness contract', async () => {
  const ci = await readFile(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const native = ci.split('\n  native-test:')[1]?.split('\n  native-node:')[0];
  assert.ok(native, 'native Rust gate remains present');
  assert.ok(native.includes('run: bash scripts/production-readiness-contract/run.sh'));
});
