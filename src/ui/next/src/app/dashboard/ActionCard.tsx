import React from 'react';

export interface ActionCardProps {
  id: string;
  agentType: string;
  context: string;
  draftContent?: string;
  actionType?: string;
  priority?: string;
  isProactive?: boolean;
  onApprove: (id: string) => void;
  onDismiss: (id: string) => void;
}

export function ActionCard({
  id,
  agentType,
  context,
  draftContent,
  actionType,
  priority = 'Normal',
  isProactive = false,
  onApprove,
  onDismiss,
}: ActionCardProps) {
  const badgeTone = (priority: string) => {
    switch (priority?.toLowerCase()) {
      case 'high': return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300';
      case 'low': return 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300';
      default: return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
    }
  };

  if (isProactive) {
    return (
      <div
        data-testid={`action-card-${id}`}
        className="mb-4 p-5 rounded-[16px] glassmorphism border border-orange-400/50 dark:border-orange-500/30 bg-orange-50/50 dark:bg-orange-900/10 shadow-lg relative overflow-hidden w-full max-w-full"
      >
        <div className="absolute top-0 left-0 w-1 h-full bg-orange-500"></div>
        <div className="flex justify-between items-start mb-3">
          <div>
            <h2 className="text-xl font-bold font-outfit text-orange-900 dark:text-orange-100 flex items-center gap-2">
              <span className="text-2xl">✨</span> Needs Attention Today
            </h2>
            <p className="text-orange-800/80 dark:text-orange-200/80 mt-1 text-sm font-medium">{context}</p>
          </div>
          <span className={`px-2 py-1 rounded text-xs font-bold uppercase tracking-wider ${badgeTone(priority)}`}>{priority}</span>
        </div>

        {actionType && (
          <div className="mt-4 mb-5 p-4 rounded-xl bg-white/60 dark:bg-black/40 border border-orange-200 dark:border-orange-900/50">
            <div className="text-xs uppercase tracking-wider font-semibold text-orange-800 dark:text-orange-300 mb-1">Suggested Action: {actionType}</div>
            <div className="text-sm font-medium text-gray-900 dark:text-gray-100">{draftContent}</div>
          </div>
        )}

        <div className="flex flex-col sm:flex-row gap-3 mt-2 w-full">
          <button
            data-testid={`approve-btn-${id}`}
            onClick={() => onApprove(id)}
            className="flex-1 px-6 py-2.5 min-h-[44px] rounded-[16px] bg-orange-500 hover:bg-orange-600 text-white font-medium shadow-sm transition-colors w-full"
          >
            Approve & Execute
          </button>
          <button
            data-testid={`dismiss-btn-${id}`}
            onClick={() => onDismiss(id)}
            className="flex-1 px-6 py-2.5 min-h-[44px] rounded-[16px] bg-white/50 dark:bg-black/30 border border-orange-200 dark:border-orange-900/30 hover:bg-white/80 dark:hover:bg-black/50 text-orange-900 dark:text-orange-100 font-medium transition-colors w-full"
          >
            Dismiss
          </button>
        </div>
      </div>
    );
  }

  return (
    <div
      data-testid={`action-card-${id}`}
      className="mb-4 p-5 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4 w-full max-w-full"
    >
      <div className="flex flex-col gap-1">
        <div className="flex justify-between items-start">
          <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 bg-indigo-50 dark:bg-indigo-900/30 dark:text-indigo-400 px-2 py-1 rounded">
            {agentType.replace('_', ' ')}
          </span>
          <span className={`text-xs font-bold uppercase tracking-wider px-2 py-1 rounded ${badgeTone(priority)}`}>
            {priority}
          </span>
        </div>
        <h3 className="text-[15px] sm:text-[17px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit mt-2 leading-tight">
          {context}
        </h3>
      </div>

      {actionType && (
        <div className="mt-2 rounded-md border border-blue-200 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
          <div className="text-xs uppercase tracking-wider font-semibold opacity-80 mb-1">Proposed Action: {actionType}</div>
          {draftContent}
        </div>
      )}

      <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
        <button
          data-testid={`approve-btn-${id}`}
          onClick={() => onApprove(id)}
          className="flex-1 min-h-[44px] rounded-lg font-bold text-sm bg-[#0066FF] hover:bg-[#0052CC] text-white shadow-sm transition-transform active:scale-[0.98] w-full"
        >
          Approve & Execute
        </button>
        <button
          data-testid={`dismiss-btn-${id}`}
          onClick={() => onDismiss(id)}
          className="flex-1 min-h-[44px] rounded-lg font-bold text-sm bg-gray-200 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-700 text-[#1D1D1F] dark:text-[#F5F5F7] transition-transform active:scale-[0.98] w-full"
        >
          Dismiss
        </button>
      </div>
    </div>
  );
}
