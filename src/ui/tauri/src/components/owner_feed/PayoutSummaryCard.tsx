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
    <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
      <div className="flex justify-between items-start mb-2">
        <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Your Payout Summary is ready.</h3>
        <span className="inline-flex items-center rounded-[8px] bg-[#34C759]/10 px-2 py-1 text-xs font-medium text-[#34C759] ring-1 ring-inset ring-[#34C759]/20">Pending</span>
      </div>
      <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-4">
        You earned ${(totalUsdEarned / 100).toFixed(2)} USD and €{(totalEurEarned / 100).toFixed(2)} EUR this week. After conversion and fees, ${(totalUsdPayout / 100).toFixed(2)} USD will hit your Chase account tomorrow.
      </p>

      <button
        onClick={onViewDetails}
        className="w-full bg-[#0066FF] text-white min-h-[44px] px-4 rounded-[8px] text-sm font-medium hover:bg-blue-600 transition-colors"
      >
        View Details
      </button>
    </div>
  );
};
