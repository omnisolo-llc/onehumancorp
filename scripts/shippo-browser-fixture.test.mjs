import test from 'node:test';
import assert from 'node:assert/strict';
import { startShippoBrowserFixture, verifiedShippoBrowserEnvironment } from './shippo-browser-fixture.mjs';
import { finishShippoBrowserFixture, testEnvironment } from './native-e2e.mjs';

async function withFixture(t) {
  const fixture = await startShippoBrowserFixture({ runId: '012345abcdef', tenantId: 'e2e-tenant' });
  t.after(() => fixture.close());
  const env = fixture.environment;
  const send = (path, body, headers = {}) => fetch(`${env.SHIPPO_API_BASE}${path}`, {
    method: body === undefined ? 'GET' : 'POST', redirect: 'error',
    headers: { authorization: `ShippoToken ${env.SHIPPO_API_TOKEN}`, 'content-type': 'application/json', 'SHIPPO-API-VERSION': '2018-02-08', ...headers },
    ...(body === undefined ? {} : { body: JSON.stringify(body) }),
  });
  return { fixture, env, send };
}
const parcel = { length: '12', width: '10', height: '8', distance_unit: 'in', weight: '20', mass_unit: 'oz' };

async function shipment(f) {
  const body = { address_from: JSON.parse(f.env.SHIPPO_ADDRESS_FROM_JSON), address_to: JSON.parse(f.env.SHIPPO_ADDRESS_TO_JSON), parcels: [parcel], async: false };
  const response = await f.send('/shipments', body);
  assert.equal(response.status, 200);
  return { request: body, response: await response.json() };
}

test('fixture binds a random loopback port, verifies its exact scope and closes its listener', async t => {
  const f = await withFixture(t);
  assert.match(f.env.SHIPPO_API_BASE, /^http:\/\/127\.0\.0\.1:[1-9]\d*$/);
  assert.equal(verifiedShippoBrowserEnvironment(f.env).baseURL, f.env.SHIPPO_API_BASE);
  const response = await f.send('/__fixture/health');
  assert.equal(response.status, 200);
  assert.deepEqual(await response.json(), { runId: '012345abcdef', tenantId: 'e2e-tenant', accountNamespace: f.env.SHIPPO_ACCOUNT_NAMESPACE });
  await f.fixture.close();
  await assert.rejects(f.send('/__fixture/health'));
});

test('issued rates and exact PDF transactions correlate with received parcels and metadata', async t => {
  const f = await withFixture(t);
  const first = await shipment(f), second = await shipment(f);
  // serde_json emits object keys in sorted order; JSON member order is immaterial.
  const reordered = Object.fromEntries(Object.entries(first.request).map(([key, value]) => [key, key.startsWith('address_') ? Object.fromEntries(Object.entries(value).sort(([a], [b]) => a.localeCompare(b))) : value]));
  assert.equal((await f.send('/shipments', reordered)).status, 200);
  const rate = first.response.rates[0].object_id;
  assert.notEqual(rate, second.response.rates[0].object_id);
  const request = { rate, metadata: 'ohc_shipping_01234567-89ab-cdef-0123-456789abcdef', async: false, label_file_type: 'PDF' };
  const response = await f.send('/transactions', request);
  assert.equal(response.status, 200);
  const transaction = await response.json();
  assert.equal(transaction.rate, rate);
  assert.equal(transaction.metadata, request.metadata);
  assert.equal(transaction.status, 'SUCCESS');
  assert.equal(transaction.test, true);
  assert.equal(transaction.tracking_carrier, 'usps');
  assert.match(transaction.label_url, /^https:\/\/app\.goshippo\.com\/labels\/transaction_[a-f0-9]+\.pdf$/);
  const evidence = await (await f.send(`/__fixture/receipts/${transaction.object_id}`)).json();
  assert.deepEqual(evidence, { shipmentRequest: first.request, purchaseRequest: request, response: transaction, purchaseCount: 1 });
  assert.equal((await f.send('/transactions', request)).status, 409, 'duplicate provider purchase must fail, never look idempotently successful');
  assert.equal((await (await f.send(`/__fixture/receipts/${transaction.object_id}`)).json()).purchaseCount, 2);
});

test('fixture rejects foreign rates, malformed parcels, unknown endpoints and unverified credentials', async t => {
  const f = await withFixture(t);
  assert.equal((await f.send('/transactions', { rate: 'rate_foreign', metadata: 'ohc_shipping_01234567-89ab-cdef-0123-456789abcdef', async: false, label_file_type: 'PDF' })).status, 400);
  assert.equal((await f.send('/shipments', { parcels: [{ ...parcel, length: '0' }], async: false })).status, 400);
  assert.equal((await f.send('/tracks/usps/anything')).status, 404, 'the fixture supplies no carrier evidence');
  assert.equal((await f.send('/shipments', {}, { authorization: 'ShippoToken live_wrong' })).status, 401);
  assert.equal((await f.send('/shipments', {}, { host: 'api.goshippo.com' })).status, 400);
});

test('no inherited Shippo credentials or endpoints can enter the native runner', () => {
  const inherited = Object.fromEntries(['SHIPPO_API_BASE', 'SHIPPO_API_TOKEN', 'SHIPPO_TENANT_ID', 'SHIPPO_ACCOUNT_NAMESPACE', 'SHIPPO_WEBHOOK_MODE', 'SHIPPO_ADDRESS_FROM_JSON', 'SHIPPO_ADDRESS_TO_JSON', 'OMNISOLO_E2E_SHIPPO_RUN_ID'].map(key => [key, 'untrusted']));
  assert.deepEqual(testEnvironment(inherited), {});
});

test('browser fixture refuses non-loopback or changed provider scope before any request', async t => {
  const f = await withFixture(t);
  for (const change of [
    { SHIPPO_API_BASE: 'https://api.goshippo.com' },
    { SHIPPO_API_BASE: 'http://localhost:1234' },
    { SHIPPO_API_BASE: `${f.env.SHIPPO_API_BASE}/redirect` },
    { SHIPPO_API_TOKEN: 'shippo_live_real' },
    { SHIPPO_WEBHOOK_MODE: 'live' },
    { SHIPPO_TENANT_ID: 'another-tenant' },
    { SHIPPO_ACCOUNT_NAMESPACE: 'another-account' },
    { OMNISOLO_E2E_SHIPPO_RUN_ID: '' },
  ]) assert.throws(() => verifiedShippoBrowserEnvironment({ ...f.env, ...change }), /runner-owned loopback Shippo/);
});


test('runner still cleans its containers and temporary files after fixture shutdown fails', async () => {
  const events = [], failure = new Error('fixture close failed');
  const fixture = { close: async () => { events.push('fixture'); throw failure; } };
  const cleanup = async () => { events.push('containers'); events.push('temporary files'); };
  await assert.rejects(finishShippoBrowserFixture(fixture, cleanup, false), error => error === failure);
  assert.deepEqual(events, ['fixture', 'containers', 'temporary files']);
  events.length = 0;
  await finishShippoBrowserFixture(fixture, cleanup, true);
  assert.deepEqual(events, ['fixture', 'containers', 'temporary files'], 'preserve the original test failure after all owned cleanup');
});
