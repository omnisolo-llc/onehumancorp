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
async function auditPages(pages) {
  let callback;
  let failures;
  const visited = [];
  const requestUrls = [];
  const listeners = new Map();
  const register = (name, body) => { if (name === 'every app page loads without visible crash output') callback = body; };
  register.describe = (_name, body) => body();
  register.describe.configure = () => {};
  register.setTimeout = () => {};
  const expect = value => ({
    toBeTruthy: () => assert.ok(value),
    toBeGreaterThan: minimum => assert.ok(value > minimum),
    toEqual: expected => { assert.equal(expected.length, 0); failures = Array.from(value); },
  });
  const dependencies = {
    './fixtures': { test: register, expect },
    '../../scripts/ui-click-audit.cjs': { discoverAppRoutes: () => pages.map(page => page.route) },
    './support/ui_click_audit': {},
    './authenticate': {},
    './identities': {},
    './support/ui_audit_navigation': { createAuditNavigation: () => async (_page, route) => {
      visited.push(route);
      const row = pages.find(page => page.route === route);
      if (row.runtimeError) listeners.get('pageerror')(new Error(row.runtimeError));
      return { finalUrl: `http://127.0.0.1:44041${route}` };
    } },
    './support/dashboard_audit_fixture': {},
    '../../scripts/ui-audit-fixture.cjs': {},
    '../../scripts/ui-audit-inventory.cjs': {},
  };
  vm.runInNewContext(compiled, {
    exports: {}, __dirname: path.dirname(filename), process,
    console: { info() {} },
    require: name => name in dependencies ? dependencies[name] : require(name),
  }, { filename });
  assert.equal(typeof callback, 'function');
  const current = () => pages.find(page => page.route === visited.at(-1));
  await callback({ page: {
    on: (name, handler) => listeners.set(name, handler),
    request: { get: async url => { requestUrls.push(url); return { status: () => current().status ?? 200 }; } },
    locator: () => ({ innerText: async () => current().text }),
  } });
  assert.deepEqual(visited, pages.map(page => page.route), 'every discovered route must still be audited');
  assert.deepEqual(requestUrls, pages.map(page => `http://127.0.0.1:44041${page.route}`));
  return failures;
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
