#!/usr/bin/env node
/**
 * run-playwright.mjs — Bazel js_test entry point for all Playwright E2E tests.
 *
 * Design goals:
 *   • No graceful skip — the test FAILS if the stack cannot be started.
 *   • Start the container stack ONCE across all concurrent Bazel test shards
 *     using an exclusive file-lock at /tmp/ohc-e2e-stack.lock so resources
 *     are not wasted spinning up duplicate stacks.
 *   • Prefer lighter CRI: tries podman compose first, falls back to
 *     docker compose.
 *   • Runs ALL *.spec.ts files discovered by Playwright (no per-spec targeting)
 *     so the CRI warm-up cost is paid once for the entire suite.
 *
 * Bazel usage (js_test):
 *   entry_point = "run-playwright.mjs"
 *   tags        = ["local", "requires-docker"]
 */

import { spawnSync, spawn, execFileSync } from 'node:child_process';
import {
  existsSync,
  mkdirSync,
  readdirSync,
  openSync,
  closeSync,
  unlinkSync,
  realpathSync,
  writeFileSync,
} from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

// ── Workspace root (works for both local dev and Bazel local tests) ───────────

/**
 * Resolve the monorepo workspace root by walking up from this script's real
 * location on disk.  The script lives at <ws>/srcs/tests/e2e/run-playwright.mjs,
 * so three levels up is the workspace root.
 *
 * With tags = ["local"], Bazel runs without a sandbox and the script's realpath
 * points back into the source tree, making this reliable without needing to
 * declare non-JS files in `data` (which would require copy_to_bin).
 */
function findWorkspaceRoot() {
  let scriptPath = import.meta.url
    ? fileURLToPath(import.meta.url)
    : resolve(__dirname, 'run-playwright.mjs');
  try { scriptPath = realpathSync(scriptPath); } catch { /* use as-is */ }
  // srcs/tests/e2e/run-playwright.mjs → up 3 dirs = workspace root
  return resolve(dirname(scriptPath), '..', '..', '..');
}

const WS = findWorkspaceRoot();

// ── Bazel runfiles root (for declared data deps, e.g. node_modules) ──────────

function runfilesRoot() {
  if (process.env.JS_BINARY__RUNFILES) return process.env.JS_BINARY__RUNFILES;
  const srcdir = process.env.TEST_SRCDIR;
  if (srcdir) {
    const ws = process.env.TEST_WORKSPACE ?? '_main';
    const candidate = resolve(srcdir, ws);
    return existsSync(candidate) ? candidate : srcdir;
  }
  return WS;
}

const RF = runfilesRoot();

/** Resolve a declared-data path from the Bazel runfiles tree. */
function rf(...parts) {
  const direct = resolve(RF, ...parts);
  if (existsSync(direct)) return direct;
  const bzlmod = resolve(RF, '_main', ...parts);
  if (existsSync(bzlmod)) return bzlmod;
  // When run directly (not under `bazel test`), aspect_rules_js places
  // node_modules under bazel-bin rather than the workspace root.
  const bazelbinFallback = resolve(WS, 'bazel-bin', ...parts);
  if (existsSync(bazelbinFallback)) return bazelbinFallback;
  return direct;
}

// ── Configuration ─────────────────────────────────────────────────────────────

const BASE_URL   = process.env.OHC_E2E_BASE_URL   ?? 'http://localhost:8080';
const BROWSERS   = process.env.PLAYWRIGHT_BROWSERS_PATH ?? '/tmp/ohc-playwright-browsers';
const STACK_LOCK = '/tmp/ohc-e2e-stack.lock';

// Compose file is resolved via the real workspace path (not runfiles) so that
// the volume mounts inside the file resolve correctly relative to deploy/.
const COMPOSE_SRC = resolve(WS, 'deploy', 'docker-compose.e2e.yml');

// E2E source dir: prefer runfiles, fall back to the real source path.
const E2E_DIR = (() => {
  const fromRf = rf('srcs', 'tests', 'e2e');
  if (existsSync(resolve(fromRf, 'playwright.config.ts'))) return fromRf;
  return resolve(WS, 'srcs', 'tests', 'e2e');
})();
const PW_CLI  = rf('node_modules', '@playwright', 'test', 'cli.js');

