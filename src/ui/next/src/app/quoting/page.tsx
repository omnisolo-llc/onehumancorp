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
  const [isEditing, setIsEditing] = useState(false);
  const [editLineItems, setEditLineItems] = useState<any[]>([]);

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
          setEditLineItems(data.line_items);
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
      const res = await fetch(`/api/quotes/${quoteId}/accept`, {
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

  const handleSaveEdit = async () => {
    try {
      const res = await fetch(`/api/quotes/${quoteId}`, {
        method: 'PUT',
        headers: {
          'x-tenant-id': 'tenant-1',
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ line_items: editLineItems })
      });
      if (res.ok) {
        const data = await res.json();
        setQuoteData(data);
        setIsEditing(false);
      } else {
        alert('Failed to update quote');
      }
    } catch (err) {
      alert('Error connecting to server');
    }
  };

  const updateLineItem = (index: number, field: string, value: any) => {
    const newItems = [...editLineItems];
    newItems[index] = { ...newItems[index], [field]: value };
    setEditLineItems(newItems);
  };

  if (loading) {
    return <div className="p-8 text-center">Loading quote...</div>;
  }

  if (error || !quoteData) {
    return <div className="p-8 text-center text-red-600">{error || 'Quote not found'}</div>;
  }

  const { quote, line_items } = quoteData;
  const displayItems = isEditing ? editLineItems : line_items;
  const totalCents = displayItems.reduce((sum: number, item: any) => sum + (item.unit_price_cents * item.quantity), 0);
  const total = (totalCents / 100).toFixed(2);

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter relative">
      <header className="px-6 py-4 bg-white border-b sticky top-0 z-10 flex items-center justify-between shadow-sm">
        <h1 className="text-xl font-bold font-outfit text-gray-900">Project Proposal</h1>
        <div className="text-sm px-3 py-1 bg-blue-50 text-blue-700 rounded-full font-medium">
          {accepted ? 'Accepted' : quote.status}
        </div>
      </header>

      <main className="p-6 md:p-10 flex-1 max-w-3xl mx-auto w-full">
        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden relative">
          <div className="p-6 md:p-8 border-b border-gray-100 bg-gray-50/50">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Quote Summary</h2>
            <p className="text-gray-600">Review the scope and pricing below.</p>
          </div>

          <div className="p-6 md:p-8 pb-32">
            <div className="space-y-6">
              <h3 className="text-lg font-semibold text-gray-900 border-b pb-2">Line Items</h3>
              {displayItems.map((item: any) => (
                <div key={item.id || item.description} className="flex justify-between items-start gap-4">
                  <div className="flex-1">
                    <h4 className="font-medium text-gray-900">{item.description}</h4>
                    <p className="text-sm text-gray-500 mt-1">Qty: {item.quantity}</p>
                  </div>
                  <div className="font-semibold text-gray-900">
                    ${((item.unit_price_cents * item.quantity) / 100).toFixed(2)}
                  </div>
                </div>
              ))}

              <div className="pt-6 mt-6 border-t border-gray-200">
                <div className="flex justify-between items-center">
                  <span className="text-xl font-bold text-gray-900 font-outfit">Total Estimate</span>
                  <span className="text-2xl font-bold text-[#0066FF] font-outfit">${total}</span>
                </div>
              </div>
            </div>
          </div>

          {!accepted && !isEditing && (
            <div className="p-6 bg-gray-50 border-t border-gray-100 flex flex-col sm:flex-row gap-4 absolute bottom-0 left-0 right-0 w-full">
              <button
                onClick={() => setIsEditing(true)}
                className="w-full sm:flex-1 py-4 bg-white border border-gray-300 hover:bg-gray-50 text-gray-800 font-bold rounded-xl shadow-sm transition-all text-lg flex items-center justify-center min-h-[44px]"
              >
                Edit Proposal
              </button>
              <button
                onClick={handleAccept}
                className="w-full sm:flex-1 py-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold rounded-xl shadow-md transition-all text-lg flex items-center justify-center min-h-[44px]"
              >
                Accept Proposal
              </button>
            </div>
          )}
          {accepted && (
            <div className="p-6 bg-green-50 border-t border-green-100 text-center">
              <div className="text-green-600 text-4xl mb-2">✅</div>
              <h3 className="text-lg font-bold text-green-800">Proposal Accepted</h3>
              <p className="text-green-700 text-sm mt-1">Thank you! We'll be in touch with the next steps.</p>
            </div>
          )}
        </div>
      </main>

      {/* Edit Mode Bottom Sheet */}
      {isEditing && (
        <div className="fixed inset-0 z-50 flex flex-col justify-end bg-black/40 backdrop-blur-sm transition-all">
          <div className="bg-white/80 backdrop-blur-xl border-t border-white/40 shadow-2xl rounded-t-3xl p-6 w-full max-w-3xl mx-auto flex flex-col max-h-[80vh] overflow-y-auto glassmorphism">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Edit Line Items</h2>
            <div className="space-y-4 flex-1 overflow-y-auto">
              {editLineItems.map((item: any, idx: number) => (
                <div key={idx} className="bg-white p-4 rounded-xl shadow-sm border border-gray-200 flex flex-col gap-3">
                  <div>
                    <label className="block text-xs font-semibold text-gray-600 mb-1">Description</label>
                    <input
                      type="text"
                      value={item.description}
                      onChange={(e) => updateLineItem(idx, 'description', e.target.value)}
                      className="w-full bg-gray-50 border border-gray-200 rounded-lg p-3 text-sm min-h-[44px]"
                    />
                  </div>
                  <div className="flex gap-4">
                    <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-600 mb-1">Quantity</label>
                      <input
                        type="number"
                        value={item.quantity}
                        onChange={(e) => updateLineItem(idx, 'quantity', parseInt(e.target.value) || 0)}
                        className="w-full bg-gray-50 border border-gray-200 rounded-lg p-3 text-sm min-h-[44px]"
                      />
                    </div>
                    <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-600 mb-1">Unit Price ($)</label>
                      <input
                        type="number"
                        step="0.01"
                        value={(item.unit_price_cents / 100).toFixed(2)}
                        onChange={(e) => updateLineItem(idx, 'unit_price_cents', Math.round(parseFloat(e.target.value) * 100) || 0)}
                        className="w-full bg-gray-50 border border-gray-200 rounded-lg p-3 text-sm min-h-[44px]"
                      />
                    </div>
                  </div>
                </div>
              ))}
            </div>

            <div className="mt-6 flex flex-col sm:flex-row gap-3 border-t border-gray-200 pt-4">
               <button
                onClick={() => {
                  setEditLineItems(line_items);
                  setIsEditing(false);
                }}
                className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-gray-200 text-gray-800 hover:bg-gray-300 min-h-[44px] transition-all"
              >
                Cancel
              </button>
              <button
                onClick={handleSaveEdit}
                className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-[#0066FF] text-white hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all min-h-[44px]"
              >
                Save Changes
              </button>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism {
          background: rgba(255, 255, 255, 0.7);
          backdrop-filter: blur(20px);
          -webkit-backdrop-filter: blur(20px);
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
