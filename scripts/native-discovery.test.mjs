import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { nativeBinaryPaths, testEnvironment } from './native-e2e.mjs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { lstat } from 'node:fs/promises';

test('browser runtime uses the same native Cargo target directory as the build', () => {
  const repository = path.resolve('fixture-repository');
  assert.deepEqual(nativeBinaryPaths(repository, {}, 'linux'), {
    server: path.join(repository, 'target/debug/server'),
    agent: path.join(repository, 'target/debug/omnisolo-builtin-agent'),
  });
  const custom = path.join(repository, 'isolated-build');
  assert.equal(nativeBinaryPaths(repository, { CARGO_TARGET_DIR: 'isolated-build' }, 'linux').server,
    path.join(custom, 'debug/server'));
  assert.equal(nativeBinaryPaths(repository, { CARGO_TARGET_DIR: custom }, 'win32').agent,
    path.join(custom, 'debug/omnisolo-builtin-agent.exe'));
  assert.throws(() => nativeBinaryPaths(repository, { CARGO_TARGET_DIR: '  ' }), /nonempty/);
  assert.equal(testEnvironment({ CARGO_TARGET_DIR: custom }).CARGO_TARGET_DIR, undefined,
    'build-only settings must not enter the isolated application environment');
});

test('obsolete Bazel launchers are absent, including dangling symlinks', async () => {
  for (const name of ['bazel', 'bazelisk']) {
    await assert.rejects(lstat(new URL(`../src/ui/next/bin/${name}`, import.meta.url)), { code: 'ENOENT' });
  }
});

test('complete browser discovery works without Docker, built binaries or provider credentials', () => {
  const result = spawnSync(process.execPath, [fileURLToPath(new URL('./run-playwright.mjs', import.meta.url)), '--list'], {
    cwd: fileURLToPath(new URL('..', import.meta.url)), env: testEnvironment(),
    encoding: 'utf8', timeout: 90000, maxBuffer: 8 * 1024 * 1024,
  });
  assert.ifError(result.error);
  assert.equal(result.status, 0, result.stderr + result.stdout.slice(-3000));
  assert.match(result.stdout, /Total:\s*[1-9]\d* tests? in [1-9]\d* files/);
  assert.doesNotMatch(result.stdout + result.stderr, /Requiring @playwright\/test second time|unknown parameter|not a function/);
});

