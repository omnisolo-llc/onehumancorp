import { describe, expect, it } from 'vitest';
import { messageDeliveryStatus } from './messageDeliveryStatus';

describe('message delivery evidence', () => {
  it('labels provider acceptance as awaiting delivery confirmation', () => {
    expect(messageDeliveryStatus('provider_accepted')).toEqual({ label: 'Accepted by provider · delivery unconfirmed', tone: 'warn' });
  });
  it('never upgrades old unverified success labels into delivery or handled claims', () => {
    for (const status of ['sent', 'auto_replied', 'replied']) {
      expect(messageDeliveryStatus(status)).toEqual({ label: 'Legacy send · outcome unverified', tone: 'warn' });
    }
  });
  it('shows unknown outcomes as held for reconciliation', () => {
    expect(messageDeliveryStatus('delivery_unknown')).toEqual({ label: 'Outcome unknown · reconcile before retrying', tone: 'warn' });
  });
  it('keeps rejection and no-send prerequisites distinct', () => {
    expect(messageDeliveryStatus('delivery_failed')?.label).toBe('Provider rejected message');
    expect(messageDeliveryStatus('delivery_blocked')?.label).toBe('Not sent · connection or review required');
  });
});
