import React from 'react';

type ProposedAction = {
  action_type: string;
  payload?: any;
};

type ActionCardProps = {
  id: string;
  eventSource: string;
  priority?: string;
  context: string;
  proposedAction?: ProposedAction;
  createdAt: string;
  onDecision: (id: string, approved: boolean) => void;
};

function badgeTone(priority?: string) {
  const normalized = (priority || "").toLowerCase();
  if (["urgent", "high"].includes(normalized)) return "bad";
  if (["action needed", "medium"].includes(normalized)) return "warn";
  if (["fyi", "low"].includes(normalized)) return "good";
  return "neutral";
}

export function ActionCard({
  id,
  eventSource,
  priority,
  context,
  proposedAction,
  createdAt,
  onDecision,
}: ActionCardProps) {
  return (
    <div
      className="mb-4 p-5 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-3 min-h-[44px]"
      data-testid={`triage-card-${id}`}
    >
      <div className="flex items-start justify-between">
        <span className="text-xs font-bold font-outfit uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
          {eventSource || "Unknown Source"}
        </span>
        <span className={`app-badge ${badgeTone(priority)}`}>
          {priority || "Normal"}
        </span>
      </div>

      <div className="mt-2 text-sm leading-6 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium">
        {context || "No context"}
      </div>

      {proposedAction?.action_type && (
        <div className="mt-2 rounded-md border border-blue-200 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
          <div className="text-xs uppercase tracking-wider font-semibold mb-1 opacity-80">
            Proposed: {proposedAction.action_type}
          </div>
          <div>{proposedAction.payload || "No specific payload"}</div>
        </div>
      )}

      <div className="text-xs text-gray-500 font-inter mt-1 mb-2">
        {new Date(createdAt).toLocaleString()}
      </div>

      <div className="flex flex-col sm:flex-row gap-3 mt-auto">
        <button
          className="app-btn-primary flex-1 min-h-[44px] min-w-[44px] justify-center"
          data-testid="approve-proposal"
          onClick={() => onDecision(id, true)}
        >
          ✨ Approve &amp; Execute
        </button>
        <button
          className="px-4 py-2 rounded-[16px] border border-white/40 dark:border-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 hover:bg-white/80 dark:hover:bg-black/40 flex-1 min-h-[44px] min-w-[44px] font-medium transition-colors backdrop-blur-md flex items-center justify-center"
          data-testid="reject-proposal"
          onClick={() => onDecision(id, false)}
        >
          Dismiss
        </button>
      </div>
    </div>
  );
}
