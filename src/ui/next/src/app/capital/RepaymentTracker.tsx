import React from 'react';

interface Props {
  totalOwed: number;
  amountRepaid: number;
}

export default function RepaymentTracker({ totalOwed, amountRepaid }: Props) {
  const percentage = Math.min(100, Math.max(0, (amountRepaid / totalOwed) * 100));
  const remaining = totalOwed - amountRepaid;

  return (
    <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 flex items-center space-x-6">
      <div className="relative w-16 h-16 flex-shrink-0">
        <svg className="w-full h-full -rotate-90 transform" viewBox="0 0 36 36">
          {/* Background Ring */}
          <path
            className="text-gray-100"
            strokeWidth="3"
            stroke="currentColor"
            fill="none"
            d="M18 2.0845
              a 15.9155 15.9155 0 0 1 0 31.831
              a 15.9155 15.9155 0 0 1 0 -31.831"
          />
          {/* Progress Ring */}
          <path
            className="text-black transition-all duration-1000 ease-out"
            strokeWidth="3"
            strokeDasharray={`${percentage}, 100`}
            strokeLinecap="round"
            stroke="currentColor"
            fill="none"
            d="M18 2.0845
              a 15.9155 15.9155 0 0 1 0 31.831
              a 15.9155 15.9155 0 0 1 0 -31.831"
          />
        </svg>
        <div className="absolute inset-0 flex items-center justify-center text-xl">
          ✨
        </div>
      </div>

      <div>
        <h3 className="text-sm font-medium text-gray-900">Capital Boost Active</h3>
        <p className="text-xs text-gray-500 mt-1">
          ${remaining.toFixed(2)} remaining to repay
        </p>
      </div>
    </div>
  );
}
