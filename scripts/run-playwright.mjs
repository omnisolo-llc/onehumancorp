// run-playwright.mjs - Orchestrates E2E tests
import { spawn } from 'child_process';
import { setTimeout } from 'timers/promises';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';

const ROOT = path.resolve(fs.realpathSync('.'));
const PLAYWRIGHT_ARGS = process.argv.slice(2);
const IN_BAZEL = !!process.env.TEST_TMPDIR;

async function waitForPort(port, maxAttempts = 30) {
  for (let i = 0; i < maxAttempts; i++) {
    try {
      const { default: net } = await import('net');
      await new Promise((resolve, reject) => {
        const s = net.connect(port, '127.0.0.1', () => {
          s.destroy();
          resolve();
        });
        s.on('error', reject);
        s.setTimeout(1000);
      });
      return true;
    } catch {
      await setTimeout(1000);
    }
  }
  return false;
}

async function freePort() {
  const { default: net } = await import('net');
  return await new Promise((resolve, reject) => {
    const server = net.createServer();
    server.unref();
    server.on('error', reject);
    server.listen(0, '127.0.0.1', () => {
      const address = server.address();
      server.close(() => {
        if (address && typeof address === 'object') {
          resolve(address.port);
        } else {
          reject(new Error('Could not allocate a local port.'));
        }
      });
    });
  });
}

function firstExisting(paths) {
  return paths.find((candidate) => candidate && fs.existsSync(candidate));
}

function serverBinary() {
  const serverBin = firstExisting([
    process.env.SERVER_BIN,
    path.join(ROOT, 'src/server/server'),
    path.join(ROOT, 'bazel-bin/src/server/server'),
  ]);

  if (!serverBin) {
    throw new Error('Server binary not found. Build //src/server:server before running Playwright.');
  }

  return serverBin;
}

function playwrightCommand() {
  const command = firstExisting([
    path.join(ROOT, 'node_modules/.bin/playwright'),
    path.join(ROOT, 'node_modules/@playwright/test/cli.js'),
  ]);

  if (command) {
    return command;
  }
  if (IN_BAZEL) {
    throw new Error('Declared Playwright binary not found in Bazel runfiles.');
  }
  return 'npx';
}

function playwrightArgs() {
  const command = playwrightCommand();
  if (path.basename(command) === 'npx') {
    return ['playwright', 'test', ...PLAYWRIGHT_ARGS];
  }
  return ['test', ...PLAYWRIGHT_ARGS];
}

function testRunId() {
  const raw = process.env.TEST_TARGET
    ? `${process.env.TEST_TARGET}-${process.env.TEST_SHARD_INDEX ?? '0'}-${process.pid}`
    : `local-${process.pid}`;
  return raw.toLowerCase().replace(/[^a-z0-9_-]+/g, '-').replace(/^-+|-+$/g, '');
}

function artifactDir() {
  return process.env.TEST_UNDECLARED_OUTPUTS_DIR
    ?? process.env.TEST_TMPDIR
    ?? path.join(os.tmpdir(), `ohc-playwright-${process.pid}`);
}

function materializedTestDir(outputs) {
  const testDir = path.join(outputs, 'materialized', 'src', 'e2e');
  fs.rmSync(testDir, { recursive: true, force: true });
  fs.mkdirSync(path.dirname(testDir), { recursive: true });
  fs.cpSync(path.join(ROOT, 'src', 'e2e'), testDir, {
    dereference: true,
    recursive: true,
  });
  return testDir;
}

function composeArgs(project, args) {
  return ['compose', '-p', project, '-f', 'deploy/docker-compose.e2e.yml', ...args];
}

async function main() {
  const runId = testRunId();
  const outputs = artifactDir();
  fs.mkdirSync(outputs, { recursive: true });

  const appPort = Number(process.env.OHC_PORT ?? await freePort());
  const grpcPort = Number(process.env.OHC_GRPC_PORT ?? await freePort());
  const postgresPort = Number(process.env.E2E_POSTGRES_PORT ?? await freePort());
  const redisPort = Number(process.env.E2E_REDIS_PORT ?? await freePort());
  const composeProject = process.env.E2E_COMPOSE_PROJECT ?? `ohc-e2e-${runId}`;
  const databaseUrl = process.env.DATABASE_URL ?? `postgres://ohc:ohc@127.0.0.1:${postgresPort}/ohc`;
  const redisUrl = process.env.REDIS_URL ?? `redis://127.0.0.1:${redisPort}`;
  const baseUrl = !IN_BAZEL && process.env.BASE_URL
    ? process.env.BASE_URL
    : `http://127.0.0.1:${appPort}`;
  const testDir = process.env.PLAYWRIGHT_TEST_DIR ?? (IN_BAZEL ? materializedTestDir(outputs) : '');
  const env = {
    ...process.env,
    BASE_URL: baseUrl,
    DATABASE_URL: databaseUrl,
    E2E_AUTH_DIR: process.env.E2E_AUTH_DIR ?? path.join(outputs, 'auth'),
    E2E_COMPOSE_PROJECT: composeProject,
    E2E_POSTGRES_PORT: String(postgresPort),
    E2E_REDIS_PORT: String(redisPort),
    OHC_DEFAULT_TENANT_ID: process.env.OHC_DEFAULT_TENANT_ID ?? 'e2e-tenant',
    OHC_GRPC_PORT: String(grpcPort),
    OHC_PORT: String(appPort),
    OHC_SQLITE_KEY: process.env.OHC_SQLITE_KEY ?? 'bazel-playwright-e2e-test-key',
    NODE_OPTIONS: [
      process.env.NODE_OPTIONS,
      IN_BAZEL ? '--preserve-symlinks --preserve-symlinks-main' : '',
    ].filter(Boolean).join(' '),
    NODE_PATH: [process.env.NODE_PATH, path.join(ROOT, 'node_modules')].filter(Boolean).join(path.delimiter),
    PLAYWRIGHT_LIST_REPORTER: process.env.PLAYWRIGHT_LIST_REPORTER ?? (IN_BAZEL ? '1' : ''),
    PLAYWRIGHT_OUTPUT_DIR: process.env.PLAYWRIGHT_OUTPUT_DIR ?? path.join(outputs, 'playwright'),
    PLAYWRIGHT_TEST_DIR: testDir,
    REDIS_URL: redisUrl,
  };

  console.log('[run-playwright] Starting infrastructure...');
  if (process.env.E2E_SKIP_DOCKER !== 'true') {
    await runCommand('docker', composeArgs(composeProject, ['up', '-d', '--wait']), env);
  }

  console.log('[run-playwright] Server already built in outer execution');

  const serverBin = serverBinary();
  console.log('[run-playwright] Starting server...');
  const server = spawn(serverBin, [], {
    cwd: ROOT,
    stdio: 'inherit',
    env,
  });

  const appReady = await waitForPort(appPort, 60);
  if (!appReady) {
    server.kill();
    throw new Error('App server did not become ready.');
  }

  try {
    await runCommand(playwrightCommand(), playwrightArgs(), env);
  } finally {
    server.kill();
    if (process.env.E2E_SKIP_DOCKER !== 'true') {
      await runCommand('docker', composeArgs(composeProject, ['down', '--volumes', '--remove-orphans']), env);
    }
  }

  console.log('[run-playwright] Done');
}

async function runCommand(command, args, env = process.env) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: ROOT,
      stdio: 'inherit',
      env,
      shell: false,
    });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
      }
    });
  });
}

main().catch((e) => {
  console.error('[run-playwright] Error:', e);
  process.exit(1);
});
