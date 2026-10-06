/** A provider acceptance receipt is not proof of customer delivery. */
export function messageDeliveryStatus(status?: string): { label: string; tone: string } | undefined {
  switch ((status || '').toLowerCase()) {
    case 'provider_accepted': return { label: 'Accepted by provider · delivery unconfirmed', tone: 'warn' };
    case 'delivery_unknown': return { label: 'Outcome unknown · reconcile before retrying', tone: 'warn' };
    case 'delivery_failed': return { label: 'Provider rejected message', tone: 'bad' };
    case 'delivery_blocked': return { label: 'Not sent · connection or review required', tone: 'bad' };
    case 'sent':
    case 'replied':
    case 'auto_replied': return { label: 'Legacy send · outcome unverified', tone: 'warn' };
    default: return undefined;
  }
}
