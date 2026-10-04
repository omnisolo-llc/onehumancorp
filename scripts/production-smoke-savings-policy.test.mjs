import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import vm from 'node:vm';
import ts from 'typescript';

const root = new URL('../', import.meta.url);
const compile = source => ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
}).outputText;
function loadPolicy(name, dependencies = {}) {
  const exports = {};
  vm.runInNewContext(compile(fs.readFileSync(new URL(`src/e2e/support/${name}.ts`, root), 'utf8')), {
    exports, URL, require: name => {
      assert.ok(name in dependencies, `Unexpected policy dependency: ${name}`);
      return dependencies[name];
    },
  });
  return exports;
}
const runtime = loadPolicy('runtime_policy');
const policy = loadPolicy('hosted_voice_policy', { './runtime_policy': runtime });
const actor = { userId: 'smoke-owner', tenantId: 'smoke-tenant' };
const valid = {
  origin: 'http://127.0.0.1:43583', url: 'http://127.0.0.1:43583/api/v1/growth/time-savings',
  method: 'GET', status: 501,
  headers: { 'x-ohc-expected-user': actor.userId, 'x-ohc-expected-tenant': actor.tenantId },
  body: {
    success: false, code: 'capability_unavailable', capability: 'measured_time_savings',
    message: 'No verified result is available. This capability is not implemented; no action was completed.',
  },
};
const results = () => ({ failures: [], httpFailures: [], verifiedPolicyUrls: new Set(), policyChecks: [] });
function record(input, result, owner = actor, json = async () => input.body) {
  policy.recordSmokeHttpResponse({
    status: () => input.status, url: () => input.url,
    request: () => ({ method: () => input.method, headers: () => input.headers }), json,
  }, input.origin, result, owner);
}

test('smoke accepts the exact owner-bound measured-savings unavailable response', async () => {
  const result = results();
  record(valid, result);
  await Promise.all(result.policyChecks);
  assert.deepEqual(result.failures, []);
  assert.deepEqual(result.httpFailures, []);
  assert.deepEqual([...result.verifiedPolicyUrls], [valid.url]);
});

const rejected = [
  ['server error', { status: 500 }], ['wrong prerequisite status', { status: 503 }],
  ['write request', { method: 'POST' }], ['foreign origin', { url: 'https://other.test/api/v1/growth/time-savings' }],
  ['other API', { url: valid.origin + '/api/v1/other' }], ['child path', { url: valid.url + '/details' }],
  ['query', { url: valid.url + '?tenant=other' }], ['fragment', { url: valid.url + '#extra' }],
  ['malformed URL', { url: 'invalid' }], ['missing owner', { headers: {} }],
  ['wrong user', { headers: { ...valid.headers, 'x-ohc-expected-user': 'other' } }],
  ['wrong tenant', { headers: { ...valid.headers, 'x-ohc-expected-tenant': 'other' } }],
  ['missing tenant', { headers: { 'x-ohc-expected-user': actor.userId } }],
  ['null body', { body: null }], ['array body', { body: [] }], ['generic error', { body: { error: 'not implemented' } }],
  ['wrong code', { body: { ...valid.body, code: 'upstream_error' } }],
  ['wrong capability', { body: { ...valid.body, capability: 'trial_entitlement' } }],
  ['successful claim', { body: { ...valid.body, success: true } }],
  ['wrong message', { body: { ...valid.body, message: 'Recorded 12 hours saved.' } }],
  ['fabricated metrics', { body: { ...valid.body, hours_saved: 12 } }],
  ...Object.keys(valid.body).map(key => [`missing ${key}`, { body: Object.fromEntries(Object.entries(valid.body).filter(([field]) => field !== key)) }]),
];
for (const [name, change] of rejected) {
  test(`smoke preserves the savings failure for ${name}`, async () => {
    const input = { ...valid, ...change }, result = results();
    record(input, result);
    await Promise.all(result.policyChecks);
    assert.deepEqual(result.failures, [`${input.status} ${input.url}`]);
    assert.equal(result.verifiedPolicyUrls.size, 0);
  });
}

test('an exact response is not accepted without the authenticated smoke actor', async () => {
  const result = results();
  record(valid, result, null);
  await Promise.all(result.policyChecks);
  assert.deepEqual(result.failures, [`501 ${valid.url}`]);
  assert.equal(result.verifiedPolicyUrls.size, 0);
});

test('the exact savings candidate awaits its actual body; unreadable JSON stays a failure', async () => {
  let resolve;
  const result = results();
  record(valid, result, actor, () => new Promise(done => { resolve = done; }));
  assert.equal(result.policyChecks.length, 1);
  assert.equal(result.verifiedPolicyUrls.size, 0);
  resolve(valid.body);
  await Promise.all(result.policyChecks);
  assert.deepEqual([...result.verifiedPolicyUrls], [valid.url]);
  const malformed = results();
  record(valid, malformed, actor, async () => { throw new Error('malformed JSON'); });
  await Promise.all(malformed.policyChecks);
  assert.deepEqual(malformed.failures, [`501 ${valid.url}`]);
  assert.equal(malformed.verifiedPolicyUrls.size, 0);
});