// ── CRI / compose detection ───────────────────────────────────────────────────

/**
 * Probe available container runtimes in order of preference:
 *   1. podman compose  — daemonless, rootless, lighter resource footprint
 *   2. docker compose  — widely available fallback
 *
 * Each candidate is probed by running `compose version` to confirm the
 * compose sub-command is operational (not just that the binary exists).
 *
 * Returns an object { bin, composeCmd } where composeCmd is an argv prefix
 * for the compose sub-command (e.g. ['podman', 'compose'] or ['docker', 'compose']).
 */
function detectRuntime() {
  const candidates = [
    { bin: 'podman', composeCmd: ['podman', 'compose'] },
    { bin: 'docker', composeCmd: ['docker', 'compose'] },
  ];

  for (const rt of candidates) {
    try {
      const r = spawnSync(rt.composeCmd[0], [...rt.composeCmd.slice(1), 'version'], {
        encoding: 'utf8',
        timeout: 10_000,
        env: { ...process.env },
      });
      if (r.status !== 0 || r.error) continue;

      // Skip podman compose when it delegates to an external Docker Compose
      // provider: in that mode it sets DOCKER_HOST to the Podman socket which
      // breaks docker-compose even though Docker is running separately.
      const delegating =
        (r.stderr ?? '').includes('external compose provider') ||
        (r.stdout ?? '').includes('external compose provider');
      if (delegating) {
        console.log(`[e2e] Skipping ${rt.bin} compose (delegates to external provider; falling back)`);
        continue;
      }

      console.log(`[e2e] Container runtime: ${rt.bin} compose ✓`);
      return rt;
    } catch { /* try next */ }
  }

  throw new Error(
    '[e2e] No container runtime found (podman compose / docker compose). ' +
    'Install podman (preferred) or docker and ensure it is on PATH.',
  );
}

// ── Server health probe ───────────────────────────────────────────────────────

async function probeServer(url, timeoutMs = 4_000) {
  for (const path of ['/health', '/healthz', '/api/health', '/']) {
    try {
      const ctrl = new AbortController();
      const tid = setTimeout(() => ctrl.abort(), timeoutMs);
      const r = await fetch(`${url}${path}`, { signal: ctrl.signal }).catch(() => null);
      clearTimeout(tid);
      if (r && (r.ok || r.status < 500)) return true;
    } catch { /* ignore */ }
  }
  return false;
}

async function waitForServer(url, maxAttempts = 72, intervalMs = 5_000) {
  process.stdout.write(`[e2e] Waiting for server at ${url} `);
  for (let i = 0; i < maxAttempts; i++) {
    if (await probeServer(url)) {
      process.stdout.write('✓\n');
      return;
    }
    process.stdout.write('.');
    await new Promise(r => setTimeout(r, intervalMs));
  }
  process.stdout.write('\n');
  throw new Error(
    `[e2e] Server at ${url} not ready after ${(maxAttempts * intervalMs) / 1_000}s. ` +
    'Ensure the container stack started correctly.',
  );
}

// ── OHC server binary location ────────────────────────────────────────────────

/**
 * Find the Bazel-built OHC server binary.
 * rules_go outputs the ELF binary at: bazel-bin/srcs/server/ohc_/ohc
 * WS/bazel-bin is a symlink that Bazel maintains in the workspace root.
 */
function findOhcBinary() {
  const candidates = [
    // From workspace bazel-bin symlink (reliable in Bazel local tests).
    resolve(WS, 'bazel-bin', 'srcs', 'server', 'ohc_', 'ohc'),
    // From runfiles (if //srcs/server:ohc is in data — future use).
    rf('srcs', 'server', 'ohc_', 'ohc'),
  ];
  for (const p of candidates) {
    if (existsSync(p)) return p;
  }
  throw new Error(
    '[e2e] OHC server binary not found. Run: bazelisk build //srcs/server:ohc',
  );
}

// ── Stack startup (with exclusive file-lock) ──────────────────────────────────

/**
 * Start infrastructure services (postgres, redis, ohc-core stub) via compose.
 * The server binary is NOT started here — see startServerProcess().
 */
