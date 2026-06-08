"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";
import { motion, AnimatePresence } from "framer-motion";
import { FiCheck, FiX, FiDollarSign, FiClock, FiPlus, FiMessageSquare } from "react-icons/fi";

// Mock data type - In a real app this would come from an API
interface QuoteItem {
  id: string;
  description: string;
  price: number;
  quantity: number;
  isOptional: boolean;
  selected: boolean;
}

interface Quote {
  id: string;
  customerName: string;
  customerPhotoUrl?: string;
  requestText: string;
  status: 'DRAFT' | 'PENDING_APPROVAL' | 'SENT' | 'ACCEPTED';
  items: QuoteItem[];
}

export default function MobileQuotingPage() {
  const [quotes, setQuotes] = useState<Quote[]>([]);
  const [activeQuoteId, setActiveQuoteId] = useState<string | null>(null);

  // Mock loading initial data
  useEffect(() => {
    setQuotes([
      {
        id: "quote-1",
        customerName: "Alex Rivera",
        customerPhotoUrl: "https://i.pravatar.cc/150?u=alex",
        requestText: "Hi Carlos, the pipe under my kitchen sink started leaking yesterday. It's a steady drip. Can you take a look?",
        status: "DRAFT",
        items: [
          { id: "item-1", description: "Callout Fee & Diagnostics", price: 75.00, quantity: 1, isOptional: false, selected: true },
          { id: "item-2", description: "Standard P-Trap Replacement", price: 120.00, quantity: 1, isOptional: false, selected: true },
          { id: "item-3", description: "Emergency Weekend Surcharge", price: 50.00, quantity: 1, isOptional: true, selected: false }
        ]
      }
    ]);
    setActiveQuoteId("quote-1");
  }, []);

  const activeQuote = quotes.find(q => q.id === activeQuoteId);

  const toggleOptionalItem = (itemId: string) => {
    setQuotes(prev => prev.map(q => {
      if (q.id === activeQuoteId) {
        return {
          ...q,
          items: q.items.map(item =>
            item.id === itemId ? { ...item, selected: !item.selected } : item
          )
        };
      }
      return q;
    }));
  };

  const updateItemPrice = (itemId: string, newPrice: number) => {
    setQuotes(prev => prev.map(q => {
      if (q.id === activeQuoteId) {
        return {
          ...q,
          items: q.items.map(item =>
            item.id === itemId ? { ...item, price: newPrice } : item
          )
        };
      }
      return q;
    }));
  };

  const calculateTotal = (quote: Quote) => {
    return quote.items
      .filter(item => item.selected)
      .reduce((sum, item) => sum + (item.price * item.quantity), 0);
  };

  const handleApprove = () => {
    if (!activeQuote) return;
    // In a real app, this would make an API call to approve and send via Stripe
    setQuotes(prev => prev.map(q =>
      q.id === activeQuoteId ? { ...q, status: 'SENT' as const } : q
    ));
    setTimeout(() => {
        alert("Quote approved and Stripe Payment Link sent!");
    }, 500);
  };

  if (!activeQuote) {
    return (
      <div className="flex h-screen items-center justify-center bg-gray-50 text-gray-500">
        <p>No active quotes to review.</p>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100 font-sans pb-24">
      {/* Top Navigation */}
      <header className="sticky top-0 z-10 bg-white/80 backdrop-blur-xl border-b border-gray-200 px-4 py-4 shadow-sm">
        <div className="flex items-center justify-between">
          <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <FiX className="text-2xl" />
          </Link>
          <h1 className="text-lg font-semibold text-gray-900">Review Draft Quote</h1>
          <div className="w-6" /> {/* Spacer for centering */}
        </div>
      </header>

      <main className="px-4 py-6 max-w-md mx-auto space-y-6">

        {/* Customer Context Card */}
        <section className="app-card rounded-2xl p-5 shadow-sm border border-gray-100">
          <div className="flex items-start space-x-4">
            {activeQuote.customerPhotoUrl ? (
              <img src={activeQuote.customerPhotoUrl} alt={activeQuote.customerName} className="w-12 h-12 rounded-full object-cover shadow-sm" />
            ) : (
              <div className="w-12 h-12 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center font-bold text-xl">
                {activeQuote.customerName.charAt(0)}
              </div>
            )}
            <div>
              <h2 className="font-semibold text-gray-900">{activeQuote.customerName}</h2>
              <p className="text-sm text-gray-500 mt-1 line-clamp-3">"{activeQuote.requestText}"</p>
            </div>
          </div>
        </section>

        {/* AI Suggestions Badge */}
        <div className="flex items-center space-x-2 text-sm text-purple-700 bg-purple-50 px-3 py-2 rounded-lg font-medium">
          <FiMessageSquare />
          <span>AI drafted this based on "Leaky Pipe" heuristics</span>
        </div>

        {/* Line Items */}
        <section className="space-y-4">
          <h3 className="font-semibold text-gray-900 px-1">Itemized Breakdown</h3>

          <div className="space-y-3">
            <AnimatePresence>
              {activeQuote.items.map((item) => (
                <motion.div
                  key={item.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  className={`bg-white rounded-xl p-4 border transition-all ${item.selected ? 'border-gray-200 shadow-sm' : 'border-gray-100 opacity-60'}`}
                >
                  <div className="flex justify-between items-start mb-2">
                    <div className="flex items-center space-x-3">
                      {item.isOptional && (
                        <button
                          onClick={() => toggleOptionalItem(item.id)}
                          className={`w-6 h-6 rounded-full flex items-center justify-center border transition-colors ${item.selected ? 'bg-blue-600 border-blue-600 text-white' : 'border-gray-300 text-transparent'}`}
                        >
                          <FiCheck className="text-sm" />
                        </button>
                      )}
                      <div>
                        <p className={`font-medium ${item.selected ? 'text-gray-900' : 'text-gray-500'}`}>{item.description}</p>
                        {item.isOptional && <span className="text-xs text-gray-400 font-medium bg-gray-100 px-2 py-0.5 rounded-full mt-1 inline-block">Optional</span>}
                      </div>
                    </div>
                  </div>

                  <div className="flex justify-end mt-3 pl-9">
                    <div className="relative">
                      <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 font-medium">$</span>
                      <input
                        type="number"
                        inputMode="decimal"
                        value={item.price}
                        onChange={(e) => updateItemPrice(item.id, parseFloat(e.target.value) || 0)}
                        className={`w-28 pl-7 pr-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-right font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all ${!item.selected && 'opacity-50'}`}
                        disabled={!item.selected}
                      />
                    </div>
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>

          <button className="w-full py-3 flex items-center justify-center space-x-2 text-blue-600 font-medium bg-blue-50 rounded-xl hover:bg-blue-100 transition-colors">
            <FiPlus />
            <span>Add custom item</span>
          </button>
        </section>

        {/* Deposit Required Section */}
        <section className="bg-gray-50 rounded-xl p-4 border border-gray-200">
           <div className="flex justify-between items-center mb-1">
             <span className="text-gray-600 font-medium">Total Estimate</span>
             <span className="text-xl font-bold text-gray-900">${calculateTotal(activeQuote).toFixed(2)}</span>
           </div>
           <div className="flex justify-between items-center text-sm">
             <span className="text-gray-500">Deposit required to book (50%)</span>
             <span className="text-gray-700 font-semibold">${(calculateTotal(activeQuote) / 2).toFixed(2)}</span>
           </div>
        </section>

      </main>

      {/* Floating Action Bar */}
      <div className="fixed bottom-0 left-0 right-0 p-4 bg-white/90 backdrop-blur-xl border-t border-gray-200 shadow-lg pb-safe">
        <div className="max-w-md mx-auto flex space-x-3">
          <button className="flex-1 py-3.5 px-4 bg-gray-100 text-gray-700 font-semibold rounded-xl hover:bg-gray-200 transition-colors">
            Edit Later
          </button>
          <button
            onClick={handleApprove}
            disabled={activeQuote.status === 'SENT'}
            className={`flex-[2] py-3.5 px-4 font-semibold rounded-xl transition-all shadow-sm flex justify-center items-center space-x-2 ${
                activeQuote.status === 'SENT'
                ? 'bg-green-500 text-white'
                : 'bg-blue-600 hover:bg-blue-700 text-white hover:shadow-md'
            }`}
          >
            {activeQuote.status === 'SENT' ? (
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
