import React from 'react';

type ReviewReplyCardProps = {
  approval: any;
  onApprove?: () => void;
  onDismiss?: () => void;
  isEditing?: boolean;
  editContent?: string;
  onEdit?: () => void;
  onCancelEdit?: () => void;
  onSaveEdit?: () => void;
  setEditContent?: (content: string) => void;
};

export const ReviewReplyCard: React.FC<ReviewReplyCardProps> = ({
  approval,
  onApprove,
  onDismiss,
  isEditing = false,
  editContent = "",
  onEdit,
  onCancelEdit,
  onSaveEdit,
  setEditContent
}) => {
  const payloadSource = approval.payload?.original_payload || approval.payload || approval.proposed_action || approval.context_payload || {};
  const pastOrders = payloadSource.past_orders;
  const rating = payloadSource.rating || 5;
  const comment = payloadSource.comment || "No comment provided.";
  const source = payloadSource.source || "unknown";

  const isPositive = rating >= 3;
  const sentimentColor = isPositive ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400';
  const sentimentBg = isPositive ? 'bg-green-100 dark:bg-green-900/30' : 'bg-red-100 dark:bg-red-900/30';

  return (
    <div className="app-list-item mb-4 p-4 bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] flex flex-col gap-3" data-testid="review-reply-card">
      <div className={`font-bold mb-2 ${sentimentColor}`}>
        {isPositive ? `Positive Review on ${source}` : `Action Required: Negative Review on ${source}`}
      </div>

      {pastOrders && (
        <div className="bg-blue-50/50 dark:bg-blue-900/20 rounded-xl p-3 border border-blue-100 dark:border-blue-800/30">
          <div className="flex items-center gap-2 mb-2">
             <svg className="w-4 h-4 text-[#0066FF] dark:text-[#3388FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
             </svg>
             <span className="text-xs font-semibold uppercase tracking-wider text-[#0066FF] dark:text-[#3388FF]">Customer Context</span>
          </div>
          <div className="flex flex-col gap-2 text-xs text-gray-700 dark:text-gray-300">
             <div className="flex items-start gap-2">
                <span className="text-lg leading-none">🛍️</span>
                <span>{pastOrders}</span>
             </div>
          </div>
        </div>
      )}

      <div className="flex flex-col sm:flex-row gap-2 sm:items-center text-gray-700 dark:text-gray-300 font-semibold text-sm">
        <span className={`text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-[8px] self-start sm:self-center ${sentimentBg} ${sentimentColor}`}>
          {rating} Stars
        </span>
      </div>
      <div className="bg-white/50 dark:bg-black/20 p-3 rounded-[8px] text-xs text-[#1D1D1F] dark:text-[#F5F5F7] italic shadow-sm break-words">
        "{comment}"
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
      {isEditing ? (
        <div className="flex flex-col gap-3 w-full mt-2">
          <textarea
            className="w-full min-h-[44px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
            rows={4}
            value={editContent}
            onChange={(e) => setEditContent && setEditContent(e.target.value)}
            data-testid="feed-edit-input"
            autoFocus
          />
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={(e) => {
                e.stopPropagation();
                onSaveEdit && onSaveEdit();
              }}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Save & Send Draft"
              data-testid="feed-save-edit-btn"
            >
              Save & Send
            </button>
            <button
              onClick={(e) => {
                e.stopPropagation();
                onCancelEdit && onCancelEdit();
              }}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Cancel Edit"
              data-testid="feed-cancel-edit-btn"
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <>
          <div className="bg-[#0066FF] p-3 rounded-[8px] text-xs text-white shadow-inner">
            {payloadSource.generated_response || "Ready to send."}
          </div>
          <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
            {onApprove && (
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onApprove();
                }}
                className="flex-[2] min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                aria-label="Approve & Send" data-testid="feed-approve-btn"
              >
                Approve & Send
              </button>
            )}
            {onEdit && (
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onEdit();
                }}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Edit"
                data-testid="feed-edit-btn"
              >
                Edit
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
        </>
      )}
    </div>
  );
};
