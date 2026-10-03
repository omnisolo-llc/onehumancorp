import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
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
