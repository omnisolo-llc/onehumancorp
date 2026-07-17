import React from 'react';

interface DraftPurchaseOrderCardProps {
  vendorName: string;
  resourceName: string;
  suggestedQuantity: number;
  totalCost: number;
  onApprove: () => void;
  onEdit: () => void;
}

export const DraftPurchaseOrderCard: React.FC<DraftPurchaseOrderCardProps> = ({
  vendorName,
  resourceName,
  suggestedQuantity,
  totalCost,
  onApprove,
  onEdit
}) => {
  return (
    <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
      <div className="flex justify-between items-start mb-2">
        <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Draft Purchase Order</h3>
        <span className="inline-flex items-center rounded-[8px] bg-[#FF9500]/10 px-2 py-1 text-xs font-medium text-[#FF9500] ring-1 ring-inset ring-[#FF9500]/20">Low Stock</span>
      </div>
      <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-4">
        {resourceName} running low based on upcoming orders.
      </p>

      <div className="bg-white/40 dark:bg-black/20 backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] p-3 mb-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
        <div className="flex justify-between text-sm">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Vendor:</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">{vendorName}</span>
        </div>
        <div className="flex justify-between text-sm mt-1">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Quantity:</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">{suggestedQuantity}</span>
        </div>
        <div className="flex justify-between text-sm mt-1">
          <span className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Est. Total Cost:</span>
          <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">${totalCost.toFixed(2)}</span>
        </div>
      </div>

      <div className="flex space-x-3">
        <button
          onClick={onApprove} data-testid="feed-approve-btn"
          className="flex-1 bg-[#0066FF] text-white min-h-[44px] px-4 rounded-[8px] text-sm font-medium hover:bg-blue-600 transition-colors"
        >
          Approve & Send PO
        </button>
        <button
          onClick={onEdit} data-testid="feed-dismiss-btn"
          className="flex-1 bg-white/50 dark:bg-gray-800/50 text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] px-4 rounded-[8px] text-sm font-medium border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] hover:bg-white/80 dark:hover:bg-gray-700/50 transition-colors"
        >
          Edit Quantities
        </button>
      </div>
    </div>
  );
};
