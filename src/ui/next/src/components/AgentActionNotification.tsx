"use client";

import React, { useState } from 'react';

interface AgentActionNotificationProps {
  id: string;
  summary: string;
  draftResponse: string;
  actionSummary: string;
  onApprove: (id: string) => void;
  onEdit: (id: string) => void;
  onDecline: (id: string) => void;
}

export function AgentActionNotification({
  id,
  summary,
  draftResponse,
  actionSummary,
  onApprove,
  onEdit,
  onDecline
}: AgentActionNotificationProps) {
  const [isVisible, setIsVisible] = useState(true);

  if (!isVisible) return null;

  return (
    <div className="fixed bottom-4 left-0 right-0 z-50 flex justify-center px-4 sm:px-0 pointer-events-none">
      <div className="w-full max-w-sm pointer-events-auto bg-white/80 dark:bg-[#1C1C1E]/80 backdrop-blur-2xl saturate-150 rounded-[20px] shadow-[0_8px_30px_rgba(0,0,0,0.12)] border border-white/20 dark:border-white/10 overflow-hidden animate-in slide-in-from-bottom duration-300">

        {/* Header/Summary */}
        <div className="p-4 border-b border-gray-100 dark:border-gray-800 flex items-start gap-3 bg-[#0066FF]/5">
          <div className="w-8 h-8 rounded-full bg-[#0066FF]/20 flex items-center justify-center shrink-0">
            <svg className="w-4 h-4 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <div className="flex-1 min-w-0">
            <h4 className="text-sm font-bold text-[#1D1D1F] dark:text-[#F5F5F7] truncate">{summary}</h4>
            <p className="text-xs text-gray-500 mt-0.5">{actionSummary}</p>
          </div>
        </div>

        {/* Draft Response */}
        <div className="p-4">
          <div className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2">Drafted Response</div>
          <div className="p-3 bg-gray-50 dark:bg-black/20 rounded-[12px] border border-gray-100 dark:border-white/5">
            <p className="text-sm text-gray-700 dark:text-gray-300 italic line-clamp-3">"{draftResponse}"</p>
          </div>
        </div>

        {/* Actions */}
        <div className="p-3 flex gap-2 border-t border-gray-100 dark:border-gray-800 bg-gray-50/50 dark:bg-white/5">
          <button
            onClick={() => { setIsVisible(false); onDecline(id); }}
            className="flex-1 py-2 px-3 bg-white dark:bg-transparent border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 text-gray-600 dark:text-gray-300 text-xs font-bold rounded-[12px] transition-colors"
          >
            Decline
          </button>
          <button
            onClick={() => { setIsVisible(false); onEdit(id); }}
            className="flex-1 py-2 px-3 bg-white dark:bg-transparent border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 text-gray-600 dark:text-gray-300 text-xs font-bold rounded-[12px] transition-colors"
          >
            Edit
          </button>
          <button
            onClick={() => { setIsVisible(false); onApprove(id); }}
            className="flex-[2] py-2 px-3 bg-[#0066FF] hover:bg-[#0052CC] text-white text-xs font-bold rounded-[12px] shadow-sm transition-colors"
          >
            Approve & Send
          </button>
        </div>

      </div>
    </div>
  );
}
