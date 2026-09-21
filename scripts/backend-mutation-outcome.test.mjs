import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { createServer } from 'node:http';
import test from 'node:test';
import { runInNewContext } from 'node:vm';

const require = createRequire(import.meta.url);
const ts = require('typescript');
const authRoot = new URL('../src/ui/next/src/lib/auth/', import.meta.url);

// Execute the production transport and URL validator, not a copied algorithm.
// Only session verification is substituted at the module boundary. These are
// transport unit tests: they do not certify authentication, Rust or a provider.
function loadTypeScript(name, imports = {}) {
  const source = readFileSync(new URL(name, authRoot), 'utf8');
  const output = ts.transpileModule(source, {
    fileName: name,
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.CommonJS,
    },
    reportDiagnostics: true,
  });
  const errors = (output.diagnostics ?? []).filter(
    (entry) => entry.category === ts.DiagnosticCategory.Error,
  );
  assert.equal(errors.length, 0, `TypeScript syntax errors in ${name}`);
  const module = { exports: {} };
  runInNewContext(output.outputText, {
    module,
    exports: module.exports,
    require(specifier) {
      assert.ok(Object.hasOwn(imports, specifier), `unexpected import ${specifier}`);
      return imports[specifier];
    },
    Request, Response, Headers, URL, AbortController, ReadableStream,
    TextEncoder, TextDecoder, Uint8Array, ArrayBuffer, setTimeout, clearTimeout,
  }, { filename: name });
  return module.exports;
}

const session = {
  accessToken: 'verified-test-token',
  user: { id: 'owner-1', organizationId: 'tenant-1', roles: ['owner'] },
  exp: 2_000_003_600,
};
const { proxyAuthenticatedRequest } = loadTypeScript('backendTransport.ts', {
  './url': loadTypeScript('url.ts'),
  './serverSession': {
    readServerSession: async (_request, dependencies) => dependencies.testSession,
    liveServerSessionDependencies: async () => {
      throw new Error('unit tests must not read live session configuration');
    },
  },
});

function dependencies(fetchImpl, overrides = {}) {
  return {
    config: { backendOrigin: 'https://backend.example' },
    testSession: session,
    now: () => 2_000_000_000,
    fetchImpl,
    timeoutMs: 1_000,
    requestLimitBytes: 128,
    responseLimitBytes: 128,
    ...overrides,
  };
}

function input(method = 'POST', init = {}) {
  return new Request('https://app.example/api/v1/agents/approvals/approval-1', {
    method,
    ...init,
  });
}
const path = '/api/v1/agents/approvals/approval-1';

async function assertUnknown(response, expectedStatus, expectedError) {
  assert.equal(response.status, expectedStatus);
  assert.equal(response.headers.get('cache-control'), 'private, no-store');
  assert.equal(response.headers.get('x-content-type-options'), 'nosniff');
  const payload = await response.json();
  // Keep the existing error field while adding a machine-readable recovery rule.
  assert.equal(payload.error, expectedError);
  assert.equal(payload.code, 'BACKEND_OUTCOME_UNKNOWN');
  assert.equal(payload.outcome, 'unknown');
  assert.equal(payload.reconciliation_required, true);
  assert.equal(payload.retry_safe, false);
  assert.match(payload.recovery, /before retrying/i);
  assert.doesNotMatch(JSON.stringify(payload), /private-key|tenant-secret/);
}

for (const method of ['POST', 'PUT', 'PATCH', 'DELETE']) {
  test(`${method}: interrupted dispatched write requires reconciliation and is not retried`, async () => {
    let attempts = 0;
    const deps = dependencies(async () => {
      attempts += 1;
      throw new Error('upstream private-key tenant-secret');
    });
    const response = await proxyAuthenticatedRequest(input(method), path, deps);
    await assertUnknown(response, 502, 'backend unavailable');
    assert.equal(attempts, 1);
  });
}

