'use client';

import React, { useEffect, useState } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { AppShell } from '../../components/AppShell';

interface LineItem {
  id: string;
  description: string;
  unit_price_cents: number;
  quantity: number;
  is_optional: boolean;
}

interface Quote {
  id: string;
  customer_id: string;
  status: string;
  total_amount_cents: number;
  required_deposit_cents: number;
  stripe_payment_link?: string;
  proposed_slot_id?: string;
  line_items?: LineItem[];
}

export default function QuoteReviewPage() {
  const params = useParams();
  const router = useRouter();
  const id = params.id as string;
  const [quote, setQuote] = useState<Quote | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [sending, setSending] = useState(false);
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    async function fetchQuote() {
      try {
        const res = await fetch(`/api/quotes/${id}`);
        if (!res.ok) throw new Error('Failed to fetch quote');
        const data = await res.json();
        setQuote(data);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }
    fetchQuote();
  }, [id]);

  const handleSend = async () => {
    try {
      setSending(true);
      const res = await fetch(`/api/quotes?id=${id}`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ ...quote, status: 'SENT' }) });
      if (!res.ok) throw new Error('Failed to send quote');
      const updated = await res.json();
      setQuote(updated);
      if (updated.stripe_payment_link) {
        alert('Quote Sent!');
      }
    } catch (err: any) {
      alert(err.message);
    } finally {
      setSending(false);
    }
  };

  const handleUpdateLineItem = (itemId: string, newPrice: number) => {
    if (!quote) return;
    const newItems = quote.line_items?.map(item =>
      item.id === itemId ? { ...item, unit_price_cents: newPrice } : item
    );
    const newTotal = newItems?.reduce((sum, item) => sum + (item.unit_price_cents * item.quantity), 0) || 0;
    setQuote({
      ...quote,
      line_items: newItems,
      total_amount_cents: newTotal,
      required_deposit_cents: Math.floor(newTotal / 3)
    });
  };

  const saveQuoteChanges = async () => {
    try {
      setSending(true);
      const res = await fetch(`/api/quotes?id=${id}`, {
        method: 'POST', // Based on route.ts POST handles update if id is present
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(quote)
      });
      if (!res.ok) throw new Error('Failed to save changes');
      setIsEditing(false);
    } catch (err: any) {
      alert(err.message);
    } finally {
      setSending(false);
    }
  };

  if (loading) return <AppShell title="Loading Quote..."><div className="p-4 text-center">Loading...</div></AppShell>;
  if (error) return <AppShell title="Error"><div className="p-4 text-center text-[#FF3B30]">{error}</div></AppShell>;
  if (!quote) return <AppShell title="Not Found"><div className="p-4 text-center">Quote not found</div></AppShell>;

  return (
    <AppShell title="Review Estimate" subtitle={`Quote #${id.slice(0, 8)}`}>
      <div className="w-full max-w-md mx-auto p-4 space-y-6">
        <div className="glassmorphism p-6 space-y-4">
          <div className="flex justify-between items-center">
            <span className="text-sm font-medium text-gray-500">Status</span>
            <span className={`text-xs font-bold px-2 py-1 rounded-full ${quote.status === 'ACCEPTED' ? 'bg-green-100 text-green-700' : 'bg-blue-100 text-blue-700'}`}>
              {quote.status}
            </span>
          </div>

          <div className="space-y-3">
            <div className="flex justify-between items-center">
              <h3 className="text-[11px] font-bold uppercase tracking-wider text-gray-400">Line Items</h3>
              {quote.status === 'DRAFT' && !isEditing && (
                <button onClick={() => setIsEditing(true)} id="edit-quote-btn" className="text-[10px] text-[#0066FF] font-bold">EDIT</button>
              )}
            </div>
            {quote.line_items?.map((item) => (
              <div key={item.id} className="flex flex-col gap-1 py-2 border-b border-gray-50 dark:border-gray-800 last:border-0">
                <div className="flex justify-between text-sm">
                  <span>{item.description} (x{item.quantity})</span>
                  {isEditing ? (
                    <div className="flex items-center gap-1">
                      <span className="text-gray-400">$</span>
                      <input
                        type="number"
                        value={(item.unit_price_cents / 100).toFixed(2)}
                        onChange={(e) => handleUpdateLineItem(item.id, Math.round(parseFloat(e.target.value) * 100))}
                        className="w-20 text-right bg-gray-100 dark:bg-gray-800 rounded px-1 focus:outline-none focus:ring-1 focus:ring-[#0066FF]"
                      />
                    </div>
                  ) : (
                    <span className="font-medium">${((item.unit_price_cents * item.quantity) / 100).toFixed(2)}</span>
                  )}
                </div>
              </div>
            ))}
          </div>

          <div className="pt-4 border-t border-gray-100 dark:border-gray-800 space-y-2">
            <div className="flex justify-between items-center font-bold">
              <span>Total Amount</span>
              <span>${(quote.total_amount_cents / 100).toFixed(2)}</span>
            </div>
            <div className="flex justify-between items-center text-sm text-gray-500">
              <span>Required Deposit</span>
              <span>${(quote.required_deposit_cents / 100).toFixed(2)}</span>
            </div>
            {quote.proposed_slot_id && (
              <div className="flex justify-between items-center text-sm text-indigo-600 dark:text-indigo-400 font-medium">
                <span>Proposed Schedule</span>
                <span>Slot Available</span>
              </div>
            )}
          </div>

          {quote.stripe_payment_link && (
            <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <p className="text-[11px] font-bold text-[#0071E3] dark:text-blue-400 mb-1 uppercase">Stripe Payment Link</p>
              <a href={quote.stripe_payment_link} target="_blank" className="text-sm text-[#0066FF] underline break-all">
                {quote.stripe_payment_link}
              </a>
            </div>
          )}
        </div>

        {quote.status === 'DRAFT' && (
          isEditing ? (
            <button
              id="btn-save-edits"
              onClick={saveQuoteChanges}
              disabled={sending}
              className="w-full min-h-[44px] bg-[#0066FF] text-white font-bold shadow-lg hover:bg-[#0052CC] transition-all disabled:opacity-50"
            >
              {sending ? 'Saving...' : 'Save Changes'}
            </button>
          ) : (
            <button
              onClick={handleSend}
              disabled={sending}
              className="w-full min-h-[44px] bg-[#0066FF] text-white font-bold shadow-lg hover:bg-[#0052CC] transition-all disabled:opacity-50"
            >
              {sending ? 'Sending...' : 'Send Quote to Client'}
            </button>
          )
        )}

        <button
          onClick={() => router.back()}
          className="w-full min-h-[44px] border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white font-medium hover:bg-gray-50 dark:hover:bg-gray-800 transition-all"
        >
          Back to Feed
        </button>
      </div>
    </AppShell>
  );
}
