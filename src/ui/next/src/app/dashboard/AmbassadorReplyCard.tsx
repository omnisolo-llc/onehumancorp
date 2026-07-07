import React from 'react';

type AmbassadorReplyCardProps = {
  approval: any;
  onApprove?: () => void;
  onDismiss?: () => void;
};

export const AmbassadorReplyCard: React.FC<AmbassadorReplyCardProps> = ({ approval, onApprove, onDismiss }) => {
  const payloadSource = approval.payload?.original_payload || approval.payload || approval.proposed_action || approval.context_payload || {};
  const pastOrders = payloadSource.past_orders;
  const contextUsed = payloadSource.context_used;

  return (
    <div className="app-list-item mb-4 p-4 bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="ambassador-reply-card">
      <div className="text-gray-900 dark:text-gray-100 font-bold mb-2">1 New Message from {(approval.payload?.source || (approval.proposed_action || approval.context_payload)?.source || (approval.proposed_action || approval.context_payload)?.original_payload?.source || approval.payload?.original_payload?.source || "unknown").replace("_", " ")}</div>

      {(pastOrders || contextUsed) && (
        <div className="bg-blue-50/50 dark:bg-blue-900/20 rounded-xl p-3 border border-blue-100 dark:border-blue-800/30">
          <div className="flex items-center gap-2 mb-2">
             <svg className="w-4 h-4 text-[#0066FF] dark:text-[#3388FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
             </svg>
             <span className="text-xs font-semibold uppercase tracking-wider text-[#0066FF] dark:text-[#3388FF]">Customer Context</span>
          </div>
          <div className="flex flex-col gap-2 text-xs text-gray-700 dark:text-gray-300">
             {pastOrders && (
                <div className="flex items-start gap-2">
                   <span className="text-lg leading-none">🛍️</span>
                   <span>{pastOrders}</span>
                </div>
             )}
             {contextUsed && (
                <div className="flex items-start gap-2">
                   <span className="text-lg leading-none">🧠</span>
                   <span className="italic line-clamp-2">{contextUsed.substring(0, 150)}{contextUsed.length > 150 ? '...' : ''}</span>
                </div>
             )}
          </div>
        </div>
      )}

      <div className="flex flex-col sm:flex-row gap-2 sm:items-center text-[#0066FF] font-semibold text-sm">
        {approval.lifecycle_state === "PENDING_APPROVAL" && (
          <span className="text-[10px] font-bold uppercase tracking-wider text-green-700 bg-green-100 px-2 py-1 rounded-[8px] self-start sm:self-center">
            Action Required: Approve Reply
          </span>
        )}
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
        </svg>
        Customer Inquiry
      </div>
      <div className="bg-white/50 dark:bg-black/20 p-3 rounded-[8px] text-xs text-[#1D1D1F] dark:text-[#F5F5F7] italic shadow-sm break-words">
        "{approval.payload?.original_message || (approval.proposed_action || approval.context_payload)?.original_message || (approval.proposed_action || approval.context_payload)?.original_payload?.original_message || approval.payload?.original_payload?.original_message || "Customer message"}"
      </div>
      <div className="text-[#0066FF] font-semibold text-sm mt-2 flex items-center gap-2">
        <span className="text-[10px] font-bold uppercase tracking-wider text-gray-500 bg-gray-100 px-2 py-1 rounded-[8px] mr-2">
          AI Draft
        </span>
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
        </svg>
        Draft Reply
      </div>
      <div className="bg-[#0066FF] p-3 rounded-[8px] text-xs text-white shadow-inner">
        {approval.payload?.generated_response || (approval.proposed_action || approval.context_payload)?.generated_response || (approval.proposed_action || approval.context_payload)?.original_payload?.generated_response || approval.payload?.original_payload?.generated_response || "Ready to send."}
      </div>
      <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
        {onApprove && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onApprove();
            }}
            className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
            aria-label="✨ Approve & Send Draft" data-testid="feed-approve-btn"
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
            data-testid="feed-dismiss-btn"
          >
            Dismiss
          </button>
        )}
      </div>
    </div>
  );
};
