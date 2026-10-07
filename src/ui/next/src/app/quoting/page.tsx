"use client";
import { Suspense } from "react";
import { isQuoteVersion } from "@/lib/quoteVersion";
import { useState, useEffect, useRef } from 'react';
import { useSearchParams } from 'next/navigation';
import type { QuotePayload, BusinessLineItem } from '@/lib/business-records';
import { SyncManager } from '@/lib/sync/SyncManager';
type EditableLineItem = BusinessLineItem & { is_optional?: boolean; service_item_id?: string | null };

function QuotingContent() {
  const searchParams = useSearchParams();
  const quoteId = searchParams.get('id');

  const [quoteData, setQuoteData] = useState<{ quote: QuotePayload; line_items: EditableLineItem[] } | null>(null);
  const [lineItems, setLineItems] = useState<EditableLineItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [outcome, setOutcome] = useState<'idle' | 'saving' | 'approved' | 'held'>('idle');
  const [isOffline, setIsOffline] = useState(false);
  const [offlineAccepted, setOfflineAccepted] = useState(false);
  const [notice, setNotice] = useState('');
  const [mutationError, setMutationError] = useState('');

  useEffect(() => {
    setIsOffline(!navigator.onLine);
    const updateOfflineStatus = () => setIsOffline(!navigator.onLine);
    window.addEventListener('online', updateOfflineStatus);
    window.addEventListener('offline', updateOfflineStatus);
    window.addEventListener('omnisolo_queue_updated', updateOfflineStatus);
    return () => {
      window.removeEventListener('online', updateOfflineStatus);
      window.removeEventListener('offline', updateOfflineStatus);
      window.removeEventListener('omnisolo_queue_updated', updateOfflineStatus);
    };
  }, []);
  const epoch = useRef(0);
  const busy = useRef(false);
  const accepted = quoteData?.quote.status?.toUpperCase() === 'ACCEPTED';
  const versionAvailable = isQuoteVersion(quoteData?.quote.updated_at);
  const eligibleStatus = ['DRAFT', 'SENT', 'APPROVED'].includes(quoteData?.quote.status?.toUpperCase() ?? '');
  const termsAvailable = !!quoteData && Number.isSafeInteger(quoteData.quote.total_amount_cents)
    && Number.isSafeInteger(quoteData.quote.required_deposit_cents) && quoteData.line_items.length > 0;
  const readOnly = accepted || !versionAvailable || !eligibleStatus || !termsAvailable || outcome !== 'idle';

  useEffect(() => {
    const current = ++epoch.current;
    const controller = new AbortController();
    busy.current = false;
    setQuoteData(null); setLineItems([]); setLoading(true); setError('');
    setOutcome('idle'); setNotice(''); setMutationError('');
    if (!quoteId) {
      setError('Quote ID is missing'); setLoading(false);
      return () => { epoch.current += 1; controller.abort(); };
    }
    void (async () => {
      try {
        const response = await fetch(`/api/v1/quotes?id=${encodeURIComponent(quoteId)}`, { signal: controller.signal });
        const data = await response.json();
        if (epoch.current !== current) return;
        if (!response.ok || data?.quote?.id !== quoteId || !Array.isArray(data.line_items)) throw new Error('Invalid quote response');
        setQuoteData(data); setLineItems(data.line_items);
      } catch {
        if (epoch.current === current) setError('Could not load the current quote. No changes were confirmed.');
      } finally {
        if (epoch.current === current) setLoading(false);
      }
    })();
    return () => { epoch.current += 1; controller.abort(); busy.current = false; };
  }, [quoteId]);

  const handleItemChange = <K extends keyof EditableLineItem,>(id: string, field: K, value: EditableLineItem[K]) => {
    if (readOnly || busy.current) return;
    setLineItems(prev => prev.map(item => {
      if (item.id === id) {
        return { ...item, [field]: value };
      }
      return item;
    }));
  };

  const handleApproveAndSend = async () => {
    if (!quoteData || !quoteId || quoteData.quote.id !== quoteId || readOnly || busy.current) return;
    const totalAmountCents = lineItems.reduce((sum, item) => sum + item.unit_price_cents * item.quantity, 0);
    if (!Number.isSafeInteger(totalAmountCents) || lineItems.some(item => !Number.isSafeInteger(item.unit_price_cents)
        || !Number.isSafeInteger(item.quantity) || item.quantity < 1)) {
      setMutationError('Review the line-item amounts and quantities before approving.'); return;
    }
    const id = quoteId;
    const current = epoch.current;
    const active = () => epoch.current === current;
    const updatePayload = {
      expected_updated_at: quoteData.quote.updated_at,
      total_amount_cents: totalAmountCents,
      line_items: lineItems.map(item => ({ description: item.description, unit_price_cents: item.unit_price_cents,
        quantity: item.quantity, is_optional: item.is_optional || false, service_item_id: item.service_item_id ?? null })),
    };
    busy.current = true; setOutcome('saving'); setMutationError(''); setNotice('');

    if (isOffline) {
      try {
        await SyncManager.getInstance().enqueue({ id: crypto.randomUUID(), type: 'update_quote', quoteId: id, payload: updatePayload, timestamp: Date.now() });
        await SyncManager.getInstance().enqueue({ id: crypto.randomUUID(), type: 'approve_quote', quoteId: id, payload: { expected_updated_at: quoteData.quote.updated_at }, timestamp: Date.now() });
        setOfflineAccepted(true);
        setOutcome('approved');
        setQuoteData(previous => previous ? { ...previous, quote: { ...previous.quote, status: 'ACCEPTED', total_amount_cents: totalAmountCents }, line_items: updatePayload.line_items as any } : previous);
      } catch {
        setOutcome('held');
        setMutationError('Could not queue changes offline. Keep your edits and retry.');
      } finally {
        busy.current = false;
      }
      return;
    }

    let changesSaved = false;
    try {
      const update = await fetch(`/api/v1/quotes?id=${encodeURIComponent(id)}`, {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(updatePayload),
      });
      const updated = await update.json();
      if (!active()) return;
      if (!update.ok || updated?.success !== true || updated.error != null) throw new Error('Unconfirmed changes');
      changesSaved = true;
      if (!isQuoteVersion(updated.updated_at)) throw new Error('Missing committed quote version');
      const approval = await fetch(`/api/v1/quotes/${encodeURIComponent(id)}/approve`, {
        method: 'PATCH', headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ expected_updated_at: updated.updated_at }),
      });
      const approved = await approval.json();
      if (!active()) return;
      if (!approval.ok || approved?.success === false || approved?.error != null
          || approved?.quote?.id !== id || approved.quote.status !== 'SENT') throw new Error('Unconfirmed approval');
      setQuoteData(previous => previous ? { ...previous, quote: { ...previous.quote, ...approved.quote } } : previous);
      setOutcome('approved');
      setNotice('Approval saved. Quote is marked SENT; customer delivery and acceptance are not confirmed.');
    } catch {
      if (!active()) return;
      setOutcome('held');
      setMutationError(changesSaved ? 'Quote changes were saved, but approval could not be confirmed. Reload the quote to reconcile before retrying.'
          : 'Could not confirm saved or queued changes. Keep your edits and reconcile before retrying.');
    } finally {
      if (active()) busy.current = false;
    }
  };

  if (loading) {
    return <div className="p-8 text-center">Loading quote...</div>;
  }

  if (error || !quoteData) {
    return <div className="p-8 text-center text-red-600">{error || 'Quote not found'}</div>;
  }

  const { quote } = quoteData;
  const totalCents = lineItems.reduce((sum, item) => sum + (item.unit_price_cents * item.quantity), 0);
  const total = termsAvailable ? `$${(totalCents / 100).toFixed(2)}` : 'Not available';

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter">
      <header className="px-6 py-4 bg-white/65 backdrop-blur-3xl saturate-200 border-b border-white/40 sticky top-0 z-10 flex items-center justify-between shadow-sm">
        <h1 className="text-xl font-bold font-outfit text-[#1D1D1F]">Project Proposal</h1>
        <div className="text-sm px-3 py-1 bg-[#0066FF]/10 text-[#0066FF] rounded-full font-medium">
          {quote.status}
        </div>
      </header>

      <main className="p-4 md:p-10 flex-1 max-w-3xl mx-auto w-full">
        <div className="bg-white/65 backdrop-blur-3xl saturate-200 shadow-sm border border-white/40 overflow-hidden">
          <div className="p-6 md:p-8 border-b border-gray-100">
            <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] mb-2">Quote Summary</h2>
            <p className="text-gray-600">Review the scope and pricing below. You can adjust the quantity and price if needed.</p>
          </div>

          <div className="p-6 md:p-8">
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-[#1D1D1F] border-b border-gray-200 pb-2">Line Items</h3>
              {lineItems.map((item) => (
                <div key={item.id} className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 p-4 bg-gray-50 border border-gray-100">
                  <div className="flex-1">
                    <h4 className="font-medium text-[#1D1D1F]">{item.description}</h4>
                  </div>
                  <div className="flex items-center gap-4 self-end sm:self-auto">
                    <div className="flex items-center gap-2">
                      <label htmlFor={`quote-item-quantity-${item.id}`} className="text-xs text-gray-500 font-medium uppercase tracking-wider">Qty{' '}<span className="sr-only">for {item.description}</span></label>
                      <input
                        id={`quote-item-quantity-${item.id}`}
                        type="number"
                        min="1"
                        value={item.quantity}
                        onChange={(e) => handleItemChange(item.id, 'quantity', parseInt(e.target.value) || 1)}
                        className="w-16 px-2 py-1.5 text-sm bg-white border border-gray-300 focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-center text-[#1D1D1F]"
                        disabled={readOnly}
                        data-testid={`quote-item-quantity-${item.id}`}
                      />
                    </div>
                    <div className="flex items-center gap-2">
                      <label htmlFor={`quote-item-price-${item.id}`} className="text-xs text-gray-500 font-medium uppercase tracking-wider">Price ($){' '}<span className="sr-only">for {item.description}</span></label>
                      <input
                        id={`quote-item-price-${item.id}`}
                        type="number"
                        min="0"
                        step="0.01"
                        value={(item.unit_price_cents / 100).toFixed(2)}
                        onChange={(e) => handleItemChange(item.id, 'unit_price_cents', Math.round(parseFloat(e.target.value || '0') * 100))}
                        className="w-24 px-2 py-1.5 text-sm bg-white border border-gray-300 focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-right text-[#1D1D1F]"
                        disabled={readOnly}
                        data-testid={`quote-item-price-${item.id}`}
                      />
                    </div>
                  </div>
                </div>
              ))}

              <div className="pt-6 mt-6 border-t border-gray-200">
                <div className="flex justify-between items-center">
                  <span className="text-xl font-bold text-[#1D1D1F] font-outfit">Total Estimate</span>
                  <span className="text-2xl font-bold text-[#0066FF] font-outfit" data-testid="quote-total">{total}</span>
                </div>
              </div>
            </div>
          </div>

          {(!eligibleStatus || !termsAvailable) && !accepted && <p role="status" className="p-6 text-gray-700">This quote is not ready for approval. Reload when complete, eligible terms have been saved.</p>}
          {!versionAvailable && <p role="alert" className="p-6 text-red-700">Quote version is unavailable. Reload before making changes.</p>}
          {mutationError && <p role="alert" className="p-6 text-red-700">{mutationError}</p>}
          {notice && <p role="status" className="p-6 text-gray-700">{notice}</p>}
          {!accepted && (
            <div className="p-6 bg-gray-50 border-t border-gray-100 flex flex-col sm:flex-row gap-4">
              <button
                onClick={handleApproveAndSend}
                disabled={readOnly}
                className="w-full min-h-[44px] py-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold shadow-sm transition-all text-lg flex items-center justify-center active:scale-[0.98]"
                data-testid="quote-approve-btn"
              >
                {outcome === 'saving' ? 'Saving approval...' : 'Approve quote'}
              </button>
            </div>
          )}
          {accepted && offlineAccepted && (
            <div className="p-6 bg-[#34C759]/10 border-t border-[#34C759]/20 text-center">
              <div className="text-[#34C759] text-4xl mb-2">✅</div>
              <h3 className="text-lg font-bold text-[#1D1D1F]">Proposal Accepted</h3>
              <p className="text-gray-600 text-sm mt-1">Thank you! This quote has been approved.</p>
            </div>
          )}
          {accepted && !offlineAccepted && (
            <div className="p-6 bg-[#34C759]/10 border-t border-[#34C759]/20 text-center">
              <div className="text-[#34C759] text-4xl mb-2">✅</div>
              <h3 className="text-lg font-bold text-[#1D1D1F]">Recorded quote status: ACCEPTED</h3>
              <p className="text-gray-600 text-sm mt-1">Customer acceptance is recorded. Payment and delivery are not established by this page.</p>
            </div>
          )}
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `

        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}

export default function QuotingPage() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <QuotingContent />
    </Suspense>
  );
}
