import React from 'react';

export interface ActionCardProps {
  id: string;
  agentType: string;
  context: string;
  draftContent?: string;
  priority?: string;
  onApprove: (id: string) => void;
  onDismiss: (id: string) => void;
  isActionInProgress?: boolean;
}

function badgeTone(priority?: string) {
  const normalized = (priority || "").toLowerCase();
  if (["urgent", "high"].includes(normalized)) return "bad";
  if (["action needed", "medium"].includes(normalized)) return "warn";
  if (["fyi", "low"].includes(normalized)) return "good";
  return "neutral";
}

export function ActionCard({
  id,
  agentType,
  context,
  draftContent,
  priority,
  onApprove,
  onDismiss,
  isActionInProgress = false
}: ActionCardProps) {
  return (
    <div
      className="mb-4 glassmorphism rounded-[16px] overflow-hidden border border-white/40 dark:border-white/10 shadow-sm transition-all flex flex-col"
      data-testid={`action-card-${id}`}
    >
      <div className="p-4 flex flex-col gap-3">
        <div className="flex justify-between items-start gap-2">
          <div className="flex items-center gap-2">
            <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 bg-indigo-100 dark:bg-indigo-900/30 dark:text-indigo-300 px-2 py-1 rounded">
              {agentType || "Agent"}
            </span>
          </div>
          <span className={`app-badge ${badgeTone(priority)} whitespace-nowrap`}>
            {priority || "Normal"}
          </span>
        </div>

        <h3 className="text-base font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit leading-snug">
          {context}
        </h3>

        {draftContent && (
          <div className="mt-2 rounded-md border border-blue-200 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 p-3 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
            <span className="text-xs uppercase tracking-wider font-semibold opacity-70 block mb-1">Proposed Action</span>
            {draftContent}
          </div>
        )}
      </div>

      <div className="p-4 border-t border-white/20 dark:border-white/5 bg-white/30 dark:bg-black/20 flex flex-col sm:flex-row gap-3 mt-auto">
        <button
          className="app-btn-primary flex-1 min-h-[44px] justify-center text-sm font-bold shadow-sm"
          data-testid="approve-btn"
          onClick={() => onApprove(id)}
          disabled={isActionInProgress}
        >
          ✨ Approve
        </button>
        <button
          className="flex-1 min-h-[44px] justify-center px-4 rounded-[6px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#2C2C2E] text-[#1D1D1F] dark:text-[#F5F5F7] text-sm font-semibold hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors shadow-sm disabled:opacity-50"
          data-testid="dismiss-btn"
          onClick={() => onDismiss(id)}
          disabled={isActionInProgress}
        >
          Dismiss
        </button>
      </div>
    </div>
  );
}
