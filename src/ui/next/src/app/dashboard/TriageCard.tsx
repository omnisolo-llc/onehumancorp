import React, { useState } from 'react';

export type TriageItem = {
  id: string;
  source: string;
  priority: string;
  context: string;
  draft_response?: string;
  proposed_action?: any;
  status: string;
  created_at: string;
};

interface TriageCardProps {
  item: TriageItem;
  onApprove: (id: string) => Promise<void>;
  onDismiss: (id: string) => Promise<void>;
}

export function TriageCard({ item, onApprove, onDismiss }: TriageCardProps) {
  const [loading, setLoading] = useState(false);

  const priorityColor = item.priority === 'URGENT'
    ? 'text-red-600 bg-red-50 dark:bg-red-900/30'
    : item.priority === 'ACTION_NEEDED'
      ? 'text-amber-600 bg-amber-50 dark:bg-amber-900/30'
      : 'text-blue-600 bg-blue-50 dark:bg-blue-900/30';

  const handleApprove = async () => {
    setLoading(true);
    await onApprove(item.id);
    setLoading(false);
  };

  const handleDismiss = async () => {
    setLoading(true);
    await onDismiss(item.id);
    setLoading(false);
  };

  return (
    <div className="glassmorphism p-4 sm:p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-3 transition-all hover:shadow-md mb-3 min-w-[320px] max-w-full">
      <div className="flex items-center justify-between">
        <span className="text-xs font-bold font-outfit uppercase tracking-wider text-gray-500 bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded-md">
          {item.source}
        </span>
        <span className={`text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-md ${priorityColor}`}>
          {item.priority.replace('_', ' ')}
        </span>
      </div>

      <div className="text-sm text-[#1D1D1F] dark:text-[#F5F5F7] font-medium leading-relaxed">
        {item.context}
      </div>

      {item.draft_response && (
        <div className="mt-2 p-3 bg-white/50 dark:bg-black/20 rounded-[8px] text-sm text-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-700">
          <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1 block">Draft Reply</span>
          {item.draft_response}
        </div>
      )}

      {item.proposed_action?.description && (
        <div className="mt-2 text-sm text-gray-600 dark:text-gray-400">
          <span className="font-semibold text-gray-700 dark:text-gray-300">Action: </span>
          {item.proposed_action.description}
        </div>
      )}

      <div className="flex flex-col sm:flex-row gap-3 mt-2 w-full">
        <button
          onClick={handleApprove}
          disabled={loading}
          className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md flex items-center justify-center disabled:opacity-50"
          aria-label="Approve & Send"
          data-testid="approve-triage"
        >
          {loading ? "Processing..." : "Approve & Send"}
        </button>
        <button
          onClick={handleDismiss}
          disabled={loading}
          className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center disabled:opacity-50"
          aria-label="Dismiss item"
          data-testid="dismiss-triage"
        >
          Dismiss
        </button>
      </div>
    </div>
  );
}
