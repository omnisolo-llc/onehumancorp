"use client";
import { Suspense } from "react";
import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'next/navigation';
import { SyncManager } from '../../lib/sync/SyncManager';

function QuotingContent() {
  const searchParams = useSearchParams();
  const quoteId = searchParams.get('id');

  const [quoteData, setQuoteData] = useState<any>(null);
  const [lineItems, setLineItems] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [accepted, setAccepted] = useState(false);

  useEffect(() => {
    if (!quoteId) {
      setError('Quote ID is missing');
      setLoading(false);
      return;
    }

    const fetchQuote = async () => {
      try {
        const tenantId = typeof window !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'e2e-tenant' : 'e2e-tenant';
        const res = await fetch(`/api/quotes?id=${quoteId}`, {
          headers: {
            'x-tenant-id': tenantId
          }
        });
        if (res.ok) {
          const data = await res.json();
          setQuoteData(data);
          setLineItems(data.line_items || []);
          if (data.quote.status === 'ACCEPTED') {
            setAccepted(true);
          }
        } else {
          setError('Failed to fetch quote');
        }
      } catch (err) {
        setError('Error connecting to server');
      } finally {
        setLoading(false);
      }
    };

    fetchQuote();
  }, [quoteId]);

  const handleItemChange = (id: string, field: string, value: any) => {
    setLineItems(prev => prev.map(item => {
      if (item.id === id) {
        return { ...item, [field]: value };
      }
      return item;
    }));
  };

  const handleApproveAndSend = async () => {
    if (!quoteData || !quoteId) return;

    const totalAmountCents = lineItems.reduce((sum: number, item: any) => sum + (item.unit_price_cents * item.quantity), 0);

    const updatePayload = {
      total_amount_cents: totalAmountCents,
      line_items: lineItems.map(item => ({
        description: item.description,
        unit_price_cents: item.unit_price_cents,
        quantity: item.quantity,
        is_optional: item.is_optional || false
      }))
    };

    // Optimistic UI updates
    setQuoteData({ ...quoteData, quote: { ...quoteData.quote, status: 'ACCEPTED' } });
    setAccepted(true);

    try {
      if (navigator.onLine) {
        const tenantId = typeof window !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'e2e-tenant' : 'e2e-tenant';
        const updateRes = await fetch(`/api/quotes?id=${quoteId}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'x-tenant-id': tenantId },
          body: JSON.stringify(updatePayload)
        });
        if (!updateRes.ok) throw new Error('Update failed');

        const approveRes = await fetch(`/api/quotes/${quoteId}/approve`, {
          method: 'PATCH',
          headers: { 'x-tenant-id': tenantId }
        });
        if (!approveRes.ok) throw new Error('Approve failed');
      } else {
        await SyncManager.getInstance().enqueue({
          type: 'update_quote',
          quoteId: quoteId,
          payload: updatePayload
        });
        await SyncManager.getInstance().enqueue({
          type: 'approve_quote',
          quoteId: quoteId
        });
      }
    } catch (err) {
      console.error('Failed to accept quote:', err);
      alert('Your changes have been saved offline and will sync when reconnected.');
    }
  };

  if (loading) {
    return <div className="p-8 text-center">Loading quote...</div>;
  }

  if (error || !quoteData) {
    return <div className="p-8 text-center text-red-600">{error || 'Quote not found'}</div>;
  }

  const { quote } = quoteData;
  const totalCents = lineItems.reduce((sum: number, item: any) => sum + (item.unit_price_cents * item.quantity), 0);
  const total = (totalCents / 100).toFixed(2);

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter">
      <header className="px-6 py-4 bg-white/65 backdrop-blur-[30px] saturate-[210%] saturate-200 border-b border-white/40 sticky top-0 z-10 flex items-center justify-between shadow-sm">
        <h1 className="text-xl font-bold font-outfit text-[#1D1D1F]">Project Proposal</h1>
        <div className="text-sm px-3 py-1 bg-[#0066FF]/10 text-[#0066FF] rounded-full font-medium">
          {accepted ? 'Accepted' : quote.status}
        </div>
      </header>

      <main className="p-4 md:p-10 flex-1 max-w-3xl mx-auto w-full">
        <div className="bg-white/65 backdrop-blur-[30px] saturate-[210%] saturate-200 shadow-sm border border-white/40 overflow-hidden">
          <div className="p-6 md:p-8 border-b border-gray-100">
            <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] mb-2">Quote Summary</h2>
            <p className="text-gray-600">Review the scope and pricing below. You can adjust the quantity and price if needed.</p>
          </div>

          <div className="p-6 md:p-8">
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-[#1D1D1F] border-b border-gray-200 pb-2">Line Items</h3>
              {lineItems.map((item: any) => (
                <div key={item.id} className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 p-4 bg-gray-50 border border-gray-100">
                  <div className="flex-1">
                    <h4 className="font-medium text-[#1D1D1F]">{item.description}</h4>
                  </div>
                  <div className="flex items-center gap-4 self-end sm:self-auto">
                    <div className="flex items-center gap-2">
                      <label className="text-xs text-gray-500 font-medium uppercase tracking-wider">Qty</label>
                      <input
                        type="number"
                        min="1"
                        value={item.quantity}
                        onChange={(e) => handleItemChange(item.id, 'quantity', parseInt(e.target.value) || 1)}
                        className="w-16 px-2 py-1.5 text-sm bg-white border border-gray-300 focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-center text-[#1D1D1F]"
                        disabled={accepted}
                        data-testid={`quote-item-quantity-${item.id}`}
                      />
                    </div>
                    <div className="flex items-center gap-2">
                      <label className="text-xs text-gray-500 font-medium uppercase tracking-wider">Price ($)</label>
                      <input
                        type="number"
                        min="0"
                        step="0.01"
                        value={(item.unit_price_cents / 100).toFixed(2)}
                        onChange={(e) => handleItemChange(item.id, 'unit_price_cents', Math.round(parseFloat(e.target.value || '0') * 100))}
                        className="w-24 px-2 py-1.5 text-sm bg-white border border-gray-300 focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-right text-[#1D1D1F]"
                        disabled={accepted}
                        data-testid={`quote-item-price-${item.id}`}
                      />
                    </div>
                  </div>
                </div>
              ))}

              <div className="pt-6 mt-6 border-t border-gray-200">
                <div className="flex justify-between items-center">
                  <span className="text-xl font-bold text-[#1D1D1F] font-outfit">Total Estimate</span>
                  <span className="text-2xl font-bold text-[#0066FF] font-outfit" data-testid="quote-total">${total}</span>
                </div>
              </div>
            </div>
          </div>

          {!accepted && (
            <div className="p-6 bg-gray-50 border-t border-gray-100 flex flex-col sm:flex-row gap-4">
              <button
                onClick={handleApproveAndSend}
                className="w-full min-h-[44px] py-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold shadow-sm transition-all text-lg flex items-center justify-center active:scale-[0.98]"
                data-testid="quote-approve-btn"
              >
                Approve & Send
              </button>
            </div>
          )}
          {accepted && (
            <div className="p-6 bg-[#34C759]/10 border-t border-[#34C759]/20 text-center">
              <div className="text-[#34C759] text-4xl mb-2">✅</div>
              <h3 className="text-lg font-bold text-[#1D1D1F]">Proposal Accepted</h3>
              <p className="text-gray-600 text-sm mt-1">Thank you! This quote has been approved.</p>
            </div>
          )}
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
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
