'use client';

import { useEffect, useRef, useState } from 'react';
import { isQuoteVersion } from '@/lib/quoteVersion';
import {
  hasQuoteTerms, moneyInput, parseQuoteDetail, QUOTE_ID_PATTERN, safePaymentLink, sameQuoteTerms, type QuoteDetail,
} from '@/app/quotes/[id]/quoteDetail';

type AcceptanceReceipt = {
  success: true;
  status: 'accepted';
  quote_id: string;
  invoice_id: string;
  invoice_status: string;
  payment_status: string;
  checkout_status: 'available' | 'not_configured' | 'pending' | 'reconciliation';
  stripe_payment_link: string;
  reason: string | null;
};

// These are the public fields of the committed backend receipt. A status-only
// response, historical ACCEPTED flag, or saved quote URL is not that receipt.
function parseReceipt(value: unknown, id: string): AcceptanceReceipt | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const receipt = value as Record<string, unknown>;
  if (receipt.success !== true || receipt.status !== 'accepted'
      || typeof receipt.quote_id !== 'string' || receipt.quote_id.toLowerCase() !== id
      || typeof receipt.invoice_id !== 'string' || !QUOTE_ID_PATTERN.test(receipt.invoice_id)
      || typeof receipt.invoice_status !== 'string' || !receipt.invoice_status.trim()
      || typeof receipt.payment_status !== 'string' || !receipt.payment_status.trim()
      || typeof receipt.checkout_status !== 'string'
      || !['available', 'not_configured', 'pending', 'reconciliation'].includes(receipt.checkout_status)
      || typeof receipt.stripe_payment_link !== 'string'
      || (receipt.reason !== null && typeof receipt.reason !== 'string')) return null;
  return receipt as AcceptanceReceipt;
}

function committedReceipt(quote: QuoteDetail): AcceptanceReceipt | null {
  return quote.status.toUpperCase() === 'ACCEPTED' ? parseReceipt(quote.acceptance, quote.id.toLowerCase()) : null;
}

async function readQuote(id: string, signal?: AbortSignal): Promise<QuoteDetail | null> {
  const response = await fetch(`/api/v1/quotes/${id}`, { cache: 'no-store', signal });
  if (response.status === 404) return null;
  if (!response.ok) throw new Error('Quote read failed');
  return parseQuoteDetail(await response.json(), id);
}

function money(cents: number | null): string {
  if (cents === null) return 'Not available';
  // Integer cents stay exact even near Number.MAX_SAFE_INTEGER. Dividing by
  // 100 before formatting can silently round a real cent away.
  const [whole, fraction] = moneyInput(cents).replace('-', '').split('.');
  return `${cents < 0 ? '-' : ''}$${whole.replace(/\B(?=(\d{3})+(?!\d))/g, ',')}.${fraction}`;
}

function paymentContinuation(receipt: AcceptanceReceipt, quote: QuoteDetail): string | null {
  // A paid/terminal/processing/unknown invoice must not offer another payment.
  if (receipt.checkout_status !== 'available' || quote.total_amount_cents === null || quote.total_amount_cents <= 0
      || !['draft', 'open', 'sent', 'viewed', 'accepted', 'pending', 'overdue'].includes(receipt.invoice_status.toLowerCase())
      || !['unpaid', 'unverified'].includes(receipt.payment_status.toLowerCase())) return null;
  return safePaymentLink(receipt.stripe_payment_link);
}

const buttonClass = 'app-button min-h-11 px-5 py-2.5 disabled:opacity-60';

