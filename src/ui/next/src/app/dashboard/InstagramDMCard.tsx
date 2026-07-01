import React from 'react';

type InstagramDMCardProps = {
  onApprove?: () => void;
  onDismiss?: () => void;
  approval: any;
};

export const InstagramDMCard: React.FC<InstagramDMCardProps> = ({ approval, onApprove, onDismiss }) => {
  return (
    <div className="mb-4 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3 shadow-sm rounded-[16px]" data-testid="instagram-dm-card">
      <div className="flex items-center gap-2 text-pink-600 font-semibold text-sm">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        <strong className="font-outfit tracking-wide">Instagram DM</strong>
        {approval.lifecycle_state === "PENDING_APPROVAL" && (
          <span className="text-[10px] font-bold uppercase tracking-wider text-green-700 bg-green-100 px-2 py-1 rounded-[8px] ml-auto">
            Action Required: Approve Reply
          </span>
        )}
      </div>

      <div className="bg-white/50 dark:bg-black/20 p-3 rounded-[8px] text-xs text-[#1D1D1F] dark:text-[#F5F5F7] shadow-sm flex flex-col gap-2">
        <div className="flex items-center justify-between">
            <div className="font-semibold text-xs uppercase text-gray-500">Customer Context:</div>
            <div className="flex items-center gap-2">
                <span className="text-lg leading-none">👤</span>
                <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7] triage-context">{(approval.proposed_action || approval.context_payload)?.sender || (approval.proposed_action || approval.context_payload)?.customer_id || approval.context_payload?.sender_id || approval.payload?.sender_id || "@customer"}</span>
            </div>
        </div>
        <div className="text-xs italic break-words mt-1 bg-white/30 dark:bg-black/30 p-2 rounded border border-gray-200 dark:border-gray-700">
            "{(approval.proposed_action || approval.context_payload)?.customer_message || (approval.proposed_action || approval.context_payload)?.original_message || (approval.proposed_action || approval.context_payload)?.description || approval.payload?.original_message || approval.payload?.original_payload?.original_message || "Customer message"}"
        </div>
      </div>

      <div className="text-[#0066FF] font-semibold text-sm mt-1 flex items-center gap-2">
        <span className="text-[10px] font-bold uppercase tracking-wider text-gray-500 bg-gray-100 px-2 py-1 rounded-[8px] mr-2">
          AI Draft
        </span>
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
        </svg>
        Draft Reply
      </div>
      <div className="bg-[#0066FF] p-3 rounded-[8px] text-xs text-white shadow-inner whitespace-pre-wrap break-words">
        {(approval.proposed_action || approval.context_payload)?.draft_reply || (approval.proposed_action || approval.context_payload)?.generated_response || (approval.proposed_action || approval.context_payload)?.final_draft || "Thank you for reaching out!"}
      </div>

      <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
        {onApprove && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onApprove();
            }}
            className="triage-btn-approve flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-pink-600 text-white font-medium hover:bg-pink-700 transition-all duration-200 shadow-md flex items-center justify-center"
            aria-label="Approve & Send" data-testid="approve-instagram-dm" id="approve-instagram-dm"
          >
            ✨ Approve & Send Draft
          </button>
        )}
        {onDismiss && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDismiss();
            }}
            className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
            aria-label="Dismiss"
            data-testid="dismiss-instagram-dm"
          >
            Dismiss
          </button>
        )}
      </div>
    </div>
  );
};
