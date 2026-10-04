import { expect, test } from 'vitest';
import { isSonaRuntimeUnavailable, isVerifiedRuntimePolicyDiagnostic, runtimeUnavailableMessage } from '../../../../e2e/support/runtime_policy';
import { recordSmokeHttpResponse } from '../../../../e2e/support/hosted_voice_policy';
const valid = { url: 'http://127.0.0.1:1234/api/v1/sona', origin: 'http://127.0.0.1:1234', method: 'GET', status: 503, body: { error: runtimeUnavailableMessage } };

test.each([{}, { status: 500 }, { method: 'POST' }, { url: valid.url + '?x=1' }, { url: valid.url + '#x' }, { url: valid.origin + '/api/v1/other' }, { url: 'https://other.test/api/v1/sona' }, { body: null }, { body: { error: 'Agent outcome unavailable' } }, { body: { ...valid.body, patterns: [] } }])('only classifies the exact SONA missing-runtime boundary: %j', async change => {
  const input = { ...valid, ...change };
  const confirmed = Object.keys(change).length === 0;
  expect(isSonaRuntimeUnavailable(input)).toBe(confirmed);
  const results = { failures: [] as string[], httpFailures: [] as string[], verifiedPolicyUrls: new Set<string>(), policyChecks: [] as Promise<void>[] };
  recordSmokeHttpResponse({ status: () => input.status, url: () => input.url, request: () => ({ method: () => input.method }), json: async () => input.body }, input.origin, results);
  await Promise.all(results.policyChecks);
  expect([...results.verifiedPolicyUrls]).toEqual(confirmed ? [valid.url] : []);
  expect(results.failures).toEqual(confirmed ? [] : [`${input.status} ${input.url}`]);
});

test('only classifies the resource diagnostic after verifying the actual runtime prerequisite body', () => {
  const text = 'Failed to load resource: the server responded with a status of 503 (Service Unavailable)';
  expect(isVerifiedRuntimePolicyDiagnostic(text, valid.url, new Set())).toBe(false);
  expect(isVerifiedRuntimePolicyDiagnostic(text, valid.url, new Set([valid.url]))).toBe(true);
  expect(isVerifiedRuntimePolicyDiagnostic('Application failed', valid.url, new Set([valid.url]))).toBe(false);
  expect(isVerifiedRuntimePolicyDiagnostic(text.replace('503', '500'), valid.url, new Set([valid.url]))).toBe(false);
});

const goose = { ...valid, url: valid.origin + '/api/v1/agents/goose' };
test.each([
  { input: goose, confirmed: true },
  { input: { ...goose, url: 'https://other.invalid/api/v1/agents/goose' }, confirmed: false },
  { input: { ...goose, method: 'POST' }, confirmed: false },
  { input: { ...goose, status: 500 }, confirmed: false },
  { input: { ...goose, url: goose.url + '/execute' }, confirmed: false },
  { input: { ...goose, url: goose.url + '?method=execute' }, confirmed: false },
  { input: { ...goose, body: { error: 'Database unavailable' } }, confirmed: false },
  { input: { ...goose, body: { error: runtimeUnavailableMessage, result: [] } }, confirmed: false },
])('only verifies the exact Goose unavailable read: $confirmed', async ({ input, confirmed }) => {
  const results = { failures: [] as string[], httpFailures: [] as string[], verifiedPolicyUrls: new Set<string>(), policyChecks: [] as Promise<void>[] };
  recordSmokeHttpResponse({ status: () => input.status, url: () => input.url, request: () => ({ method: () => input.method }), json: async () => input.body }, input.origin, results);
  await Promise.all(results.policyChecks);
  expect(results.verifiedPolicyUrls.has(input.url)).toBe(confirmed);
  expect(results.failures).toEqual(confirmed ? [] : [`${input.status} ${input.url}`]);
});


test('only accepts the Goose browser resource diagnostic after its exact receipt was verified', () => {
  const text = 'Failed to load resource: the server responded with a status of 503 (Service Unavailable)';
  expect(isVerifiedRuntimePolicyDiagnostic(text, goose.url, new Set())).toBe(false);
  expect(isVerifiedRuntimePolicyDiagnostic(text, goose.url, new Set([goose.url]))).toBe(true);
  expect(isVerifiedRuntimePolicyDiagnostic('Unexpected runtime exception', goose.url, new Set([goose.url]))).toBe(false);
});