// Each route keys this component by the canonical ID. Operation fencing also
// protects unmount/refresh even when a transport ignores its abort signal.
export function CustomerQuoteView({ id }: { id: string }) {
  const [quote, setQuote] = useState<QuoteDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [refresh, setRefresh] = useState(0);
  const [busy, setBusy] = useState(false);
  const [reconcile, setReconcile] = useState(false);
  const operation = useRef(0);
  const busyRef = useRef(false);

  useEffect(() => {
    const current = ++operation.current;
    const controller = new AbortController();
    setLoading(true);
    setQuote(null);
    setLoadError(null);
    setActionError(null);
    setReconcile(false);
    if (!QUOTE_ID_PATTERN.test(id)) {
      setLoadError('Quote not found.');
      setLoading(false);
      return () => { ++operation.current; controller.abort(); };
    }
    readQuote(id, controller.signal).then(value => {
      if (current !== operation.current) return;
      setQuote(value);
      if (!value) setLoadError('Quote not found.');
    }).catch(() => {
      if (current === operation.current) setLoadError('This quote is unavailable.');
    }).finally(() => {
      if (current === operation.current) setLoading(false);
    });
    return () => { ++operation.current; controller.abort(); };
  }, [id, refresh]);

  const status = quote?.status.toUpperCase();
  const pending = status === 'PENDING' || status === 'DRAFTING';
  const versionAvailable = isQuoteVersion(quote?.updated_at);
  // Acceptance materializes an invoice whose persisted cents column is i32.
  // The quote itself remains readable across its wider exact-integer range.
  const invoiceAmountAvailable = quote?.total_amount_cents != null
    && quote.total_amount_cents >= -2147483648 && quote.total_amount_cents <= 2147483647;
  const eligible = !!quote && hasQuoteTerms(quote) && invoiceAmountAvailable && versionAvailable && quote.acceptance === null
    && ['DRAFT', 'SENT', 'APPROVED'].includes(status ?? '');
  const receipt = quote ? committedReceipt(quote) : null;

  async function acceptQuote() {
    if (!quote || !eligible || busyRef.current || reconcile) return;
    busyRef.current = true;
    setBusy(true);
    setActionError(null);
    const current = operation.current;
    let returnedReceipt: AcceptanceReceipt | null = null;
    try {
      try {
        const response = await fetch(`/api/v1/quotes/${id}/accept`, {
          method: 'POST', headers: { 'Content-Type': 'application/json' },
          // Send the exact GET token. Date serialization would lose microseconds.
          body: JSON.stringify({ expected_updated_at: quote.updated_at }),
        });
        if (current !== operation.current) return;
        if (response.ok) returnedReceipt = parseReceipt(await response.json(), id);
      } catch {
        // A lost/malformed response can follow a committed local invoice. One
        // owned readback can prove that result; never automatically POST again.
      }
      if (current !== operation.current) return;
      const saved = await readQuote(id);
      if (current !== operation.current) return;
      const savedReceipt = saved ? committedReceipt(saved) : null;
      if (!saved || !savedReceipt || saved.customer_id !== quote.customer_id || !sameQuoteTerms(saved, quote)
          || (returnedReceipt && returnedReceipt.invoice_id.toLowerCase() !== savedReceipt.invoice_id.toLowerCase())) {
        throw new Error('Acceptance receipt could not be verified');
      }
      setQuote(saved);
    } catch {
      if (current !== operation.current) return;
      setReconcile(true);
      setActionError('Acceptance could not be confirmed. Refresh the quote and review its current terms before trying again.');
    } finally {
      if (current === operation.current) {
        busyRef.current = false;
        setBusy(false);
      }
    }
  }

  const refreshButton = <button className={buttonClass} disabled={busy} onClick={() => setRefresh(value => value + 1)} type="button">Refresh quote</button>;
  if (loading) return <p className="py-10 text-sm text-gray-600" role="status">Loading quote...</p>;
  if (!quote) return <section className="app-panel max-w-lg space-y-4 rounded-lg p-5">
    <p role="alert">{loadError ?? 'Quote not found.'}</p>
    {refreshButton}
  </section>;

  const paymentLink = receipt ? paymentContinuation(receipt, quote) : null;
  return <div className="max-w-2xl space-y-5">
    <section className="app-panel rounded-lg p-5">
      <h1 className="text-xl font-semibold">Your Quote</h1>
      <p className="mt-1 font-mono text-sm">Quote ID: {quote.id}</p>
      <p className="mt-1 text-sm">Recorded status: {quote.status}</p>
      {pending && <p role="status">Quote preparation is pending. Refresh to check for saved terms.</p>}
      <div className="mt-5 divide-y divide-gray-200 dark:divide-gray-700">
        {quote.line_items.map(item => <div className="flex items-start justify-between gap-4 py-3 text-sm" key={item.id}>
          <div><span>{item.description} x{item.quantity}</span>{item.is_optional && <span className="block text-xs">Optional</span>}</div>
          <span className="font-medium">{money(item.unit_price_cents * item.quantity)}</span>
        </div>)}
      </div>
      <div className="mt-4 flex justify-between"><span>Total</span><strong>{money(pending ? null : quote.total_amount_cents)}</strong></div>
      <div className="mt-4 flex justify-between"><span>Required deposit</span><strong>{money(pending ? null : quote.required_deposit_cents)}</strong></div>
    </section>

    {receipt ? <section className="app-panel space-y-2 rounded-lg p-5" aria-label="Acceptance receipt">
      <h2 className="text-xl font-semibold">Quote accepted</h2>
      <p>Acceptance and the invoice are recorded. Payment and checkout states are shown below.</p>
      <p>Invoice: {receipt.invoice_id}</p>
      <p>Invoice status: {receipt.invoice_status}</p>
      <p>Payment status: {receipt.payment_status}</p>
      <p>Checkout status: {receipt.checkout_status}</p>
      {receipt.checkout_status === 'not_configured' && <p>Checkout is not configured. No online payment link is available.</p>}
      {receipt.checkout_status === 'pending' && <p>Checkout preparation is pending. Refresh to check its state.</p>}
      {receipt.checkout_status === 'reconciliation' && <p>Checkout requires reconciliation before any payment continuation.</p>}
      {receipt.checkout_status === 'available' && !paymentLink && <p>Payment continuation is unavailable for the current invoice, payment state or link.</p>}
      {paymentLink && <>
        <p>A checkout link does not confirm payment. Continue only if you intend to pay this invoice.</p>
        <a className={buttonClass} href={paymentLink} target="_blank" rel="noopener noreferrer">Continue to payment</a>
      </>}
    </section> : (status === 'ACCEPTED' || quote.acceptance !== null) ? <p role="status">Acceptance requires reconciliation. A verified invoice receipt is unavailable.</p> : <>
      {!versionAvailable && <p role="status">Quote version is unavailable. Refresh before accepting.</p>}
      {!pending && quote.total_amount_cents !== null && !invoiceAmountAvailable && <p role="status">Quote amount is outside the supported invoice range. Acceptance is unavailable.</p>}
      {!hasQuoteTerms(quote) && <p role="status">Quote terms are incomplete. Acceptance is unavailable until complete terms are saved.</p>}
      {eligible && <button className={`${buttonClass} bg-[#0066FF] font-semibold text-white`} disabled={busy || reconcile} onClick={() => void acceptQuote()} type="button">
        {busy ? 'Accepting...' : 'Accept quote'}
      </button>}
    </>}
    {actionError && <p role="alert">{actionError}</p>}
    {refreshButton}
  </div>;
}
