// Test-only provider boundary. This module never runs in the application and
// never makes outbound requests. Wire shapes match shipping-integrity-contract.
import { createServer } from 'node:http';
import { isDeepStrictEqual } from 'node:util';
import { randomBytes, randomUUID } from 'node:crypto';

const tenant = 'e2e-tenant';
const addressFrom = { name: 'Isolated shipping sender', street1: '1 Fixture Way', city: 'San Francisco', state: 'CA', zip: '94103', country: 'US' };
const addressTo = { name: 'Isolated shipping recipient', street1: '2 Fixture Way', city: 'San Francisco', state: 'CA', zip: '94107', country: 'US' };
const fail = () => { throw new Error('Shipping browser tests require the current runner-owned loopback Shippo fixture'); };

export function verifiedShippoBrowserEnvironment(environment = process.env) {
  const runId = environment.OMNISOLO_E2E_SHIPPO_RUN_ID;
  let url;
  try { url = new URL(environment.SHIPPO_API_BASE); } catch { fail(); }
  if (!/^[a-f0-9]{12}$/.test(runId ?? '')
      || url.protocol !== 'http:' || url.hostname !== '127.0.0.1' || !url.port
      || url.origin !== environment.SHIPPO_API_BASE || url.username || url.password
      || environment.SHIPPO_TENANT_ID !== tenant
      || environment.SHIPPO_ACCOUNT_NAMESPACE !== `e2e-shippo-${runId}`
      || environment.SHIPPO_WEBHOOK_MODE !== 'test'
      || !new RegExp(`^shippo_test_local_${runId}_[a-f0-9]{32}$`).test(environment.SHIPPO_API_TOKEN ?? '')
      || environment.SHIPPO_ADDRESS_FROM_JSON !== JSON.stringify(addressFrom)
      || environment.SHIPPO_ADDRESS_TO_JSON !== JSON.stringify(addressTo)) fail();
  return { baseURL: url.origin, token: environment.SHIPPO_API_TOKEN, runId, tenantId: tenant, accountNamespace: environment.SHIPPO_ACCOUNT_NAMESPACE };
}

export async function startShippoBrowserFixture({ runId, tenantId }) {
  if (!/^[a-f0-9]{12}$/.test(runId ?? '') || tenantId !== tenant) fail();
  const token = `shippo_test_local_${runId}_${randomBytes(16).toString('hex')}`;
  const accountNamespace = `e2e-shippo-${runId}`;
  const rates = new Map(), transactions = new Map();
  let host, closed = false;
  const json = (response, status, body) => {
    response.writeHead(status, { 'content-type': 'application/json', 'cache-control': 'no-store' });
    response.end(JSON.stringify(body));
  };
  const server = createServer(async (request, response) => {
    if (request.headers.host !== host) return json(response, 400, { error: 'Only the exact loopback provider origin is accepted' });
    if (request.headers.authorization !== `ShippoToken ${token}`) return json(response, 401, { error: 'Fixture authentication required' });
    const path = request.url;
    if (request.method === 'GET' && path === '/__fixture/health') {
      return json(response, 200, { runId, tenantId, accountNamespace });
    }
    if (request.method === 'GET' && /^\/__fixture\/receipts\/transaction_[a-f0-9]{32}$/.test(path)) {
      const receipt = transactions.get(path.split('/').at(-1));
      return json(response, receipt ? 200 : 404, receipt ? { ...receipt, purchaseCount: rates.get(receipt.purchaseRequest.rate).purchaseCount } : { error: 'Unknown transaction' });
    }
    if (request.method !== 'POST' || !['/shipments', '/transactions'].includes(path)) {
      return json(response, 404, { error: 'No fixture endpoint or carrier evidence exists at this path' });
    }
    if (request.headers['shippo-api-version'] !== '2018-02-08') return json(response, 400, { error: 'Unexpected provider API version' });
    let body;
    try {
      const chunks = []; let length = 0;
      for await (const chunk of request) {
        length += chunk.length;
        if (length > 16384) return json(response, 413, { error: 'Fixture request is too large' });
        chunks.push(chunk);
      }
      body = JSON.parse(Buffer.concat(chunks).toString('utf8'));
    } catch { return json(response, 400, { error: 'Expected a JSON request' }); }
    if (!body || typeof body !== 'object' || Array.isArray(body) || body.async !== false) return json(response, 400, { error: 'Expected a synchronous provider request' });
    if (path === '/shipments') {
      const parcel = body.parcels?.[0];
      if (!Array.isArray(body.parcels) || body.parcels.length !== 1
          || !isDeepStrictEqual(body.address_from, addressFrom)
          || !isDeepStrictEqual(body.address_to, addressTo)
          || !parcel || parcel.distance_unit !== 'in' || parcel.mass_unit !== 'oz'
          || !['length', 'width', 'height', 'weight'].every(key => typeof parcel[key] === 'string' && Number.isFinite(Number(parcel[key])) && Number(parcel[key]) > 0 && Number(parcel[key]) <= 100000)) {
        return json(response, 400, { error: 'Invalid fixture address or parcel' });
      }
      if (rates.size >= 1000) return json(response, 429, { error: 'Fixture request limit reached' });
      const rate = `rate_${randomUUID().replaceAll('-', '')}`;
      rates.set(rate, { shipmentRequest: body, purchaseCount: 0 });
      return json(response, 200, { rates: [{ object_id: rate, provider: 'USPS', servicelevel: { name: 'Priority Mail' }, amount: '8.25', currency: 'USD', estimated_days: 2 }] });
    }
    const issued = rates.get(body.rate);
    if (!issued || body.label_file_type !== 'PDF' || !/^ohc_shipping_[a-f0-9-]{36}$/.test(body.metadata ?? '')) {
      return json(response, 400, { error: 'Transaction must match an issued rate and durable purchase metadata' });
    }
    issued.purchaseCount += 1;
    if (issued.purchaseCount !== 1) return json(response, 409, { error: 'Duplicate provider purchase detected' });
    const transactionId = `transaction_${randomUUID().replaceAll('-', '')}`;
    const receipt = {
      status: 'SUCCESS', object_id: transactionId, test: true, rate: body.rate, metadata: body.metadata,
      label_url: `https://app.goshippo.com/labels/${transactionId}.pdf`,
      tracking_number: `fixture_tracking_${transactionId.slice(12)}`, tracking_carrier: 'usps',
    };
    transactions.set(transactionId, { shipmentRequest: issued.shipmentRequest, purchaseRequest: body, response: receipt });
    return json(response, 200, receipt);
  });
  server.requestTimeout = 5000; server.headersTimeout = 5000;
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  host = `127.0.0.1:${server.address().port}`;
  const environment = {
    OMNISOLO_E2E_SHIPPO_RUN_ID: runId,
    SHIPPO_API_BASE: `http://${host}`, SHIPPO_API_TOKEN: token,
    SHIPPO_TENANT_ID: tenantId, SHIPPO_ACCOUNT_NAMESPACE: accountNamespace, SHIPPO_WEBHOOK_MODE: 'test',
    SHIPPO_ADDRESS_FROM_JSON: JSON.stringify(addressFrom), SHIPPO_ADDRESS_TO_JSON: JSON.stringify(addressTo),
  };
  verifiedShippoBrowserEnvironment(environment);
  return {
    environment,
    async close() {
      if (closed) return;
      closed = true;
      server.closeAllConnections();
      await new Promise((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
    },
  };
}
