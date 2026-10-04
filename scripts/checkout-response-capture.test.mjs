import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer, request } from 'node:http';
import { gzipSync } from 'node:zlib';
import { startCheckoutEgressProxy } from './checkout-browser-fixture.mjs';

const checkoutPath = '/api/v1/billing/create-checkout-session';
const product = { tenantId: 'e2e-wire-owner', productId: 'e2e-wire-product', title: 'Owned wire product', amountCents: 1999 };
const submitted = JSON.stringify({ is_subscription: false, product_id: product.productId, quantity: 1 });

async function transport(t, respond) {
  const received = [];
  const app = createServer(async (req, res) => {
    const chunks = [];
    try { for await (const chunk of req) chunks.push(chunk); } catch { return; }
    received.push({ method: req.method, path: req.url, body: Buffer.concat(chunks).toString(), cookie: req.headers.cookie });
    respond(req, res);
  });
  await new Promise(resolve => app.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise(resolve => { app.closeAllConnections(); app.close(resolve); }));
  const origin = `http://127.0.0.1:${app.address().port}`;
  const proxy = await startCheckoutEgressProxy({ appOrigin: origin });
  t.after(() => proxy.close());
  const send = ({ body = submitted, route = checkoutPath, method = 'POST' } = {}) => new Promise((resolve, reject) => {
    const req = request(proxy.server, { path: `${origin}${route}`, method, agent: false,
      headers: { 'content-type': 'application/json', cookie: 'private_auth=must-not-be-recorded' } }, res => {
      const chunks = [];
      res.on('data', chunk => chunks.push(chunk));
      res.on('error', reject);
      res.on('end', () => resolve({ status: res.statusCode, body: Buffer.concat(chunks) }));
    });
    req.on('error', reject); req.end(body);
  });
  return { proxy, send, received, origin };
}

for (const status of [200, 409, 501]) {
  test(`owned forwarding retains the exact completed ${status} body after the client closes, without replay`, async t => {
    const bytes = Buffer.from(JSON.stringify({ genuine_upstream_status: status, unique: `wire-${status}` }));
    const { proxy, send, received } = await transport(t, (_req, res) => {
      res.writeHead(status, { 'content-type': 'application/json', 'set-cookie': 'private_reply=must-not-be-recorded' });
      res.write(bytes.subarray(0, 7)); res.end(bytes.subarray(7));
    });
    proxy.registerCheckout(product);
    assert.deepEqual(await send(), { status, body: bytes });
    await proxy.close();
    const records = proxy.evidence().checkoutResponses;
    assert.equal(records.length, 1);
    assert.deepEqual(records[0], { method: 'POST', path: checkoutPath, requestBody: submitted,
      status, bodyBase64: bytes.toString('base64'), complete: true, error: null });
    assert.deepEqual(received, [{ method: 'POST', path: checkoutPath, body: submitted, cookie: 'private_auth=must-not-be-recorded' }]);
    assert.doesNotMatch(JSON.stringify(records), /private_auth|private_reply|cookie|set-cookie/);
  });
}

test('only the exact checkout method/path is observed, and an unregistered body is rejected without retaining it', async t => {
  const { proxy, send, received } = await transport(t, (_req, res) => res.end('real response'));
  proxy.registerCheckout(product);
  await send({ route: '/api/v1/auth/login', body: 'sensitive-unrelated-input' });
  await send({ method: 'GET', body: '' });
  await send({ route: `${checkoutPath}?unexpected=1` });
  assert.deepEqual(proxy.evidence().checkoutResponses, []);
  await send({ body: JSON.stringify({ ...JSON.parse(submitted), product_id: 'foreign-secret' }) });
  const [record] = proxy.evidence().checkoutResponses;
  assert.equal(record.error, 'Checkout request does not match an owned registration');
  assert.equal(record.requestBody, null);
  assert.equal(record.complete, false);
  assert.equal(record.bodyBase64, null);
  assert.equal(received.length, 4);
  assert.doesNotMatch(JSON.stringify(record), /foreign-secret|sensitive-unrelated-input/);
});

test('duplicate real checkout requests remain separate observations', async t => {
  const { proxy, send, received } = await transport(t, (_req, res) => res.end('actual response'));
  proxy.registerCheckout(product);
  await send(); await send();
  assert.equal(received.length, 2);
  assert.equal(proxy.evidence().checkoutResponses.length, 2);
});

