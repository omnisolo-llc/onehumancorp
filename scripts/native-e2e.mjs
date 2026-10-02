import { spawn } from 'node:child_process';
import { createRequire } from 'node:module';
import { randomBytes, randomUUID } from 'node:crypto';
import { createServer } from 'node:net';
import { createWriteStream, existsSync } from 'node:fs';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';
import os from 'node:os';
import { validateWebArtifact } from './package-web.mjs';
import { runNativeCommand } from './native-process.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const require = createRequire(path.join(root, 'package.json'));
const postgresImage = 'pgvector/pgvector:pg15@sha256:18d16372b8406bb38a9f94cbff15d125c463d71fde2770aa8b5c64bfcc1578ee';
const valkeyImage = 'valkey/valkey:8-alpine@sha256:94365b275456ae14621001c03556c732b1d93a0cdeacc317d1bdd52eba680885';

// Deliberately do not load .env or inherit model/payment/production credentials.
export function testEnvironment(source = process.env) {
  const keep = ['PATH', 'HOME', 'USERPROFILE', 'SYSTEMROOT', 'WINDIR', 'TMP', 'TEMP',
    'TMPDIR', 'LANG', 'LC_ALL', 'CI', 'PLAYWRIGHT_BROWSERS_PATH',
    'PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH'];
  return Object.fromEntries(keep.filter((key) => source[key] !== undefined).map((key) => [key, source[key]]));
}

export function nativeBinaryPaths(repository = root, environment = process.env, platform = process.platform) {
  const configured = environment.CARGO_TARGET_DIR;
  if (configured !== undefined && !configured.trim()) {
    throw new Error('CARGO_TARGET_DIR must be a nonempty directory');
  }
  const directory = path.resolve(repository, configured ?? 'target', 'debug');
  const suffix = platform === 'win32' ? '.exe' : '';
  return {
    server: path.join(directory, `server${suffix}`),
    agent: path.join(directory, `omnisolo-builtin-agent${suffix}`),
  };
}

function command(executable, args, options = {}) {
  return runNativeCommand(executable, args, { cwd: root, ...options });
}

async function freePort() {
  const socket = createServer();
  await new Promise((resolve, reject) => { socket.once('error', reject); socket.listen(0, '127.0.0.1', resolve); });
  const port = socket.address().port;
  await new Promise((resolve) => socket.close(resolve));
  return port;
}

async function waitHttp(url, child, seconds = 120, signal) {
  const deadline = Date.now() + seconds * 1000;
  while (Date.now() < deadline) {
    signal?.throwIfAborted();
    if (child.exitCode !== null || child.signalCode !== null) throw new Error(`Local process exited before ${url} was ready`);
    try {
      const timeout = AbortSignal.timeout(Math.max(1, Math.min(2000, deadline - Date.now())));
      const response = await fetch(url, { signal: signal ? AbortSignal.any([signal, timeout]) : timeout, redirect: 'manual' });
      await response.body?.cancel();
      if (response.ok) return;
    } catch { signal?.throwIfAborted(); }
    await delay(Math.max(1, Math.min(1000, deadline - Date.now())), undefined, { signal });
  }
  throw new Error(`Local test service did not become ready: ${url}`);
}

async function stop(child) {
  if (!child || child.exitCode !== null || child.signalCode !== null) return;
  child.kill('SIGTERM');
  for (let i = 0; i < 30 && child.exitCode === null && child.signalCode === null; i++) await delay(100);
  if (child.exitCode === null && child.signalCode === null) child.kill('SIGKILL');
}

