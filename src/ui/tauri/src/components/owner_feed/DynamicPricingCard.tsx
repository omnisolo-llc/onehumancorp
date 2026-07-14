import React from 'react';

interface DynamicPricingCardProps {
  targetTitle: string;
  recommendation: string;
  basePriceCents?: number;
  onApprove: () => void;
  onEdit: () => void;
}

export const DynamicPricingCard: React.FC<DynamicPricingCardProps> = ({
  targetTitle,
  recommendation,
  basePriceCents,
  onApprove,
  onEdit
}) => {
  return (
    <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
      <div className="flex justify-between items-start mb-2">
        <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">✨ Pricing Suggestion</h3>
        <span className="inline-flex items-center rounded-[8px] bg-[#0066FF]/10 px-2 py-1 text-xs font-medium text-[#0066FF] ring-1 ring-inset ring-[#0066FF]/20">Yield Management</span>
      </div>

      <p className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">
        {targetTitle}
      </p>

      {basePriceCents !== undefined && (
         <p className="text-xs text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 mb-2">
            Base price: ${(basePriceCents / 100).toFixed(2)}
         </p>
      )}

      <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-3 rounded-[8px] mb-4">
        {recommendation}
      </p>

      <div className="flex flex-col gap-2 mt-2">
          <button
              onClick={onApprove} data-testid="feed-approve-btn"
              className="w-full min-h-[44px] bg-[#0066FF] hover:bg-blue-600 text-white rounded-[8px] font-medium transition-colors"
          >
              Approve & Run Sale
          </button>
          <button
              onClick={onEdit} data-testid="feed-dismiss-btn"
              className="w-full min-h-[44px] bg-white/50 dark:bg-gray-800/50 hover:bg-white/80 dark:hover:bg-gray-700/50 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] font-medium transition-colors"
          >
              Adjust Details
          </button>
      </div>
    </div>
  );
};