test('compressed complete upstream bodies are decoded within the capture limit', async t => {
  const bytes = Buffer.from(JSON.stringify({ checkout_url: 'https://checkout.stripe.com/c/pay/cs_test_owned' }));
  const { proxy, send } = await transport(t, (_req, res) => {
    res.writeHead(200, { 'content-encoding': 'gzip' }); res.end(gzipSync(bytes));
  });
  proxy.registerCheckout(product);
  await send();
  assert.equal(proxy.evidence().checkoutResponses[0].bodyBase64, bytes.toString('base64'));
});

for (const failure of ['oversized-request', 'oversized-response', 'compressed-expansion', 'invalid-compression', 'truncated-response']) {
  test(`${failure} fails capture explicitly while preserving real forwarding`, async t => {
    const { proxy, send } = await transport(t, (_req, res) => {
      if (failure === 'truncated-response') {
        res.writeHead(200, { 'content-length': '1000' }); res.write('partial');
        setImmediate(() => res.destroy());
      } else if (failure === 'compressed-expansion') {
        res.writeHead(200, { 'content-encoding': 'gzip' }); res.end(gzipSync(Buffer.alloc(65537)));
      } else if (failure === 'invalid-compression') {
        res.writeHead(200, { 'content-encoding': 'gzip' }); res.end('not gzip');
      } else res.end(failure === 'oversized-response' ? Buffer.alloc(65537) : 'actual response');
    });
    proxy.registerCheckout(product);
    const sending = send({ body: failure === 'oversized-request' ? `${submitted}${' '.repeat(4097)}` : submitted });
    if (failure === 'truncated-response') await assert.rejects(sending);
    else await sending;
    const [record] = proxy.evidence().checkoutResponses;
    assert.equal(record.complete, false);
    assert.equal(record.bodyBase64, null);
    assert.equal(typeof record.error, 'string');
    assert.ok(record.error.length > 0);
  });
}

test('a client abort marks an incomplete checkout observation instead of certifying partial bytes', async t => {
  const { proxy, origin } = await transport(t, (_req, res) => { res.writeHead(200); res.write('partial'); });
  proxy.registerCheckout(product);
  await new Promise(resolve => {
    const req = request(proxy.server, { method: 'POST', path: `${origin}${checkoutPath}`, agent: false }, res => {
      res.once('data', () => { res.destroy(); resolve(); });
    });
    req.end(submitted);
  });
  await proxy.close();
  const [record] = proxy.evidence().checkoutResponses;
  assert.equal(record.complete, false);
  assert.equal(record.bodyBase64, null);
  assert.ok(record.error);
});

test('an aborted request cannot certify a response or retain its partial request body', async t => {
  const { proxy, origin, received } = await transport(t, (_req, res) => res.end('should not finish'));
  proxy.registerCheckout(product);
  const req = request(proxy.server, { method: 'POST', path: `${origin}${checkoutPath}`, agent: false,
    headers: { 'content-length': '1000' } });
  req.on('error', () => {});
  req.write(submitted);
  for (let attempt = 0; attempt < 100 && !proxy.evidence().checkoutResponses.length; attempt += 1) {
    await new Promise(resolve => setTimeout(resolve, 5));
  }
  req.destroy();
  for (let attempt = 0; attempt < 100 && !proxy.evidence().checkoutResponses[0]?.error; attempt += 1) {
    await new Promise(resolve => setTimeout(resolve, 5));
  }
  const [record] = proxy.evidence().checkoutResponses;
  assert.equal(record.complete, false);
  assert.equal(record.requestBody, null);
  assert.equal(record.bodyBase64, null);
  assert.ok(record.error);
  assert.deepEqual(received, []);
});

test('upstream transport failure remains failed capture even when the proxy reports502', async t => {
  const { proxy, send } = await transport(t, (req) => req.socket.destroy());
  proxy.registerCheckout(product);
  assert.equal((await send()).status, 502);
  const [record] = proxy.evidence().checkoutResponses;
  assert.equal(record.status, null);
  assert.equal(record.complete, false);
  assert.equal(record.bodyBase64, null);
  assert.equal(record.error, 'Checkout upstream request failed');
});
