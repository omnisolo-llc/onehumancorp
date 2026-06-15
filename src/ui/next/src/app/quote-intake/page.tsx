"use client";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../../lib/sync/SyncManager';

export default function QuoteIntakePage() {
  const [isOffline, setIsOffline] = useState(false);
  const [jobDetails, setJobDetails] = useState("");
  const [isListening, setIsListening] = useState(false);
  const [quoteDraft, setQuoteDraft] = useState<any>(null);
  const [paymentSaved, setPaymentSaved] = useState(false);

  useEffect(() => {
    setIsOffline(!navigator.onLine);
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);
    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  const handleVoiceInput = () => {
    setIsListening(true);
    setTimeout(() => {
      setJobDetails("I need a quote for deck repair, 10x12");
      setIsListening(false);
      generateQuoteLocally("I need a quote for deck repair, 10x12");
    }, 1500);
  };

  const generateQuoteLocally = (details: string) => {
    // Simple mock logic simulating an edge AI parser
    const price = 15000; // 150.00
    const deposit = 5000; // 50.00

    setQuoteDraft({
      total_amount: price,
      required_deposit: deposit,
      line_items: [
        {
          description: "Deck Repair Service",
          unit_price_cents: price,
          quantity: 1,
          is_optional: false
        }
      ]
    });
  };

  const collectDepositOffline = async () => {
    if (!quoteDraft) return;

    // Simulate Stripe Terminal Tap-to-Pay success offline
    const offlineMutation = {
      id: crypto.randomUUID(),
      type: 'offline_quote_deposit',
      timestamp: Date.now(),
      amount: quoteDraft.required_deposit,
      quoteDetails: {
        total_amount: quoteDraft.total_amount,
        required_deposit: quoteDraft.required_deposit,
        line_items: quoteDraft.line_items,
        notes: jobDetails,
      }
    };

    await SyncManager.getInstance().enqueue(offlineMutation);
    setPaymentSaved(true);
  };

  return (
    <div className="min-h-screen bg-gray-50 font-inter max-w-[375px] mx-auto overflow-hidden relative shadow-lg">
      <header className="px-6 py-4 bg-white/80 backdrop-blur-md border-b sticky top-0 z-10 flex items-center justify-between">
        <h1 className="text-xl font-bold font-outfit text-gray-900">Job Intake</h1>
        {isOffline ? (
          <span className="text-xs px-2 py-1 bg-orange-100 text-orange-700 rounded-full font-semibold border border-orange-200">
            Saved Offline
          </span>
        ) : (
          <span className="text-xs px-2 py-1 bg-green-100 text-green-700 rounded-full font-semibold border border-green-200">
            Online
          </span>
        )}
      </header>

      <main className="p-6">
        {!quoteDraft && !paymentSaved && (
          <div className="flex flex-col items-center justify-center mt-12 space-y-8">
            <div className="text-center">
              <h2 className="text-2xl font-bold text-gray-800 font-outfit mb-2">Capture Demand</h2>
              <p className="text-gray-500 text-sm">Tell OHC about the job to create an instant quote.</p>
            </div>

            <button
              onClick={handleVoiceInput}
              disabled={isListening}
              className={`w-32 h-32 rounded-full shadow-xl flex items-center justify-center transition-all duration-300 border-4 backdrop-blur-md ${
                isListening
                  ? 'bg-red-500/90 border-red-400 scale-110 animate-pulse'
                  : 'bg-blue-600/90 border-blue-400 hover:scale-105'
              }`}
              aria-label="Tell OHC about the job"
            >
              <span className="text-5xl drop-shadow-md">
                {isListening ? '🎙️' : '🎤'}
              </span>
            </button>

            {isListening && <p className="text-blue-600 font-medium animate-pulse">Listening...</p>}
          </div>
        )}

        {quoteDraft && !paymentSaved && (
          <div className="bg-white/70 backdrop-blur-xl border border-white/40 p-6 rounded-2xl shadow-sm mt-4 relative overflow-hidden before:absolute before:inset-0 before:-z-10 before:bg-gradient-to-br before:from-white/40 before:to-white/10" data-testid="quote-draft-card">
            <h3 className="text-lg font-bold text-gray-900 mb-4 font-outfit border-b pb-2">Draft Quote</h3>

            <div className="space-y-3 mb-6">
              {quoteDraft.line_items.map((item: any, i: number) => (
                <div key={i} className="flex justify-between items-center text-sm">
                  <span className="text-gray-600 font-medium">{item.description}</span>
                  <span className="text-gray-900 font-semibold">${(item.unit_price_cents / 100).toFixed(2)}</span>
                </div>
              ))}

              <div className="pt-3 border-t border-gray-100 flex justify-between items-center">
                <span className="font-bold text-gray-900">Total</span>
                <span className="font-bold text-gray-900 text-lg">${(quoteDraft.total_amount / 100).toFixed(2)}</span>
              </div>
              <div className="flex justify-between items-center text-sm text-blue-600 font-semibold">
                <span>Required Deposit</span>
                <span>${(quoteDraft.required_deposit / 100).toFixed(2)}</span>
              </div>
            </div>

            <button
              onClick={collectDepositOffline}
              className="w-full py-4 bg-gray-900 text-white rounded-xl font-bold shadow-md hover:bg-gray-800 transition-colors flex items-center justify-center gap-2"
              data-testid="collect-deposit-btn"
            >
              <span>💳</span> Collect Deposit
            </button>
          </div>
        )}

        {paymentSaved && (
          <div className="mt-8 bg-green-50/80 backdrop-blur-md border border-green-200 p-6 rounded-2xl text-center shadow-sm">
            <div className="text-4xl mb-3">✅</div>
            <h3 className="text-lg font-bold text-green-900 mb-1" data-testid="payment-saved-offline-msg">
              Payment Saved {isOffline ? 'Offline' : ''}
            </h3>
            <p className="text-green-700 text-sm">
              {isOffline
                ? "The transaction will sync automatically when you reconnect."
                : "The transaction has been successfully synced."}
            </p>
          </div>
        )}
      </main>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
