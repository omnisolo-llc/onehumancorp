import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import vm from 'node:vm';
import { createRequire } from 'node:module';
import ts from 'typescript';

const require = createRequire(import.meta.url);
const filename = path.resolve('src/e2e/comprehensive_ui_contract.spec.ts');
const compiled = ts.transpileModule(fs.readFileSync(filename, 'utf8'), {
  compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022, esModuleInterop: true },
}).outputText;

// Execute the maintained audit callback. Only the browser boundary is supplied
// explicitly; no DOM fixtures, HTTP requests or browser transport are started.
async function auditPages(pages, capture = {}) {
  let callback;
  let failures;
  const visited = [];
  const requestUrls = [];
  const listeners = new Map();
  const attachments = [];
  const steps = [];
  const logs = [];
  const register = (name, body) => { if (name === 'every app page loads without visible crash output') callback = body; };
  register.describe = (_name, body) => body();
  register.describe.configure = () => {};
  register.setTimeout = () => {};
  register.step = async (name, body) => { steps.push(name); return body(); };
  register.info = () => ({ attach: (name, attachment) => {
    if (capture.attachmentError) throw capture.attachmentError;
    attachments.push({ name, ...attachment });
    return Promise.resolve();
  } });
  const expect = value => ({
    toBeTruthy: () => assert.ok(value),
    toBeGreaterThan: minimum => assert.ok(value > minimum),
    toEqual: expected => { assert.equal(expected.length, 0); failures = Array.from(value); },
  });
  const quoteRoutes = ['/quotes/e2e-id', '/quote/e2e-id', '/quoting', '/proposals/customer-view'];
  const preparedQuotes = [];
  const dependencies = {
    './fixtures': { test: register, expect },
    '../../scripts/ui-click-audit.cjs': { discoverAppRoutes: () => pages.map(page => page.route) },
    './support/ui_click_audit': {},
    './authenticate': {},
    './identities': {},
    './support/ui_audit_navigation': { createAuditNavigation: () => async (_page, route) => {
      visited.push(route);
      const row = pages.find(page => page.route === route);
      if (row.navigationError) throw row.navigationError;
      if (row.runtimeError) listeners.get('pageerror')(new Error(row.runtimeError));
      return { finalUrl: row.finalUrl ?? `http://127.0.0.1:44041${route}` };
    } },
    './support/dashboard_audit_fixture': {},
    './support/quote_audit_fixture': { quoteAuditRoutes: new Set(quoteRoutes), prepareQuoteAudit: async (_page, _baseURL, route) => {
      preparedQuotes.push(route); visited.push(route);
      return { finalUrl: `http://127.0.0.1:44041${pages.find(page => page.route === route).recordRoute}` };
    } },
    '../../scripts/ui-audit-fixture.cjs': {},
    '../../scripts/ui-audit-inventory.cjs': {},
  };
  vm.runInNewContext(compiled, {
    exports: {}, __dirname: path.dirname(filename), process: { env: { ...process.env, BASE_URL: 'http://127.0.0.1:44041' } },
    console: { info: (...args) => logs.push(args.join(' ')) }, Buffer, URL,
    require: name => name in dependencies ? dependencies[name] : require(name),
  }, { filename });
  assert.equal(typeof callback, 'function');
  const current = () => pages.find(page => page.route === visited.at(-1));
  let error;
  try { await callback({ page: {
    on: (name, handler) => listeners.set(name, handler),
    request: { get: async url => { requestUrls.push(url); if (current().requestError) throw current().requestError; return { status: () => current().status ?? 200 }; } },
    locator: () => ({ innerText: async () => current().text }),
  } }); } catch (caught) { error = caught; }
  Object.assign(capture, { attachments, steps, logs, visited, requestUrls, error });
  if (error) {
    if (!capture.allowFailure) throw error;
    return failures;
  }
  assert.deepEqual(visited, pages.map(page => page.route), 'every discovered route must still be audited');
  assert.deepEqual(requestUrls, pages.map(page => page.finalUrl ?? `http://127.0.0.1:44041${page.recordRoute ?? page.route}`));
  assert.deepEqual(preparedQuotes, pages.filter(page => quoteRoutes.includes(page.route)).map(page => page.route));
  return failures;
}

function progress(capture) {
  const attachment = capture.attachments.find(item => item.name === 'ohc-load-audit-progress-v1');
  assert.ok(attachment, 'the actual load audit must retain progress even when an operation throws');
  assert.equal(attachment.contentType, 'application/json');
  return JSON.parse(attachment.body.toString('utf8'));
}

