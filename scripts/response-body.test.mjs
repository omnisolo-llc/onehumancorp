import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { request } from '@playwright/test';
import { captureResponseBody } from './playwright/response-body.mjs';

for (const status of [200, 409, 501]) {
  test(`retains actual HTTP ${status} bytes after the transport context closes`, async t => {
    const payload = Buffer.from(JSON.stringify({ outcome: 'owned transport receipt', value: '€', status }));
    const server = createServer((_incoming, response) => {
      response.writeHead(status, { 'content-type': 'application/json', 'content-length': payload.length });
      response.end(payload);
    });
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    t.after(() => new Promise(resolve => { server.closeAllConnections(); server.close(resolve); }));
    const context = await request.newContext({ baseURL: `http://127.0.0.1:${server.address().port}` });
    t.after(() => context.dispose());
    const captured = await captureResponseBody(context.get('/actual-response'));
    assert.equal(captured.response.status(), status);
    await context.dispose();
    assert.deepEqual(await captured.body(), payload);
    assert.deepEqual(await captured.body(), payload, 'subsequent assertions must use the same captured bytes');
  });
}

test('propagates the original transport failure without a fabricated response', async () => {
  const failure = new Error('Owned response request failed');
  await assert.rejects(captureResponseBody(Promise.reject(failure)), error => error === failure);
});

test('does not publish a captured receipt when its real response body is unavailable', async t => {
  const server = createServer((_incoming, response) => response.end('actual body'));
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise(resolve => { server.closeAllConnections(); server.close(resolve); }));
  const context = await request.newContext({ baseURL: `http://127.0.0.1:${server.address().port}` });
  t.after(() => context.dispose());
  const response = await context.get('/actual-response');
  await context.dispose();
  await assert.rejects(captureResponseBody(Promise.resolve(response)), /disposed|closed/i);
});
