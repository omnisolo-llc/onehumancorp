'use client';

import React, { useEffect, useRef, useState } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { AppShell } from '../../components/AppShell';
import {
  hasQuoteTerms, moneyInput, parseMoneyInput, parseQuoteDetail,
  QUOTE_ID_PATTERN, safePaymentLink, sameQuoteTerms, type QuoteDetail,
} from './quoteDetail';

type Edits = { prices: string[]; total: string; deposit: string };
const actionClass = 'w-full min-h-[44px] bg-[#0066FF] text-white font-bold shadow-lg hover:bg-[#0052CC] transition-all disabled:opacity-50';
const secondaryClass = 'w-full min-h-[44px] border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white font-medium disabled:opacity-50';

async function readQuote(id: string, signal?: AbortSignal): Promise<QuoteDetail | null> {
  const response = await fetch(`/api/v1/quotes/${id}`, { cache: 'no-store', signal });
  if (response.status === 404) return null;
  if (!response.ok) throw new Error('Quote read failed');
  return parseQuoteDetail(await response.json(), id);
}

export default function QuoteReviewPage() {
  const params = useParams();
  const id = typeof params.id === 'string' ? params.id.toLowerCase() : '';
  // Navigation discards both unsaved edits and in-flight results for the old ID.
  return <QuoteReview key={id} id={id} />;
}

