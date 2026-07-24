import React, { useState } from 'react';

interface ReviewDraftQuoteCardProps {
  customerName: string;
  projectDescription: string;
  totalCost: number;
  onApprove: () => void;
  onEdit: (newQuote: string) => void;
}

export const ReviewDraftQuoteCard: React.FC<ReviewDraftQuoteCardProps> = ({
  customerName,
  projectDescription,
  totalCost,
  onApprove,
  onEdit
}) => {
  const [isEditing, setIsEditing] = useState(false);
  const [editedQuote, setEditedQuote] = useState(projectDescription);

  const handleSave = () => {
    onEdit(editedQuote);
    setIsEditing(false);
  };

  const handleCancel = () => {
    setEditedQuote(projectDescription);
    setIsEditing(false);
  };

  return (
    <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
      <div className="flex justify-between items-start mb-2">
        <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Draft Quote Ready</h3>
        <span className="inline-flex items-center rounded-[8px] bg-[#FF9500]/10 px-2 py-1 text-xs font-medium text-[#FF9500] ring-1 ring-inset ring-[#FF9500]/20">Action Required</span>
      </div>

      {isEditing ? (
        <div className="flex flex-col gap-2 mb-4">
          <textarea
            value={editedQuote}
            onChange={(e) => setEditedQuote(e.target.value)}
            className="text-sm w-full min-h-[100px] text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/60 dark:bg-black/40 backdrop-blur-[30px] backdrop-saturate-[210%] border border-blue-500/50 p-3 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-blue-500/50"
            data-testid="feed-edit-textarea"
          />
        </div>
      ) : (
        <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-4">
          {editedQuote} for {customerName}
        </p>
      )}

      <div className="bg-white/40 dark:bg-black/20 backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] p-3 mb-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
        <div className="flex justify-between text-sm">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Total Cost:</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${(totalCost / 100).toFixed(2)}</span>
        </div>
        <div className="flex justify-between text-sm mt-1">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Deposit Required (33%):</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${((totalCost / 3) / 100).toFixed(2)}</span>
        </div>
      </div>

      {isEditing ? (
        <div className="flex gap-2">
          <button
            onClick={handleSave}
            data-testid="feed-save-btn"
            className="flex-1 min-h-[44px] bg-[#0066FF] hover:bg-blue-600 text-white rounded-[8px] text-sm font-medium transition-colors"
          >
            Save
          </button>
          <button
            onClick={handleCancel}
            data-testid="feed-cancel-btn"
            className="flex-1 min-h-[44px] bg-white/50 dark:bg-gray-800/50 hover:bg-white/80 dark:hover:bg-gray-700/50 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-sm font-medium transition-colors"
          >
            Cancel
          </button>
        </div>
      ) : (
        <div className="flex space-x-3">
          <button
            onClick={onApprove} data-testid="feed-approve-btn"
            className="flex-1 bg-[#0066FF] text-white min-h-[44px] px-4 rounded-[8px] text-sm font-medium hover:bg-blue-600 transition-colors"
          >
            Approve & Send
          </button>
          <button
            onClick={() => setIsEditing(true)} data-testid="feed-dismiss-btn"
            className="flex-1 bg-white/50 dark:bg-gray-800/50 text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] px-4 rounded-[8px] text-sm font-medium border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] hover:bg-white/80 dark:hover:bg-gray-700/50 transition-colors"
          >
            Edit
          </button>
        </div>
      )}
    </div>
  );
};
