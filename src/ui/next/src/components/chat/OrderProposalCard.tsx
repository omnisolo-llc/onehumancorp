import React from 'react';

export type OrderProposalCardProps = {
  id: string;
  customerName?: string;
  customerEmail?: string;
  scope: string;
  suggestedPrice: number;
  depositRequired: number;
  status: 'pending' | 'approved' | 'dismissed';
  onApprove: (id: string) => void;
  onEdit: (id: string) => void;
  onDiscard?: (id: string) => void;
};

export const OrderProposalCard: React.FC<OrderProposalCardProps> = ({
  id,
  customerName,
  customerEmail,
  scope,
  suggestedPrice,
  depositRequired,
  status,
  onApprove,
  onEdit,
  onDiscard
}) => {
  return (
    <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] mb-4" data-testid={`order-proposal-card-${id}`}>
      <div className="flex justify-between items-start mb-2">
        <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Order Proposal Ready</h3>
        {status === 'pending' && (
          <span className="inline-flex items-center rounded-[8px] bg-[#FF9500]/10 px-2 py-1 text-xs font-medium text-[#FF9500] ring-1 ring-inset ring-[#FF9500]/20">Action Required</span>
        )}
        {status === 'approved' && (
          <span className="inline-flex items-center rounded-[8px] bg-green-100 dark:bg-green-900/30 px-2 py-1 text-xs font-medium text-green-800 dark:text-green-300 ring-1 ring-inset ring-green-600/20">Approved</span>
        )}
      </div>

      {customerName && (
        <p className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">
          For: {customerName} {customerEmail ? `(${customerEmail})` : ''}
        </p>
      )}

      <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-4 whitespace-pre-wrap break-words">
        {scope}
      </p>

      <div className="bg-white/40 dark:bg-black/20 backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] p-3 mb-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
        <div className="flex justify-between text-sm mb-1">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Total Cost:</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${suggestedPrice.toFixed(2)}</span>
        </div>
        <div className="flex justify-between text-sm">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Deposit Required:</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${depositRequired.toFixed(2)}</span>
        </div>
      </div>

      {status === 'pending' && (
        <div className="flex flex-wrap gap-2 w-full">
          <button
            onClick={() => onApprove(id)}
            data-testid="approve-proposal-btn"
            className="flex-1 bg-[#0066FF] text-white min-h-[44px] min-w-[44px] px-4 rounded-[8px] text-sm font-medium hover:bg-[#0052CC] transition-colors active:scale-[0.98] shadow-md cursor-pointer flex items-center justify-center"
          >
            Approve & Send
          </button>
          <button
            onClick={() => onEdit(id)}
            data-testid="edit-proposal-btn"
            className="flex-1 bg-white/50 dark:bg-black/20 text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] min-w-[44px] px-4 rounded-[8px] text-sm font-medium border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] hover:bg-white/80 dark:hover:bg-black/40 transition-colors active:scale-[0.98] cursor-pointer flex items-center justify-center glassmorphism"
          >
            Edit
          </button>
          {onDiscard && (
            <button
              onClick={() => onDiscard(id)}
              data-testid="discard-proposal-btn"
              className="flex-none bg-red-50 hover:bg-red-100 text-red-700 min-h-[44px] min-w-[44px] px-4 rounded-[8px] text-sm font-medium border border-red-200 hover:border-red-300 transition-colors active:scale-[0.98] cursor-pointer flex items-center justify-center"
            >
              Discard
            </button>
          )}
        </div>
      )}
    </div>
  );
};
