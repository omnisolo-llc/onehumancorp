"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";
import { useSearchParams } from 'next/navigation';
import { Suspense } from 'react';
import { motion, AnimatePresence } from "framer-motion";
import { FiCheck, FiX, FiDollarSign, FiClock, FiPlus, FiMessageSquare, FiLoader } from "react-icons/fi";

interface QuoteItem {
  id: string;
  description: string;
  unit_price_cents: number;
  quantity: number;
  is_optional: boolean;
  selected?: boolean; // UI state for optional items
}

interface Quote {
  id: string;
  customer_id: string;
  status: 'DRAFT' | 'PENDING_APPROVAL' | 'SENT' | 'ACCEPTED';
  items?: QuoteItem[];
}

export default function MobileQuotingPage() {
  return (
    <Suspense fallback={<div className="flex h-screen items-center justify-center bg-gray-50 text-gray-500">Loading quote assistant...</div>}>
      <MobileQuotingPageContent />
    </Suspense>
  );
}

function MobileQuotingPageContent() {
  const searchParams = useSearchParams();
  const quoteId = searchParams.get('id');
  const approvalId = searchParams.get('approval_id');
  const [quote, setQuote] = useState<Quote | null>(null);
  const [items, setItems] = useState<QuoteItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [approving, setApproving] = useState(false);

  useEffect(() => {
    if (quoteId) {
      fetchQuote(quoteId);
    }
  }, [quoteId]);

  const fetchQuote = async (id: string) => {
    try {
      setLoading(true);
      const [quoteRes, itemsRes] = await Promise.all([
        fetch(`/api/v1/quoting/quotes/${id}`),
        fetch(`/api/v1/quoting/quotes/${id}/items`)
      ]);

      if (quoteRes.ok && itemsRes.ok) {
        const quoteData = await quoteRes.json();
        const itemsData = await itemsRes.json();
        setQuote(quoteData);
        setItems(itemsData.map((item: QuoteItem) => ({
            ...item,
            selected: !item.is_optional // Initially select non-optional items
        })));
      }
    } catch (error) {
      console.error("Failed to fetch quote:", error);
    } finally {
      setLoading(false);
    }
  };

  const toggleOptionalItem = async (itemId: string) => {
    const item = items.find(i => i.id === itemId);
    if (!item) return;

    const newSelected = !item.selected;
    setItems(prev => prev.map(i => i.id === itemId ? { ...i, selected: newSelected } : i));

    // If it's saved in DB as is_optional, we might want to update its status or just keep UI selection
    // For this flow, "selected" is UI only until approval, then we could filter items.
    // Or we update the DB's is_optional to reflect "wanted" vs "unwanted" optional.
  };

  const updateItemPrice = async (itemId: string, newPriceCents: number) => {
    try {
      const res = await fetch(`/api/v1/quoting/quotes/${quoteId}/items/${itemId}`, {
        method: 'PATCH',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ unit_price_cents: newPriceCents })
      });

      if (res.ok) {
        const updatedItem = await res.json();
        setItems(prev => prev.map(i => i.id === itemId ? { ...i, unit_price_cents: updatedItem.unit_price_cents } : i));
      }
    } catch (error) {
      console.error("Failed to update price:", error);
    }
  };

  const addCustomItem = async () => {
    const description = prompt("Item description:");
    const price = parseFloat(prompt("Price ($):") || "0");

    if (description && !isNaN(price)) {
        try {
            const res = await fetch(`/api/v1/quoting/quotes/${quoteId}/items`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    description,
                    unit_price_cents: Math.round(price * 100),
                    quantity: 1,
                    is_optional: false
                })
            });
            if (res.ok) {
                const newItem = await res.json();
                setItems(prev => [...prev, { ...newItem, selected: true }]);
            }
        } catch (error) {
            console.error("Failed to add item:", error);
        }
    }
  };

  const calculateTotal = () => {
    return items
      .filter(item => item.selected)
      .reduce((sum, item) => sum + (item.unit_price_cents * item.quantity), 0) / 100;
  };

  const handleApprove = async () => {
    if (!quote || !approvalId) return;
    setApproving(true);
    try {
      const res = await fetch(`/api/agents/approvals/${approvalId}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ approved: true }),
      });

      if (res.ok) {
        setQuote({ ...quote, status: 'SENT' });
      }
    } catch (error) {
      console.error("Failed to approve:", error);
    } finally {
      setApproving(false);
    }
  };

  if (loading) {
    return (
      <div className="flex h-screen flex-col items-center justify-center bg-gray-50 text-gray-500 space-y-4">
        <FiLoader className="text-3xl animate-spin text-blue-600" />
        <p className="font-medium">Loading AI-drafted quote...</p>
      </div>
    );
  }

  if (!quote) {
    return (
      <div className="flex h-screen items-center justify-center bg-gray-50 text-gray-500 p-8 text-center">
        <div className="max-w-xs">
            <FiX className="text-4xl mx-auto mb-4 text-gray-300" />
            <p>We couldn't find that quote. It may have been deleted or moved.</p>
            <Link href="/dashboard" className="mt-6 inline-block text-blue-600 font-semibold">Return to Dashboard</Link>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100 font-sans pb-24">
      <header className="sticky top-0 z-10 bg-white/80 backdrop-blur-xl border-b border-gray-200 px-4 py-4 shadow-sm">
        <div className="flex items-center justify-between">
          <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <FiX className="text-2xl" />
          </Link>
          <h1 className="text-lg font-semibold text-gray-900">Review Draft Quote</h1>
          <div className="w-6" />
        </div>
      </header>

      <main className="px-4 py-6 max-w-md mx-auto space-y-6">
        <section className="app-card rounded-2xl p-5 shadow-sm border border-gray-100 bg-white">
          <div className="flex items-start space-x-4">
            <div className="w-12 h-12 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center font-bold text-xl">
              C
            </div>
            <div>
              <h2 className="font-semibold text-gray-900">Project Context</h2>
              <p className="text-sm text-gray-500 mt-1">This quote was generated automatically based on the recent customer inquiry.</p>
            </div>
          </div>
        </section>

        <div className="flex items-center space-x-2 text-sm text-purple-700 bg-purple-50 px-3 py-2 rounded-lg font-medium border border-purple-100">
          <FiMessageSquare className="shrink-0" />
          <span>AI drafted this using the "Standard Service" heuristic</span>
        </div>

        <section className="space-y-4">
          <h3 className="font-semibold text-gray-900 px-1">Itemized Breakdown</h3>

          <div className="space-y-3">
            <AnimatePresence>
              {items.map((item) => (
                <motion.div
                  key={item.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  className={`bg-white rounded-xl p-4 border transition-all ${item.selected ? 'border-gray-200 shadow-sm' : 'border-gray-100 opacity-60'}`}
                >
                  <div className="flex justify-between items-start mb-2">
                    <div className="flex items-center space-x-3">
                      {item.is_optional && (
                        <button
                          onClick={() => toggleOptionalItem(item.id)}
                          className={`w-6 h-6 rounded-full flex items-center justify-center border transition-colors ${item.selected ? 'bg-blue-600 border-blue-600 text-white' : 'border-gray-300 text-transparent'}`}
                        >
                          <FiCheck className="text-sm" />
                        </button>
                      )}
                      <div>
                        <p className={`font-medium ${item.selected ? 'text-gray-900' : 'text-gray-500'}`}>{item.description}</p>
                        {item.is_optional && <span className="text-xs text-gray-400 font-medium bg-gray-100 px-2 py-0.5 rounded-full mt-1 inline-block">Optional</span>}
                      </div>
                    </div>
                  </div>

                  <div className="flex justify-end mt-3 pl-9">
                    <div className="relative">
                      <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 font-medium">$</span>
                      <input
                        type="number"
                        inputMode="decimal"
                        value={item.unit_price_cents / 100}
                        onChange={(e) => updateItemPrice(item.id, Math.round(parseFloat(e.target.value) * 100) || 0)}
                        className={`w-28 pl-7 pr-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-right font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all ${!item.selected && 'opacity-50'}`}
                        disabled={!item.selected}
                      />
                    </div>
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>

          <button
            onClick={addCustomItem}
            className="w-full py-3 flex items-center justify-center space-x-2 text-blue-600 font-medium bg-blue-50 rounded-xl hover:bg-blue-100 transition-colors"
          >
            <FiPlus />
            <span>Add custom item</span>
          </button>
        </section>

        <section className="bg-gray-50 rounded-xl p-4 border border-gray-200">
           <div className="flex justify-between items-center mb-1">
             <span className="text-gray-600 font-medium">Total Estimate</span>
             <span className="text-xl font-bold text-gray-900">${calculateTotal().toFixed(2)}</span>
           </div>
           <div className="flex justify-between items-center text-sm">
             <span className="text-gray-500">Deposit required to book (20%)</span>
             <span className="text-gray-700 font-semibold">${(calculateTotal() * 0.2).toFixed(2)}</span>
           </div>
        </section>
      </main>

      <div className="fixed bottom-0 left-0 right-0 p-4 bg-white/90 backdrop-blur-xl border-t border-gray-200 shadow-lg pb-safe">
        <div className="max-w-md mx-auto flex space-x-3">
          <button className="flex-1 py-3.5 px-4 bg-gray-100 text-gray-700 font-semibold rounded-xl hover:bg-gray-200 transition-colors">
            Edit Later
          </button>
          <button
            onClick={handleApprove}
            disabled={quote.status === 'SENT' || approving}
            className={`flex-[2] py-3.5 px-4 font-semibold rounded-xl transition-all shadow-sm flex justify-center items-center space-x-2 ${
                quote.status === 'SENT'
                ? 'bg-green-500 text-white'
                : 'bg-blue-600 hover:bg-blue-700 text-white hover:shadow-md disabled:bg-blue-400'
            }`}
          >
            {approving ? (
                <FiLoader className="animate-spin text-lg" />
            ) : quote.status === 'SENT' ? (
                <>
                  <FiCheck className="text-lg" />
                  <span>Sent to Customer</span>
                </>
            ) : (
                <span>Approve & Send</span>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