test('a synchronous diagnostic attachment error preserves the original document failure and console progress', async () => {
  const original = new Error('original final-document transport failure');
  const capture = { allowFailure: true, attachmentError: new Error('attachment unavailable after timeout') };
  await auditPages([{ route: '/dashboard', text: 'Dashboard', requestError: original }], capture);
  assert.equal(capture.error, original);
  const line = capture.logs.find(value => value.startsWith('Load audit progress: '));
  assert.ok(line, 'bounded progress must survive attachment teardown');
  const report = JSON.parse(line.slice('Load audit progress: '.length));
  assert.equal(report.recentPhases.at(-1).phase, 'final-document');
  assert.equal(report.recentPhases.at(-1).completed, false);
  assert.ok(capture.logs.includes('Load audit progress attachment unavailable.'));
});

test('the load audit records completed routes and bounded phase timings without changing coverage', async () => {
  const capture = {};
  const pages = Array.from({ length: 12 }, (_, index) => ({ route: `/fixture-${index}`, text: 'Rendered page' }));
  assert.deepEqual(await auditPages(pages, capture), []);
  const report = progress(capture);
  assert.equal(report.routeCount, 12);
  assert.equal(report.completedRoutes, 12);
  assert.equal(report.recentPhases.length, 20);
  assert.equal(report.omittedPhases, 16);
  assert.deepEqual(Object.keys(report.phaseTotalsMs).sort(), ['final-document', 'navigation', 'rendered-content']);
  assert.ok(Object.values(report.phaseTotalsMs).every(value => Number.isFinite(value) && value >= 0));
  assert.ok(report.recentPhases.every(row => row.completed && Number.isFinite(row.elapsedMs) && row.elapsedMs >= 0));
  assert.equal(capture.steps.length, pages.length * 3, 'all three maintained checks run on every discovered page');
  assert.equal(capture.logs.filter(line => line.startsWith('Load audit progress: ')).length, 1);
});

for (const [failure, phase, expectedRequests] of [['navigationError', 'navigation', 1], ['requestError', 'final-document', 2]]) {
  test(`the load audit retains the exact failing ${phase} and original error without private URL data`, async () => {
    const original = new Error('transport failure at /private-record?token=PRIVATE_TOKEN');
    const capture = { allowFailure: true };
    await auditPages([
      { route: '/first', text: 'Rendered page' },
      { route: '/failed', text: 'Rendered page', [failure]: original },
      { route: '/not-reached', text: 'Must remain unvisited' },
    ], capture);
    assert.equal(capture.error, original, 'diagnostics must not replace or suppress the original failure');
    const report = progress(capture);
    assert.equal(report.completedRoutes, 1);
    assert.equal(report.routeCount, 3);
    assert.equal(report.recentPhases.at(-1).route, '/failed');
    assert.equal(report.recentPhases.at(-1).phase, phase);
    assert.equal(report.recentPhases.at(-1).completed, false);
    assert.equal(capture.requestUrls.length, expectedRequests);
    assert.deepEqual(capture.visited, ['/first', '/failed']);
    assert.doesNotMatch(JSON.stringify({ report, logs: capture.logs, steps: capture.steps }), /PRIVATE_TOKEN|private-record/);
  });
}

test('load progress identifies the source route template without recording the real quote ID or query', async () => {
  const capture = {};
  await auditPages([{ route: '/quotes/e2e-id', recordRoute: '/quotes/private-customer-quote?token=PRIVATE_TOKEN', text: 'Actual persisted quote' }], capture);
  const report = progress(capture);
  assert.equal(report.recentPhases[0].route, '/quotes/[id]');
  assert.doesNotMatch(JSON.stringify({ report, logs: capture.logs, steps: capture.steps }), /private-customer-quote|PRIVATE_TOKEN|e2e-id/);
});

test('load progress identifies a known final landing document independently of its source route', async () => {
  const capture = {};
  await auditPages([
    { route: '/share-card', finalUrl: 'http://127.0.0.1:44041/onboarding?ref=PRIVATE_TOKEN#secret-fragment', text: 'Ready onboarding' },
    { route: '/onboarding', text: 'Ready onboarding' },
  ], capture);
  const report = progress(capture);
  const final = report.recentPhases.find(row => row.route === '/share-card' && row.phase === 'final-document');
  assert.equal(final.documentPath, '/onboarding');
  assert.doesNotMatch(JSON.stringify({ report, logs: capture.logs, steps: capture.steps }), /PRIVATE_TOKEN|secret-fragment|127\.0\.0\.1/);
});

