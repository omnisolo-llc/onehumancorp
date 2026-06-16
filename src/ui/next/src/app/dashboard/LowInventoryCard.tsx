"use client";

import React from 'react';

interface LowInventoryCardProps {
  approval: any;
  onApprove: (id: string) => void;
  onDismiss: (id: string) => void;
}

export function LowInventoryCard({ approval, onApprove, onDismiss }: LowInventoryCardProps) {
  const payload = approval.proposed_action || approval.context_payload;

  return (
    <div className="flex flex-col gap-3" data-testid="low-inventory-card">
      <div className="flex justify-between items-center text-sm">
        <span className="text-gray-500 dark:text-gray-400 font-semibold">Low Stock Alert</span>
        <span className="text-amber-500 font-bold text-xs">Action Recommended</span>
      </div>

      <div className="app-card dark:bg-gray-800 p-4 rounded-lg border border-amber-100 dark:border-amber-900/50">
        <div className="flex justify-between items-start mb-2">
          <div>
            <h4 className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7] text-sm">
              {payload.product_name || 'Product'}
            </h4>
            <p className="text-[11px] text-gray-500">ID: {payload.product_id}</p>
          </div>
          <div className="text-right">
            <span className="text-lg font-bold text-red-600 dark:text-red-400">
              {payload.remaining_stock}
            </span>
            <p className="text-[10px] text-gray-400 uppercase font-bold">Left</p>
          </div>
        </div>

        <p className="text-xs text-gray-700 dark:text-gray-300 leading-relaxed mb-3 italic">
          "{payload.message}"
        </p>

        <div className="flex flex-col sm:flex-row gap-3 w-full">
          <button
            onClick={() => onApprove(approval.id)}
            className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-amber-500 text-white font-medium hover:bg-amber-600 transition-all duration-200 shadow-md flex items-center justify-center"
            data-testid="approve-restock-btn"
          >
            Approve Restock
          </button>
          <button
            onClick={() => onDismiss(approval.id)}
            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
            data-testid="dismiss-restock-btn"
          >
            Dismiss
          </button>
        </div>
      </div>
    </div>
  );
}
