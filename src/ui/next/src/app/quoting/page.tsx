"use client";
import { Suspense } from "react";
import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'next/navigation';

function QuotingContent() {
  const searchParams = useSearchParams();
  const quoteId = searchParams.get('id');

  const [quoteData, setQuoteData] = useState<any>(null);
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
        const res = await fetch(`/api/quotes?id=${quoteId}`, {
          headers: {
            'x-tenant-id': 'tenant-1' // hardcoded for test
          }
        });
        if (res.ok) {
          const data = await res.json();
          setQuoteData(data);
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

  const handleAccept = async () => {
    try {
      const res = await fetch(`/api/quotes?id=${quoteId}`, {
        method: 'POST',
        headers: {
          'x-tenant-id': 'tenant-1' // hardcoded for test
        },
        body: JSON.stringify({})
      });
      if (res.ok) {
        setAccepted(true);
      } else {
        alert('Failed to accept quote');
      }
    } catch (err) {
      alert('Error connecting to server');
    }
  };

  if (loading) {
    return <div className="p-8 text-center">Loading quote...</div>;
  }

  if (error || !quoteData) {
    return <div className="p-8 text-center text-red-600">{error || 'Quote not found'}</div>;
  }

  const { quote, line_items } = quoteData;
  const totalCents = line_items.reduce((sum: number, item: any) => sum + (item.unit_price_cents * item.quantity), 0);
  const total = (totalCents / 100).toFixed(2);
  const requiredDeposit = quote.required_deposit ? (quote.required_deposit / 100).toFixed(2) : (totalCents / 200).toFixed(2);

  return (
    <div className="flex flex-col min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F] font-inter">
      <header className="px-6 py-4 glassmorphism border-b border-white/40 dark:border-white/10 sticky top-0 z-10 flex items-center justify-between shadow-sm">
        <h1 className="text-xl font-bold font-outfit text-gray-900 dark:text-gray-100">Project Proposal</h1>
        <div className="text-sm px-3 py-1 bg-[#0066FF]/10 text-[#0066FF] rounded-full font-medium">
          {accepted ? 'Accepted' : quote.status}
        </div>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-3xl mx-auto w-full">
        <div className="glassmorphism rounded-[16px] shadow-sm border border-white/40 dark:border-white/10 overflow-hidden">
          <div className="p-6 md:p-8 border-b border-white/40 dark:border-white/10 bg-white/50 dark:bg-black/20">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-gray-100 mb-2">Quote Summary</h2>
            <p className="text-gray-600 dark:text-gray-400">Review the scope and pricing below.</p>
          </div>

          <div className="p-6 md:p-8">
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100 border-b border-white/40 dark:border-white/10 pb-2">Line Items</h3>
              {line_items.map((item: any) => (
                <div key={item.id} className="flex justify-between items-start gap-4">
                  <div className="flex-1">
                    <h4 className="font-medium text-gray-900 dark:text-gray-100">{item.description}</h4>
                    <p className="text-sm text-gray-500 mt-1">Qty: {item.quantity}</p>
                  </div>
                  <div className="font-semibold text-gray-900 dark:text-gray-100">
                    ${((item.unit_price_cents * item.quantity) / 100).toFixed(2)}
                  </div>
                </div>
              ))}

              <div className="pt-6 mt-6 border-t border-white/40 dark:border-white/10 space-y-3">
                <div className="flex justify-between items-center">
                  <span className="text-xl font-bold text-gray-900 dark:text-gray-100 font-outfit">Total Estimate</span>
                  <span className="text-2xl font-bold text-gray-900 dark:text-gray-100 font-outfit">${total}</span>
                </div>
                <div className="flex justify-between items-center bg-[#0066FF]/5 p-4 rounded-xl border border-[#0066FF]/20">
                  <span className="text-lg font-semibold text-[#0066FF] font-outfit">Required Deposit</span>
                  <span className="text-xl font-bold text-[#0066FF] font-outfit">${requiredDeposit}</span>
                </div>
              </div>
            </div>
          </div>

          {!accepted && (
            <div className="p-6 bg-white/50 dark:bg-black/20 border-t border-white/40 dark:border-white/10 flex flex-col sm:flex-row gap-4">
              <button
                onClick={handleAccept}
                className="w-full sm:flex-1 py-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold rounded-[8px] shadow-md transition-all text-lg flex items-center justify-center"
              >
                Accept Proposal & Pay Deposit
              </button>
            </div>
          )}
          {accepted && (
            <div className="p-6 bg-[#34C759]/10 border-t border-[#34C759]/20 text-center">
              <div className="text-[#34C759] text-4xl mb-2">✅</div>
              <h3 className="text-lg font-bold text-[#34C759]">Proposal Accepted</h3>
              <p className="text-[#34C759]/80 text-sm mt-1">Thank you! We'll be in touch with the next steps.</p>
            </div>
          )}
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
        }
        @media (prefers-color-scheme: dark) {
          .glassmorphism {
            background: rgba(22, 22, 26, 0.7);
            backdrop-filter: blur(30px) saturate(210%);
          }
        }
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
