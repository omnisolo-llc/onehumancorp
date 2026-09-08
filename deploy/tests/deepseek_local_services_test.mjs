import assert from 'node:assert/strict';
import {localServiceSpawnSpec} from '../harness/deepseek/local-services-env.mjs';

const spec = {command: 'bash', env: {TERM: 'dumb', OMNISOLO_LOCAL_SERVICE_TOKEN: 'untrusted'}};
const environment = {
  OMNISOLO_LOCAL_SERVICE_URL: 'http://127.0.0.1:12345/v1',
  OMNISOLO_LOCAL_SERVICE_TOKEN: 'issued-lease',
  OPENAI_API_KEY: 'provider-secret',
  OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN: 'control-secret',
};
const actual = localServiceSpawnSpec(spec, environment);
assert.equal(actual.env.OMNISOLO_LOCAL_SERVICE_TOKEN, 'issued-lease');
assert.equal(actual.env.OMNISOLO_LOCAL_SERVICE_URL, environment.OMNISOLO_LOCAL_SERVICE_URL);
assert.equal(actual.env.TERM, 'dumb');
assert.equal(spec.env.OMNISOLO_LOCAL_SERVICE_TOKEN, 'untrusted');
assert.equal(actual.env.OPENAI_API_KEY, undefined);
assert.equal(actual.env.OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN, undefined);
assert.equal(localServiceSpawnSpec(spec, {}), spec);
assert.throws(() => localServiceSpawnSpec(spec, {OMNISOLO_LOCAL_SERVICE_TOKEN: 'lease'}));
for (const url of ['https://example.com/v1', 'http://127.0.0.1:123/v1?key=x', 'http://127.0.0.1:123/other']) {
  assert.throws(() => localServiceSpawnSpec(spec, {...environment, OMNISOLO_LOCAL_SERVICE_URL: url}));
}
console.log('DeepSeek explicit service environment contract passed');
