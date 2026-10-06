import { expect, test } from 'vitest';
import { parseManualInboxReceipt, manualInboxReceiptStatus } from './inboxManualReceipt';
const receipt = { request_id: '10b562b2-4c75-4f85-b6fa-43a99d9b655a', message_id: 'message-1', state: 'pending', provider_message_id: null, detail: null, draft_reply: 'Reply' };
test.each(['pending', 'retired', 'unknown', 'accepted', 'rejected', 'blocked', 'dismissed', 'resolved'])('accepts a complete %s receipt', state => {
  const value = { ...receipt, state, provider_message_id: state === 'accepted' ? 'provider-1' : null };
  expect(parseManualInboxReceipt(value, 'message-1')).toEqual(value);
});
test.each([null, [], {}, { ...receipt, request_id: '' }, { ...receipt, message_id: 'other' }, { ...receipt, state: 'delivered' }, { ...receipt, state: ['accepted'] }, { ...receipt, state: 'accepted' }, { ...receipt, provider_message_id: ' ' }, { ...receipt, draft_reply: 42 }, { ...receipt, draft_reply: 'a'.repeat(16_001) }, { ...receipt, detail: {} }, { ...receipt, success: false }, { ...receipt, error: 'bad' }])('rejects untrusted receipt %j', value => {
  expect(() => parseManualInboxReceipt(value, 'message-1')).toThrow();
});
test('binds readback to the immutable request and exact submitted text', () => {
  expect(() => parseManualInboxReceipt(receipt, 'message-1', { request_id: 'changed', edited_reply: 'Reply' })).toThrow();
  expect(() => parseManualInboxReceipt(receipt, 'message-1', { request_id: receipt.request_id, edited_reply: 'Changed' })).toThrow();
});


test.each(['dismissed', 'resolved'] as const)('does not present %s as recalling an already-started provider send', state => {
  expect(manualInboxReceiptStatus({ ...receipt, state })).toMatch(/cannot be recalled/);
  expect(manualInboxReceiptStatus({ ...receipt, state })).toMatch(/provider status/i);
});

const priorSend = { request_id: 'prior-send', state: 'unknown', provider_message_id: null, detail: null, draft_reply: 'Original provider attempt' };
test('validates prior provider evidence on a durable dismissal receipt', () => {
  const value = { ...receipt, state: 'dismissed', prior_send: priorSend };
  expect(parseManualInboxReceipt(value, 'message-1')).toEqual(value);
});
test.each([
  { ...priorSend, state: 'accepted' }, { ...priorSend, state: 'pending' },
  { ...priorSend, request_id: '../foreign' }, { ...priorSend, draft_reply: 123 },
  { ...priorSend, prior_send: priorSend }, { ...priorSend, message_id: 'other' },
])('rejects malformed nested provider evidence %j', prior => {
  expect(() => parseManualInboxReceipt({ ...receipt, state: 'dismissed', prior_send: prior }, 'message-1')).toThrow();
});