for (const [source, finalUrl, expected] of [
  ['/quotes/e2e-id', 'http://127.0.0.1:44041/quotes/Ali', '/quotes/[id]'],
  ['/onboarding', 'http://127.0.0.1:44041/onboarding/Ali', '/[redacted]'],
  ['/onboarding', 'http://127.0.0.1:44041/private/Ali', '/[redacted]'],
  ['/onboarding', 'https://outside.invalid/onboarding', '/[redacted]'],
  ['/onboarding', 'http://private:secret@127.0.0.1:44041/onboarding', '/[redacted]'],
]) {
  test(`load progress retains only known final document templates: ${source} -> ${expected} (${finalUrl})`, async () => {
    const capture = { allowFailure: true };
    const original = new Error('Final document request failed');
    await auditPages([{ route: source, recordRoute: '/quotes/Ali', finalUrl, requestError: original, text: 'Rendered page' }], capture);
    assert.equal(capture.error, original);
    const report = progress(capture);
    const final = report.recentPhases.at(-1);
    assert.equal(final.phase, 'final-document');
    assert.equal(final.completed, false);
    assert.equal(final.documentPath, expected);
    assert.doesNotMatch(JSON.stringify({ report, logs: capture.logs, steps: capture.steps }), /Ali|outside\.invalid|private|secret|127\.0\.0\.1/);
  });
}

const validText = [
  '<iframe src="http://127.0.0.1:44041/embed/widget?tenant_id=e2e-tenant" title="Intake Widget"></iframe>',
  'http://127.0.0.1:44041/api/v1/growth/insight-widget/embed?tenant=e2e-tenant',
  'Join my journey\nhttp://127.0.0.1:44041/onboarding?ref=e2e-tenant',
  'http://127.0.0.1:44041/unlock?tenant=e2e-tenant&code=SECRET20',
  '<iframe src="http://127.0.0.1:44041/api/v1/growth/viral-countdown-widget/embed"></iframe>',
  '<iframe src="http://127.0.0.1:44041/api/v1/growth/viral-goal-tracker?target=10"></iframe>',
  'Business Snapshot\nRevenue\n$404\nJoin my journey\nhttp://127.0.0.1:44041/onboarding?ref=e2e-tenant',
  'Customer ID 0404a6b7-4044-4404-8404-004044040404',
  '404 orders recorded, 1,404 customers, $404.50 in sales',
];
for (const [index, text] of validText.entries()) {
  test(`real load audit does not misclassify a displayed URL or business value ${index + 1}`, async () => {
    assert.deepEqual(await auditPages([{ route: '/arbitrary-page', text }]), []);
  });
}

for (const text of [
  '404\nThis page could not be found.',
  '404',
  'Page not found',
  'Customer not found.',
  'Application error: a client-side exception has occurred',
  'Failed to load customer history.',
  '500\nInternal Server Error',
  'HTTP 404: Resource unavailable',
]) {
  test(`real load audit retains rendered error detection: ${text}`, async () => {
    const failures = await auditPages([{ route: '/arbitrary-page', text }]);
    assert.equal(failures.length, 1);
    assert.match(failures[0], /visible error/);
    assert.ok(text.split('\n').some(line => failures[0].includes(line)), 'report the full matching text');
  });
}

test('real load audit retains HTTP errors and uncaught exceptions independently of text', async () => {
  const failures = await auditPages([
    { route: '/missing', status: 404, text: '' },
    { route: '/failed', status: 500, text: 'Welcome' },
    { route: '/runtime', runtimeError: 'Cannot read properties of undefined', text: 'Welcome' },
  ]);
  assert.deepEqual(failures, ['/missing: HTTP 404', '/failed: HTTP 500', 'uncaught page error: Cannot read properties of undefined']);
});

for (const route of ['/quotes/e2e-id', '/quote/e2e-id', '/quoting', '/proposals/customer-view']) {
  test(`the global load audit prepares and inspects the actual record document for ${route}`, async () => {
    const id = '11111111-1111-4111-8111-111111111111';
    const recordRoute = route.endsWith('/e2e-id') ? route.replace('e2e-id', id) : `${route}?id=${id}`;
    assert.deepEqual(await auditPages([{ route, recordRoute, text: 'Actual persisted quote' }]), []);
    const failed = await auditPages([{ route, recordRoute, text: 'Not Found\nQuote not found' }]);
    assert.equal(failed.length, 1, 'preparation must never whitelist a broken quote page');
  });
}
