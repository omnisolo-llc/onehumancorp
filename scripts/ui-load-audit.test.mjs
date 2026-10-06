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

const documentsFile = path.resolve('src/e2e/support/ui_audit_documents.ts');
const documents = {};
vm.runInNewContext(ts.transpileModule(fs.readFileSync(documentsFile, 'utf8'), {
  compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
}).outputText, { exports: documents, URL }, { filename: documentsFile });

// Execute the maintained audit callback. Only the browser boundary is supplied
// explicitly; no DOM fixtures, HTTP requests or browser transport are started.
async function auditPages(pages, capture = {}) {
  let callback;
  let failures;
  const visited = [];
  const requestUrls = [];
  const laneStates = new WeakMap();
  const visits = [];
  const requests = [];
  const rendered = [];
  const timeouts = [];
  const attachments = [];
  const steps = [];
  const logs = [];
  const register = (name, body) => { if (name === 'every app page loads without visible crash output') callback = body; };
  register.describe = (_name, body) => body();
  register.describe.configure = () => {};
  register.setTimeout = timeout => timeouts.push(timeout);
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
    './support/ui_audit_documents': documents,
    './authenticate': {},
    './identities': {},
    './support/ui_audit_navigation': { createAuditNavigation: () => async (page, route) => {
      const row = await navigate(page, route);
      return { finalUrl: row.finalUrl ?? `http://127.0.0.1:44041${route}` };
    } },
    './support/dashboard_audit_fixture': {},
    './support/quote_audit_fixture': { quoteAuditRoutes: new Set(quoteRoutes), prepareQuoteAudit: async (page, _baseURL, route) => {
      preparedQuotes.push(route);
      const row = await navigate(page, route);
      return { finalUrl: `http://127.0.0.1:44041${row.recordRoute}` };
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
  async function navigate(page, route) {
    const state = laneStates.get(page);
    const row = pages.find(page => page.route === route);
    state.current = row;
    visited.push(route);
    visits.push({ route, lane: state.lane });
    await capture.onNavigate?.({ route, lane: state.lane });
    if (row.navigationError) throw row.navigationError;
    if (row.runtimeError) state.listeners.get('pageerror')(new Error(row.runtimeError));
    return row;
  }
  function browserPage(lane) {
    const state = { lane, current: undefined, listeners: new Map(), context: {} };
    const page = {
      context: () => capture.sharedContext ?? state.context,
      on: (name, handler) => state.listeners.set(name, handler),
      request: { get: async url => {
        const row = state.current;
        requestUrls.push(url);
        requests.push({ route: row.route, url, lane });
        await capture.onRequest?.({ route: row.route, lane });
        if (row.requestError) throw row.requestError;
        return { status: () => row.status ?? 200 };
      } },
      evaluateHandle: async predicate => {
        const row = state.current;
        await capture.onRender?.({ route: row.route, lane });
        rendered.push({ route: row.route, lane });
        const document = { body: { innerText: row.text }, querySelectorAll: () => [] };
        const result = vm.runInNewContext(`(${predicate.toString()})()`, { document });
        return { evaluate: async (evaluate, argument) => evaluate(result, argument), dispose: async () => {} };
      },
    };
    laneStates.set(page, state);
    return page;
  }
  const browserPages = [browserPage(0), browserPage(1)];
  Object.assign(capture, { attachments, steps, logs, visited, visits, requestUrls, requests, rendered, timeouts });
  let error;
  try { await callback({ page: browserPages[0], anonymousPage: browserPages[1] }); }
  catch (caught) { error = caught; }
  capture.error = error;
  if (error) {
    if (!capture.allowFailure) throw error;
    return failures;
  }
  assert.deepEqual([...visited].sort(), pages.map(page => page.route).sort(), 'every discovered route must be audited exactly once');
  assert.deepEqual([...requestUrls].sort(), pages.map(page => page.finalUrl ?? `http://127.0.0.1:44041${page.recordRoute ?? page.route}`).sort(), 'every actual final document must be requested exactly once');
  assert.deepEqual([...preparedQuotes].sort(), pages.filter(page => quoteRoutes.includes(page.route)).map(page => page.route).sort());
  return failures;
}

function progress(capture) {
  const attachment = capture.attachments.find(item => item.name === 'ohc-load-audit-progress-v1');
  assert.ok(attachment, 'the actual load audit must retain progress even when an operation throws');
  assert.equal(attachment.contentType, 'application/json');
  return JSON.parse(attachment.body.toString('utf8'));
}

function barrier() {
  let release;
  const pending = new Promise(resolve => { release = resolve; });
  return { pending, release };
}
const nextTurn = () => new Promise(resolve => setImmediate(resolve));

test('all 213 routes run exactly once in two isolated, sequential lanes under the unchanged deadline', async () => {
  const gate = barrier();
  const activeLanes = new Set();
  let peak = 0;
  const capture = { onNavigate: async ({ lane }) => {
    assert.ok(!activeLanes.has(lane), 'a lane cannot replace its document before its previous read finishes');
    activeLanes.add(lane);
    peak = Math.max(peak, activeLanes.size);
    try { await gate.pending; } finally { activeLanes.delete(lane); }
  } };
  const pages = Array.from({ length: 213 }, (_, index) => ({ route: `/route-${index}`, text: 'Rendered page' }));
  const running = auditPages(pages, capture);
  try {
    await nextTurn();
    assert.deepEqual([...capture.visits], [{ route: '/route-0', lane: 0 }, { route: '/route-1', lane: 1 }], 'both independent contexts must start before either navigation completes');
    assert.equal(capture.attachments.length, 0, 'no completion receipt while work is pending');
  } finally { gate.release(); await running; }
  assert.equal(peak, 2);
  for (const lane of [0, 1]) {
    const expected = pages.filter((_, index) => index % 2 === lane).map(page => page.route);
    assert.deepEqual(capture.visits.filter(row => row.lane === lane).map(row => row.route), expected);
    assert.deepEqual(capture.requests.filter(row => row.lane === lane).map(row => row.route), expected);
    assert.deepEqual(capture.rendered.filter(row => row.lane === lane).map(row => row.route), expected);
  }
  assert.deepEqual(capture.timeouts, [180000]);
  assert.equal(capture.steps.length, 213 * 3);
  assert.equal(progress(capture).completedRoutes, 213);
  assert.equal(progress(capture).routeCount, 213);
  assert.deepEqual(progress(capture).lanes, [
    { lane: 0, routeCount: 107, completedRoutes: 107 },
    { lane: 1, routeCount: 106, completedRoutes: 106 },
  ]);
});

for (const phase of ['onRequest', 'onRender']) {
  test(`each lane completes ${phase} before replacing its document or writing the final receipt`, async () => {
    const gate = barrier();
    const capture = { [phase]: async () => { await gate.pending; } };
    const pages = Array.from({ length: 4 }, (_, index) => ({ route: `/route-${index}`, text: 'Rendered page' }));
    const running = auditPages(pages, capture);
    try {
      await nextTurn();
      assert.deepEqual([...capture.visited], ['/route-0', '/route-1'], 'neither page may navigate onward while its document checks are pending');
      assert.equal(capture.attachments.length, 0);
    } finally { gate.release(); await running; }
    assert.equal(capture.rendered.length, 4);
    assert.equal(progress(capture).completedRoutes, 4);
  });
}

test('load lanes reject a shared BrowserContext before any route is visited', async () => {
  const capture = { allowFailure: true, sharedContext: {} };
  await auditPages([{ route: '/first', text: 'Rendered page' }], capture);
  assert.match(String(capture.error), /independent browser contexts/);
  assert.deepEqual(capture.visited, []);
});

for (const failedLane of [0, 1]) {
  test(`lane ${failedLane} transport failure waits for the other lane before the final receipt and original rejection`, async () => {
    const original = new Error(`lane ${failedLane} original document failure`);
    const gate = barrier();
    let settled = false;
    const pages = Array.from({ length: 4 }, (_, index) => ({ route: `/route-${index}`, text: 'Rendered page',
      ...(index === failedLane ? { requestError: original } : {}) }));
    const capture = { allowFailure: true, onRequest: async ({ route }) => {
      if (route === `/route-${1 - failedLane}`) await gate.pending;
    } };
    const running = auditPages(pages, capture).then(() => { settled = true; });
    try {
      await nextTurn();
      assert.equal(settled, false, 'a rejected lane must not leave another lane running after the callback returns');
      assert.equal(capture.attachments.length, 0, 'finally progress must wait for the remaining lane');
    } finally { gate.release(); await running; }
    assert.equal(capture.error, original);
    const healthyRoutes = pages.filter((_, index) => index % 2 !== failedLane).map(page => page.route);
    assert.deepEqual(capture.rendered.map(row => row.route), healthyRoutes);
    assert.ok(!capture.visited.includes(`/route-${failedLane + 2}`), 'a failed lane does not resume navigation');
    const report = progress(capture);
    assert.equal(report.completedRoutes, 2);
    assert.equal(report.routeCount, 4);
    assert.equal(report.recentPhases.find(row => row.route === `/route-${failedLane}` && row.phase === 'final-document').completed, false);
    assert.equal(capture.attachments.length, 1);
  });
}

test('uncaught page errors from both isolated contexts remain failures', async () => {
  const failures = await auditPages([
    { route: '/first', text: 'Rendered page', runtimeError: 'first context crashed' },
    { route: '/second', text: 'Rendered page', runtimeError: 'second context crashed' },
  ]);
  assert.deepEqual(failures.sort(), ['uncaught page error: first context crashed', 'uncaught page error: second context crashed']);
});

test('concurrent final document statuses stay attached to their own routes', async () => {
  const gate = barrier();
  const capture = { onRequest: async ({ lane }) => { if (lane === 0) await gate.pending; } };
  const running = auditPages([
    { route: '/slow-document', status: 404, text: '' },
    { route: '/fast-document', status: 500, text: '' },
  ], capture);
  try {
    await nextTurn();
    assert.equal(capture.requests.length, 2);
  } finally { gate.release(); }
  assert.deepEqual((await running).sort(), ['/fast-document: HTTP 500', '/slow-document: HTTP 404']);
  const documents = progress(capture).recentPhases.filter(row => row.phase === 'final-document');
  assert.deepEqual(documents.map(row => [row.route, row.status]), [['/fast-document', 500], ['/slow-document', 404]]);
});

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

for (const [failure, phase, expectedRequests] of [['navigationError', 'navigation', 2], ['requestError', 'final-document', 3]]) {
  test(`the load audit retains the exact failing ${phase} and original error without private URL data`, async () => {
    const original = new Error('transport failure at /private-record?token=PRIVATE_TOKEN');
    const capture = { allowFailure: true };
    await auditPages([
      { route: '/first', text: 'Rendered page' },
      { route: '/failed', text: 'Rendered page', [failure]: original },
      { route: '/other-lane-finishes', text: 'Healthy lane must finish' },
    ], capture);
    assert.equal(capture.error, original, 'diagnostics must not replace or suppress the original failure');
    const report = progress(capture);
    assert.equal(report.completedRoutes, 2, 'healthy lane must finish before failure escapes');
    assert.equal(report.routeCount, 3);
    const failedPhase = report.recentPhases.find(row => row.route === '/failed' && row.phase === phase);
    assert.equal(failedPhase.completed, false);
    assert.equal(capture.requestUrls.length, expectedRequests);
    assert.deepEqual(capture.visited, ['/first', '/failed', '/other-lane-finishes']);
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
