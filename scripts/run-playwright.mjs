// run-playwright.mjs - Orchestrates E2E tests
import { spawn } from 'child_process';
import { setTimeout } from 'timers/promises';
import * as path from 'path';
import * as fs from 'fs';

const ROOT = path.resolve(fs.realpathSync('.'));

function loadDotEnv() {
  const envPath = path.join(ROOT, '.env');
  if (!fs.existsSync(envPath)) return;

  const lines = fs.readFileSync(envPath, 'utf8').split(/\r?\n/);
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) continue;
    const index = trimmed.indexOf('=');
    if (index <= 0) continue;
    const key = trimmed.slice(0, index).trim();
    let value = trimmed.slice(index + 1).trim();
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    if (!process.env[key]) {
      process.env[key] = value;
    }
  }
}

function resolveExistingPath(...candidates) {
  for (const candidate of candidates) {
    if (candidate && fs.existsSync(candidate)) {
      return candidate;
    }
  }
  return '';
}

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
  loadDotEnv();

  console.log('[run-playwright] Starting infrastructure...');
  if (process.env.E2E_SKIP_DOCKER !== 'true') {
    await runCommand('docker', ['compose', '-f', 'deploy/docker-compose.e2e.yml', 'up', '-d']);
  }

  console.log('[run-playwright] Server already built in outer execution');

  const serverBin = process.env.SERVER_BIN || path.join(ROOT, 'bazel-bin/src/server/server');
  const agentBin = resolveExistingPath(
    process.env.OHC_BUILTIN_AGENT_BINARY,
    process.env.AGENT_BIN,
    path.join(ROOT, 'bazel-bin/src/agents/builtin/ohc-builtin-agent'),
    path.join(ROOT, 'src/agents/builtin/ohc-builtin-agent'),
  );
  if (agentBin) {
    process.env.OHC_BUILTIN_AGENT_BINARY = agentBin;
  } else if (process.env.OHC_BUILTIN_AGENT_BINARY && !fs.existsSync(process.env.OHC_BUILTIN_AGENT_BINARY)) {
    delete process.env.OHC_BUILTIN_AGENT_BINARY;
  }
  if (process.env.MINIMAX_API_KEY) {
    process.env.OHC_LLM_PROVIDER = process.env.OHC_LLM_PROVIDER || 'minimax';
    process.env.OHC_LLM_MODEL = process.env.OHC_LLM_MODEL || 'MiniMax-M3';
    process.env.MINIMAX_MODEL = process.env.MINIMAX_MODEL || 'MiniMax-M3';
  }
  process.env.OHC_AGENT_TASK_TIMEOUT_SECS = process.env.OHC_AGENT_TASK_TIMEOUT_SECS || '240';
  process.env.OHC_LLM_TIMEOUT_SECS = process.env.OHC_LLM_TIMEOUT_SECS || '180';

  console.log(`[run-playwright] Starting server at ${serverBin}...`);
  const server = spawn(serverBin, [], {
    cwd: ROOT,
    stdio: 'inherit',
    env: {
      ...process.env,
      DATABASE_URL: process.env.DATABASE_URL ?? 'postgres://ohc:ohc@localhost:5432/ohc',
      REDIS_URL: process.env.REDIS_URL ?? 'redis://localhost:6379',
      OHC_DEFAULT_TENANT_ID: process.env.OHC_DEFAULT_TENANT_ID ?? 'e2e-tenant',
      OHC_AGENT_AUTH_DISABLED: 'true',
      OHC_ENV: 'development',
    },
  });

  const appReady = await waitForPort(Number(process.env.OHC_PORT ?? 18789), 60);
  if (!appReady) {
    server.kill();
    throw new Error('App server did not become ready.');
  }

  try {
    const args = process.argv.slice(2);
    await runCommand('npx', ['playwright', 'test', ...args]);
  } finally {
    server.kill();
    if (process.env.E2E_SKIP_DOCKER !== 'true') {
      await runCommand('docker', ['compose', '-f', 'deploy/docker-compose.e2e.yml', 'down']);
    }
  }

  console.log('[run-playwright] Done');
}

async function runCommand(command, args) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: ROOT,
      stdio: 'inherit',
      env: process.env,
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
