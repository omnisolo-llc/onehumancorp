import React from 'react';

export function InstantQuoteCard({ quote, onApprove, onReject }: { quote: any, onApprove: (id: string) => void, onReject: (id: string) => void }) {
  if (!quote) return null;

  return (
    <div className="glassmorphism p-6 rounded-2xl border border-white/40 shadow-xl bg-white/80 backdrop-blur-lg mb-6 transition-all hover:shadow-2xl">
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="text-lg font-bold text-gray-900 flex items-center">
            <span className="bg-blue-100 text-blue-700 text-xs font-semibold px-2 py-1 rounded-full mr-2">New Quote Request</span>
            {quote.customerName}
          </h3>
          <p className="text-sm text-gray-500 mt-1">{quote.serviceRequested} • Calculated Instantly</p>
        </div>
        <div className="text-right">
          <div className="text-2xl font-extrabold text-blue-600">${quote.estimatedPrice.toFixed(2)}</div>
          <div className="text-xs text-gray-400 mt-1">Rules applied: {quote.appliedRules.join(', ')}</div>
        </div>
      </div>

      <div className="bg-gray-50 p-4 rounded-xl mb-4 border border-gray-100">
        <p className="text-sm text-gray-700 font-medium mb-2">AI Sales Agent Draft:</p>
        <p className="text-sm text-gray-600 italic border-l-4 border-blue-300 pl-3">"{quote.draftMessage}"</p>
      </div>

      <div className="flex gap-3 mt-2">
        <button
          onClick={() => onApprove(quote.id)}
          className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded-xl shadow-sm transition-colors active:scale-[0.98]"
        >
          Approve & Send
        </button>
        <button
          onClick={() => onReject(quote.id)}
          className="flex-1 bg-white hover:bg-gray-50 text-gray-700 border border-gray-200 font-medium py-3 px-4 rounded-xl shadow-sm transition-colors active:scale-[0.98]"
        >
          Adjust Manually
        </button>
      </div>
    </div>
  );
}
