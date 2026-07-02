"use client";

import React from "react";

export interface DailyWorkItem {
  id: string;
  intent: string;
  customer_info?: { name?: string };
  suggested_actions?: Array<{ action_type?: string; message?: string }>;
  status: string;
}

interface Props {
  item: DailyWorkItem;
  onApprove: (id: string) => void;
  onDismiss: (id: string) => void;
}

export function DailyWorkCard({ item, onApprove, onDismiss }: Props) {
  const customerName = item.customer_info?.name || "Unknown Customer";
  const actionDetails = item.suggested_actions?.[0];
  const title = actionDetails?.action_type || item.intent;
  const description = actionDetails?.message || "Please review this item.";

  return (
    <div
      data-testid={`daily-work-card-${item.id}`}
      className="glassmorphism p-5 shadow-sm flex flex-col gap-4 mb-4"
    >
      <div className="flex justify-between items-start">
        <div className="flex flex-col">
          <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md max-w-fit">
            {item.intent}
          </span>
          <h3 className="mt-2 text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-tight">
            {title}
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
            {customerName}
          </p>
        </div>
      </div>

      <div className="bg-gray-50 dark:bg-gray-800/50 p-3 rounded-lg border border-gray-100 dark:border-gray-700">
        <p className="text-sm text-gray-700 dark:text-gray-300">
          {description}
        </p>
      </div>

      <div className="flex flex-col sm:flex-row gap-3 mt-2">
        <button
          onClick={() => onApprove(item.id)}
          data-testid={`approve-${item.id}`}
          className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
        >
          Approve
        </button>
        <button
          onClick={() => onDismiss(item.id)}
          data-testid={`dismiss-${item.id}`}
          className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
        >
          Dismiss
        </button>
      </div>
    </div>
  );
}
