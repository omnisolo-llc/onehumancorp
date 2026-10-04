import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer, request } from 'node:http';
import { connect } from 'node:net';
import { execFileSync, spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { startCheckoutProvider, startCheckoutEgressProxy, checkoutProcessEnvironment, startConfiguredCheckoutFixture } from './checkout-browser-fixture.mjs';
import { snapshotNativeSource, recordNativeBinaryProof, verifyNativeBinaryProof } from './native-binary-proof.mjs';
import { testEnvironment } from './native-e2e.mjs';

const require = createRequire(import.meta.url);

const runId = '012345abcdef';
const registration = { tenantId: 'e2e-checkout-owned', productId: 'e2e-product-owned', title: 'Owned final unit', amountCents: 2500 };
const origin = 'http://127.0.0.1:32123';
function form(product = registration) {
  return new URLSearchParams({
    success_url: `${origin}/payments/return?session_id=%7BCHECKOUT_SESSION_ID%7D`,
    cancel_url: `${origin}/payments/cancel`, client_reference_id: product.tenantId,
    mode: 'payment', 'line_items[0][price_data][currency]': 'usd',
    'line_items[0][price_data][product_data][name]': product.title,
    'line_items[0][price_data][unit_amount]': String(product.amountCents),
    'line_items[0][quantity]': '1', 'payment_method_types[0]': 'card',
    'metadata[product_id]': product.productId,
  });
}
async function provider(t) {
  const fixture = await startCheckoutProvider({ runId, appOrigin: origin });
  t.after(() => fixture.close());
  fixture.register(registration);
  const send = (body = form(), headers = {}) => new Promise((resolve, reject) => {
    const req = request(`${fixture.environment.STRIPE_API_BASE}/v1/checkout/sessions`, {
      method: 'POST', agent: false,
      headers: { authorization: `Basic ${Buffer.from(`${fixture.environment.STRIPE_API_KEY}:`).toString('base64')}`,
        'content-type': 'application/x-www-form-urlencoded;charset=UTF-8',
        'idempotency-key': 'checkout:01234567-89ab-cdef-0123-456789abcdef', ...headers },
    }, response => {
      const chunks = []; response.on('data', chunk => chunks.push(chunk));
      response.on('end', () => resolve({ status: response.statusCode, json: async () => JSON.parse(Buffer.concat(chunks).toString()) }));
    });
    req.on('error', reject); req.end(body.toString());
  });
  return { fixture, send };
}

test('provider records exact received form and issues only a matched unpaid session', async t => {
  const { fixture, send } = await provider(t);
  const response = await send();
  assert.equal(response.status, 200);
  const receipt = await response.json();
  assert.match(receipt.id, /^cs_test_012345abcdef_[a-f0-9]{32}$/);
  assert.equal(receipt.url, `https://checkout.stripe.com/c/pay/${receipt.id}`);
  assert.equal(receipt.payment_status, 'unpaid');
  assert.equal(receipt.amount_total, registration.amountCents);
  assert.equal(receipt.currency, 'usd');
  assert.deepEqual(fixture.evidence().requests[0], {
    method: 'POST', path: '/v1/checkout/sessions',
    contentType: 'application/x-www-form-urlencoded;charset=UTF-8',
    idempotencyKey: 'checkout:01234567-89ab-cdef-0123-456789abcdef',
    rawBody: form().toString(), form: Object.fromEntries(form()), status: 200, receipt,
  });
  assert.equal((await send()).status, 409, 'duplicate provider requests must remain visible');
  assert.equal(fixture.evidence().requests.length, 2);
  await fixture.close();
  await assert.rejects(send());
});

test('provider rejects foreign tenants, altered terms, duplicate fields, credentials and unknown routes', async t => {
  const { fixture, send } = await provider(t);
  assert.equal((await send(form({ ...registration, tenantId: 'e2e-foreign' }))).status, 400);
  assert.equal((await send(form({ ...registration, amountCents: 1 }))).status, 400);
  const duplicate = form(); duplicate.append('mode', 'payment');
  assert.equal((await send(duplicate)).status, 400);
  assert.equal((await send(form(), { authorization: 'Basic c2tfbGl2ZTo=' })).status, 401);
  assert.equal((await send(form(), { host: 'api.stripe.com' })).status, 400);
  assert.equal((await fetch(`${fixture.environment.STRIPE_API_BASE}/v1/payment_intents`)).status, 404);
  assert.equal(fixture.evidence().requests.filter(entry => entry.status === 200).length, 0);
  assert.throws(() => fixture.register({ ...registration, tenantId: 'live-tenant' }), /owned/);
  await assert.rejects(startCheckoutProvider({ runId, appOrigin: 'https://example.com' }), /loopback/);
});

function proxyRequest(proxy, target) {
  return new Promise((resolve, reject) => {
    const req = request(proxy, { path: target, method: 'GET', agent: false }, res => {
      const chunks = []; res.on('data', chunk => chunks.push(chunk));
      res.on('end', () => resolve({ status: res.statusCode, body: Buffer.concat(chunks).toString() }));
    });
    req.on('error', reject); req.end();
  });
}

test('egress proxy forwards actual owned HTTP and refuses every external HTTP/CONNECT target', async t => {
  let received = 0;
  const app = createServer((req, res) => { received += 1; res.end(`actual app ${req.url}`); });
  await new Promise(resolve => app.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise(resolve => { app.closeAllConnections(); app.close(resolve); }));
  const appOrigin = `http://127.0.0.1:${app.address().port}`;
  const proxy = await startCheckoutEgressProxy({ appOrigin });
  t.after(() => proxy.close());
  assert.deepEqual(await proxyRequest(proxy.server, `${appOrigin}/checkout`), { status: 200, body: 'actual app /checkout' });
  assert.equal(received, 1);
  assert.equal((await proxyRequest(proxy.server, 'http://api.stripe.com/v1/checkout/sessions')).status, 403);
  assert.equal((await proxyRequest(proxy.server, `${appOrigin}@example.com/`)).status, 403);
  const blocked = await new Promise((resolve, reject) => {
    const socket = connect(Number(new URL(proxy.server).port), '127.0.0.1');
    let response = ''; socket.setTimeout(2000, () => socket.destroy(new Error('Proxy CONNECT did not close')));
    socket.on('error', reject); socket.on('data', chunk => { response += chunk; });
    socket.on('close', () => resolve(response));
    socket.on('connect', () => socket.end('CONNECT checkout.stripe.com:443 HTTP/1.1\r\nHost: checkout.stripe.com:443\r\n\r\n'));
  });
  assert.match(blocked, /^HTTP\/1\.1 403 /);
  assert.equal(received, 1);
  assert.deepEqual(proxy.evidence().connects, [{ method: 'CONNECT', authority: 'checkout.stripe.com:443', status: 403 }]);
  assert.equal(proxy.evidence().blockedHttp.length, 2);
  assert.equal(proxy.bypass, '<-loopback>');
});

test('configured child environment admits only runner state and a generated provider key', () => {
  const safe = checkoutProcessEnvironment({
    PATH: '/usr/bin', DATABASE_URL: 'postgres://owned', REDIS_URL: 'redis://owned',
    STRIPE_API_KEY: 'sk_live_leak', STRIPE_SECRET_KEY: 'sk_live_other',
    HTTPS_PROXY: 'http://external', NODE_OPTIONS: '--require untrusted', OPENAI_API_KEY: 'live',
  });
  assert.deepEqual(safe, { PATH: '/usr/bin', DATABASE_URL: 'postgres://owned', REDIS_URL: 'redis://owned' });
});

test('provider accepts equivalent placeholder escaping and rejects extra return parameters', async t => {
  const { fixture, send } = await provider(t);
  const unescaped = form(); unescaped.set('success_url', `${origin}/payments/return?session_id={CHECKOUT_SESSION_ID}`);
  assert.equal((await send(unescaped)).status, 200);
  assert.equal(fixture.evidence().requests[0].form.success_url, unescaped.get('success_url'));
  for (const value of [`${origin}/payments/return?session_id={CHECKOUT_SESSION_ID}&extra=1`,
    'https://example.com/payments/return?session_id={CHECKOUT_SESSION_ID}',
    `${origin}/payments/return?session_id=another-session`]) {
    const malformed = form(); malformed.set('success_url', value);
    assert.equal((await send(malformed)).status, 400);
  }
});

test('native proof binds actual bytes and tracked plus untracked source, refusing drift before reuse', async t => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'ohc-native-proof-test-'));
  t.after(() => rm(directory, { recursive: true, force: true }));
  execFileSync('git', ['init', '--quiet', directory]);
  await writeFile(path.join(directory, '.gitignore'), '/target/\n');
  await writeFile(path.join(directory, 'Cargo.toml'), '[workspace]\n');
  execFileSync('git', ['add', 'Cargo.toml', '.gitignore'], { cwd: directory });
  await writeFile(path.join(directory, 'untracked.rs'), '// actual untracked input\n');
  const snapshot = path.join(directory, 'target/debug/native-source-snapshot.json');
  const proof = path.join(directory, 'target/debug/native-binary-proof.json');
  await snapshotNativeSource(snapshot, directory);
  const binaries = { server: path.join(directory, 'target/debug/server'), agent: path.join(directory, 'target/debug/omnisolo-builtin-agent') };
  await writeFile(binaries.server, 'test output, never executed');
  await writeFile(binaries.agent, 'test agent, never executed');
  const recorded = await recordNativeBinaryProof(snapshot, proof, binaries, directory);
  assert.deepEqual(await verifyNativeBinaryProof(proof, binaries, directory), recorded);
  await writeFile(path.join(directory, 'untracked.rs'), '// changed during or after build\n');
  await assert.rejects(recordNativeBinaryProof(snapshot, proof, binaries, directory), /current source/);
  await assert.rejects(verifyNativeBinaryProof(proof, binaries, directory), /current source/);
  await writeFile(path.join(directory, 'untracked.rs'), '// actual untracked input\n');
  await writeFile(binaries.server, 'tampered executable');
  await assert.rejects(verifyNativeBinaryProof(proof, binaries, directory), /compiler outputs/);
});