test('unrelated 501 responses fail immediately without waiting for a body', () => {
  let bodyReads = 0;
  const result = results(), input = { ...valid, url: valid.origin + '/api/v1/other' };
  record(input, result, actor, () => { bodyReads++; return new Promise(() => {}); });
  assert.deepEqual(result.failures, [`501 ${input.url}`]);
  assert.equal(bodyReads, 0);
  assert.equal(result.policyChecks.length, 0);
});

test('only the savings resource diagnostic is suppressed after the exact receipt is verified', () => {
  assert.equal(typeof policy.isVerifiedSavingsPolicyDiagnostic, 'function');
  const text = 'Failed to load resource: the server responded with a status of 501 (Not Implemented)';
  assert.equal(policy.isVerifiedSavingsPolicyDiagnostic(text, valid.url, new Set()), false);
  assert.equal(policy.isVerifiedSavingsPolicyDiagnostic(text, valid.url, new Set([valid.url])), true);
  for (const [message, url] of [
    ['Unexpected application error', valid.url], [text.replace('501', '500'), valid.url],
    [text, valid.url + '?unexpected=1'], [text, valid.origin + '/api/v1/settings/voice'],
    [text, 'invalid'],
  ]) assert.equal(policy.isVerifiedSavingsPolicyDiagnostic(message, url, new Set([url])), false);
});

for (const input of [
  { ...valid, status: 503, url: valid.origin + '/api/v1/sona', body: { error: runtime.runtimeUnavailableMessage } },
  { ...valid, status: 503, url: valid.origin + '/api/v1/agents/goose', body: { error: runtime.runtimeUnavailableMessage } },
  { ...valid, status: 403, url: valid.origin + '/api/v1/settings/voice', body: { success: false,
    error: 'hosted_global_provisioning_unavailable', provisioning_available: false,
    provisioning_block_reason: 'hosted_global_provisioning_unavailable' } },
]) test(`existing explicit policy receipt remains verified: ${input.url}`, async () => {
  const result = results();
  record(input, result);
  await Promise.all(result.policyChecks);
  assert.deepEqual(result.failures, []);
  assert.deepEqual(result.httpFailures, []);
  assert.deepEqual([...result.verifiedPolicyUrls], [input.url]);
});

// Execute the maintained route-crawl loop with only the browser boundary supplied.
// These tests prove the audit checks the truthful widget state, not a browser run.
const smokeFile = new URL('src/ui/next/src/e2e/production_feature_smoke.spec.ts', root);
const source = ts.createSourceFile(smokeFile.pathname, fs.readFileSync(smokeFile, 'utf8'), ts.ScriptTarget.Latest, true);
let routeLoop, verifiedReceiptAssertion, responseListener;
function findLoop(node) {
  if (ts.isForOfStatement(node) && node.expression.getText(source).startsWith('discoverApplicationRoutes(')) routeLoop = node;
  if (ts.isExpressionStatement(node) && node.getText(source).startsWith('expect([...verifiedPolicyUrls].sort()')) verifiedReceiptAssertion = node;
  if (ts.isCallExpression(node) && node.expression.getText(source) === 'page.on' && node.arguments[0]?.getText(source) === '"response"') responseListener = node.arguments[1];
  ts.forEachChild(node, findLoop);
}
findLoop(source);
assert.ok(routeLoop, 'the maintained smoke must crawl the discovered application routes');

test('the maintained response listener supplies the authenticated actor to policy classification', async () => {
  assert.ok(responseListener);
  const result = results(), exports = {};
  vm.runInNewContext(compile(`exports.record = ${responseListener.getText(source)};`), {
    exports, actor, baseUrl: valid.origin, ...result, recordSmokeHttpResponse: policy.recordSmokeHttpResponse,
  });
  exports.record({ status: () => valid.status, url: () => valid.url,
    request: () => ({ method: () => valid.method, headers: () => valid.headers }), json: async () => valid.body });
  await Promise.all(result.policyChecks);
  assert.deepEqual(result.failures, []);
  assert.deepEqual([...result.verifiedPolicyUrls], [valid.url]);
});