export async function runNativeE2e(inputArgs = process.argv.slice(2)) {
  const ciSelection = inputArgs.includes('--ci');
  const args = inputArgs.filter((arg) => arg !== '--ci');
  if (args.some((arg) => arg === '--pass-with-no-tests')) throw new Error('Zero-test success is not allowed');
  const env = testEnvironment();
  env.PLAYWRIGHT_TEST_DIR = './src';
  env.PLAYWRIGHT_LIST_REPORTER = '1';
  if (ciSelection) env.CI = 'true';
  const playwright = require.resolve('@playwright/test/cli');
  // Fail on broken imports, invalid fixtures or zero selection BEFORE spending
  // time starting Docker, applying migrations or launching either application.
  const listed = await command(process.execPath, [playwright, 'test', '--config', 'playwright.config.ts',
    '--list', ...args.filter((arg) => arg !== '--list')], { env });
  if (!/Total:\s*[1-9]\d* tests?/.test(listed)) throw new Error('Playwright selected zero tests or did not report its test count');
  if (args.includes('--list')) return;
  // Respect the native Cargo output directory instead of silently executing
  // stale default-directory binaries after a custom-target build.
  const { server, agent } = nativeBinaryPaths();
  const web = path.join(root, 'target/native-web/src/ui/next/server.js');
  for (const required of [server, agent, web]) if (!existsSync(required)) {
    throw new Error(`Required native test input missing: ${required}. Build Cargo binaries and run npm run build:web first.`);
  }
  // Detect conflicting migrations before starting containers or the backend.
  await command('bash', ['src/server/migrations/sqlx_migration_contract_test.sh'], { env });
  await validateWebArtifact(path.join(root, 'target/native-web'), root);
  const temp = await mkdtemp(path.join(os.tmpdir(), 'ohc-native-e2e-'));
  const suffix = randomUUID().replaceAll('-', '').slice(0, 12);
  const pg = `ohc-e2e-pg-${suffix}`, cache = `ohc-e2e-cache-${suffix}`;
  const processes = [], logs = [];
  const start = (binary, arguments_, name, environment) => {
    const output = createWriteStream(path.join(temp, name), { mode: 0o600 });
    logs.push(output);
    const child = spawn(binary, arguments_, { cwd: root, env: environment, shell: false, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.pipe(output, { end: false }); child.stderr.pipe(output, { end: false });
    child.once('error', (error) => output.write(`Unable to start ${name}: ${error.message}\n`));
    processes.push(child); return child;
  };
  const execution = new AbortController();
  const execute = (binary, argv, options = {}) => command(binary, argv, { ...options, signal: execution.signal });
  const interrupt = () => {
    execution.abort();
    for (const child of processes) child.kill('SIGTERM');
  };
  process.once('SIGINT', interrupt); process.once('SIGTERM', interrupt);
  try {
    await execute('docker', ['info'], { env, quiet: true });
    for (const image of [postgresImage, valkeyImage]) await execute('docker', ['pull', image], { env });
    await execute('docker', ['run', '-d', '--name', pg, '-p', '127.0.0.1:0:5432',
      '-e', 'POSTGRES_USER=ohc', '-e', 'POSTGRES_PASSWORD=ohc', '-e', 'POSTGRES_DB=ohc', postgresImage], { env });
    await execute('docker', ['run', '-d', '--name', cache, '-p', '127.0.0.1:0:6379', valkeyImage], { env });
    const mapped = async (name, port) => Number((await execute('docker', ['port', name, port], { env, quiet: true })).trim().split(':').at(-1));
    const pgPort = await mapped(pg, '5432/tcp'), cachePort = await mapped(cache, '6379/tcp');
    let ready = false;
    for (let attempt = 0; attempt < 60; attempt++) {
      execution.signal.throwIfAborted();
      try {
        await execute('docker', ['exec', pg, 'pg_isready', '-U', 'ohc', '-d', 'ohc'], { env, quiet: true, timeoutMs: 5000 });
        await execute('docker', ['exec', pg, 'psql', '-v', 'ON_ERROR_STOP=1', '-U', 'ohc', '-d', 'ohc', '-c',
          "DO $$ BEGIN IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'ohc_bypassrls') THEN CREATE ROLE ohc_bypassrls NOLOGIN; END IF; END $$; GRANT ohc_bypassrls TO ohc;"], { env, quiet: true, timeoutMs: 5000 });
        ready = true;
        break;
      }
      catch { await delay(1000, undefined, { signal: execution.signal }); }
    }
    if (!ready) throw new Error('PostgreSQL test container did not become ready; no SQLite fallback is permitted');
    await execute('bash', ['deploy/tests/support/generate_test_tls.sh', temp], { env });
    const apiPort = await freePort(), grpcPort = await freePort(), webPort = await freePort();
    const apiOrigin = `http://127.0.0.1:${apiPort}`, webOrigin = `http://127.0.0.1:${webPort}`;
    Object.assign(env, {
      OMNISOLO_PORT: String(apiPort), OMNISOLO_GRPC_PORT: String(grpcPort),
      DATABASE_URL: `postgres://ohc:ohc@127.0.0.1:${pgPort}/ohc`,
      REDIS_URL: `redis://127.0.0.1:${cachePort}`, OMNISOLO_STANDALONE_MODE: 'false',
      OMNISOLO_ENV: 'test', OMNISOLO_DEFAULT_TENANT_ID: 'e2e-tenant',
      OMNISOLO_AGENT_AUTH_KEY: randomBytes(32).toString('hex'), OMNISOLO_AGENT_TOKEN: randomBytes(32).toString('hex'),
      JWT_SECRET: randomBytes(32).toString('hex'), OMNISOLO_BUILTIN_AGENT_BINARY: agent,
      OMNISOLO_GRPC_TLS_CERT_PATH: path.join(temp, 'server.crt'),
      OMNISOLO_GRPC_TLS_KEY_PATH: path.join(temp, 'server.key'),
      OMNISOLO_GRPC_CLIENT_CA_PATH: path.join(temp, 'ca.crt'),
      E2E_POSTGRES_CONTAINER: pg, API_BASE_URL: apiOrigin, BACKEND_URL: apiOrigin,
      OMNISOLO_BACKEND_URL: apiOrigin, OMNISOLO_API_URL: apiOrigin, BASE_URL: webOrigin,
      PLAYWRIGHT_BASE_URL: webOrigin,
      OMNISOLO_WEB_CANONICAL_ORIGIN: webOrigin, OMNISOLO_WEB_LOCAL_DEV: 'true',
      OMNISOLO_WEB_SESSION_KEY_ID: 'e2e-v1', OMNISOLO_WEB_SESSION_SECRET: randomBytes(32).toString('base64url'),
      PLAYWRIGHT_STORAGE_STATE: path.join(temp, 'browser-state.json'),
      OMNISOLO_E2E_SESSION_STATE_DIR: path.join(temp, 'actor-sessions'),
      OMNISOLO_LLM_CONFIG_PATH: path.join(temp, 'no-provider-config.json'),
      PLAYWRIGHT_TEST_DIR: './src', PLAYWRIGHT_LIST_REPORTER: '1',
      PLAYWRIGHT_OUTPUT_DIR: path.join(root, 'test-results/native'),
      PLAYWRIGHT_HTML_REPORT: path.join(root, 'playwright-report'), NEXT_TELEMETRY_DISABLED: '1',
    });
    const backend = start(server, [], 'server.log', env);
    await waitHttp(`${apiOrigin}/readyz`, backend, 120, execution.signal);
    await execute('docker', ['exec', '-i', pg, 'psql', '-v', 'ON_ERROR_STOP=1', '-U', 'ohc', '-d', 'ohc'], {
      env, input: await readFile(path.join(root, 'src/e2e/e2e-seed.sql')), quiet: true,
    });
    const frontend = start(process.execPath, [web], 'web.log', { ...env, PORT: String(webPort), HOSTNAME: '127.0.0.1', NODE_ENV: 'production' });
    await waitHttp(`${webOrigin}/login`, frontend, 120, execution.signal);
    // Execute exactly the complete/sharded selection checked by preflight.
    await command(process.execPath, [playwright, 'test', '--config', 'playwright.config.ts', ...args], {
      env, signal: execution.signal, timeoutMs: 24 * 60 * 1000,
    });
  } catch (error) {
    // The database contains only this run's synthetic seed. Its bounded error
    // tail makes migration/type failures diagnosable instead of a bare HTTP 503.
    try {
      console.error('Isolated PostgreSQL diagnostics:');
      await command('docker', ['logs', '--tail', '20', pg], { env });
    } catch { /* Startup may have failed before the test database existed. */ }
    for (const name of ['server.log', 'web.log']) {
      try {
        let tail = (await readFile(path.join(temp, name), 'utf8')).split('\n').slice(-35).join('\n');
        for (const [key, value] of Object.entries(env)) if (/SECRET|TOKEN|KEY/.test(key) && value) tail = tail.replaceAll(value, '<redacted>');
        console.error(`${name}:\n${tail}`);
      } catch { /* process may not have started */ }
    }
    throw error;
  } finally {
    for (const child of processes.reverse()) await stop(child);
    for (const log of logs) log.end();
    await command('docker', ['rm', '-f', pg, cache], { env, quiet: true }).catch(() => {});
    await rm(temp, { recursive: true, force: true });
    process.removeListener('SIGINT', interrupt); process.removeListener('SIGTERM', interrupt);
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  runNativeE2e().catch((error) => { console.error(error.message); process.exitCode = 1; });
}
