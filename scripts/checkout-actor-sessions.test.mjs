import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import vm from 'node:vm';
import { createServer } from 'node:http';
import { createRequire } from 'node:module';
import ts from 'typescript';
import { withOwnedBrowserContexts } from './playwright/owned-contexts.mjs';

const require = createRequire(import.meta.url);
const { expect, request: playwrightRequest } = require('@playwright/test');
const root = new URL('../', import.meta.url);
const origin = 'http://127.0.0.1:43219';
const tenantId = 'e2e-owned-suite';
const ownerId = 'e2e-owner';
const compile = name => ts.transpileModule(fs.readFileSync(new URL(name, root), 'utf8'), {
  compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
}).outputText;

// Execute the maintained fixture with explicit browser/database boundaries.
// This is fixture lifecycle proof; it does not assert real application login.
function harness({ denySecondLogin = false, identityStatus = 200, identityUser, expired = false, revokedRole = false } = {}) {
  const contexts = [], inserts = [], logins = [], identities = [];
  const state = { identityStatus, identityUser, expired, revokedRole, dirtyStorage: false };
  let secondId;
  let receipts = [];
  const cookie = id => ({ name: 'owned-session', value: id, domain: '127.0.0.1', path: '/', expires: Date.now() / 1000 + 3600, httpOnly: true, secure: false, sameSite: 'Lax' });
  const browser = { newContext: async options => {
    const context = { options, cookiesValue: [], closed: false,
      cookies: async () => structuredClone(context.cookiesValue),
      addCookies: async cookies => { context.cookiesValue = structuredClone(cookies); },
      storageState: async () => ({ cookies: structuredClone(context.cookiesValue), origins: state.dirtyStorage ? [{ origin, localStorage: [{ name: 'old-cart', value: 'old-operation' }] }] : [] }),
      close: async () => { context.closed = true; },
      newPage: async () => ({ context: () => context, request: {
        context,
        get: async path => {
          assert.equal(path, '/api/v1/auth/session-identity');
          const userId = context.cookiesValue[0]?.value;
          identities.push(userId);
          return { status: () => state.identityStatus, headers: () => ({ 'cache-control': 'private, no-store' }),
            json: async () => ({ userId: state.identityUser ?? userId, tenantId, expiresAt: Date.now() + (state.expired ? -1000 : 3600000) }) };
        },
      } }),
    };
    contexts.push(context); return context;
  } };
  const e2eDbQuery = async (sql, params) => {
    if (sql.includes('WITH actor AS')) { secondId = params[0]; return [{ user_id: secondId, tenant_id: tenantId }]; }
    if (sql.includes('normalized_admin')) return [ownerId, secondId].sort().map(id => ({ id, tenant_id: tenantId, active: true, admin_roles: !state.revokedRole, normalized_admin: !state.revokedRole }));
    if (sql.includes('INSERT INTO products')) { inserts.push(params); return [{ id: params[0] }]; }
    if (sql.includes('SELECT order_id FROM terminal_cash_receipts')) return receipts.map(order_id => ({ order_id }));
    if (sql.includes('SELECT inventory_count')) return [{ inventory_count: 1, available_quantity: 0, locked_quantity: 1 }];
    if (sql.includes('FROM orders o JOIN order_items')) return [];
    throw new Error(`Unexpected fixture query: ${sql}`);
  };
  const exports = {};
  const dependencies = {
    'node:crypto': require('node:crypto'), '@playwright/test': { expect },
    '../../../scripts/playwright/owned-contexts.mjs': { withOwnedBrowserContexts },
    // The separately reviewed clock integration adds this import. These setup
    // tests never execute clock or checkout actions; any such call must fail.
    './clock_receipts': {
      assertConfirmedClockIn: () => assert.fail('Clock assertions are outside this authentication setup test'),
      waitForClockPost: () => assert.fail('Clock actions are outside this authentication setup test'),
    },
    '../db_utils': { e2eDbQuery }, '../identities': { E2E_ADMIN_USER: { password: 'synthetic-test-input' } },
    '../growth_owner': { createGrowthOwner: async page => {
      logins.push(ownerId); page.request.context.cookiesValue = [cookie(ownerId)];
      return { tenantId, userId: ownerId, email: 'owner@example.test' };
    } },
    '../authenticate': { authenticateRequest: async request => {
      logins.push(secondId);
      if (denySecondLogin) throw new Error('E2E authentication failed with HTTP401');
      request.context.cookiesValue = [cookie(secondId)];
      return { id: secondId, organization_id: tenantId, roles: ['ADMIN'] };
    } },
  };
  vm.runInNewContext(compile('src/e2e/support/owned_checkout_stock.ts'), {
    exports, Buffer, URL, structuredClone, require: name => { assert.ok(name in dependencies, name); return dependencies[name]; },
  });
  return { browser, exports, contexts, inserts, logins, identities,
    setReceipts: value => { receipts = value; }, setState: value => Object.assign(state, value) };
}

