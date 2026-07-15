import React from 'react';

type InstagramDMCardProps = {
  onApprove?: () => void;
  onDismiss?: () => void;
  approval: any;
};

export const InstagramDMCard: React.FC<InstagramDMCardProps> = ({ approval, onApprove, onDismiss }) => {
  const replyContent = (approval.proposed_action || approval.context_payload)?.draft_reply || (approval.proposed_action || approval.context_payload)?.generated_response || "";
  const isDepositLink = replyContent.toLowerCase().includes('deposit');

  return (
    <div className="mb-4 p-4 sm:p-5 max-w-[375px] w-full mx-auto bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] flex flex-col gap-4 shadow-sm overflow-hidden" data-testid="instagram-dm-card">
      <div className="flex items-center gap-2 text-pink-600 dark:text-pink-400 font-semibold text-sm">
        <div className="w-8 h-8 rounded-full bg-pink-50 dark:bg-pink-900/30 flex items-center justify-center">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
        </div>
        <strong className="font-outfit tracking-wide uppercase text-xs">Instagram DM Triage</strong>
      </div>

      <div className="text-sm text-gray-700 dark:text-gray-300 font-medium break-words px-1">
        <span className="text-xs text-gray-500 uppercase tracking-wider block mb-1">Customer Message:</span>
        <div className="triage-context inline break-words">{(approval.proposed_action || approval.context_payload)?.customer_message || (approval.proposed_action || approval.context_payload)?.original_message || (approval.proposed_action || approval.context_payload)?.description}</div>
      </div>

      <div className="text-sm text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/60 dark:bg-black/30 p-4 rounded-[12px] break-words shadow-sm border border-gray-100 dark:border-gray-800">
        <div className="flex items-center justify-between mb-2">
          <div className="font-semibold text-xs uppercase tracking-wider text-indigo-600 dark:text-indigo-400">Proposed Reply:</div>
          {isDepositLink && (
            <span className="bg-green-100 text-green-800 dark:bg-green-900/40 dark:text-green-300 text-[10px] font-bold px-2 py-0.5 rounded-full flex items-center gap-1">
              <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
              Payment Link
            </span>
          )}
        </div>
        <div className="whitespace-pre-wrap leading-relaxed">{replyContent}</div>
      </div>

      <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
        {onApprove && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onApprove();
            }}
            className="triage-btn-approve flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[12px] bg-[#0066FF] text-white font-bold hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-200 shadow-md flex items-center justify-center gap-2"
            aria-label="Approve & Send" data-testid="feed-approve-btn" id="approve-instagram-dm"
          >
            <span>✨</span> Approve & Send
          </button>
        )}
        {onDismiss && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDismiss();
            }}
            className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[12px] bg-[rgba(255,255,255,0.5)] dark:bg-[rgba(22,22,26,0.5)] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-[#1D1D1F] dark:text-[#F5F5F7] font-semibold hover:bg-[rgba(255,255,255,0.8)] dark:hover:bg-[rgba(22,22,26,0.8)] active:scale-[0.98] transition-all duration-200 flex items-center justify-center shadow-sm"
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
