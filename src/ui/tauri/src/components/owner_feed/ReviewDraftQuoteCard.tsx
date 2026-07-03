
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
        <h3 className="text-lg font-semibold text-gray-900">Draft Quote Ready</h3>
        <span className="inline-flex items-center rounded-md bg-blue-50 px-2 py-1 text-xs font-medium text-blue-700 ring-1 ring-inset ring-blue-700/10">Action Required</span>
      </div>
      <p className="text-sm text-gray-600 mb-4">
        {projectDescription} for {customerName}
      </p>

      <div className="bg-gray-50 rounded-[8px] p-3 mb-4 border border-gray-100">
        <div className="flex justify-between text-sm">
          <span className="text-gray-500">Total Cost:</span>
          <span className="font-medium">${(totalCost / 100).toFixed(2)}</span>
        </div>
        <div className="flex justify-between text-sm mt-1">
          <span className="text-gray-500">Deposit Required (33%):</span>
          <span className="font-medium">${((totalCost / 3) / 100).toFixed(2)}</span>
        </div>
      </div>

      <div className="flex space-x-3">
        <button
          onClick={onApprove}
          className="flex-1 bg-blue-600 text-white min-h-[44px] px-4 rounded-[8px] text-sm font-medium hover:bg-blue-700 transition-colors"
        >
          Approve & Send
        </button>
        <button
          onClick={onEdit}
          className="flex-1 bg-white text-gray-700 min-h-[44px] px-4 rounded-[8px] text-sm font-medium border border-gray-300 hover:bg-gray-50 transition-colors"
        >
          Edit
        </button>
      </div>
    </div>
  );
};
