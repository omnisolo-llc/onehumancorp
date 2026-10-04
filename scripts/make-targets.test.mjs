import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtemp, mkdir, copyFile, writeFile, readFile, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { verifyNativeBinaryProof } from './native-binary-proof.mjs';

async function runMake(target, fail = '', { cargoTarget = 'target', fault = '' } = {}) {
  const dir = await mkdtemp(path.join(os.tmpdir(), 'ohc-make-'));
  try {
    await copyFile(new URL('../Makefile', import.meta.url), path.join(dir, 'Makefile'));
    await mkdir(path.join(dir, 'scripts'));
    // Exercise the actual source/binary proof CLI. Only expensive compiler/npm
    // commands are recorded substitutes; their outputs are explicitly synthetic.
    await copyFile(new URL('./native-binary-proof.mjs', import.meta.url), path.join(dir, 'scripts/native-binary-proof.mjs'));
    await writeFile(path.join(dir, '.gitignore'), '/commands\n/target/\n/owned-build/\n');
    await writeFile(path.join(dir, 'fixture-source.rs'), '// synthetic command-contract source\n');
    execFileSync('git', ['init', '--quiet', dir]);
    const stub = path.join(dir, 'command.cjs');
    await writeFile(stub, `const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const command = process.argv.slice(2).join(' ');
fs.appendFileSync(process.env.COMMAND_LOG, command + '\\n');
if (process.env.FAIL_COMMAND && command.includes(process.env.FAIL_COMMAND)) process.exit(7);
const output = path.join(process.env.CARGO_TARGET_DIR, 'debug');
if (command.startsWith('cargo build ')) {
  const snapshot = JSON.parse(fs.readFileSync(path.join(output, 'native-source-snapshot.json'), 'utf8'));
  assert.match(snapshot.sourceSha256, /^[a-f0-9]{64}$/, 'source proof must precede the compiler');
  assert.equal(fs.existsSync(path.join(output, 'native-binary-proof.json')), false, 'successful build proof must not precede the compiler');
  fs.mkdirSync(output, { recursive: true });
  fs.writeFileSync(path.join(output, 'server'), 'synthetic server output, never executed');
  if (process.env.MAKE_FIXTURE_FAULT !== 'missing-output') fs.writeFileSync(path.join(output, 'omnisolo-builtin-agent'), 'synthetic agent output, never executed');
  if (process.env.MAKE_FIXTURE_FAULT === 'source-drift') fs.appendFileSync('fixture-source.rs', '// changed during compiler invocation\\n');
}
if (command === 'npm run build:web' && fs.existsSync(path.join(output, 'native-source-snapshot.json'))) {
  const proof = JSON.parse(fs.readFileSync(path.join(output, 'native-binary-proof.json'), 'utf8'));
  assert.match(proof.server.sha256, /^[a-f0-9]{64}$/, 'native proof must be recorded before the later web build');
}
`);
    const runner = `${JSON.stringify(process.execPath)} ${JSON.stringify(stub)}`;
    const result = spawnSync('make', ['--no-print-directory', '-j8', target,
      `CARGO=${runner} cargo`, `NPM=${runner} npm`], {
      cwd: dir, encoding: 'utf8', timeout: 15000,
      env: { ...process.env, PATH: `${path.dirname(process.execPath)}${path.delimiter}${process.env.PATH ?? ''}`,
        MAKEFLAGS: '', CARGO_TARGET_DIR: cargoTarget, COMMAND_LOG: path.join(dir, 'commands'),
        FAIL_COMMAND: fail, MAKE_FIXTURE_FAULT: fault },
    });
    assert.ifError(result.error);
    const commands = (await readFile(path.join(dir, 'commands'), 'utf8')).trim().split('\n');
    const debug = path.join(dir, cargoTarget, 'debug');
    let proof;
    if (result.status === 0 && commands.some(command => command.startsWith('cargo build '))) {
      proof = await verifyNativeBinaryProof(path.join(debug, 'native-binary-proof.json'), {
        server: path.join(debug, 'server'), agent: path.join(debug, 'omnisolo-builtin-agent'),
      }, dir);
    }
    return { ...result, commands, proof };
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
}

test('make test covers Rust, Node, CLI, desktop UI, contracts and fresh real-stack E2E', async () => {
  const result = await runMake('test');
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.proof.sourceSha256, /^[a-f0-9]{64}$/);
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

for (const failed of ['desktop:prepare', 'cargo test', 'test:web', 'test:contracts', 'build:web', 'cargo build', 'test:e2e']) {
  test(`make test propagates ${failed} failure without reporting success`, async () => {
    const result = await runMake('test', failed);
    assert.notEqual(result.status, 0);
    assert.ok(result.commands.at(-1).includes(failed), result.commands.join('\n'));
  });
}

test('make build-e2e produces verified proof in the selected owned Cargo target directory', async () => {
  const result = await runMake('build-e2e', '', { cargoTarget: 'owned-build' });
  assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(result.commands, [
    'cargo build --locked -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins',
    'npm run build:web',
  ]);
  assert.match(result.proof.sourceSha256, /^[a-f0-9]{64}$/);
});

test('make test stops before compiler invocation when native source snapshot validation fails', async () => {
  const result = await runMake('test', '', { cargoTarget: '.' });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Native binary proof does not match/);
  assert.equal(result.commands.at(-1), 'npm run test:contracts');
  assert.equal(result.commands.some(command => command.startsWith('cargo build ')), false);
});

for (const fault of ['source-drift', 'missing-output']) {
  test(`make test stops before web/browser execution when native proof recording rejects ${fault}`, async () => {
    const result = await runMake('test', '', { fault });
    assert.notEqual(result.status, 0);
    assert.equal(result.commands.at(-1), 'cargo build --locked -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins');
    assert.equal(result.commands.includes('npm run test:e2e --'), false);
    assert.match(result.stderr, fault === 'source-drift' ? /Native binary proof does not match/ : /ENOENT.*omnisolo-builtin-agent/);
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
    'npm run typecheck:e2e',
    'npm --prefix src/cli run typecheck',
  ]);
});

test('make lint propagates linter failures', async () => {
  const result = await runMake('lint', 'lint:node');
  assert.notEqual(result.status, 0);
  assert.equal(result.commands.at(-1), 'npm run lint:node');
});

test('make lint propagates browser typecheck failures', async () => {
  const result = await runMake('lint', 'typecheck:e2e');
  assert.notEqual(result.status, 0);
  assert.equal(result.commands.at(-1), 'npm run typecheck:e2e');
});

test('required Node quality checks the complete Playwright TypeScript project', async () => {
  const packageJson = JSON.parse(await readFile(new URL('../package.json', import.meta.url), 'utf8'));
  assert.equal(packageJson.scripts['typecheck:e2e'], 'tsc --noEmit -p playwright.tsconfig.json');
  const workflow = await readFile(new URL('../.github/workflows/ci.yml', import.meta.url), 'utf8');
  const nodeJob = workflow.split('  native-node:')[1]?.split('  native-web:')[0];
  assert.ok(nodeJob, 'required Node quality job must exist');
  assert.match(nodeJob, /^ {10}npm run typecheck:e2e$/m);
  assert.doesNotMatch(nodeJob, /continue-on-error:\s*true|typecheck:e2e[^\n]+\|\|/);
});
