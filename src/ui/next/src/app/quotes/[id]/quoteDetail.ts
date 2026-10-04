// The detail endpoint returns the persisted quote and its tenant-scoped items
// separately. Never treat a transport error or a partial envelope as a quote.
export interface QuoteLineItem {
  id: string;
  quote_id: string;
  description: string;
  unit_price_cents: number;
  quantity: number;
  is_optional: boolean;
  service_item_id?: string | null;
}

export interface QuoteDetail {
  id: string;
  customer_id: string;
  status: string;
  total_amount_cents: number | null;
  required_deposit_cents: number | null;
  stripe_payment_link?: string | null;
  line_items: QuoteLineItem[];
  acceptance: Record<string, unknown> | null;
}

export const QUOTE_ID_PATTERN = /^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i;

function record(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}
function amount(value: unknown): value is number | null {
  return value === null || (typeof value === 'number' && Number.isSafeInteger(value));
}

export function parseQuoteDetail(value: unknown, id: string): QuoteDetail {
  if (!record(value) || !record(value.quote) || !Array.isArray(value.line_items)) {
    throw new Error('Invalid quote response');
  }
  const quote = value.quote;
  if (typeof quote.id !== 'string' || quote.id.toLowerCase() !== id.toLowerCase()
      || typeof quote.customer_id !== 'string' || typeof quote.status !== 'string' || !quote.status.trim()
      || !amount(quote.total_amount_cents) || !amount(quote.required_deposit_cents)
      || (quote.stripe_payment_link != null && typeof quote.stripe_payment_link !== 'string')
      || (value.acceptance !== null && !record(value.acceptance))) {
    throw new Error('Invalid quote response');
  }
  const ids = new Set<string>();
  const lineItems = value.line_items.map((line: unknown): QuoteLineItem => {
    if (!record(line) || typeof line.id !== 'string' || ids.has(line.id)
        || typeof line.quote_id !== 'string' || line.quote_id.toLowerCase() !== id.toLowerCase()
        || typeof line.description !== 'string' || typeof line.is_optional !== 'boolean'
        || typeof line.unit_price_cents !== 'number' || !Number.isSafeInteger(line.unit_price_cents)
        || typeof line.quantity !== 'number' || !Number.isSafeInteger(line.quantity)
        || !Number.isSafeInteger(line.unit_price_cents * line.quantity)
        || (line.service_item_id != null && typeof line.service_item_id !== 'string')) {
      throw new Error('Invalid quote line items');
    }
    ids.add(line.id);
    return line as unknown as QuoteLineItem;
  });
  return { ...quote, line_items: lineItems, acceptance: value.acceptance } as QuoteDetail;
}

type QuoteTerms = Pick<QuoteDetail, 'total_amount_cents' | 'required_deposit_cents'> & {
  line_items: Array<Pick<QuoteLineItem, 'description' | 'unit_price_cents' | 'quantity' | 'is_optional' | 'service_item_id'>>;
};

// PUT regenerates line IDs and GET sorts by those IDs. Compare the actual terms,
// including duplicate/optional/catalog lines, independently of identity/order.
export function sameQuoteTerms(saved: QuoteDetail, reviewed: QuoteTerms): boolean {
  const lines = (value: QuoteTerms) => value.line_items.map(line => JSON.stringify({
    description: line.description, unit_price_cents: line.unit_price_cents,
    quantity: line.quantity, is_optional: line.is_optional, service_item_id: line.service_item_id ?? null,
  })).sort();
  return saved.total_amount_cents === reviewed.total_amount_cents
    && saved.required_deposit_cents === reviewed.required_deposit_cents
    && JSON.stringify(lines(saved)) === JSON.stringify(lines(reviewed));
}

export function hasQuoteTerms(quote: QuoteDetail): boolean {
  return !!quote.customer_id && quote.total_amount_cents !== null
    && quote.required_deposit_cents !== null && quote.line_items.length > 0;
}

export function moneyInput(cents: number): string {
  const digits = Math.abs(cents).toString().padStart(3, '0');
  return `${cents < 0 ? '-' : ''}${digits.slice(0, -2)}.${digits.slice(-2)}`;
}

export function parseMoneyInput(input: string): number | null {
  if (!/^-?\d+(?:\.\d{1,2})?$/.test(input)) return null;
  const [whole, fraction = ''] = input.replace('-', '').split('.');
  const cents = Number(whole) * 100 + Number(fraction.padEnd(2, '0'));
  return Number.isSafeInteger(cents) ? (input.startsWith('-') ? -cents : cents) : null;
}

export function safePaymentLink(link?: string | null): string | null {
  if (!link) return null;
  try {
    const url = new URL(link);
    return url.protocol === 'https:' && !url.username && !url.password ? url.href : null;
  } catch {
    return null;
  }
}
