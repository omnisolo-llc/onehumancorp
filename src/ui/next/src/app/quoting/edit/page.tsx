"use client";
import { Suspense, useState, useEffect } from "react";
import { useSearchParams, useRouter } from 'next/navigation';

function EditQuoteContent() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const quoteId = searchParams.get('id');
  const feedItemId = searchParams.get('feed_item_id');

  const [quoteData, setQuoteData] = useState<any>(null);
  const [lineItems, setLineItems] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (!quoteId) {
      setError('Quote ID is missing');
      setLoading(false);
      return;
    }

    const fetchQuote = async () => {
      try {
        const res = await fetch(`/api/v1/quotes/${quoteId}`, {
          headers: { 'x-tenant-id': 'test-tenant' } // Ideally we don't hardcode this, but relying on context/cookie
        });
        if (res.ok) {
          const data = await res.json();
          setQuoteData(data);
          setLineItems(data.line_items || []);
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

  const handleUpdateItem = (index: number, field: string, value: any) => {
    const newItems = [...lineItems];
    newItems[index] = { ...newItems[index], [field]: value };
    setLineItems(newItems);
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      const res = await fetch(`/api/v1/quotes/${quoteId}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', 'x-tenant-id': 'test-tenant' },
        body: JSON.stringify({ line_items: lineItems })
      });
      if (res.ok) {
        if (feedItemId) {
           // Also approve the feed item
           await fetch(`/api/agent-feed/${feedItemId}`, {
              method: 'PUT',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ state: 'APPROVED' }),
           });
        }
        router.push('/feed');
      } else {
        alert('Failed to save quote');
      }
    } catch (err) {
      alert('Error connecting to server');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return <div className="p-8 text-center flex-1 flex items-center justify-center">Loading quote...</div>;
  }

  if (error || !quoteData) {
    return <div className="p-8 text-center text-red-600 flex-1 flex items-center justify-center">{error || 'Quote not found'}</div>;
  }

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter">
      <header className="px-6 py-4 bg-white border-b sticky top-0 z-10 flex items-center justify-between shadow-sm">
        <h1 className="text-xl font-bold font-outfit text-gray-900">Edit Quote Draft</h1>
      </header>

      <main className="p-4 md:p-6 flex-1 max-w-lg mx-auto w-full">
        <div className="bg-white/80 backdrop-blur-md rounded-2xl shadow-lg border border-gray-100 overflow-hidden">
          <div className="p-6 border-b border-gray-100">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">Adjust Line Items</h2>
            <p className="text-sm text-gray-500">Modify the suggested quote before sending.</p>
          </div>

          <div className="p-6 space-y-6">
            {lineItems.map((item, idx) => (
              <div key={item.id || idx} className="p-4 bg-gray-50 rounded-xl border border-gray-200">
                <div className="mb-3">
                  <label className="block text-xs font-semibold text-gray-600 mb-1 uppercase tracking-wide">Description</label>
                  <input
                    type="text"
                    value={item.description}
                    onChange={(e) => handleUpdateItem(idx, 'description', e.target.value)}
                    className="w-full bg-white border border-gray-300 rounded-lg px-3 py-2 min-h-[44px] text-gray-900 focus:ring-2 focus:ring-[#0066FF] outline-none"
                    data-testid={`quote-item-desc-${idx}`}
                  />
                </div>
                <div className="flex gap-4">
                  <div className="flex-1">
                    <label className="block text-xs font-semibold text-gray-600 mb-1 uppercase tracking-wide">Price ($)</label>
                    <input
                      type="number"
                      value={item.unit_price_cents / 100}
                      onChange={(e) => handleUpdateItem(idx, 'unit_price_cents', Math.round(parseFloat(e.target.value) * 100))}
                      className="w-full bg-white border border-gray-300 rounded-lg px-3 py-2 min-h-[44px] text-gray-900 focus:ring-2 focus:ring-[#0066FF] outline-none"
                      data-testid={`quote-item-price-${idx}`}
                    />
                  </div>
                  <div className="flex-1">
                    <label className="block text-xs font-semibold text-gray-600 mb-1 uppercase tracking-wide">Qty</label>
                    <input
                      type="number"
                      value={item.quantity}
                      onChange={(e) => handleUpdateItem(idx, 'quantity', parseInt(e.target.value, 10))}
                      className="w-full bg-white border border-gray-300 rounded-lg px-3 py-2 min-h-[44px] text-gray-900 focus:ring-2 focus:ring-[#0066FF] outline-none"
                      data-testid={`quote-item-qty-${idx}`}
                    />
                  </div>
                </div>
              </div>
            ))}
          </div>

          <div className="p-6 bg-white border-t border-gray-100 flex gap-4">
            <button
              onClick={() => router.push('/feed')}
              className="flex-1 py-3 px-4 bg-gray-100 hover:bg-gray-200 text-gray-800 font-bold rounded-xl min-h-[44px] transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleSave}
              disabled={saving}
              className="flex-1 py-3 px-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold rounded-xl min-h-[44px] transition-colors flex items-center justify-center shadow-md"
              data-testid="save-quote-btn"
            >
              {saving ? 'Saving...' : 'Approve & Send'}
            </button>
          </div>
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

export default function EditQuotePage() {
  return (
    <Suspense fallback={<div className="p-8 text-center flex-1 flex items-center justify-center">Loading...</div>}>
      <EditQuoteContent />
    </Suspense>
  );
}
