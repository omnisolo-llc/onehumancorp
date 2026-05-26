// run-playwright.mjs - Orchestrates E2E tests
import { spawn } from 'child_process';
import { setTimeout } from 'timers/promises';
import * as path from 'path';
import * as fs from 'fs';

const ROOT = path.resolve(fs.realpathSync('.'));

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

async function main() {
  console.log('[run-playwright] Starting infrastructure...');
  if (process.env.E2E_SKIP_DOCKER !== 'true') {
    await runCommand('docker', ['compose', '-f', 'deploy/docker-compose.e2e.yml', 'up', '-d']);
  }

  console.log('[run-playwright] Server already built in outer execution');

  const serverBin = path.join(ROOT, 'bazel-bin/src/server/server');
  console.log('[run-playwright] Starting server...');
  const server = spawn(serverBin, [], {
    cwd: ROOT,
    stdio: 'inherit',
    env: {
      ...process.env,
      DATABASE_URL: process.env.DATABASE_URL ?? 'postgres://ohc:ohc@localhost:5432/ohc',
      REDIS_URL: process.env.REDIS_URL ?? 'redis://localhost:6379',
      OHC_DEFAULT_TENANT_ID: process.env.OHC_DEFAULT_TENANT_ID ?? 'e2e-tenant',
    },
  });

  const appReady = await waitForPort(Number(process.env.OHC_PORT ?? 18789), 60);
  if (!appReady) {
    server.kill();
    throw new Error('App server did not become ready.');
  }

  try {
    // Run npx playwright test with a shard-specific cache to avoid ENOTEMPTY race conditions
    // across parallel bazel test executions.
    const npmCache = path.join(process.env.TEST_TMPDIR || '/tmp', 'npm-cache');
    await runCommand('npx', ['--yes', 'playwright', 'test'], { npm_config_cache: npmCache });
  } finally {
    server.kill();
    if (process.env.E2E_SKIP_DOCKER !== 'true') {
      await runCommand('docker', ['compose', '-f', 'deploy/docker-compose.e2e.yml', 'down']);
    }
  }

  console.log('[run-playwright] Done');
}

async function runCommand(command, args, envOverrides = {}) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: ROOT,
      stdio: 'inherit',
      env: { ...process.env, ...envOverrides },
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