test('three checkout cases reuse two real-login results with new empty contexts and unique stock', async () => {
  const f = harness();
  const pair = await f.exports.authenticateOwnedCheckoutPair(f.browser, origin, { proxy: { server: 'owned-proxy' } });
  const products = [], cases = [];
  for (let index = 0; index < 3; index += 1) {
    await f.exports.withOwnedCheckoutActors(f.browser, origin, { storageState: 'must-be-overridden' }, pair, async actors => {
      products.push(actors.stock.productId); cases.push(actors);
      assert.equal(actors.firstUserId, pair.firstUserId);
      assert.equal(actors.secondUserId, pair.secondUserId);
      assert.notEqual(actors.firstUserId, actors.secondUserId);
      assert.equal(actors.stock.tenantId, tenantId);
    });
  }
  assert.equal(f.logins.length, 2, 'the suite stays below the unchanged five-attempt source window');
  assert.equal(new Set(products).size, 3);
  assert.equal(f.contexts.length, 8, 'setup contexts plus a fresh pair for every case');
  assert.ok(f.contexts.every(context => context.closed));
  assert.ok(f.contexts.every(context => JSON.stringify(context.options.storageState) === '{"cookies":[],"origins":[]}'));
  assert.ok(f.contexts.every(context => context.options.serviceWorkers === 'block'));
  assert.equal(new Set(cases.map(actors => actors.first)).size, 3);
  assert.equal(f.identities.length, 8, 'both identities are checked during setup and each case');
});

for (const [name, options] of [
  ['expired session', { expired: true }], ['revoked session', { identityStatus: 401 }],
  ['unavailable identity', { identityStatus: 503 }], ['foreign identity', { identityUser: 'foreign-user' }],
  ['revoked role', { revokedRole: true }],
]) test(`checkout setup rejects ${name} without retrying login or creating stock`, async () => {
  const f = harness(options);
  await assert.rejects(f.exports.authenticateOwnedCheckoutPair(f.browser, origin, {}));
  assert.equal(f.logins.length, 2);
  assert.equal(f.inserts.length, 0);
  assert.ok(f.contexts.every(context => context.closed));
});

test('a denied second login remains failed and both setup contexts close', async () => {
  const f = harness({ denySecondLogin: true });
  await assert.rejects(f.exports.authenticateOwnedCheckoutPair(f.browser, origin, {}), /HTTP401/);
  assert.equal(f.logins.length, 2); assert.equal(f.inserts.length, 0);
  assert.ok(f.contexts.every(context => context.closed));
});

for (const [name, change] of [
  ['expired saved session', { expired: true }], ['revoked saved session', { identityStatus: 401 }],
  ['foreign saved identity', { identityUser: 'other-user' }], ['revoked saved role', { revokedRole: true }],
  ['residual cart or queue origin storage', { dirtyStorage: true }],
]) test(`each case rejects ${name} without reauthentication or stock insertion`, async () => {
  const f = harness();
  const pair = await f.exports.authenticateOwnedCheckoutPair(f.browser, origin, {});
  f.setState(change);
  await assert.rejects(f.exports.withOwnedCheckoutActors(f.browser, origin, {}, pair, async () => {
    assert.fail('invalid setup reached a checkout action');
  }));
  assert.equal(f.logins.length, 2); assert.equal(f.inserts.length, 0);
  assert.ok(f.contexts.every(context => context.closed));
});

