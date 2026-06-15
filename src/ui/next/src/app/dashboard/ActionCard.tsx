import React from 'react';
import Link from 'next/link';

interface ActionCardProps {
  approval: any;
  onApprove: (id: string, editedPayload?: any) => void;
  onDismiss: (id: string, editedPayload?: any) => void;
}

export function ActionCard({ approval, onApprove, onDismiss }: ActionCardProps) {
  const payload = approval.payload || approval.proposed_action || approval.context_payload;
  const source = payload?.source || "Customer";
  const customerMessage = payload?.original_message || payload?.message || payload?.customer_message || "Customer message";
  const draftReply = payload?.generated_response || payload?.draft_reply || "Ready to send.";

  return (
    <div className="app-list-item flex flex-col items-start gap-3 w-full bg-white dark:bg-[#1C1C1E] rounded-xl p-4 shadow-sm border border-gray-100 dark:border-white/5" data-testid="ambassador-reply-action-card">
      <div className="w-full">
        <div className="app-list-title text-[#1D1D1F] dark:text-[#F5F5F7] font-semibold text-[15px]">
          Action Required: Approve Reply
        </div>
        <div className="app-list-subtitle font-semibold text-gray-900 dark:text-gray-100 mt-1.5 text-[13px]">
          1 New Message from {source}
        </div>
        <div className="app-list-subtitle mt-2 bg-gray-50 dark:bg-black/20 p-3 rounded-lg border border-gray-100 dark:border-white/5 text-xs italic text-gray-700 dark:text-gray-300">
          "{customerMessage}"
        </div>
        <div className="app-list-subtitle mt-2 p-3 rounded-lg bg-blue-50/50 dark:bg-blue-900/10 border border-blue-100 dark:border-blue-900/30 text-blue-900 dark:text-blue-100 text-[13px] relative overflow-hidden">
          <div className="absolute top-0 left-0 w-1 h-full bg-[#0066FF]"></div>
          <span className="font-bold text-[#0066FF] text-[10px] uppercase tracking-wider mb-1.5 flex items-center gap-1.5">
             <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
             AI Draft
          </span>
          {draftReply}
        </div>
      </div>
      <div className="flex gap-2 w-full mt-2">
        <button
          type="button"
          className="app-btn-primary flex-[2] min-h-[44px] min-w-[44px] py-2.5 px-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-semibold rounded-[12px] shadow-sm shadow-[#0066FF]/20 flex items-center justify-center gap-2 transition-all active:scale-[0.98]"
          onClick={() => onApprove(approval.id, true)}
          data-testid="approve-ambassador-reply"
        >
          ✨ 1-Tap Approve
        </button>
        <Link
          href="/inbox"
          className="app-button flex-1 min-h-[44px] min-w-[44px] py-2.5 px-4 text-center bg-gray-100 hover:bg-gray-200 dark:bg-white/10 dark:hover:bg-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium rounded-[12px] transition-all"
        >
          Edit
        </Link>
        <button
          type="button"
          className="app-button min-h-[44px] min-w-[44px] w-[44px] p-0 flex items-center justify-center bg-white dark:bg-[#1C1C1E] hover:bg-red-50 dark:hover:bg-red-900/20 text-gray-400 hover:text-red-500 border border-gray-200 dark:border-white/10 rounded-[12px] transition-all"
          onClick={() => onDismiss(approval.id, false)}
          aria-label="Discard"
          data-testid="dismiss-ambassador-reply"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
        </button>
      </div>
    </div>
  );
}
