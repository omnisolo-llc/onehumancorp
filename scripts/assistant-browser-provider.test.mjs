import assert from 'node:assert/strict';
import test from 'node:test';
import { request } from 'node:http';
import { readFile } from 'node:fs/promises';
import { startAssistantProvider, assistantRequest, ASSISTANT_PROMPT, ASSISTANT_OUTPUT, ASSISTANT_MAXIMUM } from './assistant-browser-provider.mjs';
import { checkoutProcessEnvironment, startConfiguredAssistantFixture } from './checkout-browser-fixture.mjs';

const registration = { runId: '012345abcdef', tenantId: 'e2e-growth-01234567-89ab-cdef-0123-456789abcdef' };
async function fixture(t) {
  const provider = await startAssistantProvider(registration);
  t.after(() => provider.close());
  const send = (body = assistantRequest(), headers = {}, path = '/chat/completions') => new Promise((resolve, reject) => {
    const req = request(provider.environment.OMNISOLO_LLM_ENDPOINT + path, {
      method: 'POST', agent: false, headers: { 'content-type': 'application/json',
        authorization: `Bearer ${provider.environment.OMNISOLO_LLM_API_KEY}`, ...headers },
    }, response => {
      const chunks = []; response.on('data', chunk => chunks.push(chunk));
      response.on('end', () => resolve({ status: response.statusCode, body: JSON.parse(Buffer.concat(chunks).toString()) }));
    });
    req.on('error', reject); req.end(typeof body === 'string' ? body : JSON.stringify(body));
  });
  return { provider, send };
}

test('owned model holds one exact request until explicit release and returns measured text', async t => {
  const { provider, send } = await fixture(t);
  const pending = send();
  await provider.waitForRequest();
  assert.deepEqual(provider.evidence().requests, [{ method: 'POST', path: '/v1/chat/completions', body: assistantRequest(), status: null }]);
  assert.equal((await send()).status, 409);
  provider.release();
  const response = await pending;
  assert.equal(response.status, 200);
  assert.equal(response.body.choices[0].message.content, ASSISTANT_OUTPUT);
  assert.equal(response.body.choices[0].finish_reason, 'stop');
  assert.deepEqual(response.body.usage, { prompt_tokens: 100, completion_tokens: 30, total_tokens: 130 });
  assert.equal(response.body.id, provider.receiptId);
  assert.equal(provider.evidence().requests.length, 2);
  await provider.close();
  await assert.rejects(send());
});

test('owned model rejects altered text, tools, credentials, host, path and malformed bodies', async t => {
  const { provider, send } = await fixture(t);
  for (const body of [{ ...assistantRequest(), model: 'external-model' }, { ...assistantRequest(), tools: [] },
    { ...assistantRequest(), messages: [{ role: 'user', content: 'Different text' }] },
    { ...assistantRequest(), max_tokens: 1000 }, { ...assistantRequest(), extra: true }, '{invalid']) {
    assert.equal((await send(body)).status, 400);
  }
  assert.equal((await send(assistantRequest(), { authorization: 'Bearer foreign' })).status, 401);
  assert.equal((await send(assistantRequest(), { host: 'api.openai.com' })).status, 400);
  assert.equal((await send(assistantRequest(), {}, '/embeddings')).status, 404);
  assert.equal((await send('x'.repeat(16385))).status, 413);
  assert.equal(provider.evidence().requests.filter(row => row.status === 200 || row.status === null).length, 0);
});

test('configuration is local, tenant-specific and positively priced; inherited credentials are discarded', async t => {
  const { provider } = await fixture(t);
  const env = provider.environment;
  assert.match(env.OMNISOLO_LLM_ENDPOINT, /^http:\/\/127\.0\.0\.1:\d+\/v1$/);
  assert.equal(env.OMNISOLO_LLM_TENANT_ID, registration.tenantId);
  assert.equal(env.OMNISOLO_USAGE_PAYER, 'managed_api');
  assert.equal(env.OMNISOLO_USAGE_MAX_REQUEST_MICROS, '20000');
  const rate = JSON.parse(env.OMNISOLO_USAGE_RATE_CARDS)['openai-compatible/owned-browser-model'];
  assert.deepEqual(rate, { revision: 'owned-browser-v1', input_micros_per_million: 1000000,
    output_micros_per_million: 2000000, cached_input_micros_per_million: 1000000 });
  assert.equal(ASSISTANT_MAXIMUM, Buffer.byteLength(assistantRequest().messages[0].content) + Buffer.byteLength(ASSISTANT_PROMPT) + 4096 + 128 * 2);
  assert.ok(ASSISTANT_MAXIMUM > 160 && ASSISTANT_MAXIMUM < 20000);
  assert.deepEqual(checkoutProcessEnvironment({ OPENAI_API_KEY: 'foreign', OMNISOLO_LLM_API_KEY: 'foreign',
    OMNISOLO_LLM_ENDPOINT: 'https://external.invalid', OMNISOLO_USAGE_PAYER: 'byok_api', NODE_OPTIONS: 'bad', PATH: '/usr/bin' }), { PATH: '/usr/bin' });
  await assert.rejects(startAssistantProvider({ ...registration, tenantId: 'production' }), /owned/);
  await assert.rejects(startAssistantProvider({ ...registration, runId: 'unowned' }), /owned/);
  await assert.rejects(startConfiguredAssistantFixture({ tenantId: registration.tenantId, environment: {} }), /runner runtime proof/);
});

test('expected wire request binds the production text-only adapter policy', async () => {
  const source = await readFile('src/agents/builtin/tenant_analysis.rs', 'utf8');
  assert.ok(source.includes(assistantRequest().messages[0].content));
  assert.match(source, /4096/);
  assert.equal(assistantRequest().messages[1].content, ASSISTANT_PROMPT);
  assert.equal(assistantRequest().max_tokens, 128);
  assert.equal(assistantRequest().temperature, 0);
  assert.equal(Object.hasOwn(assistantRequest(), 'tools'), false);
});