test('configured launcher rejects missing or non-private runner proof before any application spawn', async t => {
  await assert.rejects(startConfiguredCheckoutFixture({ environment: {} }), /runner runtime proof/);
  const directory = await mkdtemp(path.join(os.tmpdir(), 'ohc-checkout-proof-test-'));
  t.after(() => rm(directory, { recursive: true, force: true }));
  const proof = path.join(directory, 'runtime.json');
  await writeFile(proof, '{}', { mode: 0o644 });
  await assert.rejects(startConfiguredCheckoutFixture({ environment: { OMNISOLO_E2E_CHECKOUT_RUNTIME: proof } }), /Invalid checkout runtime proof/);
});

test('CI brackets its one existing Cargo build with source proof and retains it in the same artifact', async () => {
  const workflow = await readFile(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const job = workflow.split('\n  native-build:')[1].split('\n  native-test:')[0];
  const snapshot = job.indexOf('native-binary-proof.mjs snapshot target/debug/native-source-snapshot.json');
  const build = job.indexOf('cargo build --locked');
  const record = job.indexOf('native-binary-proof.mjs record target/debug/native-source-snapshot.json target/debug/native-binary-proof.json');
  assert.ok(snapshot > 0 && build > snapshot && record > build, 'source snapshot precedes the compiler and success recording follows it');
  assert.equal((job.match(/cargo build /g) ?? []).length, 1, 'reuse the existing build');
  assert.match(job, /node-version-file: \.node-version/);
  assert.match(job, /name: native-linux-binaries[\s\S]*target\/debug\/native-binary-proof\.json/);
  assert.doesNotMatch(job, /continue-on-error:\s*true|cargo build[^\n]+\|\|/);
});

test('local make build-e2e brackets the existing compiler and respects the selected Cargo target', async () => {
  const makefile = await readFile(new URL('../Makefile', import.meta.url), 'utf8');
  const recipe = makefile.split('\nbuild-e2e:\n')[1].split('\ntest-e2e:')[0];
  const snapshot = recipe.indexOf('native-binary-proof.mjs snapshot');
  const build = recipe.indexOf('$(CARGO) build --locked');
  const record = recipe.indexOf('native-binary-proof.mjs record');
  assert.ok(snapshot >= 0 && build > snapshot && record > build);
  assert.ok(recipe.indexOf('$(MAKE) build-web') > record);
  assert.equal((recipe.match(/\$\(CARGO\) build /g) ?? []).length, 1);
  assert.match(recipe, /\$\(if \$\(CARGO_TARGET_DIR\),\$\(CARGO_TARGET_DIR\),target\)\/debug\/native-binary-proof\.json/);
});

test('custom-target compiler outputs and generated Next declarations cannot invalidate native source proof', async t => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'ohc-custom-native-proof-test-'));
  t.after(() => rm(directory, { recursive: true, force: true }));
  execFileSync('git', ['init', '--quiet', directory]);
  await writeFile(path.join(directory, 'Cargo.toml'), '[workspace]\n');
  await writeFile(path.join(directory, 'next-env.d.ts'), '// before Next build\n');
  const target = path.join(directory, 'selected-build');
  const snapshot = path.join(target, 'debug/native-source-snapshot.json');
  const proof = path.join(target, 'debug/native-binary-proof.json');
  await snapshotNativeSource(snapshot, directory, target);
  const binaries = { server: path.join(target, 'debug/server'), agent: path.join(target, 'debug/omnisolo-builtin-agent') };
  await writeFile(binaries.server, 'custom compiler output, never executed');
  await writeFile(binaries.agent, 'custom agent output, never executed');
  const recorded = await recordNativeBinaryProof(snapshot, proof, binaries, directory);
  await writeFile(path.join(directory, 'next-env.d.ts'), '// generated by the independent Next build\n');
  assert.deepEqual(await verifyNativeBinaryProof(proof, binaries, directory), recorded);
});

