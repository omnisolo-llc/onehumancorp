import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import vm from 'node:vm';
import test from 'node:test';
import { createServer } from 'node:http';
import ts from 'typescript';

const require = createRequire(import.meta.url);
const { createOwnedAuditSeed } = require('./ui-audit-fixture.cjs');
const canonical = readFileSync(new URL('../src/e2e/e2e-seed.sql', import.meta.url), 'utf8');
const origin = 'http://127.0.0.1:43219';
const plain = value => JSON.parse(JSON.stringify(value));
function load(name, dependencies) {
  const source = readFileSync(new URL(`../src/e2e/${name}.ts`, import.meta.url), 'utf8');
  const exports = {};
  vm.runInNewContext(ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText, { exports, URL, require: dependency => {
    assert.ok(dependency in dependencies, `Unexpected dependency: ${dependency}`);
    return dependencies[dependency];
  } });
  return exports;
}

// Execute the maintained journey and fixture with controlled database/request
// boundaries. A pre-insert read completes after invalidation and fills a stale
// tenant cache, matching HybridCache's admitted ordering. This proves fixture
// ordering and ownership; it is not execution of the Rust cache or backend E2E.
function boundary({ activationStatus = 200, losePending = false, loseApproval = false, foreignIdentity = false } = {}) {
  const rows = new Map([['e2e-daily-prep', { tenant: 'e2e-tenant', state: 'PENDING_APPROVAL' }]]);
  const actors = [], writes = [], reads = [], firstSnapshots = [], assertions = [];
  const caches = new Map(), inFlight = new Map();
  let body, serial = 0;
  const pwTest = (_title, callback) => { body = callback; };
  pwTest.describe = (_title, callback) => callback();
  const expect = (actual, message) => ({
    toBe: expected => assert.equal(actual, expected, message),
    toEqual: expected => assert.deepEqual(plain(actual), plain(expected), message),
    toMatchObject: expected => { for (const [key, value] of Object.entries(expected)) assert.deepEqual(actual[key], value); },
    toBeVisible: async () => { assert.ok(actual.visible(), actual.label); assertions.push(actual.label); },
    not: { toBeVisible: async () => assert.equal(actual.visible(), false, actual.label) },
  });
  expect.poll = (predicate, options) => ({ toBe: async expected => {
    assert.equal(options.timeout, 10000, 'keep the original readiness deadline');
    assert.equal(await predicate(), expected, options.message);
  } });
  const db = { query: async (sql, parameters) => {
    if (sql.startsWith('INSERT INTO agent_feed_items')) {
      const [id, tenant, eventSource, context, action] = parameters;
      assert.equal(eventSource, 'operations');
      assert.equal(JSON.parse(context).feature_type, 'daily_prep_checklist');
      assert.equal(JSON.parse(action).action_type, 'Daily Prep Checklist');
      rows.set(id, { tenant, state: 'PENDING_APPROVAL' });
      return [];
    }
    assert.match(sql, /SELECT lifecycle_state FROM agent_feed_items WHERE id = \$1 AND tenant_id = \$2/);
    const [id, tenant] = parameters, row = rows.get(id);
    reads.push({ id, tenant, state: row?.state });
    return row?.tenant === tenant && !(losePending && row.state === 'PENDING_APPROVAL')
      ? [{ lifecycle_state: row.state }] : [];
  } };
  const seed = load('feed-fixtures', {
    'node:crypto': { randomUUID: () => `fixture-${++serial}` }, '@playwright/test': { expect }, './db_utils': { db },
    './identities': { E2E_ADMIN_USER: { organizationId: 'e2e-tenant' } },
  });
  load('operations_feed_automation.spec', {
    './fixtures': { test: pwTest, expect }, './feed-fixtures': seed, './db_utils': { db },
    './support/dashboard_audit_fixture': { seedDashboardAuditOwner: async () => {
      const digit = String(actors.length + 1);
      const actor = createOwnedAuditSeed(canonical, `audit-${digit.repeat(8)}-${digit.repeat(4)}-4${digit.repeat(3)}-8${digit.repeat(3)}-${digit.repeat(12)}`);
      assert.ok(actor.sql.includes(`'${actor.namespace}-e2e-daily-prep'`));
      actors.push(actor);
      rows.set(`${actor.namespace}-e2e-daily-prep`, { tenant: actor.tenantId, state: 'PENDING_APPROVAL' });
      return actor;
    } },
    './authenticate': { authenticateRequest: async (request, credentials, requestedOrigin) => {
      assert.equal(requestedOrigin, origin);
      const actor = actors.find(actor => actor.tenantId === credentials.organizationId);
      assert.equal(credentials.username, actor.email); assert.equal(credentials.password, actor.password);
      request.authenticate(actor.tenantId);
    } },
  });
  async function run() {
    let tenant = 'e2e-tenant', mounted = false, expanded = false, responseWaiter;
    let shown = [];
    const pending = () => [...rows].filter(([, row]) => row.tenant === tenant && row.state === 'PENDING_APPROVAL').map(([id]) => id);
    const activate = id => {
      assert.equal(rows.get(id)?.tenant, tenant, 'fixture PUT must be authorized for its exact tenant');
      writes.push({ id, tenant, state: 'PENDING_APPROVAL', mounted });
      caches.delete(tenant);
      if (inFlight.has(tenant)) {
        caches.set(tenant, inFlight.get(tenant)); inFlight.delete(tenant);
      }
      return { status: activationStatus, body: '{}' };
    };
    const visibleRows = () => shown.filter(id => rows.get(id)?.state === 'PENDING_APPROVAL');
    const locator = (kind, label, id) => ({
      label, first() { return this; },
      visible: () => kind === 'heading' ? mounted : kind === 'group' ? visibleRows().length > 1
        : visibleRows().includes(id) && (expanded || visibleRows().length === 1),
      isVisible: async function () { return this.visible(); },
      getByRole: (_role, { name }) => locator(name === 'Review Individually' ? 'expand' : 'complete', name, id),
      getByTestId: testId => locator('button', testId, id),
      click: async () => {
        if (kind === 'expand') { expanded = true; return; }
        assert.equal(kind, 'complete'); assert.equal(label, 'Mark Complete');
        assert.ok(visibleRows().includes(id));
        writes.push({ id, tenant, state: 'APPROVED', mounted });
        if (!loseApproval) rows.get(id).state = 'APPROVED';
        shown = shown.filter(value => value !== id);
        const response = { url: () => `${origin}/api/v1/agent-feed/${id}`, request: () => ({ method: () => 'PUT' }), status: () => 200 };
        assert.ok(responseWaiter.predicate(response)); responseWaiter.resolve(response);
      },
    });
    const navigate = async () => {
      mounted = true; expanded = false;
      if (!firstSnapshots.some(snapshot => snapshot.tenant === tenant)) {
        const snapshot = pending(); firstSnapshots.push({ tenant, ids: snapshot });
        // Deliberately leave a read started before fixture insertion in flight.
        if (!snapshot.some(id => id.startsWith('e2e-feed-'))) inFlight.set(tenant, snapshot);
        else caches.set(tenant, snapshot);
      }
      shown = caches.get(tenant) ?? pending();
    };
    const page = {
      goto: navigate, reload: navigate,
      locator: () => locator('heading', 'Dashboard'),
      getByTestId: id => id.startsWith('grouped-triage-card-') ? locator('group', id)
        : locator('card', id, id.slice('triage-card-'.length)),
      evaluate: async (_callback, id) => activate(id),
      request: {
        authenticate: value => { tenant = value; },
        get: async path => {
          assert.equal(path, '/api/v1/auth/session-identity');
          return { status: () => 200, json: async () => ({ userId: actors.at(-1).userId, tenantId: foreignIdentity ? 'foreign' : tenant }), dispose: async () => {} };
        },
        put: async (url, options) => {
          assert.equal(new URL(url, origin).origin, origin);
          assert.equal(options.headers.origin, origin); assert.equal(options.headers['sec-fetch-site'], 'same-origin');
          assert.deepEqual(plain(options.data), { state: 'PENDING_APPROVAL' });
          const result = activate(new URL(url, origin).pathname.split('/').at(-1));
          return { status: () => result.status, text: async () => result.body, dispose: async () => {} };
        },
      },
      waitForResponse: predicate => new Promise(resolve => { responseWaiter = { predicate, resolve }; }),
    };
    await body({ page, anonymousPage: page, baseURL: origin });
  }
  return { run, rows, actors, writes, reads, firstSnapshots, assertions };
}

