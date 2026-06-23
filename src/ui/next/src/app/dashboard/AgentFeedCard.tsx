import React from "react";
import { AmbassadorReplyCard } from "./AmbassadorReplyCard";
import { InstagramDMCard } from "./InstagramDMCard";

export function AgentFeedCard({
  approval,
  queuedActionIds,
  editingId,
  editContent,
  setEditingId,
  setEditContent,
  handleDecision,
  editQuotePrice,
  setEditQuotePrice,
  editQuoteScope,
  setEditQuoteScope
}: any) {
  return (
    <div
      key={approval.id}
      className="glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 rounded-[16px] shadow-sm flex flex-col gap-4 transition-all duration-300"
      data-testid={`triage-card-${approval.id}`}
    >
      <div className="flex flex-col gap-1">
        <div className="flex items-center gap-2">
          <span className="text-xs font-bold uppercase tracking-wider text-[#0066FF] bg-[#0066FF]/10 dark:bg-[#0066FF]/20 px-2 py-1 rounded-[8px]">
            Approval
          </span>
          <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-[8px]">
            {approval.event_source.replace("_", " ")}
          </span>
          {approval.lifecycle_state === "PENDING_APPROVAL" && (
            <span className="text-xs font-bold uppercase tracking-wider text-green-700 bg-green-100 px-2 py-1 rounded-[8px]">
              Requires Review
            </span>
          )}
          {queuedActionIds.has(approval.id) && (
            <span
              className="text-xs font-bold uppercase tracking-wider text-yellow-600 bg-yellow-50 px-2 py-1 rounded-[8px] shadow-sm border border-yellow-200"
              data-testid="queued-badge"
            >
              Queued
            </span>
          )}
        </div>
        <h3 className="text-lg font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1 tracking-wide">
          {approval.context_payload?.description ||
            approval.proposed_action?.message ||
            approval.proposed_action?.action_type ||
            approval.event_source}
        </h3>
        {((approval.proposed_action || approval.context_payload)?.context ||
          (approval.proposed_action || approval.context_payload)?.remaining_stock !== undefined ||
          (approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" ||
          (approval.proposed_action || approval.context_payload)?.feature_type === "social_post_draft" ||
          (approval.proposed_action || approval.context_payload)?.feature_type === "ambassador_reply" ||
          (approval.proposed_action || approval.context_payload)?.feature_type === "incident_resolution" ||
          (approval.proposed_action || approval.context_payload)?.feature_type === "booking_draft" ||
          (approval.proposed_action || approval.context_payload)?.feature_type === "instagram_dm") && (
          <div className="mt-2 flex flex-col gap-1 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-[8px]">
            {(approval.proposed_action || approval.context_payload)?.feature_type === "incident_resolution" && (
              <div
                className="mb-4 p-4 rounded-[16px] bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/50 flex flex-col gap-3"
                data-testid="incident-resolution-card"
              >
                <div className="flex items-center gap-2 text-red-600 font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                    />
                  </svg>
                  CRITICAL INCIDENT
                </div>
                <p className="text-gray-700 dark:text-gray-300 text-sm break-words">
                  {(approval.proposed_action || approval.context_payload)?.description ||
                    "An operational issue requires immediate attention."}
                </p>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)?.feature_type === "onboarding_welcome" && (
              <div
                className="mb-4 p-4 rounded-[16px] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3"
                data-testid="onboarding-welcome-card"
              >
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                  Setup Complete
                </div>
                <p className="text-sm text-gray-800 dark:text-gray-200">
                  {approval.context_payload?.description ||
                    approval.proposed_action?.description ||
                    "Welcome to OHC! I've set up your business. Click here to review your new storefront."}
                </p>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)?.feature_type === "instagram_dm" && (
              <InstagramDMCard approval={approval} />
            )}
            {(approval.proposed_action || approval.context_payload)?.feature_type === "ambassador_reply" && (
              <AmbassadorReplyCard approval={approval} />
            )}
            {(approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" && (
              <div
                className="mb-4 p-4 rounded-[16px] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3"
                data-testid="quote-draft-card"
              >
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                  ESTIMATE DRAFT
                </div>
                <div className="flex flex-col gap-2">
                  <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                    Proposed Scope
                  </span>
                  {editingId === approval.id ? (
                    <input
                      type="text"
                      className="w-full min-h-[44px] p-2 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all"
                      value={editQuoteScope}
                      onChange={(e) => setEditQuoteScope(e.target.value)}
                      placeholder="e.g., 3 Custom Tier Cakes"
                    />
                  ) : (
                    <p className="text-sm text-gray-800 dark:text-gray-200">
                      {(approval.proposed_action || approval.context_payload)?.proposed_scope ||
                        "Standard Service"}
                    </p>
                  )}
                  <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider mt-2">
                    Proposed Price
                  </span>
                  {editingId === approval.id ? (
                    <div className="relative">
                      <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500">
                        $
                      </span>
                      <input
                        type="number"
                        className="w-full min-h-[44px] pl-7 p-2 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all"
                        value={editQuotePrice}
                        onChange={(e) => setEditQuotePrice(e.target.value)}
                      />
                    </div>
                  ) : (
                    <p className="text-lg font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
                      ${(approval.proposed_action || approval.context_payload)?.proposed_price || "0.00"}
                    </p>
                  )}
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)?.feature_type === "social_post_draft" && (
              <div
                className="mb-4 p-4 rounded-[16px] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3"
                data-testid="social-post-draft-card"
              >
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                    />
                  </svg>
                  SOCIAL POST
                </div>
                {editingId === approval.id ? (
                  <textarea
                    className="w-full p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all min-h-[100px] resize-none"
                    value={editContent}
                    onChange={(e) => setEditContent(e.target.value)}
                  />
                ) : (
                  <p className="text-sm text-gray-800 dark:text-gray-200 whitespace-pre-wrap italic">
                    "
                    {(approval.proposed_action || approval.context_payload)?.generated_response ||
                      (approval.proposed_action || approval.context_payload)?.draft_reply}
                    "
                  </p>
                )}
                <div className="flex flex-wrap gap-2 mt-2">
                  {((approval.proposed_action || approval.context_payload)?.suggested_hashtags || []).map(
                    (tag: string, i: number) => (
                      <span
                        key={i}
                        className="text-xs text-[#0066FF] bg-[#0066FF]/10 px-2 py-1 rounded-full"
                      >
                        #{tag}
                      </span>
                    )
                  )}
                </div>
              </div>
            )}
            {editingId !== approval.id &&
              (approval.proposed_action || approval.context_payload)?.feature_type !== "quote_draft" &&
              (approval.proposed_action || approval.context_payload)?.feature_type !== "social_post_draft" &&
              (approval.proposed_action || approval.context_payload)?.feature_type !== "incident_resolution" &&
              (approval.proposed_action || approval.context_payload)?.feature_type !== "ambassador_reply" &&
              (approval.proposed_action || approval.context_payload)?.feature_type !== "instagram_dm" && (
                <pre className="text-xs text-gray-500 font-mono overflow-x-hidden whitespace-pre-wrap break-words max-h-32 overflow-y-auto">
                  {(approval.proposed_action || approval.context_payload)?.generated_response ||
                    (approval.proposed_action || approval.context_payload)?.draft_reply ||
                    (approval.proposed_action || approval.context_payload)?.context}
                </pre>
              )}
            {(approval.proposed_action || approval.context_payload)?.remaining_stock !== undefined && (
              <span className="text-xs font-semibold mt-1">
                Stock:{" "}
                {(approval.proposed_action || approval.context_payload)?.remaining_stock}
              </span>
            )}
          </div>
        )}
      </div>

      <div className="mt-2 flex flex-col gap-3 w-full">
        {(approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" &&
        editingId === approval.id ? (
          <div className="flex flex-col gap-3 w-full">
            <div className="flex gap-3">
              <button
                onClick={() => {
                  const updatedPayload = {
                    ...approval.context_payload,
                    proposed_price: parseFloat(editQuotePrice) || 0,
                    proposed_scope: editQuoteScope,
                  };
                  handleDecision(
                    approval.id,
                    true,
                    JSON.stringify(updatedPayload),
                    approval.event_source
                  );
                  setEditingId(null);
                }}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                data-testid="save-quote-draft"
              >
                Save & Approve
              </button>
              <button
                onClick={() => setEditingId(null)}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                data-testid="cancel-edit-quote"
              >
                Cancel
              </button>
            </div>
            <button
              onClick={() =>
                handleDecision(approval.id, false, undefined, approval.event_source)
              }
              className="w-full min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Reject proposal"
              data-testid="reject-proposal"
            >
              Deny
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)?.feature_type === "incident_resolution" ? (
          <>
            <button
              onClick={() =>
                handleDecision(approval.id, true, undefined, approval.event_source)
              }
              className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-1"
              aria-label="Acknowledge and Resolve"
              data-testid="approve-proposal"
            >
              Acknowledge & Resolve
            </button>
            <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
              <button
                onClick={() =>
                  handleDecision(approval.id, false, undefined, approval.event_source)
                }
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Ask Agent to Adjust"
                data-testid="reject-proposal"
              >
                Ask Agent to Adjust
              </button>
            </div>
          </>
        ) : editingId === approval.id ? (
          <div className="flex flex-col gap-3 w-full">
            <textarea
              className="w-full min-h-[44px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
              rows={4}
              value={editContent}
              onChange={(e) => setEditContent(e.target.value)}
              data-testid="edit-proposal-textarea"
              autoFocus
            />
            <div className="flex gap-3">
              <button
                onClick={() => {
                  handleDecision(approval.id, true, editContent, approval.event_source);
                  setEditingId(null);
                }}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                data-testid="save-proposal"
              >
                Save & Approve
              </button>
              <button
                onClick={() => setEditingId(null)}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                data-testid="cancel-edit-proposal"
              >
                Cancel
              </button>
            </div>
          </div>
        ) : (
          <>
            <button
              onClick={() =>
                handleDecision(approval.id, true, undefined, approval.event_source)
              }
              className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-green-500 text-white font-medium hover:bg-green-600 transition-all duration-200 shadow-md flex items-center justify-center mb-3"
              aria-label="Approve proposal"
              data-testid="approve-proposal"
            >
              Approve
            </button>
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => {
                  setEditingId(approval.id);
                  const pLoad = approval.proposed_action || approval.context_payload;
                  if (pLoad?.feature_type === "quote_draft") {
                    setEditQuotePrice(pLoad?.proposed_price?.toString() || "");
                    setEditQuoteScope(pLoad?.proposed_scope || "");
                  } else {
                    const textToEdit =
                      pLoad?.generated_response ||
                      pLoad?.draft_reply ||
                      pLoad?.description ||
                      approval.proposed_action?.message ||
                      approval.proposed_action?.action_type ||
                      approval.event_source;
                    setEditContent(textToEdit || "");
                  }
                }}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Edit proposal"
                data-testid={
                  (approval.proposed_action || approval.context_payload)?.feature_type === "ambassador_reply"
                    ? "edit-ambassador-reply"
                    : "edit-proposal"
                }
              >
                Edit
              </button>
              <button
                onClick={() =>
                  handleDecision(approval.id, false, undefined, approval.event_source)
                }
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Reject proposal"
                data-testid={`triage-dismiss-${approval.id}`}
              >
                Deny
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
