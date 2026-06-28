import React from 'react';

interface PayoutSummaryCardProps {
  totalUsdEarned: number;
  totalEurEarned: number;
  totalUsdPayout: number;
  onViewDetails: () => void;
}

export const PayoutSummaryCard: React.FC<PayoutSummaryCardProps> = ({
  totalUsdEarned,
  totalEurEarned,
  totalUsdPayout,
  onViewDetails
}) => {
  return (
    <div className="bg-white/80 backdrop-blur-md rounded-xl p-4 shadow-sm border border-gray-200">
      <div className="flex justify-between items-start mb-2">
        <h3 className="text-lg font-semibold text-gray-900">Your Payout Summary is ready.</h3>
        <span className="inline-flex items-center rounded-md bg-green-50 px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-700/10">Pending</span>
      </div>
      <p className="text-sm text-gray-600 mb-4">
        You earned ${(totalUsdEarned / 100).toFixed(2)} USD and €{(totalEurEarned / 100).toFixed(2)} EUR this week. After conversion and fees, ${(totalUsdPayout / 100).toFixed(2)} USD will hit your Chase account tomorrow.
      </p>

      <button
        onClick={onViewDetails}
        className="w-full bg-blue-600 text-white py-2 px-4 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors"
      >
        View Details
      </button>
    </div>
  );
};