const expectedPolicyUrls = [
  '/api/v1/settings/voice', '/api/v1/sona', '/api/v1/agents/goose', '/api/v1/growth/time-savings',
].map(path => new URL(path, valid.origin).href).sort();
function verifyReceiptSet(urls) {
  assert.ok(verifiedReceiptAssertion, 'the smoke must require exact verified policy receipts');
  vm.runInNewContext(compile(verifiedReceiptAssertion.getText(source)), {
    URL, baseUrl: valid.origin, verifiedPolicyUrls: new Set(urls),
    expect: actual => ({ toEqual: expected => assert.deepEqual(Array.from(actual), Array.from(expected)) }),
  });
}
test('the maintained smoke requires the exact voice, runtime and measured-savings receipt set', () => {
  verifyReceiptSet(expectedPolicyUrls);
});
for (const missing of expectedPolicyUrls) {
  test(`the maintained smoke rejects a missing verified receipt: ${missing}`, () => {
    assert.throws(() => verifyReceiptSet(expectedPolicyUrls.filter(url => url !== missing)));
  });
}
test('the maintained smoke rejects an extra exempted URL', () => {
  assert.throws(() => verifyReceiptSet([...expectedPolicyUrls, valid.origin + '/api/v1/other']));
});

async function crawlDashboard({ unavailable = true, metrics = false, routes = ['/dashboard'],
  policyResult = results(), onNavigate = () => {} } = {}) {
  const exports = {}, checks = [];
  const locator = (kind, value, parent = '') => ({
    kind, value, parent,
    getByRole: (role, options = {}) => locator(role, options.name, kind),
    getByText: text => locator('text', text, kind),
  });
  vm.runInNewContext(compile(`async function crawl() {
    const routeFailures: string[] = [], contentFailures: string[] = [];
    ${routeLoop.getText(source)}
    return routeFailures;
  }; exports.crawl = crawl;`), {
    exports, URL, baseUrl: valid.origin, actor, discoverApplicationRoutes: () => routes,
    policyChecks: policyResult.policyChecks,
    runtimeUnavailableMessage: runtime.runtimeUnavailableMessage,
    page: {
      goto: async url => { onNavigate(new URL(url).pathname); return { status: () => 200 }; },
      getByRole: (role, options = {}) => locator(role, options.name),
      locator: () => ({ innerText: async () => 'Dashboard' }),
    },
    expect: value => {
      const matches = () => value.kind === 'heading' || value.kind === 'region'
        ? value.value === 'Recorded time savings'
        : typeof value.value === 'string' ? unavailable && value.value === 'Recorded time-savings data is unavailable.'
          : metrics && value.value.test('Recorded estimate: 12 hours saved. Customer inquiries handled: 45. Appointments scheduled: 8.');
      return {
        toBeVisible: async () => { checks.push('visible'); assert.ok(matches(), `Expected visible ${value.value}`); },
        toHaveCount: async count => { checks.push('count'); assert.equal(Number(matches()), count); },
        not: { toBeVisible: async () => { checks.push('absent'); assert.ok(!matches(), 'Fabricated metrics were visible'); } },
      };
    },
  });
  return { failures: Array.from(await exports.crawl()), checks };
}

test('the maintained dashboard smoke verifies unavailable savings and no metrics', async () => {
  const result = await crawlDashboard();
  assert.deepEqual(result.failures, []);
  assert.ok(result.checks.length >= 2, 'dashboard savings must be inspected during the actual crawl');
});
test('the maintained dashboard smoke rejects a missing unavailable message', async () => {
  assert.equal((await crawlDashboard({ unavailable: false })).failures.length, 1);
});
test('the maintained dashboard smoke rejects sample savings metrics', async () => {
  assert.equal((await crawlDashboard({ metrics: true })).failures.length, 1);
});

for (const readable of [true, false]) {
  test(`the maintained crawl retains the dashboard until its savings body settles (readable: ${readable})`, async () => {
    const policyResult = results(), navigations = [];
    let resolveBody, rejectBody, enteredDashboard;
    const body = new Promise((resolve, reject) => { resolveBody = resolve; rejectBody = reject; });
    const entered = new Promise(resolve => { enteredDashboard = resolve; });
    const crawl = crawlDashboard({ routes: ['/dashboard', '/next'], policyResult, onNavigate: route => {
      navigations.push(route);
      if (route === '/dashboard') {
        record(valid, policyResult, actor, () => body);
        enteredDashboard();
      }
    } });
    await entered;
    // Let every immediately runnable crawl continuation finish. Only the real
    // response-body promise should be able to prevent the next navigation.
    await new Promise(setImmediate);
    const beforeBody = [...navigations];
    if (readable) resolveBody(valid.body);
    else rejectBody(new Error('Browser response body unavailable'));
    await crawl;
    await Promise.all(policyResult.policyChecks);
    assert.deepEqual(beforeBody, ['/dashboard']);
    assert.deepEqual(navigations, ['/dashboard', '/next']);
    assert.deepEqual(policyResult.failures, readable ? [] : [`501 ${valid.url}`]);
    assert.deepEqual([...policyResult.verifiedPolicyUrls], readable ? [valid.url] : []);
  });
}
