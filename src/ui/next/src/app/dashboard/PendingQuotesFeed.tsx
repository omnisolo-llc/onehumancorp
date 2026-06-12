import React, { useState, useEffect, useMemo } from 'react';

type PendingQuote = {
  id: string;
  tenant_id: string;
  customer_id?: string;
  total_amount: number;
  required_deposit: number;
  status: string;
  expires_at?: string;
};

export function PendingQuotesFeed({ tenantId }: { tenantId: string }) {
  const [quotes, setQuotes] = useState<PendingQuote[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [selectedQuoteId, setSelectedQuoteId] = useState<string | null>(null);

  useEffect(() => {
    async function fetchQuotes() {
      try {
        setLoading(true);
        // We will need to create this API endpoint or use a dummy fetch for now
        const res = await fetch(`/api/ui/quotes?tenant_id=${encodeURIComponent(tenantId)}`);
        if (!res.ok) throw new Error("Failed to fetch pending quotes");
        const data = await res.json();
        setQuotes(Array.isArray(data) ? data.filter((q: PendingQuote) => q.status === 'proposed') : []);
      } catch (err: any) {
        setError(err.message || "Unknown error");
      } finally {
        setLoading(false);
      }
    }
    fetchQuotes();
  }, [tenantId]);

  const selectedQuote = useMemo(() => quotes.find(q => q.id === selectedQuoteId), [quotes, selectedQuoteId]);

  const handleApprove = async (id: string) => {
    try {
      const res = await fetch(`/api/ui/quotes/action?tenant_id=${encodeURIComponent(tenantId)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ quote_id: id, approved: true })
      });
      if (res.ok) {
        setQuotes(prev => prev.filter(q => q.id !== id));
        setSelectedQuoteId(null);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleEdit = (id: string) => {
    // In a real app this would pop open a bottom sheet
    alert("Edit bottom sheet placeholder for Quote ID: " + id);
  };

  if (loading) return null;
  if (quotes.length === 0) return null;

  return (
    <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-blue-400/50 dark:border-blue-500/30 bg-blue-50/50 dark:bg-blue-900/10 shadow-lg relative overflow-hidden">
      <div className="absolute top-0 left-0 w-1 h-full bg-blue-500"></div>
      <div className="flex justify-between items-start mb-3">
        <div>
          <h2 className="text-xl font-bold font-outfit text-blue-900 dark:text-blue-100 flex items-center gap-2">
            <span className="text-2xl">📝</span> Pending Quotes
          </h2>
          <p className="text-blue-800/80 dark:text-blue-200/80 mt-1 text-sm font-medium">Review and approve AI-drafted quotes</p>
        </div>
      </div>

      <div className="mt-4 space-y-4">
        {quotes.map(quote => (
          <div key={quote.id} className="p-4 rounded-xl bg-white/60 dark:bg-black/40 border border-blue-200 dark:border-blue-900/50 cursor-pointer transition-colors hover:bg-white/80 dark:hover:bg-black/60" onClick={() => setSelectedQuoteId(quote.id)}>
             <div className="flex justify-between items-center">
                 <div>
                    <div className="text-sm font-semibold text-gray-900 dark:text-gray-100">Quote Draft #{quote.id.substring(0, 8)}</div>
                    <div className="text-xs text-gray-600 dark:text-gray-400 mt-1">Status: {quote.status}</div>
                 </div>
                 <div className="text-right">
                    <div className="text-sm font-bold text-gray-900 dark:text-gray-100">${(quote.total_amount / 100).toFixed(2)}</div>
                    <div className="text-xs text-blue-600 dark:text-blue-400">Deposit: ${(quote.required_deposit / 100).toFixed(2)}</div>
                 </div>
             </div>

             {selectedQuoteId === quote.id && (
                 <div className="mt-4 pt-4 border-t border-blue-100 dark:border-blue-900/30">
                    <div className="flex gap-3">
                      <button
                        onClick={(e) => { e.stopPropagation(); handleApprove(quote.id); }}
                        className="px-6 py-2.5 flex-1 rounded-[16px] bg-blue-500 hover:bg-blue-600 text-white font-medium shadow-sm transition-colors min-h-[44px]"
                      >
                        Approve & Send
                      </button>
                      <button
                        onClick={(e) => { e.stopPropagation(); handleEdit(quote.id); }}
                        className="px-6 py-2.5 flex-1 rounded-[16px] bg-white/50 dark:bg-black/30 border border-blue-200 dark:border-blue-900/30 hover:bg-white/80 dark:hover:bg-black/50 text-blue-900 dark:text-blue-100 font-medium transition-colors min-h-[44px]"
                      >
                        Edit Quote
                      </button>
                    </div>
                 </div>
             )}
          </div>
        ))}
      </div>
    </div>
  );
}
