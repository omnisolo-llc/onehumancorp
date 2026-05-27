import React from 'react';
import Link from 'next/link';

interface LimitReachedModalProps {
  onClose: () => void;
  limitType: 'agents' | 'products';
}

export default function LimitReachedModal({ onClose, limitType }: LimitReachedModalProps) {
  const limitName = limitType === 'agents' ? '1 Agent Limit' : '10 Products Limit';

  return (
    <div className="fixed inset-0 bg-black/60 z-[70] flex items-center justify-center p-4">
      <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-orange-100">
        <div className="flex justify-between items-start mb-4">
          <div className="w-12 h-12 bg-orange-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-orange-600">
            ⭐
          </div>
          <button
            onClick={onClose}
            className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You've hit your limit!</h2>
        <p className="text-gray-600 mb-6 text-sm leading-relaxed">
          You have reached the <strong className="text-gray-900">{limitName}</strong> on the Free plan.
          Upgrade to the Starter plan to unlock 3 agents, or the Pro plan for 10 agents, to scale your business.
        </p>

        <div className="space-y-3">
          <Link
            href="/checkout?tier=Starter"
            className="block w-full py-3 bg-gradient-to-r from-orange-500 to-red-500 text-white font-semibold rounded-xl text-center shadow-md hover:shadow-lg hover:-translate-y-0.5 transition-all"
          >
            Upgrade to Starter
          </Link>
          <button
            onClick={onClose}
            className="w-full py-3 bg-gray-100 text-gray-700 font-semibold rounded-xl hover:bg-gray-200 transition-colors"
          >
            Maybe Later
          </button>
        </div>
      </div>
    </div>
  );
}
