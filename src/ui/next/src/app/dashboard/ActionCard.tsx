import React from 'react';

export type ActionCardProps = {
  approval: any;
  queuedActionIds: Set<string>;
  handleDecision: (id: string, approved: boolean) => void;
};

export function ActionCard({ approval, queuedActionIds, handleDecision }: ActionCardProps) {
  const isQueued = queuedActionIds.has(approval.id);
  const eventSource = approval.event_source?.replace('_', ' ') || 'Unknown Source';
  const requiresReview = approval.lifecycle_state === 'PENDING_APPROVAL';
  const contextPayload = approval.context_payload;
  const proposedAction = approval.proposed_action;
  const payload = proposedAction || contextPayload;

  const description = payload?.description || proposedAction?.message || proposedAction?.action_type || approval.event_source;
  const featureType = payload?.feature_type;

  const hasSpecificCard = payload?.context || payload?.remaining_stock !== undefined || ["quote_draft", "social_post_draft", "ambassador_reply", "incident_resolution", "instagram_dm"].includes(featureType);

  return (
    <div
      className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4 opacity-90 transition-all duration-200 w-full"
      data-testid={`action-card-${approval.id}`}
    >
      <div className="flex flex-col gap-1">
        <div className="flex items-center gap-2 flex-wrap">
          <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
            {eventSource}
          </span>
          {requiresReview && (
            <span className="text-xs font-bold uppercase tracking-wider text-red-600 bg-red-50 dark:text-red-400 dark:bg-red-900/30 px-2 py-1 rounded-md">
              Requires Review
            </span>
          )}
          {isQueued && (
            <span className="text-xs font-bold uppercase tracking-wider text-yellow-600 bg-yellow-50 dark:text-yellow-400 dark:bg-yellow-900/30 px-2 py-1 rounded-md shadow-sm border border-yellow-200" data-testid="queued-badge">
              Queued
            </span>
          )}
        </div>
        <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1">
          {description}
        </h3>

        {hasSpecificCard && (
          <div className="mt-2 flex flex-col gap-1 p-3 bg-gray-50/50 dark:bg-gray-800/50 rounded-lg">
            {featureType === "incident_resolution" && (
              <div className="mb-4 p-4 rounded-xl bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/50 flex flex-col gap-3" data-testid="incident-resolution-card">
                <div className="flex items-center gap-2 text-red-600 font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                  </svg>
                  CRITICAL INCIDENT
                </div>
                <p className="text-gray-700 dark:text-gray-300 text-sm">
                  {payload?.description || 'An operational issue requires immediate attention.'}
                </p>
              </div>
            )}
            {featureType === "instagram_dm" && (
              <div className="mb-4 p-4 rounded-xl glassmorphism border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="instagram-dm-card">
                <div className="flex items-center gap-2 text-pink-600 font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                  </svg>
                  Instagram DM
                </div>
                <div className="text-xs text-gray-500 font-medium break-words">
                  Customer: {payload.customer_message}
                </div>
                <div className="text-xs text-gray-900 dark:text-gray-100 italic line-clamp-3 bg-white/50 dark:bg-black/20 p-2 rounded break-words">
                  Draft: {payload.draft_reply}
                </div>
              </div>
            )}
            {featureType === "ambassador_reply" && (
              <div className="mb-4 p-4 rounded-xl glassmorphism border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="ambassador-reply-card">
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                  </svg>
                  Ambassador Reply
                </div>
                <div className="text-xs text-gray-500 font-medium break-words">
                  Question: {payload.question || payload.description}
                </div>
                <div className="text-xs text-[#0066FF] bg-blue-50 dark:bg-blue-900/20 p-2 rounded line-clamp-3 break-words">
                  Draft: {payload.draft || "I can help with that."}
                </div>
              </div>
            )}
            {featureType === "quote_draft" && (
              <div className="mb-4 p-4 rounded-xl glassmorphism border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="quote-draft-card">
                <div className="flex items-center gap-2 text-purple-600 font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                  Quote Prepared
                </div>
                <div className="text-xs text-gray-500 font-medium">
                  {payload.description || "A new quote draft is ready for review."}
                </div>
              </div>
            )}
            {featureType === "social_post_draft" && (
              <div className="mb-4 p-4 rounded-xl glassmorphism border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="social-post-card">
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
                  </svg>
                  Social Media Draft
                </div>
                <div className="text-xs text-gray-900 dark:text-gray-100 italic break-words">
                  "{payload.draft || payload.description}"
                </div>
              </div>
            )}
          </div>
        )}

        <div className="flex flex-col gap-3 w-full mt-2">
          {featureType === 'incident_resolution' ? (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-red-600 text-white font-medium hover:bg-red-700 transition-all duration-200 shadow-md flex items-center justify-center"
                aria-label="Execute Plan"
                data-testid="approve-incident-resolution"
              >
                Execute Plan
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Dismiss Plan"
                data-testid="dismiss-incident-resolution"
              >
                Dismiss
              </button>
            </div>
          ) : featureType === 'instagram_dm' ? (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-pink-600 text-white font-medium hover:bg-pink-700 transition-all duration-200 shadow-md flex items-center justify-center text-center leading-tight py-2"
                aria-label="Approve & Send"
                data-testid="approve-instagram-dm"
              >
                Approve & Send
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                aria-label="Dismiss"
                data-testid="dismiss-instagram-dm"
              >
                Dismiss
              </button>
            </div>
          ) : featureType === 'social_post_draft' ? (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center py-2"
                aria-label="Approve & Publish"
                data-testid="approve-social-post"
              >
                Approve & Publish
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                aria-label="Ask Agent to Adjust"
                data-testid="reject-social-post"
              >
                Adjust
              </button>
            </div>
          ) : featureType === 'ambassador_reply' ? (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center text-center leading-tight py-2"
                aria-label="Approve & Send Reply"
                data-testid="approve-ambassador-reply"
              >
                Approve & Send Reply
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                aria-label="Ask Agent to Adjust"
                data-testid="reject-ambassador-reply"
              >
                Adjust
              </button>
            </div>
          ) : payload?.context?.flash_sale === true ? (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center py-2"
                aria-label="Approve & Launch Sale"
                data-testid="approve-sale"
              >
                Approve & Launch Sale
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                aria-label="Dismiss proposal"
                data-testid="dismiss-sale"
              >
                Dismiss
              </button>
            </div>
          ) : payload?.context?.weekly_health_report === true ? (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-all duration-200 shadow-md flex items-center justify-center py-2"
                aria-label="Draft it"
                data-testid="approve-draft"
              >
                Yes, draft it!
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                aria-label="Dismiss proposal"
                data-testid="dismiss-draft"
              >
                Dismiss
              </button>
            </div>
          ) : payload?.remaining_stock !== undefined ? (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-amber-500 text-white font-medium hover:bg-amber-600 transition-all duration-200 shadow-md flex items-center justify-center py-2"
                aria-label="Approve Restock"
                data-testid="approve-restock"
              >
                Approve Restock
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                aria-label="Dismiss restock"
                data-testid="dismiss-restock"
              >
                Dismiss
              </button>
            </div>
          ) : featureType === 'quote_draft' ? (
            <>
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3 py-2"
                aria-label="Approve & Send Proposal"
                data-testid="approve-send-proposal"
              >
                Approve & Send Proposal
              </button>
              <div className="flex flex-col sm:flex-row gap-3 w-full">
                <a
                  href={`/quoting?id=${approval.id}`}
                  className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                  aria-label="Edit Draft"
                  data-testid="edit-proposal"
                >
                  Edit Draft
                </a>
                <button
                  onClick={() => handleDecision(approval.id, false)}
                  className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                  aria-label="Ask Agent to Adjust"
                  data-testid="reject-proposal"
                >
                  Adjust
                </button>
              </div>
            </>
          ) : (
            <>
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3 py-2"
                aria-label="Approve proposal"
                data-testid="approve-proposal"
              >
                Approve
              </button>
              <div className="flex flex-col sm:flex-row gap-3 w-full">
                <button
                  onClick={() => {}}
                  className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                  aria-label="Edit proposal"
                  data-testid="edit-proposal-fallback"
                >
                  Edit
                </button>
                <button
                  onClick={() => handleDecision(approval.id, false)}
                  className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center py-2"
                  aria-label="Reject proposal"
                  data-testid="reject-proposal"
                >
                  Deny
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
