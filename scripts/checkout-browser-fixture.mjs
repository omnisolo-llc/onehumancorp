// Test-only owned provider boundary. No Stripe SDK, live key, hosted payment or
// paid webhook is used here. Application requests remain real HTTP requests.
import { createServer, request as httpRequest } from 'node:http';
import { randomBytes } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { spawn } from 'node:child_process';
import { createServer as netServer, connect as connectSocket } from 'node:net';
import { lstat, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import os from 'node:os';
import { gunzipSync, inflateSync, brotliDecompressSync } from 'node:zlib';
import { verifiedFixtureDatabaseUrl } from './e2e-fixture-database.mjs';
import { validateWebArtifact } from './package-web.mjs';
import { verifyNativeBinaryProof } from './native-binary-proof.mjs';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

export function requireCheckoutLoopbackOrigin(value) {
  let url;
  try { url = new URL(value); } catch { throw new Error('An exact owned loopback origin is required'); }
  if (url.protocol !== 'http:' || url.hostname !== '127.0.0.1' || !url.port
      || url.origin !== value || url.username || url.password) {
    throw new Error('An exact owned loopback origin is required');
  }
  return url;
}

async function listen(server) {
  server.requestTimeout = 5000; server.headersTimeout = 5000;
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  return `http://127.0.0.1:${server.address().port}`;
}
function closer(server) {
  const sockets = new Set(); let closed = false;
  server.on('connection', socket => { sockets.add(socket); socket.once('close', () => sockets.delete(socket)); });
  return async () => {
    if (closed) return;
    closed = true;
    for (const socket of sockets) socket.destroy();
    server.closeAllConnections();
    await new Promise((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
  };
}
const json = (response, status, body) => {
  response.writeHead(status, { 'content-type': 'application/json', 'cache-control': 'no-store' });
  response.end(JSON.stringify(body));
};

export async function startCheckoutProvider({ runId, appOrigin }) {
  if (!/^[a-f0-9]{12}$/.test(runId ?? '')) throw new Error('An owned native run ID is required');
  requireCheckoutLoopbackOrigin(appOrigin);
  const key = `sk_test_local_${runId}_${randomBytes(16).toString('hex')}`;
  const registrations = new Map(), operations = new Set(), requests = [];
  let providerOrigin;
  const server = createServer(async (request, response) => {
    const row = { method: request.method, path: request.url,
      contentType: request.headers['content-type'] ?? null,
      idempotencyKey: request.headers['idempotency-key'] ?? null, rawBody: '', form: null };
    const reply = (status, body) => { row.status = status; if (status === 200) row.receipt = body; json(response, status, body); };
    if (requests.length >= 1000) return json(response, 429, { error: 'Owned provider request limit reached' });
    requests.push(row);
    if (request.headers.host !== new URL(providerOrigin).host) return reply(400, { error: 'Exact loopback provider host required' });
    if (request.method !== 'POST' || request.url !== '/v1/checkout/sessions') return reply(404, { error: 'No payment or webhook endpoint exists in this fixture' });
    if (request.headers.authorization !== `Basic ${Buffer.from(`${key}:`).toString('base64')}`) return reply(401, { error: 'Owned synthetic provider authentication required' });
    if (!/^application\/x-www-form-urlencoded(?:;.*)?$/i.test(row.contentType ?? '')) return reply(400, { error: 'A form-encoded checkout is required' });
    try {
      const chunks = []; let length = 0;
      for await (const chunk of request) {
        length += chunk.length;
        if (length > 16384) return reply(413, { error: 'Provider request is too large' });
        chunks.push(chunk);
      }
      row.rawBody = Buffer.concat(chunks).toString('utf8');
    } catch { return reply(400, { error: 'Incomplete provider request' }); }
    const form = new URLSearchParams(row.rawBody);
    row.form = Object.fromEntries(form);
    const product = registrations.get(`${form.get('client_reference_id')}\0${form.get('metadata[product_id]')}`);
    if (!product || form.size !== Object.keys(row.form).length) return reply(400, { error: 'Unregistered owned checkout or repeated form field' });
    let success;
    try { success = new URL(form.get('success_url')); } catch { /* checked below */ }
    if (!success || success.origin !== appOrigin || success.pathname !== '/payments/return'
        || success.username || success.password || success.hash || success.searchParams.size !== 1
        || success.searchParams.get('session_id') !== '{CHECKOUT_SESSION_ID}') {
      return reply(400, { error: 'Return URL does not match the owned app and Checkout placeholder' });
    }
    const expected = {
      // Preserve the exact raw URL in evidence while allowing URL-equivalent
      // brace escaping emitted by different standards-compliant clients.
      success_url: form.get('success_url'),
      cancel_url: `${appOrigin}/payments/cancel`, client_reference_id: product.tenantId,
      mode: 'payment', 'line_items[0][price_data][currency]': 'usd',
      'line_items[0][price_data][product_data][name]': product.title,
      'line_items[0][price_data][unit_amount]': String(product.amountCents),
      'line_items[0][quantity]': '1', 'payment_method_types[0]': 'card',
      'metadata[product_id]': product.productId,
    };
    if (!isDeepStrictEqual(row.form, expected) || !/^checkout:[a-f0-9-]{36}$/.test(row.idempotencyKey ?? '')) return reply(400, { error: 'Checkout terms or operation identity differ from the owned registration' });
    if (operations.has(row.idempotencyKey)) return reply(409, { error: 'Duplicate provider request' });
    operations.add(row.idempotencyKey);
    const id = `cs_test_${runId}_${randomBytes(16).toString('hex')}`;
    reply(200, { id, url: `https://checkout.stripe.com/c/pay/${id}`,
      payment_status: 'unpaid', amount_total: product.amountCents, currency: 'usd' });
  });
  const close = closer(server);
  providerOrigin = await listen(server);
  return {
    environment: { STRIPE_API_BASE: providerOrigin, STRIPE_API_KEY: key, PUBLIC_APP_URL: appOrigin },
    register(product) {
      if (!product || !/^e2e-[a-zA-Z0-9-]{1,180}$/.test(product.tenantId ?? '')
          || !/^[a-zA-Z0-9_-]{1,200}$/.test(product.productId ?? '')
          || typeof product.title !== 'string' || !product.title.trim() || product.title.length > 500
          || !Number.isSafeInteger(product.amountCents) || product.amountCents <= 0 || product.amountCents > 99999999
          || registrations.size >= 100) throw new Error('A bounded owned tenant/product registration is required');
      const identity = `${product.tenantId}\0${product.productId}`;
      if (registrations.has(identity)) throw new Error('The owned product is already registered');
      registrations.set(identity, structuredClone(product));
    },
    evidence: () => structuredClone({ requests }), close,
  };
}

export async function startCheckoutEgressProxy({ appOrigin }) {
  const ownedApp = requireCheckoutLoopbackOrigin(appOrigin);
  const connects = [], blockedHttp = [], forwardedHttp = [];
  const checkoutResponses = [], registeredCheckoutProducts = new Set(), captures = new Set();
  let checkoutCaptureError = null;
  // Observe only this fixture's registered checkout requests. Browser response
  // bodies may be evicted as soon as the real app navigates to provider checkout.
  // These copies come from the single existing upstream request, before forwarding
  // finishes; forwarding stays streaming, with no replacement or replay.
  const captureCheckout = (request, url) => {
    if (request.method !== 'POST' || url.pathname !== '/api/v1/billing/create-checkout-session' || url.search) return null;
    if (checkoutResponses.length >= 100) { checkoutCaptureError = 'Checkout observation limit reached'; return null; }
    const row = { method: request.method, path: url.pathname, requestBody: null,
      status: null, bodyBase64: null, complete: false, error: null };
    checkoutResponses.push(row);
    let requestBytes = 0, responseBytes = 0, requestEnded = false, responseEnded = false, encoding;
    let requestChunks = [], responseChunks = [];
    const fail = reason => {
      row.error ??= reason; row.complete = false; row.bodyBase64 = null;
      requestChunks = []; responseChunks = []; captures.delete(capture);
    };
    const finish = () => {
      if (!requestEnded || !responseEnded || row.error) return;
      try {
        let bytes = Buffer.concat(responseChunks);
        const options = { maxOutputLength: 65536 };
        if (encoding === 'gzip') bytes = gunzipSync(bytes, options);
        else if (encoding === 'deflate') bytes = inflateSync(bytes, options);
        else if (encoding === 'br') bytes = brotliDecompressSync(bytes, options);
        else if (encoding && encoding !== 'identity') throw new Error('Unsupported encoding');
        row.bodyBase64 = bytes.toString('base64'); row.complete = true;
        responseChunks = []; captures.delete(capture);
      } catch { fail('Checkout response body could not be decoded within its limit'); }
    };
    const capture = { fail, response(incoming) {
      row.status = incoming.statusCode;
      encoding = incoming.headers['content-encoding'];
      incoming.on('data', chunk => {
        if (row.error) return;
        responseBytes += chunk.length;
        if (responseBytes > 65536) fail('Checkout response body exceeded its limit');
        else responseChunks.push(Buffer.from(chunk));
      });
      incoming.once('end', () => {
        if (!incoming.complete) return fail('Checkout response body was incomplete');
        responseEnded = true; finish();
      });
      incoming.once('aborted', () => fail('Checkout response body was aborted'));
      incoming.once('error', () => fail('Checkout response body failed'));
    } };
    captures.add(capture);
    request.on('data', chunk => {
      if (row.error) return;
      requestBytes += chunk.length;
      if (requestBytes > 4096) fail('Checkout request body exceeded its limit');
      else requestChunks.push(Buffer.from(chunk));
    });
    request.once('end', () => {
      requestEnded = true;
      if (row.error) return;
      const body = Buffer.concat(requestChunks).toString('utf8'); requestChunks = [];
      let parsed;
      try { parsed = JSON.parse(body); } catch { /* fails the owned request match */ }
      if (!parsed || !registeredCheckoutProducts.has(parsed.product_id)
          || !isDeepStrictEqual(parsed, { is_subscription: false, product_id: parsed.product_id, quantity: 1 })) {
        return fail('Checkout request does not match an owned registration');
      }
      row.requestBody = body; finish();
    });
    request.once('aborted', () => fail('Checkout request body was aborted'));
    request.once('error', () => fail('Checkout request body failed'));
    return capture;
  };
  const upstreams = new Set();
  const server = createServer((request, response) => {
    let url;
    try { url = new URL(request.url); } catch { /* rejected below */ }
    if (!url || url.origin !== appOrigin || url.username || url.password
        || request.headers.upgrade || forwardedHttp.length + blockedHttp.length >= 10000) {
      if (blockedHttp.length < 1000) blockedHttp.push({ method: request.method, url: request.url, status: 403 });
      return json(response, 403, { error: 'Browser egress is restricted to the exact owned application origin' });
    }
    forwardedHttp.push({ method: request.method, path: `${url.pathname}${url.search}` });
    const capture = captureCheckout(request, url);
    const headers = { ...request.headers, host: url.host };
    delete headers['proxy-authorization']; delete headers['proxy-connection'];
    const upstream = httpRequest(url, { method: request.method, headers, agent: false, timeout: 5000 }, incoming => {
      capture?.response(incoming);
      response.writeHead(incoming.statusCode, incoming.headers); incoming.pipe(response);
      incoming.on('error', () => response.destroy());
    });
    upstreams.add(upstream); upstream.once('close', () => upstreams.delete(upstream));
    upstream.on('timeout', () => upstream.destroy(new Error('Owned application proxy timeout')));
    upstream.on('error', () => { capture?.fail('Checkout upstream request failed'); if (!response.headersSent) json(response, 502, { error: 'Owned application request failed' }); else response.destroy(); });
    response.once('close', () => {
      if (!response.writableFinished) { capture?.fail('Checkout client closed before forwarding finished'); upstream.destroy(); }
    });
    request.on('aborted', () => upstream.destroy()); request.pipe(upstream);
  });
  server.on('connect', (request, socket, head) => {
    const row = { method: 'CONNECT', authority: request.url, status: 403 };
    const capacity = connects.length < 1000;
    if (capacity) connects.push(row);
    // Playwright's API request client tunnels even plain HTTP. Match the raw
    // authority exactly; aliases, other loopback ports and all external hosts
    // stay refused, without resolving or connecting to the requested target.
    if (!capacity || request.url !== ownedApp.host) {
      socket.end('HTTP/1.1 403 Forbidden\r\nConnection: close\r\nContent-Length: 0\r\n\r\n');
      return;
    }
    socket.pause();
    const upstream = connectSocket({ host: '127.0.0.1', port: Number(ownedApp.port) });
    upstreams.add(upstream);
    upstream.once('close', () => {
      upstreams.delete(upstream);
      // pipe() ends the browser's writable side on normal EOF. Let that end
      // drain queued bytes; destroy() here can truncate a backpressured reply.
      if (row.status === 200 && !upstream.readableEnded) socket.destroy();
    });
    socket.once('close', () => upstream.destroy());
    socket.on('error', () => upstream.destroy());
    upstream.setTimeout(5000, () => upstream.destroy(new Error('Owned application tunnel timeout')));
    upstream.on('error', () => {
      if (row.status === 200) socket.destroy();
      else {
        row.status = 502;
        socket.end('HTTP/1.1 502 Bad Gateway\r\nConnection: close\r\nContent-Length: 0\r\n\r\n', () => socket.destroy());
      }
    });
    upstream.once('connect', () => {
      row.status = 200;
      socket.write('HTTP/1.1 200 Connection Established\r\n\r\n');
      if (head.length) upstream.write(head);
      socket.pipe(upstream); upstream.pipe(socket); socket.resume();
    });
  });
  const stop = closer(server), origin = await listen(server);
  return { server: origin, bypass: '<-loopback>',
    registerCheckout(product) {
      if (!product || !/^e2e-[a-zA-Z0-9-]{1,180}$/.test(product.tenantId ?? '')
          || !/^[a-zA-Z0-9_-]{1,200}$/.test(product.productId ?? '')
          || registeredCheckoutProducts.size >= 100 || registeredCheckoutProducts.has(product.productId)) {
        throw new Error('A unique bounded owned checkout registration is required');
      }
      registeredCheckoutProducts.add(product.productId);
    },
    evidence: () => structuredClone({ connects, blockedHttp, forwardedHttp, checkoutResponses, checkoutCaptureError }),
    async close() {
      for (const capture of captures) capture.fail('Checkout proxy closed before observation completed');
      for (const request of upstreams) request.destroy(); await stop();
    },
  };
}

export function checkoutProcessEnvironment(source) {
  const keep = ['PATH', 'HOME', 'USERPROFILE', 'SYSTEMROOT', 'WINDIR', 'TMP', 'TEMP', 'TMPDIR', 'LANG', 'LC_ALL',
    'DATABASE_URL', 'REDIS_URL', 'OMNISOLO_ENV', 'OMNISOLO_STANDALONE_MODE', 'OMNISOLO_DEFAULT_TENANT_ID',
    'OMNISOLO_GRPC_TLS_CERT_PATH', 'OMNISOLO_GRPC_TLS_KEY_PATH', 'OMNISOLO_GRPC_CLIENT_CA_PATH',
    'OMNISOLO_BUILTIN_AGENT_BINARY'];
  return Object.fromEntries(keep.filter(key => source[key] !== undefined).map(key => [key, source[key]]));
}

async function freePort() {
  const server = netServer();
  await new Promise((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  const port = server.address().port;
  await new Promise(resolve => server.close(resolve));
  return port;
}
async function waitReady(url, child) {
  const deadline = Date.now() + 45000;
  while (Date.now() < deadline) {
    if (child.exitCode !== null || child.signalCode !== null || child.spawnError) throw new Error('Owned checkout application exited before readiness');
    try {
      const response = await fetch(url, { redirect: 'manual', signal: AbortSignal.timeout(1000) });
      await response.body?.cancel(); if (response.ok) return;
    } catch { /* the owned listener may still be starting */ }
    await delay(100);
  }
  throw new Error(`Owned checkout application readiness timed out: ${url}`);
}
async function stopChild(child) {
  const terminate = signal => {
    if (child.ownedGroup && child.pid) {
      try { process.kill(-child.pid, signal); } catch (error) { if (error.code !== 'ESRCH') throw error; }
    } else if (child.exitCode === null && child.signalCode === null) child.kill(signal);
  };
  terminate('SIGTERM');
  for (let attempt = 0; attempt < 30 && child.exitCode === null && child.signalCode === null; attempt += 1) await delay(100);
  // Kill descendants in this fixture's newly-created process group too, even
  // when the direct child exited before its agent subprocesses did.
  terminate('SIGKILL');
  if (child.exitCode === null && child.signalCode === null) {
    for (let attempt = 0; attempt < 20 && child.exitCode === null && child.signalCode === null; attempt += 1) await delay(100);
  }
  if (child.exitCode === null && child.signalCode === null) throw new Error('Owned checkout child could not be reaped');
}

/** Starts the existing native outputs, never builds or copies dependencies. */
export async function startConfiguredCheckoutFixture({ environment = process.env } = {}) {
  const proofPath = environment.OMNISOLO_E2E_CHECKOUT_RUNTIME;
  if (!proofPath) throw new Error('Configured checkout requires the native runner runtime proof');
  const stat = await lstat(proofPath);
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 8192
      || (process.platform !== 'win32' && (stat.mode & 0o077) !== 0)) throw new Error('Invalid checkout runtime proof');
  const runtime = JSON.parse(await readFile(proofPath, 'utf8'));
  const db = verifiedFixtureDatabaseUrl(environment);
  const databaseProof = JSON.parse(await readFile(environment.OMNISOLO_E2E_FIXTURE_PROOF, 'utf8'));
  if (runtime.schemaVersion !== 1 || runtime.runId !== databaseProof.runId
      || runtime.databaseUrl !== db || runtime.redisUrl !== environment.REDIS_URL
      || runtime.databaseProof !== environment.OMNISOLO_E2E_FIXTURE_PROOF
      || runtime.agent !== environment.OMNISOLO_BUILTIN_AGENT_BINARY
      || runtime.web !== path.join(repository, 'target/native-web/src/ui/next/server.js')
      || runtime.binaryProof !== path.join(path.dirname(runtime.server), 'native-binary-proof.json')) {
    throw new Error('Checkout runtime does not belong to the current native runner');
  }
  const nativeProof = await verifyNativeBinaryProof(runtime.binaryProof, { server: runtime.server, agent: runtime.agent }, repository);
  const webProof = await validateWebArtifact(path.join(repository, 'target/native-web'), repository);
  const temp = await mkdtemp(path.join(os.tmpdir(), 'ohc-checkout-e2e-'));
  const children = [], logs = new Map(); let provider, proxy, processEnvironment, failure, closed = false;
  const artifacts = path.join(repository, 'test-results/checkout-fixtures', runtime.runId, path.basename(temp));
  const diagnostics = () => {
    let text = [...logs].map(([name, bytes]) => `${name}:\n${bytes.toString('utf8')}`).join('\n');
    for (const [key, value] of Object.entries(processEnvironment ?? {})) if (/SECRET|TOKEN|KEY/.test(key) && value) text = text.replaceAll(value, '<redacted>');
    return text.replace(/\bBearer\s+[^\s",}]+/gi, 'Bearer <redacted>')
      .replace(/("(?:access_token|refresh_token|token|cookie|authorization)"\s*:\s*)"[^"]*"/gi, '$1"<redacted>"');
  };
  const evidence = () => ({ ...provider?.evidence(), ...proxy?.evidence(),
    sourceSha256: nativeProof.sourceSha256, webSourceSha256: webProof.sourceSha256,
    processLogs: diagnostics(), ...(failure ? { failure } : {}) });
  const close = async () => {
    if (closed) return; closed = true;
    const failures = [];
    for (const child of children.reverse()) { try { await stopChild(child); } catch (error) { failures.push(error); } }
    for (const fixture of [proxy, provider]) { try { await fixture?.close(); } catch (error) { failures.push(error); } }
    try {
      await mkdir(artifacts, { recursive: true, mode: 0o700 });
      await writeFile(path.join(artifacts, 'evidence.json'), JSON.stringify(evidence(), null, 2) + '\n', { mode: 0o600, flag: 'wx' });
    } catch (error) { failures.push(error); }
    await rm(temp, { recursive: true, force: true });
    process.removeListener('SIGINT', onSignal); process.removeListener('SIGTERM', onSignal); process.removeListener('exit', onExit);
    if (failures.length) throw new AggregateError(failures, 'Owned checkout teardown failed');
  };
  const onSignal = () => { void close().catch(() => {}); };
  const onExit = () => { for (const child of children) {
    if (child.ownedGroup && child.pid) { try { process.kill(-child.pid, 'SIGKILL'); } catch { /* owned group already gone */ } }
    else if (child.exitCode === null && child.signalCode === null) child.kill('SIGKILL');
  } };
  process.once('SIGINT', onSignal); process.once('SIGTERM', onSignal); process.once('exit', onExit);
  const start = (binary, args, name, env) => {
    logs.set(name, Buffer.alloc(0));
    const capture = chunk => logs.set(name, Buffer.concat([logs.get(name), chunk]).subarray(-32768));
    const child = spawn(binary, args, { cwd: temp, env, shell: false, detached: process.platform !== 'win32', stdio: ['ignore', 'pipe', 'pipe'] });
    child.ownedGroup = process.platform !== 'win32';
    child.once('error', error => { child.spawnError = error; });
    child.stdout.on('data', capture); child.stderr.on('data', capture);
    children.push(child); return child;
  };
  try {
    const apiPort = await freePort(), grpcPort = await freePort(), webPort = await freePort();
    const apiOrigin = `http://127.0.0.1:${apiPort}`, origin = `http://127.0.0.1:${webPort}`;
    provider = await startCheckoutProvider({ runId: runtime.runId, appOrigin: origin });
    proxy = await startCheckoutEgressProxy({ appOrigin: origin });
    const home = path.join(temp, 'home'); await mkdir(home);
    const env = { ...checkoutProcessEnvironment(environment), ...provider.environment,
      HOME: home, USERPROFILE: home, OMNISOLO_PORT: String(apiPort), OMNISOLO_GRPC_PORT: String(grpcPort),
      OMNISOLO_AGENT_AUTH_KEY: randomBytes(32).toString('hex'), OMNISOLO_AGENT_TOKEN: randomBytes(32).toString('hex'),
      JWT_SECRET: randomBytes(32).toString('hex'), OMNISOLO_LLM_CONFIG_PATH: path.join(temp, 'no-provider-config.json'),
      API_BASE_URL: apiOrigin, BACKEND_URL: apiOrigin, OMNISOLO_BACKEND_URL: apiOrigin, OMNISOLO_API_URL: apiOrigin,
      BASE_URL: origin, OMNISOLO_WEB_CANONICAL_ORIGIN: origin, OMNISOLO_WEB_LOCAL_DEV: 'true',
      OMNISOLO_WEB_SESSION_KEY_ID: 'checkout-e2e-v1', OMNISOLO_WEB_SESSION_SECRET: randomBytes(32).toString('base64url'),
      NEXT_TELEMETRY_DISABLED: '1',
      // Even unrelated HTTPS attempts fail closed through our deny-all proxy.
      HTTP_PROXY: proxy.server, HTTPS_PROXY: proxy.server, ALL_PROXY: proxy.server,
      NO_PROXY: '127.0.0.1,localhost,::1',
    };
    processEnvironment = env;
    // The production backend currently binds 0.0.0.0; access here uses a fresh
    // loopback origin. Provider, browser proxy and Next bind only 127.0.0.1.
    const backend = start(runtime.server, [], 'backend.log', env);
    await waitReady(`${apiOrigin}/readyz`, backend);
    const frontend = start(process.execPath, [runtime.web], 'web.log', { ...env, PORT: String(webPort), HOSTNAME: '127.0.0.1', NODE_ENV: 'production' });
    await waitReady(`${origin}/login`, frontend);
    return { origin, apiOrigin, proxy: { server: proxy.server, bypass: proxy.bypass },
      register(product) { provider.register(product); proxy.registerCheckout(product); },
      evidence,
      close,
    };
  } catch (error) {
    failure = error.message;
    await close().catch(() => {});
    throw new Error(`${error.message}${diagnostics() ? `\n${diagnostics()}` : ''}`, { cause: error });
  }
}