test('daily prep is admitted before any dashboard read, including a controlled stale-fill ordering', async () => {
  const f = boundary(); await f.run();
  assert.equal(f.writes[0].mounted, false, 'fixture creation must precede the first dashboard read');
  assert.ok(f.firstSnapshots[0].ids.includes(f.writes[0].id));
  assert.equal(f.writes[1].id, f.writes[0].id);
  assert.equal(f.rows.get(f.writes[1].id).state, 'APPROVED');
  for (const label of ['Dashboard', 'Mark Complete', 'feed-assign-btn', 'feed-dismiss-btn']) assert.ok(f.assertions.includes(label));
  assert.deepEqual(f.reads.map(read => read.state), ['PENDING_APPROVAL', 'APPROVED']);
});

test('repeated daily-prep journeys own separate canonical groups and never consume the shared checklist', async () => {
  const f = boundary(); await f.run(); await f.run();
  assert.equal(f.actors.length, 2); assert.notEqual(f.actors[0].tenantId, f.actors[1].tenantId);
  assert.equal(f.rows.get('e2e-daily-prep').state, 'PENDING_APPROVAL');
  for (const actor of f.actors) {
    assert.equal(f.rows.get(`${actor.namespace}-e2e-daily-prep`).state, 'PENDING_APPROVAL');
    assert.equal(f.writes.filter(write => write.tenant === actor.tenantId && write.state === 'APPROVED').length, 1);
  }
});

