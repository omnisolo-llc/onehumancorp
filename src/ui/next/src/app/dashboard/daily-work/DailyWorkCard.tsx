"use client";

import React, { useState } from "react";

export interface DailyWorkItem {
  id: string;
  intent: string;
  customer_info?: { name?: string; message?: string };
  suggested_actions?: Array<{ action_type?: string; message?: string }>;
  status: string;
}

interface Props {
  item: DailyWorkItem;
  onApprove: (id: string) => void;
  onDismiss: (id: string) => void;
}

export function DailyWorkCard({ item, onApprove, onDismiss }: Props) {
  const [expanded, setExpanded] = useState(false);

  const customerName = item.customer_info?.name || "System Alert";
  const actionDetails = item.suggested_actions?.[0];
  const title = actionDetails?.action_type || item.intent;
  const description = actionDetails?.message || "Please review this item.";

  // Determine card type styling
  let badgeColor = "text-indigo-600 bg-indigo-50 dark:text-indigo-400 dark:bg-indigo-900/30";
  let badgeIcon = "📋";
  let approveText = "Approve";

  if (title.toLowerCase().includes("reply")) {
    badgeColor = "text-blue-600 bg-blue-50 dark:text-blue-400 dark:bg-blue-900/30";
    badgeIcon = "💬";
    approveText = "Review Draft & Send Reply";
  } else if (title.toLowerCase().includes("quote")) {
    badgeColor = "text-emerald-600 bg-emerald-50 dark:text-emerald-400 dark:bg-emerald-900/30";
    badgeIcon = "📄";
    approveText = "Review Draft & Send Quote";
  } else if (title.toLowerCase().includes("alert") || title.toLowerCase().includes("system")) {
    badgeColor = "text-amber-600 bg-amber-50 dark:text-amber-400 dark:bg-amber-900/30";
    badgeIcon = "⚠️";
    approveText = "Take Action";
  }

  const handleActionClick = () => {
    if (!expanded) {
      setExpanded(true);
    } else {
      onApprove(item.id);
    }
  };

  return (
    <div
      data-testid={`daily-work-card-${item.id}`}
      className="glassmorphism p-5 shadow-sm flex flex-col gap-4 mb-4 transition-all duration-300 w-full max-w-full"
    >
      <div className="flex justify-between items-start" onClick={() => setExpanded(!expanded)}>
        <div className="flex flex-col">
          <span className={`text-xs font-bold uppercase tracking-wider px-2 py-1 rounded-md max-w-fit flex items-center gap-1 ${badgeColor}`}>
            <span>{badgeIcon}</span> {item.intent}
          </span>
          <h3 className="mt-2 text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-tight">
            {customerName}
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
            {title}
          </p>
        </div>
      </div>

      {expanded && (
        <div className="bg-gray-50 dark:bg-gray-800/50 p-4 rounded-lg border border-gray-100 dark:border-gray-700 animate-in fade-in slide-in-from-top-2 duration-200">
          {item.customer_info?.message && (
             <div className="mb-3 pb-3 border-b border-gray-200 dark:border-gray-700">
                <span className="text-xs text-gray-500 uppercase tracking-wider font-semibold">Original Message</span>
                <p className="text-sm text-gray-700 dark:text-gray-300 mt-1 italic">
                  "{item.customer_info.message}"
                </p>
             </div>
          )}
          <span className="text-xs text-gray-500 uppercase tracking-wider font-semibold">AI Suggested Action</span>
          <p className="text-sm text-[#1D1D1F] dark:text-[#F5F5F7] mt-1 font-medium">
            {description}
          </p>
        </div>
      )}

      <div className="flex flex-col sm:flex-row gap-3 mt-2">
        <button
          onClick={handleActionClick}
          data-testid={`approve-${item.id}`}
          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center touch-manipulation"
        >
          {expanded ? approveText : "Review Draft"}
        </button>
        {expanded && (
          <button
            onClick={(e) => { e.stopPropagation(); onDismiss(item.id); }}
            data-testid={`dismiss-${item.id}`}
            className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center touch-manipulation"
          >
            Dismiss
          </button>
        )}
      </div>
    </div>
  );
}
