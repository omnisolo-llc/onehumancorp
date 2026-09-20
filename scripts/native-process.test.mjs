import test from 'node:test';
import assert from 'node:assert/strict';
import { runNativeCommand } from './native-process.mjs';

const run = (script, options = {}) => runNativeCommand(process.execPath, ['-e', script], { quiet: true, ...options });

test('native commands preserve argv, input and meaningful exit status', async () => {
  const result = await runNativeCommand(process.execPath, ['-e', 'process.stdout.write(process.argv[1])', 'spaces;$(not-a-shell)'], { quiet: true });
  assert.equal(result, 'spaces;$(not-a-shell)');
  assert.equal(await run('process.stdin.pipe(process.stdout)', { input: 'original input' }), 'original input');
  await assert.rejects(run('process.stderr.write("contract error");process.exitCode=7'), /failed \(7\): contract error/);
  await assert.rejects(runNativeCommand('ohc-nonexistent-command-for-test', [], { quiet: true }), /could not start/);
});

test('large command output retains a bounded diagnostic tail', async () => {
  const output = await run('process.stdout.write("x".repeat(2 * 1024 * 1024));process.stdout.write("\\nfinal-summary")');
  assert.ok(Buffer.byteLength(output) <= 128 * 1024);
  assert.ok(output.endsWith('final-summary'));
});

test('deadline expiration and cancellation cannot be reported as a pass', async () => {
  await assert.rejects(run('process.on("SIGTERM",()=>process.exit(0));setInterval(()=>{},1000)', { timeoutMs: 250 }), /deadline/);
  const controller = new AbortController();
  const promise = run('setInterval(()=>{},1000)', { signal: controller.signal });
  controller.abort();
  await assert.rejects(promise, /cancelled/);
  await assert.rejects(run('process.exit(0)', { signal: controller.signal }), /before launch/);
  await assert.rejects(run('', { timeoutMs: 0 }), TypeError);
});