function startInfra(rt, composeFile) {
  const projectDir = dirname(composeFile);
  const args = [
    ...rt.composeCmd,
    '--project-directory', projectDir,
    '-f', composeFile,
    'up', '-d', '--remove-orphans',
  ];
  console.log(`[e2e] Starting infra: ${args.join(' ')}`);
  execFileSync(args[0], args.slice(1), { stdio: 'inherit', timeout: 300_000 });
}

/**
 * Wait until postgres is accepting connections (healthcheck passes).
 * We poll `docker/podman compose ps` until postgres shows "(healthy)".
 */
async function waitForInfra(rt, composeFile, maxAttempts = 60, intervalMs = 5_000) {
  const projectDir = dirname(composeFile);
  process.stdout.write('[e2e] Waiting for postgres+redis to be healthy ');
  for (let i = 0; i < maxAttempts; i++) {
    try {
      const result = spawnSync(
        rt.composeCmd[0],
        [...rt.composeCmd.slice(1), '--project-directory', projectDir, '-f', composeFile, 'ps', '--format', 'json'],
        { encoding: 'utf8', timeout: 15_000 },
      );
      const out = (result.stdout ?? '') + (result.stderr ?? '');
      // JSON output from compose: each service has "Health" or "State".
      // "healthy" means the healthcheck passed.
      const pgReady = out.includes('"healthy"') || out.includes('healthy');
      if (pgReady) {
        process.stdout.write('✓\n');
        return;
      }
    } catch { /* ignore parse errors */ }
    process.stdout.write('.');
    await new Promise(r => setTimeout(r, intervalMs));
  }
  process.stdout.write('\n');
  throw new Error('[e2e] Infra services did not become healthy in time.');
}

/**
 * Start the OHC Go server binary as a background process.
 * Returns the child process object.
 */
function startServerProcess(ohcBin) {
  const env = {
    ...process.env,
    // Postgres mode — no SQLite fallback.
    DATABASE_URL: 'postgres://ohc:ohc@localhost:5432/ohc?sslmode=disable',
    REDIS_ADDR: 'localhost:6379',
    OHC_CORE_URL: 'http://localhost:18789',
    CHATWOOT_ENABLED: 'false',
    OHC_MULTITENANT: 'false',
    OHC_HEADLESS: 'false',
    LOG_LEVEL: 'info',
    PORT: '8080',
  };

  console.log(`[e2e] Starting OHC server: ${ohcBin}`);
  const proc = spawn(ohcBin, [], { detached: false, stdio: 'pipe', env });
  proc.stdout.on('data', d => process.stdout.write(`[ohc] ${d}`));
  proc.stderr.on('data', d => process.stderr.write(`[ohc] ${d}`));
  proc.on('exit', code => {
    if (code !== null && code !== 0) {
      console.error(`[e2e] OHC server exited with code ${code}`);
    }
  });
  return proc;
}

/**
 * Ensure the full E2E stack (infra + server) is running.
 * Uses an exclusive file-lock so that when multiple Bazel test processes start
 * concurrently, only ONE starts the stack; the others wait until the server is
 * reachable.
 */
async function ensureStackRunning(rt, ohcBin) {
  // Fast path: server already up.
  if (await probeServer(BASE_URL, 3_000)) {
    console.log(`[e2e] Stack already running at ${BASE_URL} ✓`);
    return null; // no server proc to manage — started by another process
  }

  // Try to acquire the startup lock (O_CREAT | O_EXCL — atomic on Linux tmpfs).
  let lockFd = null;
  const lockDeadline = Date.now() + 120_000;

  while (Date.now() < lockDeadline) {
    if (await probeServer(BASE_URL, 2_000)) {
      console.log(`[e2e] Stack came up while waiting for lock ✓`);
      return null;
    }
    try {
      lockFd = openSync(STACK_LOCK, 'wx');
      writeFileSync(lockFd, String(process.pid));
      break;
    } catch (e) {
      if (e.code !== 'EEXIST') throw e;
      await new Promise(r => setTimeout(r, 1_000));
    }
  }

  if (lockFd === null) {
    throw new Error(
      `[e2e] Could not acquire stack startup lock at ${STACK_LOCK} within 120s.`,
    );
  }

  let serverProc = null;
  try {
    // 1. Start infra (postgres, redis) — idempotent.
    startInfra(rt, COMPOSE_SRC);

    // 2. Wait for infra healthchecks (postgres needs to accept connections).
    await waitForInfra(rt, COMPOSE_SRC);

    // 3. Start the Go server binary directly.
    serverProc = startServerProcess(ohcBin);

    // 4. Wait for HTTP server to respond.
    await waitForServer(BASE_URL);

    // 5. Bootstrap the admin account (idempotent — server handles duplicate).
    await bootstrapAdmin(BASE_URL);
  } finally {
    try { closeSync(lockFd); } catch { /* ignore */ }
    try { unlinkSync(STACK_LOCK); } catch { /* ignore */ }
  }

  return serverProc;
}