test('CI keeps dependency caches separate from source-bound application outputs', async () => {
  const action = await readFile(new URL('../.github/actions/setup-native/action.yml', import.meta.url), 'utf8');
  assert.match(action, /prefix-key: ohc-native-dependencies-v1/);
  assert.match(action, /cache-workspace-crates: 'false'/);
  assert.match(action, /cache-bin: 'false'/);
  assert.match(action, /cache-on-failure: 'false'/);
  assert.doesNotMatch(action, /key: \$\{\{ hashFiles\('rust-toolchain/);
  const contracts = await readFile(new URL('./native-contracts.mjs', import.meta.url), 'utf8');
  const packageJson = JSON.parse(await readFile(new URL('../package.json', import.meta.url), 'utf8'));
  assert.equal(packageJson.scripts['test:scripts'], 'node --test scripts/*.test.mjs');
  assert.doesNotMatch(contracts, /process\.execPath, '--test'/);
});

test('backend CI captures real compiler timings without making the build optional', async () => {
  const workflow = await readFile(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const backend = workflow.split('  native-build:')[1]?.split('  native-test:')[0];
  assert.ok(backend);
  assert.match(backend, /cargo build --locked[^\n]+--bins --timings/);
  assert.match(backend, /name: native-backend-compiler-timings/);
  assert.match(backend, /path: target\/cargo-timings\/\*\.html/);
  assert.doesNotMatch(backend, /continue-on-error:\s*true|cargo build[^\n]+\|\|/);
});

test('CI preserves the measured compiler settings without disabling runtime checks', async () => {
  const action = await readFile(new URL('../.github/actions/setup-native/action.yml', import.meta.url), 'utf8');
  const manifest = await readFile(new URL('../Cargo.toml', import.meta.url), 'utf8');
  assert.match(action, /CARGO_INCREMENTAL=0/);
  assert.match(action, /RUNNER_OS.*Linux.*MALLOC_ARENA_MAX=2/);
  for (const name of ['dev', 'test']) {
    const section = manifest.split(`[profile.${name}]`)[1]?.split(/\n\[/)[0];
    assert.ok(section, `missing ${name} compiler profile`);
    assert.match(section, /debug\s*=\s*"line-tables-only"/);
    assert.match(section, /codegen-units\s*=\s*256/);
    if (name === 'test') {
      assert.match(section, /incremental\s*=\s*false/);
      assert.match(section, /debug-assertions\s*=\s*true/);
      assert.match(section, /overflow-checks\s*=\s*true/);
    }
  }
});

test('compiler and web caches are saved only by positively allowed trusted events', async () => {
  for (const filename of ['.github/actions/setup-native/action.yml', '.github/workflows/ci.yml']) {
    const source = await readFile(new URL(`../${filename}`, import.meta.url), 'utf8');
    const lines = source.split('\n');
    const policies = lines.filter((line, index) => line.includes('save-if:') ||
      (line.includes('if:') && lines[index - 1]?.includes('actions/cache/save@')));
    assert.ok(policies.length > 0, filename);
    for (const policy of policies) {
      assert.match(policy, /github\.event_name == 'push'/, filename);
      assert.match(policy, /github\.event_name == 'workflow_dispatch'/, filename);
      assert.match(policy, /github\.ref == 'refs\/heads\/main'/, filename);
      assert.doesNotMatch(policy, /event_name != 'pull_request'/, filename);
    }
  }
});

 test('native desktop no longer exposes plaintext-provider or fake-invite IPC handlers', async () => {
  const source = await readFile(new URL('../src/ui/tauri/src/lib.rs', import.meta.url), 'utf8');
  assert.doesNotMatch(source, /fn (save_ai_provider|test_ai_provider|generate_cloud_invite|generate_cloud_bridge_invite)\b/);
  assert.doesNotMatch(source, /ai-provider\.json|api_key:\s*Option|std::fs::write/);
  assert.match(source, /native_runtime::start_window/);
  assert.match(source, /pub mod offline/);
});

test('active native instructions do not invoke deleted Bazel tools', async () => {
  for (const filename of ['README.md', 'AGENTS.md', '.github/workflows/ci.yml', '.github/workflows/release.yml']) {
    const source = await readFile(new URL(`../${filename}`, import.meta.url), 'utf8');
    assert.doesNotMatch(source, /\bbazel(?:isk)?\s+(?:test|build|run|query|info)\b/, filename);
  }
});

test('CI shards use complete native browser spec discovery, not a smoke allowlist', async () => {
  const runner = await readFile(new URL('./native-e2e.mjs', import.meta.url), 'utf8');
  const config = await readFile(new URL('../playwright.config.ts', import.meta.url), 'utf8');
  const ci = await readFile(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  assert.match(runner, /PLAYWRIGHT_TEST_DIR:\s*['"]\.\/src['"]/);
  assert.match(config, /testMatch:\s*['"]\*\*\/\*\.spec\.ts['"]/);
  assert.doesNotMatch(runner, /const maintained\s*=|ciSelection\s*\?\s*\[/);
  assert.match(ci, /shard:\s*\[1, 2, 3, 4,[^\]]+32\]/);
  assert.match(ci, /test:e2e -- --ci --shard=/);
  assert.match(runner, /pass-with-no-tests/);
});

test('native E2E environment never inherits provider credentials or production databases', () => {
  const previous = { ...process.env };
  try {
    process.env.OPENAI_API_KEY = 'test-only-must-not-inherit';
    process.env.MINIMAX_API_KEY = 'test-only-must-not-inherit';
    process.env.ANTHROPIC_API_KEY = 'test-only-must-not-inherit';
    process.env.DATABASE_URL = 'postgres://production.invalid/not-a-real-credential';
    process.env.OMNISOLO_DATABASE_URL = process.env.DATABASE_URL;
    process.env.NODE_OPTIONS = '--require unsafe-inherited-hook';
    const env = testEnvironment();
    for (const name of ['OPENAI_API_KEY', 'MINIMAX_API_KEY', 'ANTHROPIC_API_KEY', 'DATABASE_URL', 'OMNISOLO_DATABASE_URL', 'NODE_OPTIONS']) {
      assert.notEqual(env[name], process.env[name], `${name} was inherited into the isolated test stack`);
    }
  } finally {
    for (const key of Object.keys(process.env)) if (!(key in previous)) delete process.env[key];
    Object.assign(process.env, previous);
  }
});