test('Playwright can invoke the configured helper and reach its real ownership guard without resources', async t => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'ohc-checkout-loader-test-'));
  t.after(() => rm(directory, { recursive: true, force: true }));
  const helper = fileURLToPath(new URL('../src/e2e/support/configured_checkout_fixture.ts', import.meta.url));
  const config = path.join(directory, 'playwright.config.ts');
  const nativeConfig = fileURLToPath(new URL('../playwright.config.ts', import.meta.url));
  const nativeTypes = fileURLToPath(new URL('../playwright.tsconfig.json', import.meta.url));
  // Reuse the maintained transform configuration. The only overrides isolate
  // this loader regression from application/global setup and browser resources.
  await writeFile(config, `import nativeConfig from ${JSON.stringify(nativeConfig)};
export default { ...nativeConfig, testDir: ${JSON.stringify(directory)}, testMatch: 'fixture-loader.spec.ts',
  tsconfig: ${JSON.stringify(nativeTypes)}, globalSetup: undefined, use: {}, projects: [{ name: 'loader' }],
  outputDir: ${JSON.stringify(path.join(directory, 'results'))}, workers: 1, retries: 0, reporter: 'line' };\n`);
  const runtimeProof = path.join(directory, 'runtime.json');
  await writeFile(runtimeProof, '{}', { mode: 0o600 });
  await writeFile(path.join(directory, 'fixture-loader.spec.ts'), `
import { test, expect } from ${JSON.stringify(require.resolve('@playwright/test'))};
import { startConfiguredCheckoutFixture } from ${JSON.stringify(helper)};
test('actual configured helper fails closed before resources without its runner proof', async () => {
  await expect(startConfiguredCheckoutFixture()).rejects.toThrow('Configured checkout requires the native runner runtime proof');
  process.env.OMNISOLO_E2E_CHECKOUT_RUNTIME = ${JSON.stringify(runtimeProof)};
  try {
    await expect(startConfiguredCheckoutFixture()).rejects.toThrow('E2E SQL requires the current runner-owned disposable PostgreSQL fixture');
  } finally { delete process.env.OMNISOLO_E2E_CHECKOUT_RUNTIME; }
});
`);
  const result = spawnSync(process.execPath, [require.resolve('@playwright/test/cli'), 'test', '--config', config], {
    cwd: fileURLToPath(new URL('..', import.meta.url)), env: testEnvironment(),
    encoding: 'utf8', timeout: 30000, maxBuffer: 1024 * 1024,
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stdout + result.stderr);
  assert.match(result.stdout, /1 passed/);
  assert.doesNotMatch(result.stdout + result.stderr, /Cannot use .*import\.meta|0 tests|skipped/i);
});
