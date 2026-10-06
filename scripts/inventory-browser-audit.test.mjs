import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import path from 'node:path';
import vm from 'node:vm';
import { createRequire } from 'node:module';
import ts from 'typescript';

const root = path.resolve('.');
const source = file => readFileSync(path.join(root, file), 'utf8');
test('every inventory click uses a new owned case and waits for both actual reads and stock controls', () => {
  const fixture = source('src/e2e/support/dashboard_audit_fixture.ts');
  const routes = fixture.slice(fixture.indexOf('export const isolatedClickAuditRoutes'), fixture.indexOf('export function clickAuditStates'));
  assert.match(routes, /'\/inventory'/);
  assert.match(fixture, /'\/inventory': \['\/api\/v1\/ui\/inventory', '\/api\/v1\/ui\/supply'\]/);
  assert.match(fixture, /await waitForInventoryAuditReady\(page\)/);
  const audit = source('src/e2e/comprehensive_ui_contract.spec.ts');
  assert.match(audit, /observeInventoryAuditClick\(owned\.page, target, owned\.actor, \(\) => observeClickEffects\(owned\.page, target\)\)/);
  assert.match(audit, /assertSameClickInventory\(entryKeys, restored\.map/);
});

test('malformed supply keeps the observed browser failure and proves the same still-malformed owned row by authenticated read', () => {
  const browser = source('src/e2e/inventory2.spec.ts');
  assert.match(browser, /expect\(response\.status\(\)\)\.toBe\(500\)/);
  assert.doesNotMatch(browser, /await response\.(?:json|body|text)\(/);
  assert.match(browser, /page\.request\.get\('\/api\/v1\/ui\/supply'/);
  assert.match(browser, /expect\(unavailable\.status\(\)\)\.toBe\(500\)/);
  assert.match(browser, /expect\(await unavailable\.json\(\)\)\.toEqual\(\{ error: 'supply_unavailable', success: false \}\)/);
  assert.ok(browser.indexOf('SELECT current_quantity') > browser.indexOf("region.getByRole('alert')"));
  assert.ok(browser.indexOf('SET current_quantity=2') > browser.indexOf('await unavailable.json()'));
  assert.match(browser, /toEqual\(\[\{ current_quantity: null \}\]\)/);
});

function load() {
  const filename = path.join(root, 'src/e2e/support/inventory_audit.ts');
  assert.ok(existsSync(filename), 'The inventory audit must verify its actual committed receipt before retiring the case');
  const exports = {};
  const nativeRequire = createRequire(filename);
  let rows = [];
  const queries = [];
  const code = ts.transpileModule(readFileSync(filename, 'utf8'), { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022, esModuleInterop: true } }).outputText;
  const alerts = {};
  const alertCode = ts.transpileModule(source('src/e2e/support/application_alerts.ts'), { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText;
  vm.runInThisContext(`(function(exports) { ${alertCode}\n })`)(alerts);
  vm.runInThisContext(`(function(require, exports) { ${code}\n })`, { filename })(name => name === '../db_utils' ? { e2eDbQuery: async (sql, args) => { queries.push({ sql, args }); return rows; } } : name === './application_alerts' ? alerts : nativeRequire(name), exports);
  return { ...exports, queries, persisted: value => { rows = value; } };
}
function boundary(module, fault, label = 'Increase stock') {
  const delta = label === 'Decrease stock' ? -1 : 1;
  const owner = { userId: 'owned-user', tenantId: 'owned-tenant' };
  const request = { id: 'actual-operation', payload: { item_id: 'owned-product', quantity_change: delta, expected_version: 'a'.repeat(64) } };
  const outcome = { id: request.id, item_id: request.payload.item_id, quantity_change: delta, previous_version: 'a'.repeat(64), inventory_version: 'b'.repeat(64), status: 'acknowledged', stock: 3 + delta };
  const events = [];
  const target = { getAttribute: async () => label, evaluate: async () => 'owned-product' };
  const captured = { postDataJSON: () => [request], headers: () => ({ 'x-ohc-expected-user': owner.userId, 'x-ohc-expected-tenant': fault === 'wrong-owner' ? 'foreign' : owner.tenantId }) };
  let stock = 3, requestPredicate, disposed = 0;
  module.persisted([{ receipt_json: JSON.stringify(fault === 'wrong-database' ? { ...outcome, stock: 19 } : outcome) }]);
  const page = {
    url: () => 'http://127.0.0.1:1234/inventory',
    waitForRequest: predicate => { requestPredicate = predicate; events.push('observe request'); return Promise.resolve(captured); },
    getByTestId: () => ({ textContent: async () => String(stock) }),
    getByText: () => ({ waitFor: async () => { events.push('confirmed UI'); if (fault === 'unknown-outcome') throw new Error('UI remains unconfirmed'); } }),
    request: { get: async url => {
      events.push(url);
      const body = fault === 'wrong-receipt' ? { success: true, outcomes: [{ ...outcome, id: 'other' }] } : { success: true, outcomes: [outcome] };
      return { status: () => fault === 'missing-receipt' ? 404 : 200, json: async () => { if (fault === 'unreadable-receipt') throw new Error('Receipt body unavailable'); return body; }, dispose: async () => { disposed += 1; } };
    } },
  };
  let clicks = 0;
  const observe = async () => { clicks += 1; events.push('click'); stock = 3 + delta; return { changed: true, requestSeen: true }; };
  return { owner, target, page, events, observe, clicks: () => clicks, disposed: () => disposed, matches: (url, method) => requestPredicate({ url: () => url, method: () => method }) };
}

test('a stock click awaits UI confirmation then the matching authoritative receipt and tenant-scoped persisted record', async () => {
  const module = load(), b = boundary(module);
  const observed = await module.observeInventoryAuditClick(b.page, b.target, b.owner, b.observe);
  assert.equal(observed.changed, true); assert.equal(b.clicks(), 1);
  assert.deepEqual(b.events.slice(0, 3), ['observe request', 'click', 'confirmed UI']);
  assert.match(b.events[3], /adjustment_id=actual-operation/);
  assert.equal(module.queries.length, 1);
  assert.match(module.queries[0].sql, /tenant_id=\$1 AND client_mutation_id=\$2 AND item_id=\$3/);
  assert.deepEqual([...module.queries[0].args], ['owned-tenant', 'actual-operation', 'owned-product']);
});
for (const fault of ['wrong-owner', 'unknown-outcome', 'missing-receipt', 'wrong-receipt', 'wrong-database']) {
  test(`an unconfirmed inventory click is never credited or replayed: ${fault}`, async () => {
    const module = load(), b = boundary(module, fault);
    await assert.rejects(module.observeInventoryAuditClick(b.page, b.target, b.owner, b.observe));
    assert.equal(b.clicks(), 1);
    assert.equal(b.events.filter(value => value.includes('adjustment_id=')).length <= 1, true);
  });
}
test('a non-stock control is observed once without receipt requests or database reads', async () => {
  const module = load(), b = boundary(module);
  b.target.getAttribute = async () => 'Reload inventory';
  await module.observeInventoryAuditClick(b.page, b.target, b.owner, b.observe);
  assert.equal(b.clicks(), 1); assert.deepEqual(b.events, ['click']); assert.equal(module.queries.length, 0);
});

test('decrease controls require their own negative-delta receipt and matching persisted count', async () => {
  const module = load(), b = boundary(module, undefined, 'Decrease stock');
  await module.observeInventoryAuditClick(b.page, b.target, b.owner, b.observe);
  assert.equal(b.clicks(), 1); assert.equal(module.queries.length, 1);
});
test('stock observation rejects foreign origins, receipt reads and other POST destinations', async () => {
  const module = load(), b = boundary(module);
  await module.observeInventoryAuditClick(b.page, b.target, b.owner, b.observe);
  assert.equal(b.matches('http://127.0.0.1:1234/api/v1/ui/inventory', 'POST'), true);
  assert.equal(b.matches('https://foreign.example/api/v1/ui/inventory', 'POST'), false);
  assert.equal(b.matches('http://127.0.0.1:1234/api/v1/ui/inventory?adjustment_id=id', 'GET'), false);
  assert.equal(b.matches('http://127.0.0.1:1234/api/v1/ui/supply', 'POST'), false);
});

test('inventory readiness uses the exact shared empty-Next-announcer exclusion without dropping app alerts', () => {
  const helper = source('src/e2e/support/inventory_audit.ts');
  assert.match(helper, /import \{ applicationAlertTexts \} from '\.\/application_alerts'/);
  assert.match(helper, /expect\.poll\(\(\) => page\.getByRole\('alert'\)\.evaluateAll\(applicationAlertTexts\)\)\.toEqual\(\[\]\)/);
});
for (const fault of [undefined, 'missing-receipt', 'wrong-receipt', 'wrong-database', 'unreadable-receipt']) {
  test(`the authenticated inventory receipt handle is disposed even after rejection: ${fault ?? 'success'}`, async () => {
    const module = load(), b = boundary(module, fault);
    if (fault) await assert.rejects(module.observeInventoryAuditClick(b.page, b.target, b.owner, b.observe));
    else await module.observeInventoryAuditClick(b.page, b.target, b.owner, b.observe);
    assert.equal(b.disposed(), 1);
    assert.equal(b.clicks(), 1);
  });
}