test('failed fixture activation stops before a dashboard is mounted', async () => {
  const f = boundary({ activationStatus: 503 }); await assert.rejects(f.run(), /activate feed fixture/);
  assert.equal(f.firstSnapshots.length, 0);
});

test('a different signed tenant stops before fixture insertion or dashboard reads', async () => {
  const f = boundary({ foreignIdentity: true }); await assert.rejects(f.run());
  assert.equal(f.firstSnapshots.length, 0); assert.equal(f.writes.length, 0);
  assert.equal([...f.rows.keys()].filter(id => id.startsWith('e2e-feed-')).length, 0);
});

test('missing durable pending fixture cannot proceed to the dashboard', async () => {
  const f = boundary({ losePending: true }); await assert.rejects(f.run());
  assert.equal(f.firstSnapshots.length, 0);
});

test('an HTTP 200 and optimistic disappearance cannot replace durable approval evidence', async () => {
  const f = boundary({ loseApproval: true }); await assert.rejects(f.run());
  assert.equal(f.writes.at(-1).state, 'APPROVED');
  assert.equal(f.reads.at(-1).state, 'PENDING_APPROVAL');
});

test('pre-navigation activation sends the request context cookie with same-origin CSRF headers', async t => {
  const received = [];
  const server = createServer(async (request, response) => {
    let body = ''; for await (const chunk of request) body += chunk;
    received.push({ path: request.url, method: request.method, headers: request.headers, body: JSON.parse(body) });
    response.setHeader('content-type', 'application/json'); response.end('{}');
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise(resolve => server.close(resolve)));
  const baseURL = `http://127.0.0.1:${server.address().port}`;
  const { request, expect } = require('@playwright/test');
  const context = await request.newContext({ baseURL, storageState: { cookies: [{
    name: 'fixture-session', value: 'signed-owner-placeholder', domain: '127.0.0.1', path: '/',
    expires: -1, httpOnly: true, secure: false, sameSite: 'Lax',
  }], origins: [] } });
  t.after(() => context.dispose());
  const seed = load('feed-fixtures', { 'node:crypto': require('node:crypto'), '@playwright/test': { expect },
    './db_utils': { db: { query: async () => [] } }, './identities': { E2E_ADMIN_USER: { organizationId: 'unused' } },
  });
  const id = await seed.seedFeedItem({ request: context, evaluate: () => assert.fail('No document may be mounted for this request mode') },
    { event_source: 'operations', context_payload: {}, proposed_action: {} }, 'owned-tenant', { requestOrigin: baseURL });
  assert.equal(received.length, 1); assert.equal(received[0].path, `/api/v1/agent-feed/${id}`);
  assert.equal(received[0].method, 'PUT'); assert.equal(received[0].headers.cookie, 'fixture-session=signed-owner-placeholder');
  assert.equal(received[0].headers.origin, baseURL); assert.equal(received[0].headers['sec-fetch-site'], 'same-origin');
  assert.deepEqual(received[0].body, { state: 'PENDING_APPROVAL' });
});
