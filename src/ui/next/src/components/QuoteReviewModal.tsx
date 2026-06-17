"use client";

import React, { useState, useEffect } from 'react';

interface LineItem {
  description: string;
  unit_price_cents: number;
  quantity: number;
}

interface QuoteReviewModalProps {
  isOpen: boolean;
  onClose: () => void;
  onApprove: (updatedPayload: any) => void;
  initialPayload: any;
}

export function QuoteReviewModal({ isOpen, onClose, onApprove, initialPayload }: QuoteReviewModalProps) {
  const [lineItems, setLineItems] = useState<LineItem[]>([]);
  const [requireDeposit, setRequireDeposit] = useState(true);

  useEffect(() => {
    if (initialPayload) {
      const items = initialPayload.line_items || [
        {
          description: initialPayload.scope || initialPayload.service || "Service",
          unit_price_cents: Math.round((initialPayload.suggested_price || initialPayload.price || 0) * 100),
          quantity: 1,
        }
      ];
      setLineItems(items);
    }
  }, [initialPayload]);

  if (!isOpen) return null;

  const handleUpdateItem = (index: number, field: keyof LineItem, value: any) => {
    const newItems = [...lineItems];
    newItems[index] = { ...newItems[index], [field]: value };
    setLineItems(newItems);
  };

  const totalCents = lineItems.reduce((sum, item) => sum + (item.unit_price_cents * item.quantity), 0);
  const totalDisplay = (totalCents / 100).toFixed(2);

  const handleApproveClick = () => {
    const updatedPayload = {
      ...initialPayload,
      line_items: lineItems,
      suggested_price: totalCents / 100,
      price: totalCents / 100,
      require_deposit: requireDeposit,
      deposit_amount_cents: requireDeposit ? Math.round(totalCents * 0.5) : 0,
    };
    onApprove(updatedPayload);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-end sm:items-center justify-center p-0 sm:p-4 bg-black/40 backdrop-blur-sm animate-in fade-in duration-200">
      <div
        className="w-full max-w-lg bg-white/80 dark:bg-[#1C1C1E]/80 backdrop-blur-2xl saturate-150 rounded-t-[24px] sm:rounded-[24px] shadow-2xl border border-white/20 dark:border-white/10 flex flex-col max-h-[90vh] overflow-hidden animate-in slide-in-from-bottom duration-300"
        role="dialog"
        aria-modal="true"
      >
        <div className="p-4 border-b border-gray-200/50 dark:border-gray-700/50 flex items-center justify-between">
          <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Review Quote</h2>
          <button
            onClick={onClose}
            className="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            aria-label="Close"
          >
            <svg className="w-6 h-6 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="flex-1 overflow-y-auto p-6 space-y-6">
          <div>
            <label className="block text-xs font-bold text-gray-500 uppercase tracking-wider mb-3">Line Items</label>
            <div className="space-y-3">
              {lineItems.map((item, idx) => (
                <div key={idx} className="p-4 rounded-[16px] bg-white/50 dark:bg-black/20 border border-gray-200/50 dark:border-white/5 space-y-3">
                  <input
                    type="text"
                    value={item.description}
                    onChange={(e) => handleUpdateItem(idx, 'description', e.target.value)}
                    className="w-full bg-transparent font-medium text-[#1D1D1F] dark:text-[#F5F5F7] outline-none border-b border-transparent focus:border-[#0066FF] transition-colors"
                    placeholder="Description"
                  />
                  <div className="flex items-center gap-4">
                    <div className="flex-1 flex items-center gap-2">
                      <span className="text-xs text-gray-500 font-medium">Qty</span>
                      <input
                        type="number"
                        value={item.quantity}
                        onChange={(e) => handleUpdateItem(idx, 'quantity', parseInt(e.target.value) || 1)}
                        className="w-16 p-2 text-sm bg-gray-100 dark:bg-white/5 rounded-[12px] text-center outline-none focus:ring-2 focus:ring-[#0066FF]"
                      />
                    </div>
                    <div className="flex-1 flex items-center gap-2">
                      <span className="text-xs text-gray-500 font-medium">Price $</span>
                      <input
                        type="number"
                        step="0.01"
                        value={(item.unit_price_cents / 100).toFixed(2)}
                        onChange={(e) => handleUpdateItem(idx, 'unit_price_cents', Math.round(parseFloat(e.target.value || '0') * 100))}
                        className="w-full p-2 text-sm bg-gray-100 dark:bg-white/5 rounded-[12px] text-right outline-none focus:ring-2 focus:ring-[#0066FF]"
                      />
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="flex items-center justify-between p-4 rounded-[16px] bg-[#0066FF]/5 border border-[#0066FF]/10">
            <div className="flex flex-col">
              <span className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Require 50% Deposit</span>
              <span className="text-xs text-gray-500">Stripe payment link will be included</span>
            </div>
            <button
              onClick={() => setRequireDeposit(!requireDeposit)}
              className={`w-12 h-6 rounded-full transition-colors relative ${requireDeposit ? 'bg-[#34C759]' : 'bg-gray-300 dark:bg-gray-700'}`}
              role="switch"
              aria-checked={requireDeposit}
            >
              <div className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-all ${requireDeposit ? 'left-7' : 'left-1'}`} />
            </button>
          </div>

          <div className="pt-4 border-t border-gray-200/50 dark:border-gray-700/50">
            <div className="flex justify-between items-center">
              <span className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Total Amount</span>
              <span className="text-2xl font-bold font-outfit text-[#0066FF]" data-testid="modal-quote-total">${totalDisplay}</span>
            </div>
          </div>
        </div>

        <div className="p-6 bg-gray-50/50 dark:bg-white/5 border-t border-gray-200/50 dark:border-gray-700/50">
          <button
            onClick={handleApproveClick}
            className="w-full h-[44px] bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold rounded-[16px] shadow-lg shadow-[#0066FF]/20 transition-all active:scale-[0.98] flex items-center justify-center text-[17px]"
            data-testid="modal-approve-btn"
          >
            Approve & Send
          </button>
        </div>
      </div>
    </div>
  );
}
