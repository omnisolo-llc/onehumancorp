import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer, request } from 'node:http';
import { gzipSync } from 'node:zlib';
import { startCheckoutEgressProxy } from './checkout-browser-fixture.mjs';

const clockPath = '/api/v1/staff/timecard';
const owner = { tenantId: 'e2e-clock-owned', actorId: 'e2e-clock-owner' };
const event = { id: 'clock-owned-1', staff_id: owner.actorId, event_type: 'CLOCK_IN', offline_timestamp: '2026-10-04T08:00:00.123Z' };
const body = JSON.stringify({ events: [event] });
const acknowledgment = { success: true, outcomes: [{ id: event.id, route: clockPath, status: 'acknowledged' }] };

// These are actual HTTP socket tests of the test-only fault boundary. They do
// not certify timecard persistence; the real-stack browser spec checks PostgreSQL.
async function transport(t, respond) {
  const received = [], observedReplies = [];
  const app = createServer(async (req, res) => {
    const chunks = [];
    try { for await (const chunk of req) chunks.push(chunk); } catch { return; }
    received.push({ method: req.method, path: req.url, body: Buffer.concat(chunks).toString(),
      actor: req.headers['x-ohc-expected-user'], tenant: req.headers['x-ohc-expected-tenant'],
      idempotencyKey: req.headers['idempotency-key'] });
    respond(req, res);
  });
  await new Promise(resolve => app.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise(resolve => { app.closeAllConnections(); app.close(resolve); }));
  const origin = `http://127.0.0.1:${app.address().port}`;
  const proxy = await startCheckoutEgressProxy({ appOrigin: origin });
  t.after(() => proxy.close());
  const send = ({ route = clockPath, method = 'POST', data = body, headers = {}, timeout = 2000 } = {}) => new Promise((resolve, reject) => {
    const req = request(proxy.server, { path: `${origin}${route}`, method, agent: false,
      headers: { 'content-type': 'application/json', 'idempotency-key': event.id,
        'x-ohc-expected-user': owner.actorId, 'x-ohc-expected-tenant': owner.tenantId,
        cookie: 'private_auth=never-record-this', ...headers } }, res => {
      const chunks = [];
      const observed = { status: res.statusCode, headers: res.headers, chunks }; observedReplies.push(observed);
      res.on('data', chunk => chunks.push(chunk)); res.on('error', reject);
      res.on('end', () => resolve({ status: res.statusCode, body: Buffer.concat(chunks), headers: res.headers }));
    });
    req.setTimeout(timeout, () => req.destroy(new Error('Owned clock transport timed out')));
    req.on('error', reject); req.end(data);
  });
  return { proxy, send, received, observedReplies };
}

function arm(proxy) { proxy.armClockResponseLoss({ ...owner, event }); }

test('the clock hook loses exactly one complete successful upstream reply after forwarding the original request once', async t => {
  let finish;
  const reached = new Promise(resolve => { finish = resolve; });
  let release;
  const completed = new Promise(resolve => { release = resolve; });
  let requests = 0;
  const bytes = Buffer.from(JSON.stringify(acknowledgment));
  const { proxy, send, received, observedReplies } = await transport(t, (_req, res) => {
    requests += 1;
    if (requests > 1) { res.end(bytes); return; }
    res.writeHead(200, { 'content-type': 'application/json', 'content-length': bytes.length });
    res.write(bytes.subarray(0, 12)); finish();
    void completed.then(() => res.end(bytes.subarray(12)));
  });
  arm(proxy);
  const sending = assert.rejects(send(), /socket hang up|reset|aborted/i);
  await reached;
  assert.equal(proxy.evidence().clockResponseLoss.dropped, false, 'a partial success body cannot activate response loss');
  assert.deepEqual(observedReplies, [], 'do not cut or certify a reply before upstream completion');
  release(); await sending;
  assert.equal(observedReplies.length, 1);
  assert.equal(observedReplies[0].status, 200, 'the client sees the genuine upstream status before the body is lost');
  assert.deepEqual(Buffer.concat(observedReplies[0].chunks), bytes.subarray(0, 1), 'only a literal incomplete prefix reaches the client');
  assert.equal(received.length, 1);
  assert.deepEqual(received[0], { method: 'POST', path: clockPath, body,
    actor: owner.actorId, tenant: owner.tenantId, idempotencyKey: event.id });
  assert.deepEqual(proxy.evidence().clockResponseLoss, {
    tenantId: owner.tenantId, actorId: owner.actorId, eventId: event.id,
    requestBody: body, status: 200, dropped: true, complete: true, error: null,
  });
  assert.deepEqual((await send()).body, bytes, 'a second actual request is forwarded normally, never lost by the one-shot hook');
  assert.equal(received.length, 2, 'the proxy itself must never replay a request');
  assert.throws(() => arm(proxy), /one-shot/);
  assert.doesNotMatch(JSON.stringify(proxy.evidence()), /never-record-this|private_auth/);
});