// ── Admin bootstrap ───────────────────────────────────────────────────────────

/**
 * Create the initial admin account via the setup API.
 * The endpoint returns 409 if the admin already exists — that is treated as
 * success.
 */
async function bootstrapAdmin(baseUrl) {
  const username = process.env.OHC_E2E_ADMIN_USER ?? 'admin';
  const password = process.env.OHC_E2E_ADMIN_PASS ?? 'admin';
  const deadline = Date.now() + 60_000;

  while (Date.now() < deadline) {
    try {
      const ctrl = new AbortController();
      const tid  = setTimeout(() => ctrl.abort(), 8_000);
      const r = await fetch(`${baseUrl}/api/setup/admin`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password, role: 'admin' }),
        signal: ctrl.signal,
      }).catch(() => null);
      clearTimeout(tid);
      if (r && (r.ok || r.status === 409 || r.status === 201)) {
        console.log(`[e2e] Admin account ready ✓ (HTTP ${r?.status})`);
        return;
      }
    } catch { /* retry */ }
    await new Promise(r => setTimeout(r, 3_000));
  }
  // Non-fatal: the tests themselves will surface auth failures.
  console.warn('[e2e] Admin bootstrap did not confirm within 60 s — proceeding anyway.');
}

// ── Playwright browser install ────────────────────────────────────────────────

function ensureBrowsers(cli) {
  const installed =
    existsSync(BROWSERS) &&
    readdirSync(BROWSERS).some(d => d.startsWith('chromium-'));
  if (installed) return;

  console.log('[e2e] Installing Playwright Chromium …');
  mkdirSync(BROWSERS, { recursive: true });
  const r = spawnSync(
    process.execPath,
    [cli, 'install', 'chromium', '--with-deps'],
    { stdio: 'inherit', env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: BROWSERS } },
  );
  if (r.status !== 0) {
    throw new Error('[e2e] Playwright browser installation failed.');
  }
}

// ── Entry point ───────────────────────────────────────────────────────────────

async function main() {
  // 1. Detect container runtime (podman preferred, docker fallback).
  const rt = detectRuntime();

  // 2. Find the Bazel-built OHC server binary.
  const ohcBin = findOhcBinary();
  console.log(`[e2e] OHC binary: ${ohcBin}`);

  // 3. Ensure infra + server are running (exactly once, shared across processes).
  const serverProc = await ensureStackRunning(rt, ohcBin);

  // 4. Verify Playwright CLI is available.
  if (!existsSync(PW_CLI)) {
    throw new Error(
      `[e2e] Playwright CLI not found at: ${PW_CLI}\n` +
      `      Runfiles root: ${RF}\n` +
      '      Ensure //:node_modules/@playwright/test is in the data deps.',
    );
  }

  // 5. Install browsers if absent.
  ensureBrowsers(PW_CLI);

  // 6. Run ALL specs discovered by playwright.config.ts.
  const config = resolve(E2E_DIR, 'playwright.config.ts');
  console.log('[e2e] Launching Playwright test suite …');
  const r = spawnSync(
    process.execPath,
    [PW_CLI, 'test', '--config', config],
    {
      stdio: 'inherit',
      cwd: E2E_DIR,
      env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: BROWSERS },
    },
  );

  // 7. Clean up the server process if we started it.
  if (serverProc) {
    try { serverProc.kill('SIGTERM'); } catch { /* ignore */ }
  }

  process.exit(r.status ?? 1);
}

main().catch(e => {
  console.error('[e2e] Fatal:', e.message ?? e);
  process.exit(1);
});