test('captured sessions cannot be transferred to a different sidecar origin', async () => {
  const f = harness();
  const pair = await f.exports.authenticateOwnedCheckoutPair(f.browser, origin, {});
  await assert.rejects(f.exports.withOwnedCheckoutActors(f.browser, 'http://127.0.0.1:43220', {}, pair, async () => {}), /origin/);
  assert.equal(f.logins.length, 2); assert.equal(f.inserts.length, 0);
});

test('each stock case preserves all previous suite receipts and detects any extra receipt', async () => {
  const f = harness();
  const pair = await f.exports.authenticateOwnedCheckoutPair(f.browser, origin, {});
  f.setReceipts(['earlier-cash-order']);
  await f.exports.withOwnedCheckoutActors(f.browser, origin, {}, pair, async actors => {
    await f.exports.assertPersistedStockOutcome(actors.stock, null);
    f.setReceipts(['earlier-cash-order', 'unexpected-other-order']);
    await assert.rejects(f.exports.assertPersistedStockOutcome(actors.stock, null));
    f.setReceipts([]);
    await assert.rejects(f.exports.assertPersistedStockOutcome(actors.stock, null));
  });
});

for (const status of [401, 429]) test(`the actual authentication helper preserves HTTP${status} and sends no retry`, async t => {
  let calls = 0;
  const app = createServer((_req, res) => {
    calls += 1;
    res.writeHead(status, { 'content-type': 'application/json', ...(status === 429 ? { 'retry-after': '300' } : {}) });
    res.end(JSON.stringify({ error: status === 429 ? 'too many requests' : 'invalid credentials' }));
  });
  await new Promise(resolve => app.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise(resolve => { app.closeAllConnections(); app.close(resolve); }));
  const baseURL = `http://127.0.0.1:${app.address().port}`;
  const context = await playwrightRequest.newContext({ baseURL });
  t.after(() => context.dispose());
  const exports = {};
  vm.runInNewContext(compile('src/e2e/authenticate.ts'), { exports, Buffer });
  await assert.rejects(exports.authenticateRequest(context, { username: 'owned-probe', password: 'invalid-test-input', organizationId: 'e2e-probe' }, baseURL), new RegExp(`HTTP ${status}`));
  assert.equal(calls, 1);
});

test('the maintained suite owns one authentication pair across its three grouped cases', async () => {
  const hooks = [], cases = [], modes = [], observed = [];
  let inOwnedSuite = false, starts = 0, authentications = 0, closes = 0;
  const pair = { opaque: 'suite-owned-pair' };
  const register = (name, callback) => { if (inOwnedSuite) cases.push({ name, callback }); };
  register.describe = (name, body) => { inOwnedSuite = name.startsWith('Owned cash'); body(); inOwnedSuite = false; };
  register.describe.configure = options => { if (inOwnedSuite) modes.push(options.mode); };
  register.beforeAll = callback => { if (inOwnedSuite) hooks.push(['before', callback]); };
  register.afterAll = callback => { if (inOwnedSuite) hooks.push(['after', callback]); };
  register.afterEach = () => {};
  const dependencies = {
    './fixtures': { test: register, expect },
    './support/configured_checkout_fixture': { startConfiguredCheckoutFixture: async () => {
      starts += 1; return { origin, proxy: { server: 'owned-proxy' }, close: async () => { closes += 1; } };
    } },
    './support/owned_checkout_stock': {
      authenticateOwnedCheckoutPair: async (_browser, actualOrigin) => { authentications += 1; assert.equal(actualOrigin, origin); return pair; },
      withOwnedCheckoutActors: async (_browser, actualOrigin, _options, actualPair) => { observed.push(actualPair); assert.equal(actualOrigin, origin); },
    },
  };
  vm.runInNewContext(compile('src/e2e/inventory_sync.spec.ts'), { exports: {},
    require: name => { assert.ok(name in dependencies, name); return dependencies[name]; } });
  await hooks.find(([kind]) => kind === 'before')[1]({ browser: {}, contextOptions: {} });
  assert.equal(cases.length, 3);
  for (const { callback } of cases) await callback({ browser: {}, contextOptions: {} });
  await hooks.find(([kind]) => kind === 'after')[1]();
  assert.deepEqual(modes, ['default'], 'the group overrides global fullyParallel without skipping later cases');
  assert.equal(starts, 1); assert.equal(authentications, 1); assert.equal(closes, 1);
  assert.equal(observed.length, 3); assert.ok(observed.every(value => value === pair));
});
