/** Provider acknowledgement is not delivery. Only the authenticated server supplies these receipts. */
export type ManualInboxReceipt = {
  request_id: string;
  message_id: string;
  state: 'pending' | 'retired' | 'unknown' | 'accepted' | 'rejected' | 'blocked' | 'dismissed' | 'resolved';
  provider_message_id: string | null;
  detail: string | null;
  draft_reply: string;
  prior_send?: Omit<ManualInboxReceipt, 'message_id' | 'prior_send'> | null;
};

export const MANUAL_INBOX_REQUEST_ID = /^[A-Za-z0-9._-]{1,200}$/;

export function parseManualInboxReceipt(
  value: unknown,
  messageId: string,
  submitted?: { request_id: string; edited_reply?: string },
): ManualInboxReceipt {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid receipt');
  const receipt = value as Record<string, unknown>;
  if (typeof receipt.request_id !== 'string' || !MANUAL_INBOX_REQUEST_ID.test(receipt.request_id)
    || receipt.message_id !== messageId
    || typeof receipt.state !== 'string'
    || !['pending', 'retired', 'unknown', 'accepted', 'rejected', 'blocked', 'dismissed', 'resolved'].includes(receipt.state)
    || !(receipt.provider_message_id === null || (typeof receipt.provider_message_id === 'string' && receipt.provider_message_id.trim().length > 0))
    || (receipt.state === 'accepted' && typeof receipt.provider_message_id !== 'string')
    || !(receipt.detail === null || typeof receipt.detail === 'string')
    || typeof receipt.draft_reply !== 'string' || receipt.draft_reply.length > 16_000
    || receipt.error != null || ('success' in receipt && receipt.success !== true)
    || (submitted && (receipt.request_id !== submitted.request_id
      || (submitted.edited_reply !== undefined && receipt.draft_reply !== submitted.edited_reply)))) {
    throw new Error('Invalid or mismatched receipt');
  }
  if (receipt.prior_send != null) {
    const prior = receipt.prior_send;
    if (!['dismissed', 'resolved'].includes(receipt.state)
      || typeof prior !== 'object' || Array.isArray(prior)
      || 'prior_send' in prior || ('message_id' in prior && prior.message_id !== messageId)
      || !('state' in prior) || !['accepted', 'unknown', 'rejected', 'blocked'].includes(String(prior.state))) {
      throw new Error('Invalid prior provider evidence');
    }
    parseManualInboxReceipt({ ...prior, message_id: messageId }, messageId);
  }
  return receipt as ManualInboxReceipt;
}

export function manualInboxReceiptStatus(receipt: ManualInboxReceipt): string {
  switch (receipt.state) {
    case 'accepted': return 'Provider accepted the reply. Delivery is unconfirmed.';
    case 'pending': return 'Reply prepared, but not sent. Review the saved draft before sending.';
    case 'unknown': return 'Provider outcome is unknown. Check saved reply status; do not resend.';
    case 'retired': return 'This prepared reply was retired. Review the draft before creating a new request.';
    case 'rejected': return 'The provider rejected the reply. Your draft is preserved.';
    case 'blocked': return 'The reply was blocked before provider acceptance. Your draft is preserved.';
    case 'resolved': return 'Message closed. A previously started send cannot be recalled; check saved provider status. Your local draft is preserved.';
    case 'dismissed': return 'Message dismissed. A previously started send cannot be recalled; check saved provider status. Your local draft is preserved.';
  }
}
