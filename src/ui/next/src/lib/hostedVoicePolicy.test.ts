import { describe, expect, it } from 'vitest';
import { isHostedVoiceUnavailable, isVerifiedVoicePolicyDiagnostic, recordSmokeHttpResponse } from '../../../../e2e/support/hosted_voice_policy';
const valid = { url: 'http://127.0.0.1:1234/api/v1/settings/voice', origin: 'http://127.0.0.1:1234', method: 'GET', status: 403, body: { success: false, error: 'hosted_global_provisioning_unavailable', provisioning_available: false, provisioning_block_reason: 'hosted_global_provisioning_unavailable' } };
describe('explicit hosted voice policy in real-service smoke tests', () => {
  it('accepts the exact documented hosted GET denial', () => { expect(isHostedVoiceUnavailable(valid)).toBe(true); });
  it.each([
    { status: 401 }, { status: 500 }, { method: 'POST' },
    { url: 'https://foreign.test/api/v1/settings/voice' },
    { url: valid.url + '/provision' }, { url: valid.url + '?unexpected=1' },
    { body: { error: 'hosted_global_provisioning_unavailable' } },
    { body: { ...valid.body, success: true } }, { body: { ...valid.body, provisioning_available: true } },
    { body: { ...valid.body, error: 'authentication_required' } },
    { body: { ...valid.body, provisioning_block_reason: 'provider_not_configured' } },
    { body: { ...valid.body, phone_number: '+15550000000' } }, { body: null },
  ])('does not turn unrelated or contradictory failures into an exemption: %j', change => {
    expect(isHostedVoiceUnavailable({ ...valid, ...change })).toBe(false);
  });
  it('retains console errors until the exact response was independently verified', () => {
    const text = 'Failed to load resource: the server responded with a status of 403 (Forbidden)';
    expect(isVerifiedVoicePolicyDiagnostic(text, valid.url, new Set())).toBe(false);
    expect(isVerifiedVoicePolicyDiagnostic(text, valid.url, new Set([valid.url]))).toBe(true);
    expect(isVerifiedVoicePolicyDiagnostic('Unexpected application error', valid.url, new Set([valid.url]))).toBe(false);
    expect(isVerifiedVoicePolicyDiagnostic(text, valid.url + '/provision', new Set([valid.url]))).toBe(false);
    expect(isVerifiedVoicePolicyDiagnostic(text.replace('403', '500'), valid.url, new Set([valid.url]))).toBe(false);
  });
});

it('records an unrelated 4xx immediately without reading its unresolved body', async () => {
  let body!: ReadableStreamDefaultController<Uint8Array>;
  const response = new Response(new ReadableStream<Uint8Array>({ start(controller) { body = controller; } }), { status: 409 });
  let bodyReads = 0;
  const results = { failures: [] as string[], httpFailures: [] as string[], verifiedPolicyUrls: new Set<string>(), policyChecks: [] as Promise<void>[] };
  recordSmokeHttpResponse({ status: () => response.status, url: () => valid.origin + '/api/v1/other', request: () => ({ method: () => 'GET' }), json: () => { bodyReads += 1; return response.json(); } }, valid.origin, results);
  try {
    expect(results.httpFailures).toEqual(['409 ' + valid.origin + '/api/v1/other']);
    expect(bodyReads).toBe(0);
    expect(results.policyChecks).toEqual([]);
  } finally { body.error(new Error('fixture closed')); await Promise.all(results.policyChecks); }
});

it.each([true, false])('waits for the exact voice candidate body before classifying it (valid: %s)', async confirmed => {
  let body!: ReadableStreamDefaultController<Uint8Array>;
  const response = new Response(new ReadableStream<Uint8Array>({ start(controller) { body = controller; } }), { status: 403 });
  const results = { failures: [] as string[], httpFailures: [] as string[], verifiedPolicyUrls: new Set<string>(), policyChecks: [] as Promise<void>[] };
  recordSmokeHttpResponse({ status: () => response.status, url: () => valid.url, request: () => ({ method: () => 'GET' }), json: () => response.json() }, valid.origin, results);
  expect(results.policyChecks).toHaveLength(1); expect(results.verifiedPolicyUrls.size).toBe(0);
  body.enqueue(new TextEncoder().encode(JSON.stringify(confirmed ? valid.body : { error: 'authentication_required' }))); body.close();
  await Promise.all(results.policyChecks);
  expect([...results.verifiedPolicyUrls]).toEqual(confirmed ? [valid.url] : []);
  expect(results.httpFailures).toEqual(confirmed ? [] : ['403 ' + valid.url]);
});
