import React from 'react';

interface ReviewDraftQuoteCardProps {
  customerName: string;
  projectDescription: string;
  totalCost: number;
  onApprove: () => void;
  onEdit: () => void;
}

export const ReviewDraftQuoteCard: React.FC<ReviewDraftQuoteCardProps> = ({
  customerName,
  projectDescription,
  totalCost,
  onApprove,
  onEdit
}) => {
  return (
    <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] rounded-[16px] p-4 shadow-sm border border-white/40 dark:border-white/10">
      <div className="flex justify-between items-start mb-2">
        <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Draft Quote Ready</h3>
        <span className="inline-flex items-center rounded-[8px] bg-[#FF9500]/10 px-2 py-1 text-xs font-medium text-[#FF9500] ring-1 ring-inset ring-[#FF9500]/20">Action Required</span>
      </div>
      <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-4">
        {projectDescription} for {customerName}
      </p>

      <div className="bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] rounded-[8px] p-3 mb-4 border border-white/40 dark:border-white/10">
        <div className="flex justify-between text-sm">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Total Cost:</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${(totalCost / 100).toFixed(2)}</span>
        </div>
        <div className="flex justify-between text-sm mt-1">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Deposit Required (33%):</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${((totalCost / 3) / 100).toFixed(2)}</span>
        </div>
      </div>

      <div className="flex space-x-3">
        <button
          onClick={onApprove}
          className="flex-1 bg-[#0066FF] text-white min-h-[44px] px-4 rounded-[8px] text-sm font-medium hover:bg-blue-600 transition-colors"
        >
          Approve & Send
        </button>
        <button
          onClick={onEdit}
          className="flex-1 bg-white/50 dark:bg-gray-800/50 text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] px-4 rounded-[8px] text-sm font-medium border border-white/40 dark:border-white/10 hover:bg-white/80 dark:hover:bg-gray-700/50 transition-colors"
        >
          Edit
        </button>
      </div>
    </div>
  );
};