for (const method of ['GET', 'HEAD']) {
  test(`${method}: read failure keeps its existing response contract`, async () => {
    const deps = dependencies(async () => { throw new Error('disconnected'); });
    const response = await proxyAuthenticatedRequest(input(method), path, deps);
    assert.equal(response.status, 502);
    assert.deepEqual(await response.json(), { error: 'backend unavailable' });
  });
}

test('backend method override, not browser method, determines write uncertainty', async () => {
  const deps = dependencies(async () => { throw new Error('disconnected'); });
  const read = await proxyAuthenticatedRequest(input('POST'), path, deps, { backendMethod: 'GET' });
  assert.deepEqual(await read.json(), { error: 'backend unavailable' });
  const write = await proxyAuthenticatedRequest(input('GET'), path, deps, { backendMethod: 'POST' });
  await assertUnknown(write, 502, 'backend unavailable');
});

test('a timed-out dispatched write is unknown, not cancelled or safe to repeat', async () => {
  let attempts = 0;
  const deps = dependencies(async (_url, init) => {
    attempts += 1;
    await new Promise((_resolve, reject) => {
      init.signal.addEventListener('abort', () => reject(new Error('interrupted')), { once: true });
    });
  }, { timeoutMs: 15 });
  await assertUnknown(await proxyAuthenticatedRequest(input(), path, deps), 504, 'backend timeout');
  assert.equal(attempts, 1);
});

test('caller cancellation after dispatch preserves unknown outcome', async () => {
  const controller = new AbortController();
  const deps = dependencies(async () => {
    controller.abort();
    throw new Error('caller gone');
  });
  const response = await proxyAuthenticatedRequest(input('POST', { signal: controller.signal }), path, deps);
  await assertUnknown(response, 504, 'backend timeout');
});

for (const [name, responseFactory, message] of [
  ['redirect', () => new Response(null, { status: 307, headers: { location: 'https://elsewhere.example' } }), 'backend unavailable'],
  ['oversized declared response', () => new Response('{}', { headers: { 'content-length': '129' } }), 'backend response too large'],
  ['oversized actual response', () => new Response('x'.repeat(129)), 'backend response too large'],
  ['broken response stream', () => new Response(new ReadableStream({ start(controller) { controller.error(new Error('lost response')); } })), 'backend unavailable'],
]) {
  test(`${name} cannot establish a dispatched write's outcome`, async () => {
    let attempts = 0;
    const deps = dependencies(async () => { attempts += 1; return responseFactory(); });
    await assertUnknown(await proxyAuthenticatedRequest(input(), path, deps), 502, message);
    assert.equal(attempts, 1);
  });
}

for (const phase of ['transform', 'resolve']) {
  test(`cancellation during asynchronous ${phase} prevents late backend dispatch`, async () => {
    const controller = new AbortController();
    let attempts = 0;
    const deps = dependencies(async () => { attempts += 1; return Response.json({ success: true }); });
    const options = phase === 'transform'
      ? { transformRequestBody: async (body) => { controller.abort(); return body; } }
      : { resolveBackendPath: async () => { controller.abort(); return path; } };
    const response = await proxyAuthenticatedRequest(input('POST', { signal: controller.signal }), path, deps, options);
    assert.equal(attempts, 0, 'cancelled work must not reach fetch');
    assert.equal(response.status, 504);
    assert.deepEqual(await response.json(), { error: 'backend timeout' });
  });
}

test('already cancelled input is rejected before dispatch, without claiming unknown effects', async () => {
  let attempts = 0;
  const deps = dependencies(async () => { attempts += 1; return Response.json({ ok: true }); });
  const response = await proxyAuthenticatedRequest(input('POST', { signal: AbortSignal.abort() }), path, deps);
  assert.equal(response.status, 504);
  assert.equal(attempts, 0);
  assert.deepEqual(await response.json(), { error: 'backend timeout' });
});

