import test from 'node:test';
import assert from 'node:assert/strict';
import { copyFile, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import path from 'node:path';
const server = await readFile(new URL('../src/server/lib.rs', import.meta.url), 'utf8');

test('server startup never supplies deterministic agent credentials', () => {
  assert.equal(/e2e-dummy-token|e2e-dummy-key-that-is-at-least-thirty-two-bytes-long/.test(server), false, "deterministic agent credential found");
  assert.equal(/set_var\(\s*"OMNISOLO_AGENT_(?:TOKEN|AUTH_KEY)"/.test(server), false, "startup must not inject credentials");
});
test('standalone startup still requires actual agent authentication before database initialization', () => {
  const start = server.indexOf('let grpc_tls_config = grpc_tls_config_from_env(standalone)?;');
  const end = server.indexOf('// Initialize database', start);
  const setup = server.slice(start, end);
  assert.match(setup, /let builtin_agent_auth = if standalone/);
  assert.match(setup, /auth_mode_from_env\(\).*map_err/s);
  assert.match(setup, /invalid builtin agent authentication configuration/);
});

test('required native CI executes the non-test startup authentication regression', async () => {
  const workflow = await readFile(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const runner = await readFile(new URL('./agent-startup-auth-contract/run.sh', import.meta.url), 'utf8');
  assert.ok(workflow.includes('run: bash scripts/agent-startup-auth-contract/run.sh'));
  assert.ok(runner.includes('--test startup'));
  assert.ok(runner.includes('--locked --offline'));
  assert.ok(runner.includes('source-manifest.json'));
});

const redisBoundary = '    // Validate Redis startup before spawning workers.';
const databaseBoundary = '    // Initialize database';
const additionalAuthCheck = '    if builtin_agent_auth.is_some() { return Err(std::io::Error::other("additional auth policy")); }\n';

async function generateAuthFixture(t, fixtureSource) {
  const root = await mkdtemp(path.join(tmpdir(), 'ohc-auth-generator-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  const inputs = [
    'Cargo.lock', '.github/workflows/ci.yml', 'src/agents/builtin/auth.rs',
    'scripts/agent-startup-auth.test.mjs',
    ...['Cargo.toml', 'prepare.py', 'probe.rs', 'test.rs', 'run.sh', 'verify_lock.py', 'README.md']
      .map((name) => `scripts/agent-startup-auth-contract/${name}`),
  ];
  for (const relative of inputs) {
    const destination = path.join(root, relative);
    await mkdir(path.dirname(destination), { recursive: true });
    await copyFile(new URL(`../${relative}`, import.meta.url), destination);
  }
  await mkdir(path.join(root, 'src/server'), { recursive: true });
  await writeFile(path.join(root, 'src/server/lib.rs'), fixtureSource);
  const result = spawnSync('python3', ['scripts/agent-startup-auth-contract/prepare.py'], {
    cwd: root, encoding: 'utf8', timeout: 10_000,
  });
  const generated = result.status === 0
    ? await readFile(path.join(root, 'scripts/agent-startup-auth-contract/generated.rs'), 'utf8')
    : '';
  return { ...result, generated };
}

test('auth harness extracts actual validation without asynchronous Redis preflight', async (t) => {
  const result = await generateAuthFixture(t, server);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.generated, /auth_mode_from_env\(\)/);
  assert.match(result.generated, /invalid builtin agent authentication configuration/);
  assert.doesNotMatch(result.generated, /redis::|crate::queue|\.await/);
});

test('auth harness retains additional validation before Redis startup', async (t) => {
  const result = await generateAuthFixture(t, server.replace(redisBoundary, additionalAuthCheck + redisBoundary));
  assert.equal(result.status, 0, result.stderr);
  assert.ok(result.generated.includes(additionalAuthCheck));
});

test('auth harness rejects validation moved beyond its covered boundary', async (t) => {
  const result = await generateAuthFixture(t, server.replace(databaseBoundary, additionalAuthCheck + databaseBoundary));
  assert.notEqual(result.status, 0);
});

test('auth harness rejects missing or duplicate Redis boundaries', async (t) => {
  for (const fixture of [
    server.replace(redisBoundary, '    // Redis startup boundary removed'),
    server.replace(redisBoundary, `${redisBoundary}\n${redisBoundary}`),
  ]) {
    const result = await generateAuthFixture(t, fixture);
    assert.notEqual(result.status, 0);
  }
});

test('auth harness rejects reordered startup boundaries', async (t) => {
  const result = await generateAuthFixture(t, `${redisBoundary}\n${server.replace(redisBoundary, '')}`);
  assert.notEqual(result.status, 0);
});
