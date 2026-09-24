import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtemp, copyFile, writeFile, readFile, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';

async function runMake(target, fail = '') {
  const dir = await mkdtemp(path.join(os.tmpdir(), 'ohc-make-'));
  try {
    await copyFile(new URL('../Makefile', import.meta.url), path.join(dir, 'Makefile'));
    const stub = path.join(dir, 'command.cjs');
    await writeFile(stub, `const fs = require('node:fs');
const command = process.argv.slice(2).join(' ');
fs.appendFileSync(process.env.COMMAND_LOG, command + '\\n');
if (process.env.FAIL_COMMAND && command.includes(process.env.FAIL_COMMAND)) process.exit(7);
`);
    const runner = `${JSON.stringify(process.execPath)} ${JSON.stringify(stub)}`;
    const result = spawnSync('make', ['--no-print-directory', '-j8', target,
      `CARGO=${runner} cargo`, `NPM=${runner} npm`], {
      cwd: dir, encoding: 'utf8', timeout: 15000,
      env: { ...process.env, MAKEFLAGS: '', COMMAND_LOG: path.join(dir, 'commands'), FAIL_COMMAND: fail },
    });
    assert.ifError(result.error);
    const commands = (await readFile(path.join(dir, 'commands'), 'utf8')).trim().split('\n');
    return { ...result, commands };
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
}

test('make test covers Rust, Node, CLI, desktop UI, contracts and fresh real-stack E2E', async () => {
  const result = await runMake('test');
  assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(result.commands, [
    'npm run build:web',
    'npm run desktop:prepare',
    'cargo test --locked --workspace',
    'npm run test:scripts',
    'npm run test:web',
    'npm --prefix src/cli test',
    'npm run test:desktop-ui',
    'npm run test:contracts',
    'cargo build --locked -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins',
    'npm run build:web',
    'npm run test:e2e --',
  ]);
});

for (const failed of ['desktop:prepare', 'cargo test', 'test:web', 'test:contracts', 'build:web', 'test:e2e']) {
  test(`make test propagates ${failed} failure without reporting success`, async () => {
    const result = await runMake('test', failed);
    assert.notEqual(result.status, 0);
    assert.ok(result.commands.at(-1).includes(failed), result.commands.join('\n'));
  });
}

test('make lint checks Rust formatting/Clippy and JavaScript/TypeScript lint/typecheck', async () => {
  const result = await runMake('lint');
  assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(result.commands, [
    'npm run build:web',
    'npm run desktop:prepare',
    'cargo fmt --all -- --check',
    'cargo clippy --locked --workspace --all-targets -- -D warnings',
    'npm run lint:node',
    'npm run typecheck:web',
    'npm --prefix src/cli run typecheck',
  ]);
});

test('make lint propagates linter failures', async () => {
  const result = await runMake('lint', 'lint:node');
  assert.notEqual(result.status, 0);
  assert.equal(result.commands.at(-1), 'npm run lint:node');
});