test('an oversized input fails before dispatch and does not require reconciliation', async () => {
  let attempts = 0;
  const deps = dependencies(async () => { attempts += 1; return Response.json({ ok: true }); });
  const response = await proxyAuthenticatedRequest(input('POST', { body: 'x'.repeat(129) }), path, deps);
  assert.equal(response.status, 413);
  assert.equal(attempts, 0);
  assert.deepEqual(await response.json(), { error: 'request too large' });
});

test('unauthenticated input does not reach the backend', async () => {
  let attempts = 0;
  const deps = dependencies(async () => { attempts += 1; return Response.json({ ok: true }); }, { testSession: null });
  const response = await proxyAuthenticatedRequest(input(), path, deps);
  assert.equal(response.status, 401);
  assert.equal(attempts, 0);
  assert.deepEqual(await response.json(), { error: 'authentication required' });
});

test('invalid transformed input does not reach the backend', async () => {
  let attempts = 0;
  const deps = dependencies(async () => { attempts += 1; return Response.json({ ok: true }); });
  const response = await proxyAuthenticatedRequest(input(), path, deps, {
    transformRequestBody: () => { throw new Error('bad input'); },
  });
  assert.equal(response.status, 400);
  assert.equal(attempts, 0);
  assert.deepEqual(await response.json(), { error: 'invalid request' });
});

test('successful write retains its response and forwards the original idempotency key once', async () => {
  let attempts = 0;
  const deps = dependencies(async (_url, init) => {
    attempts += 1;
    assert.equal(new Headers(init.headers).get('idempotency-key'), 'owner-operation-7');
    assert.equal(new Headers(init.headers).get('x-tenant-id'), 'tenant-1');
    assert.equal(init.redirect, 'manual');
    return Response.json({ success: true, id: 'approved-7' }, { status: 201 });
  });
  const response = await proxyAuthenticatedRequest(input('POST', { headers: { 'idempotency-key': 'owner-operation-7' } }), path, deps);
  assert.equal(response.status, 201);
  assert.deepEqual(await response.json(), { success: true, id: 'approved-7' });
  assert.equal(attempts, 1);
});

test('a complete backend conflict remains authoritative and is not replaced', async () => {
  const deps = dependencies(async () => Response.json({ error: 'stale approval' }, { status: 409 }));
  const response = await proxyAuthenticatedRequest(input(), path, deps);
  assert.equal(response.status, 409);
  assert.deepEqual(await response.json(), { error: 'stale approval' });
});


async function withBackend(handler, run) {
  const server = createServer(handler);
  await new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', resolve);
  });
  try {
    const address = server.address();
    assert.ok(address && typeof address === 'object');
    await run(`http://127.0.0.1:${address.port}`);
  } finally {
    server.closeAllConnections();
    await new Promise((resolve, reject) => {
      server.close((error) => error ? reject(error) : resolve());
    });
  }
}

test('real HTTP: a lost reply after backend admission remains unknown without a second write', async () => {
  let writes = 0;
  await withBackend((request, response) => {
    assert.equal(request.method, 'POST');
    writes += 1;
    response.destroy();
  }, async (backendOrigin) => {
    const deps = dependencies(fetch, { config: { backendOrigin } });
    const response = await proxyAuthenticatedRequest(input(), path, deps);
    await assertUnknown(response, 502, 'backend unavailable');
    assert.equal(writes, 1);
  });
});

test('real HTTP: caller abort does not undo a write already received by the backend', async () => {
  let writes = 0;
  const controller = new AbortController();
  await withBackend(() => {
    writes += 1;
    controller.abort();
  }, async (backendOrigin) => {
    const deps = dependencies(fetch, { config: { backendOrigin } });
    const response = await proxyAuthenticatedRequest(input('POST', { signal: controller.signal }), path, deps);
    await assertUnknown(response, 504, 'backend timeout');
    assert.equal(writes, 1);
  });
});