test('clock response loss cannot alter another method, route, owner, id, body or provider response', async t => {
  const bytes = Buffer.from(JSON.stringify(acknowledgment));
  const { proxy, send, received } = await transport(t, (_req, res) => res.end(bytes));
  arm(proxy);
  const others = [
    { method: 'GET', data: '' }, { route: `${clockPath}?extra=1` }, { route: `${clockPath}/` },
    { route: '/api/v1/staff/other/../timecard' }, { route: '/api/v1/staff/%74imecard' },
    { route: '/api/v1/sync/offline' }, { route: '/api/v1/sync/operation-intents' },
    { route: '/api/v1/billing/create-checkout-session' }, { route: `${clockPath}/receipts/${event.id}`, method: 'GET', data: '' },
    { headers: { 'x-ohc-expected-user': 'e2e-another-owner' } },
    { headers: { 'x-ohc-expected-tenant': 'e2e-another-tenant' } },
    { headers: { 'idempotency-key': 'another-event' } },
    { data: JSON.stringify({ events: [{ ...event, id: 'another-event' }] }) },
    { data: JSON.stringify({ events: [{ ...event, event_type: 'CLOCK_OUT' }] }) },
    { data: JSON.stringify({ events: [event, event] }) }, { data: `${body}${' '.repeat(4097)}` },
  ];
  for (const other of others) {
    const reply = await send(other);
    assert.equal(reply.status, 200); assert.deepEqual(reply.body, bytes);
    assert.equal(proxy.evidence().clockResponseLoss.dropped, false);
    assert.equal(proxy.evidence().clockResponseLoss.requestBody, null);
  }
  assert.equal(received.length, others.length);
  await assert.rejects(send(), /socket hang up|reset|aborted/i);
  assert.equal(received.length, others.length + 1);
  assert.equal(proxy.evidence().clockResponseLoss.dropped, true);
});

for (const [name, status, reply] of [
  ['rejection', 409, acknowledgment], ['server failure', 500, acknowledgment],
  ['success-only', 200, { success: true }],
  ['wrong ID', 200, { success: true, outcomes: [{ ...acknowledgment.outcomes[0], id: 'other' }] }],
  ['wrong route', 200, { success: true, outcomes: [{ ...acknowledgment.outcomes[0], route: '/api/v1/sync/offline' }] }],
  ['unacknowledged', 200, { success: true, outcomes: [{ ...acknowledgment.outcomes[0], status: 'reconciliation' }] }],
  ['contradictory', 200, { ...acknowledgment, success: false }],
  ['duplicate outcomes', 200, { success: true, outcomes: [...acknowledgment.outcomes, ...acknowledgment.outcomes] }],
]) {
  test(`a real ${name} reply is forwarded unchanged and cannot trigger clock response loss`, async t => {
    const bytes = Buffer.from(JSON.stringify(reply));
    const { proxy, send, received } = await transport(t, (_req, res) => { res.writeHead(status); res.end(bytes); });
    arm(proxy);
    const actual = await send();
    assert.equal(actual.status, status); assert.deepEqual(actual.body, bytes);
    assert.equal(proxy.evidence().clockResponseLoss.dropped, false);
    assert.equal(received.length, 1);
  });
}

for (const [name, encoding, bytes] of [
  ['gzip acknowledgment', 'gzip', gzipSync(JSON.stringify(acknowledgment))],
  ['malformed compression', 'gzip', Buffer.from('not gzip')],
  ['over-limit body', undefined, Buffer.alloc(65537, 'x')],
  ['over-limit decompression', 'gzip', gzipSync(Buffer.alloc(65537, 'x'))],
]) {
  test(`clock loss bounds ${name} without changing nonmatching response bytes`, async t => {
    const { proxy, send, received } = await transport(t, (_req, res) => {
      res.writeHead(200, { 'content-type': 'application/json', ...(encoding ? { 'content-encoding': encoding } : {}) }); res.end(bytes);
    });
    arm(proxy);
    if (name === 'gzip acknowledgment') {
      await assert.rejects(send(), /socket hang up|reset|aborted/i);
      assert.equal(proxy.evidence().clockResponseLoss.dropped, true);
    } else {
      const actual = await send();
      assert.equal(actual.status, 200); assert.deepEqual(actual.body, bytes);
      assert.equal(proxy.evidence().clockResponseLoss.dropped, false);
    }
    assert.equal(received.length, 1);
  });
}

