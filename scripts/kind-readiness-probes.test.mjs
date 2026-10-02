import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createServer } from 'node:http';
import { spawn } from 'node:child_process';

const smoke = await readFile(new URL('../deploy/tests/kind_e2e_test.sh', import.meta.url), 'utf8');
const helper = smoke.match(/^assert_unavailable_capability\(\) \{[\s\S]*?^\}/m)?.[0];
const good = { success: false, code: 'capability_unavailable', capability: 'approval_request', message: 'No action was completed.' };

async function runProbe(status, body, cache = 'no-store') {
  assert.ok(helper, 'Kind smoke must verify unavailable capabilities explicitly');
  let requests = 0;
  let receivedBody = '';
  const server = createServer((request, response) => {
    requests += 1;
    request.on('data', value => { receivedBody += value; });
    request.on('end', () => {
      response.writeHead(status, { 'content-type': 'application/json', ...(cache ? { 'cache-control': cache } : {}) });
      response.end(JSON.stringify(body));
    });
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  try {
    const url = `http://127.0.0.1:${server.address().port}`;
    const script = `set -euo pipefail\nlog() { :; }\ncurl_bounded() { command curl --noproxy '*' --connect-timeout 2 --max-time 5 "$@"; }\nauth_headers=(-H 'Authorization: Bearer public-fixture')\n${helper}\nassert_unavailable_capability "$1" POST /api/v1/approvals/request approval_request '{"status":"approved"}'\n`;
    const child = spawn('bash', ['-c', script, 'probe', url]);
    let output = '';
    child.stdout.on('data', value => { output += value; });
    child.stderr.on('data', value => { output += value; });
    const code = await new Promise((resolve, reject) => { child.on('error', reject); child.on('close', resolve); });
    return { code, requests, output, receivedBody };
  } finally {
    await new Promise(resolve => server.close(resolve));
  }
}

test('Kind readiness probe accepts the explicit no-effect501 contract', async () => {
  const result = await runProbe(501, good);
  assert.equal(result.code, 0, result.output);
  assert.equal(result.requests, 1);
  assert.deepEqual(JSON.parse(result.receivedBody), { status: 'approved' });
});
for (const [name, status, body, cache] of [
  ['former fabricated success', 200, { id: 'approval-e2e', status: 'approved' }, 'no-store'],
  ['unrelated failure', 503, good, 'no-store'],
  ['success contradiction', 501, { ...good, success: true }, 'no-store'],
  ['invented receipt', 501, { ...good, id: 'invented' }, 'no-store'],
  ['invented business data', 501, { ...good, totalCostUSD: 0 }, 'no-store'],
  ['wrong capability', 501, { ...good, capability: 'snapshot_creation' }, 'no-store'],
  ['cacheable error', 501, good, 'public, max-age=60'],
]) {
  test(`Kind readiness probe rejects ${name}`, async () => {
    const result = await runProbe(status, body, cache);
    assert.notEqual(result.code, 0, result.output);
    assert.equal(result.requests, 1, 'never retry an operation with an uncertain outcome');
  });
}

test('every unavailable capability remains explicitly open and probed', async () => {
  const inventory = JSON.parse(await readFile(new URL('../docs/development/readiness-gaps.json', import.meta.url), 'utf8'));
  assert.equal(inventory.capabilities.length, 7);
  for (const item of inventory.capabilities) {
    assert.equal(item.status, 'unimplemented');
    assert.equal(item.positive_acceptance, 'outstanding');
    assert.ok(item.required_checks.length >= 3);
    assert.ok(smoke.includes(`assert_unavailable_capability "$` + `{backend_url}" ${item.method} ${item.path} ${item.id}`));
  }
  assert.equal((smoke.match(/assert_unavailable_capability "\$\{backend_url\}" GET \/api\/v1\/costs cost_summary/g) ?? []).length, 2);
});