function QuoteReview({ id }: { id: string }) {
  const router = useRouter();
  const [quote, setQuote] = useState<QuoteDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState(false);
  const [refresh, setRefresh] = useState(0);
  const [edits, setEdits] = useState<Edits | null>(null);
  const [busy, setBusy] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [reconcile, setReconcile] = useState(false);
  const operation = useRef(0);
  const busyRef = useRef(false);

  useEffect(() => {
    const current = ++operation.current;
    const controller = new AbortController();
    setQuote(null);
    setLoading(true);
    setLoadError(false);
    setEdits(null);
    setActionError(null);
    setNotice(null);
    setReconcile(false);
    if (!QUOTE_ID_PATTERN.test(id)) {
      setLoading(false);
      return;
    }
    readQuote(id, controller.signal).then(value => {
      if (current === operation.current) setQuote(value);
    }).catch(() => {
      if (current === operation.current) setLoadError(true);
    }).finally(() => {
      if (current === operation.current) setLoading(false);
    });
    return () => { ++operation.current; controller.abort(); };
  }, [id, refresh]);

  const status = quote?.status.toUpperCase();
  const pending = status === 'DRAFTING' || status === 'PENDING';
  const ready = !!quote && !pending && hasQuoteTerms(quote);
  const editable = ready && !quote?.acceptance && (status === 'DRAFT' || status === 'SENT' || status === 'APPROVED');
  const approvable = editable && status === 'DRAFT';

  const mutate = async (kind: 'approve' | 'save') => {
    if (!quote || busyRef.current || reconcile || (kind === 'approve' ? !approvable : !editable || !edits)) return;
    let body: string | undefined;
    let reviewed = quote;
    if (kind === 'save' && edits) {
      const total = parseMoneyInput(edits.total);
      const deposit = parseMoneyInput(edits.deposit);
      const prices = edits.prices.map(parseMoneyInput);
      if (total === null || deposit === null || prices.some((price, index) => price === null || !Number.isSafeInteger(price * quote.line_items[index].quantity))) {
        setActionError('Enter valid amounts with at most two decimal places.');
        return;
      }
      reviewed = { ...quote, total_amount_cents: total, required_deposit_cents: deposit,
        line_items: quote.line_items.map((line, index) => ({ ...line, unit_price_cents: prices[index]! })),
      };
      body = JSON.stringify({
        total_amount_cents: total,
        required_deposit_cents: deposit,
        line_items: quote.line_items.map((line, index) => ({
          description: line.description, unit_price_cents: prices[index], quantity: line.quantity,
          is_optional: line.is_optional, service_item_id: line.service_item_id ?? null,
        })),
      });
    }
    busyRef.current = true;
    setBusy(true);
    setActionError(null);
    setNotice(null);
    const current = operation.current;
    try {
      const response = await fetch(`/api/v1/quotes/${id}${kind === 'approve' ? '/approve' : ''}`, {
        method: kind === 'approve' ? 'PATCH' : 'PUT',
        ...(body ? { headers: { 'Content-Type': 'application/json' }, body } : {}),
      });
      if (!response.ok) throw new Error('Quote change failed');
      const receipt = await response.json();
      if (kind === 'save' ? receipt?.success !== true : receipt?.quote?.id?.toLowerCase() !== id) {
        throw new Error('Invalid quote change receipt');
      }
      if (current !== operation.current) return;
      const saved = await readQuote(id);
      if (!saved || !sameQuoteTerms(saved, reviewed) || (kind === 'approve' && !['SENT', 'APPROVED', 'ACCEPTED'].includes(saved.status.toUpperCase()))) {
        throw new Error('Quote change could not be verified');
      }
      if (current !== operation.current) return;
      setQuote(saved);
      setEdits(null);
      setNotice(kind === 'save' ? 'Quote changes saved.' : 'Quote approval saved. Message delivery to the customer is not confirmed.');
    } catch {
      if (current !== operation.current) return;
      // A failed response can follow a committed write. Re-read before another
      // action; never fabricate success or blindly repeat an uncertain mutation.
      setReconcile(true);
      setActionError(`${kind === 'save' ? 'Changes' : 'Approval'} could not be confirmed. Refresh the quote before trying again.`);
    } finally {
      if (current === operation.current) {
        busyRef.current = false;
        setBusy(false);
      }
    }
  };

  const navigation = <>
    <button onClick={() => setRefresh(value => value + 1)} disabled={busy} className={secondaryClass}>Refresh quote</button>
    <button onClick={() => router.back()} className={secondaryClass}>Back to Feed</button>
  </>;
  if (loading) return <AppShell title="Loading Quote..."><div className="p-4 text-center">Loading...</div></AppShell>;
  if (loadError) return <AppShell title="Quote unavailable"><div className="p-4 space-y-4"><p role="alert">Unable to load quote. Please refresh to try again.</p>{navigation}</div></AppShell>;
  if (!quote) return <AppShell title="Not Found"><div className="p-4 space-y-4"><p>Quote not found</p>{navigation}</div></AppShell>;

  const paymentLink = safePaymentLink(quote.stripe_payment_link);
  return (
    <AppShell title="Review Estimate" subtitle={`Quote #${id.slice(0, 8)}`}>
      <div className="w-full max-w-md mx-auto p-4 space-y-6">
        <div className="glassmorphism p-6 space-y-4">
          <div className="flex justify-between items-center">
            <span className="text-sm font-medium text-gray-500">Status</span>
            <span className="text-xs font-bold px-2 py-1 rounded-full bg-blue-100 text-blue-700">{quote.status}</span>
          </div>
          {pending ? <p role="status">Quote preparation is pending. Refresh to check for saved terms.</p> : <>
            {!ready && <p role="status">Quote terms are incomplete. Pricing and approval are unavailable until complete terms are saved.</p>}
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <h3 className="text-[11px] font-bold uppercase tracking-wider text-gray-400">Line Items</h3>
                {editable && !edits && <button aria-label="Edit quote" id="edit-quote-btn" disabled={busy || reconcile} onClick={() => {
                  setEdits({ prices: quote.line_items.map(line => moneyInput(line.unit_price_cents)), total: moneyInput(quote.total_amount_cents!), deposit: moneyInput(quote.required_deposit_cents!) });
                  setActionError(null);
                  setNotice(null);
                }} className="text-[10px] text-[#0066FF] font-bold">EDIT</button>}
              </div>
              {edits && <p role="status">Unsaved changes. Total and deposit are explicit terms; editing a line does not change them automatically.</p>}
              {quote.line_items.map((line, index) => <div key={line.id} className="flex justify-between gap-2 text-sm py-2 border-b border-gray-100 dark:border-gray-800">
                <span>{line.description} (x{line.quantity}){line.is_optional && <span className="block text-xs">Optional</span>}</span>
                {edits ? <input id={`quote-price-${index}`} aria-label={`Unit price for ${line.description}`} type="number" step="0.01" disabled={busy || reconcile} value={edits.prices[index]} onChange={event => setEdits({ ...edits, prices: edits.prices.map((price, i) => i === index ? event.target.value : price) })} className="w-24 text-right bg-gray-100 dark:bg-gray-800 rounded px-1" /> : <span>${moneyInput(line.unit_price_cents * line.quantity)}</span>}
              </div>)}
            </div>
            <div className="pt-4 border-t border-gray-100 dark:border-gray-800 space-y-2">
              <div className="flex justify-between items-center font-bold">
                <label htmlFor="quote-total">Total Amount</label>
                {edits ? <input id="quote-total" aria-label="Total amount" type="number" step="0.01" disabled={busy || reconcile} value={edits.total} onChange={event => setEdits({ ...edits, total: event.target.value })} className="w-24 text-right bg-gray-100 dark:bg-gray-800" /> : <span>{quote.total_amount_cents === null ? 'Not available' : `$${moneyInput(quote.total_amount_cents)}`}</span>}
              </div>
              <div className="flex justify-between items-center text-sm text-gray-500">
                <label htmlFor="quote-deposit">Required Deposit</label>
                {edits ? <input id="quote-deposit" aria-label="Required deposit" type="number" step="0.01" disabled={busy || reconcile} value={edits.deposit} onChange={event => setEdits({ ...edits, deposit: event.target.value })} className="w-24 text-right bg-gray-100 dark:bg-gray-800" /> : <span>{quote.required_deposit_cents === null ? 'Not available' : `$${moneyInput(quote.required_deposit_cents)}`}</span>}
              </div>
            </div>
            {paymentLink && <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <p className="text-xs mb-1">Saved payment link. A link does not confirm payment.</p>
              <a href={paymentLink} target="_blank" rel="noopener noreferrer" className="text-sm text-[#0066FF] underline break-all">{paymentLink}</a>
            </div>}
          </>}
        </div>
        {notice && <p role="status">{notice}</p>}
        {!notice && status === 'SENT' && <p>Recorded status: SENT. Message delivery to the customer is not confirmed.</p>}
        {actionError && <p role="alert" className="text-[#FF3B30]">{actionError}</p>}
        {edits ? <>
          <button id="btn-save-edits" onClick={() => void mutate('save')} disabled={busy || reconcile} className={actionClass}>{busy ? 'Saving...' : 'Save Changes'}</button>
          <button onClick={() => setEdits(null)} disabled={busy} className={secondaryClass}>Cancel edits</button>
        </> : approvable && <button onClick={() => void mutate('approve')} disabled={busy || reconcile} className={actionClass}>{busy ? 'Approving...' : 'Approve quote'}</button>}
        {navigation}
      </div>
    </AppShell>
  );
}