test('a truncated successful upstream reply cannot certify or trigger the clock hook', async t => {
  const prefix = Buffer.from(JSON.stringify(acknowledgment));
  const { proxy, send, observedReplies } = await transport(t, (_req, res) => {
    res.writeHead(200, { 'content-length': '1000', 'x-owned-prefix': 'genuine-upstream' }); res.write(prefix);
    setImmediate(() => res.destroy());
  });
  arm(proxy);
  await assert.rejects(send(), /socket hang up|reset|aborted/i);
  assert.equal(observedReplies.length, 1, 'a nonqualifying reply keeps its genuine upstream response');
  assert.equal(observedReplies[0].status, 200);
  assert.equal(observedReplies[0].headers['content-length'], '1000');
  assert.equal(observedReplies[0].headers['x-owned-prefix'], 'genuine-upstream');
  assert.deepEqual(Buffer.concat(observedReplies[0].chunks), prefix, 'preserve all bytes observed before upstream failure');
  assert.equal(proxy.evidence().clockResponseLoss.dropped, false);
  assert.equal(proxy.evidence().clockResponseLoss.complete, false);
});

test('clock response-loss registration is bounded, immutable, and limited to an owned self-clock event', async t => {
  const { proxy } = await transport(t, (_req, res) => res.end());
  for (const invalid of [
    { ...owner, tenantId: 'production' }, { ...owner, actorId: 'production' },
    { ...owner, event: { ...event, id: '../unsafe' } },
    { ...owner, event: { ...event, id: 12 } },
    { ...owner, event: { ...event, staff_id: 'e2e-another-owner' } },
    { ...owner, event: { ...event, event_type: 'WORK' } },
    { ...owner, event: { ...event, offline_timestamp: 'invalid' } },
  ]) assert.throws(() => proxy.armClockResponseLoss({ event, ...invalid }), /owned clock/);
  const registration = { ...owner, event: { ...event } };
  proxy.armClockResponseLoss(registration);
  registration.event.id = 'changed';
  assert.equal(proxy.evidence().clockResponseLoss.eventId, event.id);
  assert.throws(() => arm(proxy), /one-shot/);
});


test('an armed clock fault preserves registered checkout observation and exact response bytes', async t => {
  const bytes = Buffer.from(JSON.stringify({ checkout_url: 'https://checkout.stripe.com/c/pay/cs_test_owned_wire' }));
  const { proxy, send } = await transport(t, (_req, res) => {
    res.writeHead(200, { 'content-type': 'application/json', 'x-owned-evidence': 'preserved' }); res.end(bytes);
  });
  const product = { tenantId: owner.tenantId, productId: 'e2e-clock-stock' };
  proxy.registerCheckout(product); arm(proxy);
  const requestBody = JSON.stringify({ is_subscription: false, product_id: product.productId, quantity: 1 });
  const response = await send({ route: '/api/v1/billing/create-checkout-session', data: requestBody });
  assert.deepEqual(response.body, bytes); assert.equal(response.headers['x-owned-evidence'], 'preserved');
  assert.deepEqual(proxy.evidence().checkoutResponses, [{ method: 'POST', path: '/api/v1/billing/create-checkout-session',
    requestBody, status: 200, bodyBase64: bytes.toString('base64'), complete: true, error: null }]);
  assert.equal(proxy.evidence().clockResponseLoss.requestBody, null);
  assert.equal(proxy.evidence().clockResponseLoss.dropped, false);
});


test('a stalled genuine200 prefix survives the existing upstream timeout without a synthetic502', async t => {
  const prefix = Buffer.from('observed genuine partial upstream body');
  const { proxy, send, observedReplies } = await transport(t, (_req, res) => {
    res.writeHead(200, { 'content-length': '1000', 'x-owned-prefix': 'genuine-before-timeout' }); res.write(prefix);
  });
  arm(proxy);
  // Longer client timeout observes the existing production-fixture five-second
  // upstream timeout; no proxy timeout or application behavior is weakened.
  await assert.rejects(send({ timeout: 8000 }), /socket hang up|reset|aborted/i);
  assert.equal(observedReplies.length, 1);
  assert.equal(observedReplies[0].status, 200);
  assert.equal(observedReplies[0].headers['content-length'], '1000');
  assert.equal(observedReplies[0].headers['x-owned-prefix'], 'genuine-before-timeout');
  assert.deepEqual(Buffer.concat(observedReplies[0].chunks), prefix);
  assert.equal(proxy.evidence().clockResponseLoss.dropped, false);
  assert.equal(proxy.evidence().clockResponseLoss.complete, false);
});
