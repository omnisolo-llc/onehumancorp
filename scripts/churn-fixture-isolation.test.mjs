import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';
import ts from 'typescript';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const { createOwnedAuditSeed } = require('./ui-audit-fixture.cjs');
const source = readFileSync(new URL('../src/ui/next/src/e2e/subscription_churn_retention.spec.ts', import.meta.url), 'utf8');
const canonical = readFileSync(new URL('../src/e2e/e2e-seed.sql', import.meta.url), 'utf8');

// Execute the actual spec callback against a small fixture-boundary model.
// This is a unit regression for ownership, not a real browser/server pass.
// Another test already approved the global seed before this journey starts.
test('churn journey survives a consumed shared seed and repeated isolated cases', async () => {
  const rows = new Map([['e2e-feed-churn', { tenant: 'e2e-tenant', state: 'APPROVED' }]]);
  const actors = [], logins = [], writes = [];
  let body;
  const pwTest = (_title, callback) => { body = callback; };
  pwTest.describe = (_title, callback) => callback();
  pwTest.use = pwTest.setTimeout = () => {};
  const expect = actual => ({
    toBeVisible: async () => assert.ok(actual.visible(), 'required churn card must remain visible after another case consumed the shared seed'),
    not: { toBeVisible: async () => assert.equal(actual.visible(), false) },
    toBe: value => assert.equal(actual, value),
    toEqual: value => assert.deepEqual(JSON.parse(JSON.stringify(actual)), JSON.parse(JSON.stringify(value))),
  });
  const dependencies = {
    '../../../../e2e/fixtures': { test: pwTest, expect },
    '../../../../e2e/support/dashboard_audit_fixture': { seedDashboardAuditOwner: async () => {
      const digit = String(actors.length + 1);
      const namespace = `audit-${digit.repeat(8)}-${digit.repeat(4)}-4${digit.repeat(3)}-8${digit.repeat(3)}-${digit.repeat(12)}`;
      const actor = createOwnedAuditSeed(canonical, namespace);
      assert.ok(actor.sql.includes(`'${namespace}-leo@example.com'`));
      assert.ok(actor.sql.includes(`'${namespace}-e2e-feed-churn'`));
      actors.push(actor);
      rows.set(`${namespace}-e2e-feed-churn`, { tenant: actor.tenantId, state: 'PENDING_APPROVAL' });
      return actor;
    } },
    '../../../../e2e/db_utils': { e2eDbQuery: async (_sql, [id, tenant]) => {
      const row = rows.get(id);
      return row?.tenant === tenant ? [{ lifecycle_state: row.state }] : [];
    } },
  };
  vm.runInNewContext(ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS } }).outputText,
    { exports: {}, URL, require: name => { assert.ok(name in dependencies, name); return dependencies[name]; } });
  assert.equal(typeof body, 'function');
  async function run() {
    const form = {}; let tenant, pendingResponse;
    const pending = () => [...rows].filter(([,r]) => r.tenant === tenant && r.state === 'PENDING_APPROVAL');
    const locator = (kind, label = '') => ({
      fill: async value => { form[label] = value; },
      first() { return this; }, filter() { return this; },
      locator: () => locator('approve'),
      visible: () => ['cards','approve'].includes(kind) ? pending().length > 0 : true,
      click: async () => {
        if (kind === 'login') { tenant = form.Organization; logins.push({ ...form }); return; }
        assert.equal(kind, 'approve');
        const [id,row] = pending()[0]; row.state = 'APPROVED'; writes.push({ id, tenant });
        const response = { url: () => `http://127.0.0.1:3000/api/v1/agent-feed/${id}/state`, request: () => ({ method: () => 'PUT' }), status: () => 200 };
        if (pendingResponse) { assert.ok(pendingResponse.predicate(response)); pendingResponse.resolve(response); }
      },
    });
    const page = {
      goto: async () => {},
      getByRole: (role, options) => locator(role === 'button' ? 'login' : 'input', options.name),
      getByLabel: () => locator('input', 'Organization'),
      getByTestId: () => locator('container'),
      locator: selector => locator(selector.includes('agent-feed-card') ? 'cards' : 'heading'),
      waitForResponse: predicate => new Promise(resolve => { pendingResponse = { predicate, resolve }; }),
    };
    await body({ anonymousPage: page, baseURL: 'http://127.0.0.1:3000' });
  }
  await run(); await run();
  assert.equal(actors.length, 2);
  assert.notEqual(actors[0].tenantId, actors[1].tenantId);
  assert.equal(rows.get('e2e-feed-churn').state, 'APPROVED', 'never reset another case\'s row');
  assert.equal(writes.length, 2);
  for (const [index,actor] of actors.entries()) {
    assert.equal(logins[index]['Email or username'], `${actor.namespace}-leo@example.com`);
    assert.equal(logins[index].Password, actor.password);
    assert.equal(logins[index].Organization, actor.tenantId);
    assert.equal(writes[index].id, `${actor.namespace}-e2e-feed-churn`);
    assert.equal(rows.get(writes[index].id).state, 'APPROVED');
  }
});
