import { describe, expect, it } from 'vitest';
import { moneyInput, parseMoneyInput, parseQuoteDetail, safePaymentLink } from './quoteDetail';

describe('Exact persisted quote amounts', () => {
  it.each([0, 1, -1, 10000, -12345, Number.MAX_SAFE_INTEGER - 1, Number.MAX_SAFE_INTEGER])('round trips safe cents %s without changing a reviewed cent', cents => {
    expect(parseMoneyInput(moneyInput(cents))).toBe(cents);
  });
  it.each(['', '1.001', 'Infinity', 'NaN', '1e3', '90071992547409.92'])('rejects an unrepresentable amount %s', value => {
    expect(parseMoneyInput(value)).toBeNull();
  });
});

describe('Quote boundaries', () => {
  it('only retains credential-free HTTPS payment destinations', () => {
    expect(safePaymentLink('https://checkout.stripe.com/example')).toBe('https://checkout.stripe.com/example');
    for (const url of [undefined, '', 'javascript:alert(1)', '//example.test/pay', 'https://user:password@example.test/pay']) {
      expect(safePaymentLink(url)).toBeNull();
    }
  });
  it('rejects items belonging to a different quote', () => {
    expect(() => parseQuoteDetail({ quote: { id: 'one', customer_id: 'customer', status: 'DRAFT', total_amount_cents: 100, required_deposit_cents: 0 }, acceptance: null, line_items: [{ id: 'line', quote_id: 'two', description: 'Foreign terms', unit_price_cents: 100, quantity: 1, is_optional: false }] }, 'one')).toThrow('Invalid quote line items');
  });
});
