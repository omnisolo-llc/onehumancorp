import { expect, it } from 'vitest';
import * as contracts from './publicationContracts';
import vectors from './testFixtures/publicationRawNumbers.json';

const parse = contracts.parsePublicationJson;
it.each(vectors.filter(value => value.accepted))('accepts the bounded numeric token $literal without a custom number serializer', ({ literal }) => {
  expect(parse('{"value":' + literal + '}')).toEqual({ value: JSON.parse(literal) });
});
it.each(vectors.filter(value => !value.accepted))('rejects over-bound or underflow token $literal before JSON rounding', ({ literal }) => {
  expect(() => parse('{"value":' + literal + '}')).toThrow();
});
it.each([
  '{"key":1,"key":2}', '{"key":1,"k\\u0065y":2}', '{"nested":[{"key":1,"key":2}]}',
  '{"key":1,}', '{/*comment*/"key":1}', '', 'true false', '{"key":"\\ud800"}',
  '{"\\ud800":"value"}', '{"key":"nul\\u0000byte"}', '{"value":01}', '{"value":+1}',
])('rejects ambiguous, non-JSON or invalid Unicode bytes %#', text => {
  expect(() => parse(text)).toThrow();
});
it('retains ordinary Unicode, numeric-looking keys and identity-like content fields', () => {
  const expected = { '2': 'two', '10': 'ten', user_id: 'Printed label', value: 'Café\n😀' };
  expect(parse(JSON.stringify(expected))).toEqual(expected);
});
it('rejects excessive nesting before descending through the whole input', () => {
  expect(() => parse('['.repeat(1000) + '0' + ']'.repeat(1000))).toThrow();
});
it('cancels an oversized streamed receipt before reading the rest of its body', async () => {
  let chunks = 0; let cancelled = false;
  const response = new Response(new ReadableStream({ pull(controller) { chunks++; controller.enqueue(new Uint8Array(40_000)); }, cancel() { cancelled = true; } }), { headers: { 'content-type': 'application/json' } });
  await expect(contracts.readPublicationResponse(response)).rejects.toThrow('receipt limit');
  expect(cancelled).toBe(true); expect(chunks).toBeLessThanOrEqual(3);
});
it('rejects invalid UTF8 and contradictory raw receipt keys', async () => {
  await expect(contracts.readPublicationResponse(new Response(new Uint8Array([0xff]), { headers: { 'content-type': 'application/json' } }))).rejects.toThrow();
  await expect(contracts.readPublicationResponse(new Response('{"status":"failed","status":"published"}', { headers: { 'content-type': 'application/json' } }))).rejects.toThrow();
});
it('checks current owner/view before acquiring a response reader', async () => {
  const response = Response.json({ status: 'published' });
  await expect(contracts.readPublicationResponse(response, () => { throw new Error('View retired'); })).rejects.toThrow('View retired');
  expect(response.bodyUsed).toBe(false);
});
it('enforces exactly 32 JSON container levels at the wire boundary', () => {
  expect(parse('['.repeat(32) + '0' + ']'.repeat(32))).toBeDefined();
  expect(() => parse('['.repeat(33) + '0' + ']'.repeat(33))).toThrow();
});
