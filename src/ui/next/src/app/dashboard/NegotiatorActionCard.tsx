import React from 'react';

export const NegotiatorActionCard = ({ approval, onApprove, onReject }: any) => {
  const payload = approval.proposed_action || {};
  const quoteAmount = payload.quote_amount || 0;
  const currency = payload.currency || "USD";
  const proposedTime = payload.proposed_time || "TBD";
  const customerMessage = payload.customer_message || "";

  return (
    <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-indigo-100 dark:border-indigo-900/30 overflow-hidden mb-4">
      <div className="p-4 bg-indigo-50 dark:bg-indigo-900/20 flex items-start gap-3 border-b border-indigo-100 dark:border-indigo-900/30">
        <div className="w-10 h-10 rounded-full bg-indigo-100 dark:bg-indigo-800 flex items-center justify-center flex-shrink-0 text-xl">
          🤖
        </div>
        <div>
          <h3 className="font-semibold text-gray-900 dark:text-white flex items-center gap-2">
            Agentic Negotiator
            <span className="bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 text-xs px-2 py-1 rounded-md">Booking Proposal</span>
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
            Intercepted booking request and generated a quote.
          </p>
        </div>
      </div>

      <div className="p-4 space-y-4">
        <div className="bg-gray-50 dark:bg-gray-900 p-3 rounded-lg text-sm italic border-l-4 border-gray-300 dark:border-gray-600">
          "{customerMessage}"
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div className="bg-gray-50 dark:bg-gray-900 p-3 rounded-lg border border-gray-100 dark:border-gray-800">
            <p className="text-xs text-gray-500 mb-1">Proposed Quote</p>
            <p className="font-semibold text-gray-900 dark:text-white text-lg">
              {quoteAmount} {currency}
            </p>
          </div>
          <div className="bg-gray-50 dark:bg-gray-900 p-3 rounded-lg border border-gray-100 dark:border-gray-800">
            <p className="text-xs text-gray-500 mb-1">Proposed Time</p>
            <p className="font-semibold text-gray-900 dark:text-white">
              {proposedTime}
            </p>
          </div>
        </div>
      </div>

      <div className="p-4 bg-gray-50 dark:bg-gray-800/50 flex gap-3 border-t border-gray-100 dark:border-gray-700">
        <button
          onClick={() => onApprove(approval.id)}
          className="flex-1 bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-lg transition-colors flex items-center justify-center gap-2"
        >
          <span>Send Quote & Book</span>
        </button>
        <button
          onClick={() => onReject(approval.id)}
          className="flex-1 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-300 font-medium py-2 px-4 rounded-lg border border-gray-200 dark:border-gray-600 transition-colors"
        >
          Review Manually
        </button>
      </div>
    </div>
  );
};
