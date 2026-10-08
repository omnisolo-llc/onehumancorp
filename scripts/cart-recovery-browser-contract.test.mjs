import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { testEnvironment } from './native-e2e.mjs';

test('cart recovery is explicitly enabled only after the native database ownership guard', async () => {
  const source = await readFile(new URL('./native-e2e.mjs', import.meta.url), 'utf8');
  const ownedDatabase = source.indexOf('verifiedFixtureDatabaseUrl(env);');
  const backendStart = source.indexOf("const backend = start(server, [], 'server.log', env);");
  assert.ok(ownedDatabase >= 0 && backendStart > ownedDatabase);
  for (const [key, value] of [
    ['OMNISOLO_CART_RECOVERY_AGENT_QUEUE_ENABLED', 'true'],
    ['OMNISOLO_CART_RECOVERY_SCAN_INTERVAL_SECONDS', '30'],
    ['OMNISOLO_CART_RECOVERY_DISPATCH_INTERVAL_SECONDS', '5'],
  ]) {
    const configuration = source.indexOf(`${key}: '${value}'`);
    assert.ok(configuration > ownedDatabase && configuration < backendStart,
      `${key} must be fixed by the owned harness before starting the real worker`);
  }
  assert.deepEqual(testEnvironment({
    DATABASE_URL: 'postgres://production.invalid/live',
    OMNISOLO_CART_RECOVERY_AGENT_QUEUE_ENABLED: 'true',
    OMNISOLO_CART_RECOVERY_SCAN_INTERVAL_SECONDS: '1',
    SENDGRID_API_KEY: 'live-provider-secret', TWILIO_AUTH_TOKEN: 'live-provider-secret',
  }), {}, 'the harness must not inherit a database, worker policy, or delivery credentials');
});
