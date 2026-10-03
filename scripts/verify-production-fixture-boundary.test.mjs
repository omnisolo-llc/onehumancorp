import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { once } from 'node:events';
import { verifyProductionFixtureBoundary, retiredFixturePaths } from './verify-production-fixture-boundary.mjs';

async function withServer(handler, check) {
  const server = createServer(handler);
  server.listen(0, '127.0.0.1');
  await once(server, 'listening');
  try { await check(`http://127.0.0.1:${server.address().port}`); }
  finally { server.closeAllConnections(); await new Promise(resolve => server.close(resolve)); }
}

test('the negative HTTP verifier requires authenticated 404 for all 18 retired paths', async () => {
  const observed = [];
  await withServer((request, response) => {
    observed.push(request.url);
    assert.equal(request.headers.authorization, 'Bearer test-only-token');
    assert.equal(request.method, 'POST');
    response.writeHead(404).end();
  }, origin => verifyProductionFixtureBoundary(origin, 'test-only-token'));
  assert.deepEqual(observed, retiredFixturePaths);
  assert.equal(observed.length, 18);
});

for (const status of [200, 401, 403, 422, 500]) {
  test(`HTTP ${status} cannot be mistaken for an absent fixture endpoint`, async () => {
    await withServer((_request, response) => response.writeHead(status).end(),
      origin => assert.rejects(verifyProductionFixtureBoundary(origin, 'test-only-token'), new RegExp(`HTTP ${status}`)));
  });
}

test('the HTTP verifier refuses foreign destinations and missing authentication before requests', async () => {
  await assert.rejects(verifyProductionFixtureBoundary('https://app.example', 'test-only-token'), /isolated local backend/);
  await assert.rejects(verifyProductionFixtureBoundary('http://127.0.0.1:12345', ''), /authenticated owner/);
});
