import { ShiftReassignmentCard } from "../../app/dashboard/ShiftReassignmentCard";
import { InstagramDMCard } from "../../app/dashboard/InstagramDMCard";
import { AmbassadorReplyCard } from "../../app/dashboard/AmbassadorReplyCard";
import { ReviewFeedCard } from "../../app/dashboard/ReviewFeedCard";
import React from "react";

type AgentFeedItem = {
  id: string;
  tenant_id?: string;
  event_source: string;
  context_payload: any;
  payload?: any;
  proposed_action: any;
  lifecycle_state: string;
  created_at: string;
  updated_at?: string;
};

interface AgentActionCardProps {
  approval: AgentFeedItem;
  queuedActionIds: Set<string>;
  editingId: string | null;
  editContent: string;
  editQuotePrice: string;
  editQuoteScope: string;
  setEditingId: (id: string | null) => void;
  setEditContent: (content: string) => void;
  setEditQuotePrice: (price: string) => void;
  setEditQuoteScope: (scope: string) => void;
  handleDecision: (
    id: string,
    approved: boolean,
    editContent?: string,
    event_source?: string,
  ) => void | Promise<void>;
}

export const AgentActionCard: React.FC<AgentActionCardProps> = ({
  approval,
  queuedActionIds,
  editingId,
  editContent,
  editQuotePrice,
  editQuoteScope,
  setEditingId,
  setEditContent,
  setEditQuotePrice,
  setEditQuoteScope,
  handleDecision,
}) => {
  const [loadingAction, setLoadingAction] = React.useState<string | null>(null);
  const [isDraftExpanded, setIsDraftExpanded] = React.useState(false);

  const wrapDecision = async (
    id: string,
    approved: boolean,
    editContentValue?: string,
    event_source?: string,
    actionName?: string,
  ) => {
    try {
      setLoadingAction(actionName || (approved ? "approve" : "dismiss"));
      await handleDecision(id, approved, editContentValue, event_source);
    } catch (e) {
      console.error("Decision failed", e);
    } finally {
      // If the component is still mounted, remove loading state
      setLoadingAction(null);
    }
  };

  const isActionLoading = (actionName: string) => loadingAction === actionName;

  if (
    (approval.proposed_action || approval.context_payload)?.feature_type ===
    "shift_reassignment"
  ) {
    return (
      <ShiftReassignmentCard
        approval={approval}
        queuedActionIds={queuedActionIds}
        handleDecision={handleDecision}
      />
    );
  }

  return (
    <div
      key={approval.id}
      className={`glassmorphism app-list-item bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 shadow-sm flex flex-col gap-4 transition-all duration-300 overflow-hidden break-words whitespace-normal ${approval.event_source?.includes("marketing") ? "!border-t-[4px] !border-t-pink-500" : approval.event_source?.includes("operations") ? "!border-t-[4px] !border-t-blue-500" : approval.event_source?.includes("sales") || approval.event_source?.includes("triage") ? "!border-t-[4px] !border-t-green-500" : ""}`}
      data-testid={`triage-card-${approval.id}`}
    >
      <div className="flex flex-col gap-1">
        <div className="flex flex-wrap items-center gap-2">
          <span className="text-xs font-bold uppercase tracking-wider text-[#0066FF] bg-[#0066FF]/10 dark:bg-[#0066FF]/20 px-2 py-1 rounded-[8px]">
            Approval
          </span>
          <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-[8px]">
            {approval.event_source.replace("_", " ")}
          </span>
          {approval.lifecycle_state === "PENDING_APPROVAL" && (
            <span className="text-xs font-bold uppercase tracking-wider text-green-700 bg-green-100 px-2 py-1 rounded-[8px]">
              {approval.event_source === "customer_success_agent" ||
              approval.event_source === "instagram_dm" ||
              approval.context_payload?.feature_type === "ambassador_reply"
                ? "Action Required: Approve Reply"
                : "Action Needed"}
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
        <h3 className="text-lg font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1 tracking-wide break-words">
          {(approval.proposed_action || approval.context_payload)
            ?.feature_type === "invoice_draft"
            ? `Draft Invoice ready for ${(approval.proposed_action || approval.context_payload)?.milestone_name || "Phase 1"}`
            : (approval.proposed_action || approval.context_payload)
                  ?.feature_type === "ambassador_reply"
              ? "Action Required: Approve Reply"
              : (approval as any).description ||
                approval.context_payload?.description ||
                approval.proposed_action?.message ||
                approval.proposed_action?.description ||
                approval.proposed_action?.action_type ||
                approval.event_source}
        </h3>
        {((approval.proposed_action || approval.context_payload)?.context ||
          (approval.proposed_action || approval.context_payload)
            ?.remaining_stock !== undefined ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "quote_draft" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "create_product" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "social_post_draft" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "invoice_draft" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "invoice_followup" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "ambassador_reply" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "incident_resolution" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "booking_draft" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "booking_reengagement" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "instagram_dm" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "subscription_replenishment" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "subscription_churn_risk" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "invoice_followup") && (
          <div className="mt-2 flex flex-col gap-1 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-[8px]">
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "incident_resolution" && (
              <div
                className="mb-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/50 flex flex-col gap-3"
                data-testid="incident-resolution-card"
              >
                <div className="flex items-center gap-2 text-red-600 font-semibold text-sm">
                  <svg
                    className="w-5 h-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
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
                  {(approval.proposed_action || approval.context_payload)
                    ?.description ||
                    "An operational issue requires immediate attention."}
                </p>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "onboarding_welcome" && (
              <div
                className="mb-4 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3"
                data-testid="onboarding-welcome-card"
              >
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg
                    className="w-5 h-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M5 13l4 4L19 7"
                    />
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
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "instagram_dm" && (
              <InstagramDMCard
                approval={approval}
                onApprove={() =>
                  wrapDecision(
                    approval.id,
                    true,
                    undefined,
                    approval.event_source,
                  )
                }
                onDismiss={() =>
                  wrapDecision(
                    approval.id,
                    false,
                    undefined,
                    approval.event_source,
                  )
                }
              />
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "invoice_draft" && (
              <div className="flex flex-col gap-3">
                <div className="bg-green-50 dark:bg-green-900/20 p-3 rounded-lg border border-green-100 dark:border-green-800/50">
                  <p className="text-[13px] text-green-700 dark:text-green-300 font-medium mb-1">
                    Generated Invoice
                  </p>
                  <p className="text-[11px] text-green-600/70 dark:text-green-400/70">
                    Review the drafted invoice. Select "Approve & Send" to email
                    the client and generate a secure payment link.
                  </p>
                </div>
                <div className="space-y-3 mt-2">
                  <div className="p-3 bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 shadow-sm">
                    <p className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-1 flex items-center gap-1">
                      Project
                    </p>
                    <p className="text-[13px] font-medium text-gray-800 dark:text-gray-200">
                      {
                        (approval.proposed_action || approval.context_payload)
                          ?.project_name
                      }
                    </p>
                    <p className="text-[11px] text-gray-500 mt-0.5">
                      {
                        (approval.proposed_action || approval.context_payload)
                          ?.milestone_name
                      }
                    </p>
                  </div>
                  <div className="flex justify-between items-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg border border-gray-100 dark:border-gray-700">
                    <span className="text-[12px] font-medium text-gray-600 dark:text-gray-400">
                      Total Amount Due
                    </span>
                    <span className="text-[16px] font-bold text-gray-900 dark:text-white">
                      $
                      {(
                        (approval.proposed_action || approval.context_payload)
                          ?.amount_cents / 100
                      ).toFixed(2)}
                    </span>
                  </div>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "invoice_followup" && (
              <div className="flex flex-col gap-3">
                <div className="bg-amber-50 dark:bg-amber-900/20 p-3 rounded-lg border border-amber-100 dark:border-amber-800/50">
                  <p className="text-[13px] text-amber-700 dark:text-amber-300 font-medium mb-1">
                    {(approval.proposed_action || approval.context_payload)?.original_message}
                  </p>
                  <p className="text-[11px] text-amber-600/70 dark:text-amber-400/70">
                    Drafted a reminder for {(approval.proposed_action || approval.context_payload)?.suggested_channel || 'email'}.
                  </p>
                </div>
                <div className="p-3 bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 shadow-sm relative group">
                  <span className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-1 flex items-center gap-1">Drafted Message</span>
                  <p className="text-[13px] text-gray-800 dark:text-gray-200 mt-1">{(approval.proposed_action || approval.context_payload)?.generated_response}</p>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "ambassador_reply" && (
              <AmbassadorReplyCard
                approval={approval}
                isEditing={editingId === approval.id}
                editContent={editContent}
                setEditContent={setEditContent}
                onEdit={() => {
                  setEditingId(approval.id);
                  const textToEdit = approval.payload?.generated_response || (approval.proposed_action || approval.context_payload)?.generated_response || (approval.proposed_action || approval.context_payload)?.original_payload?.generated_response || approval.payload?.original_payload?.generated_response || "";
                  setEditContent(textToEdit);
                }}
                onCancelEdit={() => setEditingId(null)}
                onSaveEdit={() => {
                  handleDecision(approval.id, true, editContent, approval.event_source);
                  setEditingId(null);
                }}
                onApprove={() =>
                  wrapDecision(
                    approval.id,
                    true,
                    undefined,
                    approval.event_source,
                  )
                }
                onDismiss={() =>
                  wrapDecision(
                    approval.id,
                    false,
                    undefined,
                    approval.event_source,
                  )
                }
              />
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "review" && (
              <ReviewFeedCard
                review={approval.context_payload?.review}
                response={approval.proposed_action?.response}
                onApprove={async (id, content) => {
                  await wrapDecision(approval.id, true, content, "review");
                }}
                onDismiss={async (id) => {
                  await wrapDecision(approval.id, false, undefined, "review");
                }}
              />
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "order" && (
              <div className="mb-4 p-4 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800/50 flex flex-col gap-3">
                <div className="flex items-center gap-2 text-yellow-600 font-semibold text-sm">
                  <span className="w-5 h-5 flex items-center justify-center">
                    📦
                  </span>
                  Order Needs Fulfillment
                </div>
                <p className="text-sm text-gray-800 dark:text-gray-200">
                  {approval.context_payload?.description ||
                    "An order is waiting to be fulfilled."}
                </p>
                <div className="flex gap-2 w-full mt-1">
                  <button
                    type="button"
                    className="app-btn-primary flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 bg-[#0066FF] text-white rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, true, undefined, "order")
                    }
                    disabled={loadingAction !== null}
                  >
                    {isActionLoading("approve") ? (
                      <span className="animate-pulse">Loading...</span>
                    ) : (
                      "Fulfill Order"
                    )}
                  </button>
                  <button
                    type="button"
                    className="app-button flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 text-center bg-gray-100 dark:bg-gray-800 rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, false, undefined, "order")
                    }
                    disabled={loadingAction !== null}
                  >
                    {isActionLoading("dismiss") ? (
                      <span className="animate-pulse">Loading...</span>
                    ) : (
                      "Dismiss"
                    )}
                  </button>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "triage" && (
              <div className="mb-4 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800/50 flex flex-col gap-3">
                <div className="flex items-center gap-2 text-blue-600 font-semibold text-sm">
                  <span className="w-5 h-5 flex items-center justify-center">
                    ✉️
                  </span>
                  Message Requires Attention
                </div>
                <p className="text-sm text-gray-800 dark:text-gray-200">
                  {approval.context_payload?.description ||
                    "You have an open customer conversation waiting for your reply."}
                </p>
                <div className="flex gap-2 w-full mt-1">
                  <button
                    type="button"
                    className="app-btn-primary flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 bg-[#0066FF] text-white rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, true, undefined, "triage")
                    }
                    disabled={loadingAction !== null}
                  >
                    {isActionLoading("approve") ? (
                      <span className="animate-pulse">Loading...</span>
                    ) : (
                      "Resolve Message"
                    )}
                  </button>
                  <button
                    type="button"
                    className="app-button flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 text-center bg-gray-100 dark:bg-gray-800 rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, false, undefined, "triage")
                    }
                    disabled={loadingAction !== null}
                  >
                    {isActionLoading("dismiss") ? (
                      <span className="animate-pulse">Loading...</span>
                    ) : (
                      "Dismiss"
                    )}
                  </button>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "proactive_ops" && (
              <div className="mb-4 p-4 bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800/50 flex flex-col gap-3 relative overflow-hidden">
                <div className="absolute top-0 left-0 w-1 h-full bg-[#FF9500]"></div>
                <div className="flex items-center gap-2 text-orange-600 font-semibold text-sm">
                  <span className="w-5 h-5 flex items-center justify-center">
                    ✨
                  </span>
                  Needs Attention Today
                </div>
                <p className="text-sm text-gray-800 dark:text-gray-200 font-medium">
                  {approval.context_payload?.description ||
                    "A proactive ops task needs your attention."}
                </p>
                <div className="flex gap-2 w-full mt-1">
                  <button
                    type="button"
                    className="app-btn-primary flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 bg-[#FF9500] text-white rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, true, undefined, "operations")
                    }
                  >
                    {approval.proposed_action?.message || "Approve"}
                  </button>
                  <button
                    type="button"
                    className="app-button flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 text-center border border-orange-200 text-orange-900 dark:text-orange-100 rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, false, undefined, "operations")
                    }
                    disabled={loadingAction !== null}
                  >
                    {isActionLoading("dismiss") ? (
                      <span className="animate-pulse">Loading...</span>
                    ) : (
                      "Dismiss"
                    )}
                  </button>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "task" && (
              <div className="mb-4 p-4 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800/50 flex flex-col gap-3">
                <div className="flex items-center gap-2 text-purple-600 font-semibold text-sm">
                  <span className="w-5 h-5 flex items-center justify-center">
                    ✅
                  </span>
                  Pending Task
                </div>
                <p className="text-sm text-gray-800 dark:text-gray-200">
                  {approval.context_payload?.description ||
                    "You have a pending task."}
                </p>
                <div className="flex gap-2 w-full mt-1">
                  <button
                    type="button"
                    className="app-btn-primary flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 bg-[#0066FF] text-white rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, true, undefined, "task")
                    }
                    disabled={loadingAction !== null}
                  >
                    {isActionLoading("approve") ? (
                      <span className="animate-pulse">Loading...</span>
                    ) : (
                      "Complete Task"
                    )}
                  </button>
                  <button
                    type="button"
                    className="app-button flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 text-center bg-gray-100 dark:bg-gray-800 rounded-[8px]"
                    onClick={() =>
                      wrapDecision(approval.id, false, undefined, "task")
                    }
                    disabled={loadingAction !== null}
                  >
                    {isActionLoading("dismiss") ? (
                      <span className="animate-pulse">Loading...</span>
                    ) : (
                      "Dismiss"
                    )}
                  </button>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "autonomous_booking_quote" && (
              <div
                className="mb-4 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3"
                data-testid="autonomous-booking-quote-card"
              >
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg
                    className="w-5 h-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                  Draft quote and propose schedule for{" "}
                  {(approval.proposed_action || approval.context_payload)
                    .service || "Emergency Handyman Service"}
                </div>

                {editingId === approval.id ? (
                  <div
                    role="dialog"
                    className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50"
                  >
                    <div className="bg-white dark:bg-gray-900 p-6 max-w-sm w-full shadow-2xl border border-gray-200 dark:border-gray-800 flex flex-col gap-4">
                      <h3 className="text-lg font-bold text-gray-900 dark:text-gray-100">
                        Review Booking Quote
                      </h3>

                      <div className="flex flex-col gap-2">
                        <div className="text-sm text-gray-500 font-semibold">
                          Proposed Schedule
                        </div>
                        <div className="flex flex-wrap gap-2">
                          {(
                            (
                              approval.proposed_action ||
                              approval.context_payload
                            ).proposed_slots || []
                          ).map((slot: any, idx: number) => {
                            const timeStr = slot.start_time
                              ? new Date(slot.start_time).toLocaleTimeString(
                                  [],
                                  {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                    hour12: false,
                                  },
                                )
                              : "14:00";
                            return (
                              <button
                                key={idx}
                                className="px-3 py-1.5 rounded-lg border border-[#0066FF] text-[#0066FF] bg-[#0066FF]/10 text-sm font-medium"
                              >
                                {timeStr}
                              </button>
                            );
                          })}
                          {!(
                            approval.proposed_action || approval.context_payload
                          ).proposed_slots?.length && (
                            <button className="px-3 py-1.5 rounded-lg border border-[#0066FF] text-[#0066FF] bg-[#0066FF]/10 text-sm font-medium">
                              14:00
                            </button>
                          )}
                        </div>
                      </div>

                      <div className="flex justify-between items-center bg-gray-50 dark:bg-gray-800 p-3 rounded-lg mt-2">
                        <span className="text-sm text-gray-500">
                          Total Price
                        </span>
                        <span
                          className="font-bold text-lg text-gray-900 dark:text-gray-100"
                          data-testid="modal-quote-total"
                        >
                          $
                          {Number(
                            (
                              approval.proposed_action ||
                              approval.context_payload
                            ).suggested_price || 180,
                          ).toFixed(2)}
                        </span>
                      </div>

                      <div className="flex gap-3 mt-4">
                        <button
                          onClick={() => {
                            setEditingId(null);
                          }}
                          className="flex-1 py-3 px-4 rounded-[8px] border border-gray-300 dark:border-gray-700 font-medium text-gray-700 dark:text-gray-300"
                        >
                          Cancel
                        </button>
                        <button
                          data-testid="modal-approve-btn"
                          onClick={() => {
                            wrapDecision(
                              approval.id,
                              true,
                              undefined,
                              "operations",
                            );
                            setEditingId(null);
                          }}
                          className="flex-1 py-3 px-4 rounded-[8px] bg-[#0066FF] hover:bg-[#0052CC] font-medium text-white shadow-md"
                          disabled={loadingAction !== null}
                        >
                          {isActionLoading("approve") ? (
                            <span className="animate-pulse">Loading...</span>
                          ) : (
                            "Approve & Send"
                          )}
                        </button>
                      </div>
                    </div>
                  </div>
                ) : (
                  <div className="flex gap-2 w-full mt-2">
                    <button
                      type="button"
                      className="app-btn-primary flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 bg-[#0066FF] text-white rounded-[8px]"
                      onClick={() => setEditingId(approval.id)}
                    >
                      Review
                    </button>
                    <button
                      type="button"
                      className="app-button flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden py-2 text-center bg-gray-100 dark:bg-gray-800 rounded-[8px]"
                      onClick={() =>
                        wrapDecision(
                          approval.id,
                          false,
                          undefined,
                          "operations",
                        )
                      }
                      disabled={loadingAction !== null}
                    >
                      {isActionLoading("dismiss") ? (
                        <span className="animate-pulse">Loading...</span>
                      ) : (
                        "Dismiss"
                      )}
                    </button>
                  </div>
                )}
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "create_product" && (
              <div
                className="mb-4 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3"
                data-testid="create-product-card"
              >
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg
                    className="w-5 h-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                    />
                  </svg>
                  <span>Proposed Product</span>
                </div>
                <div className="flex flex-col gap-1">
                  <span className="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wider">
                    Product Name
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    {
                      (approval.proposed_action || approval.context_payload)
                        ?.product_name || "New Product"
                    }
                  </span>
                </div>
                <div className="flex flex-col gap-1">
                  <span className="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wider">
                    Description
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    {
                      (approval.proposed_action || approval.context_payload)
                        ?.description
                    }
                  </span>
                </div>
                <div className="flex flex-col gap-1">
                  <span className="text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wider">
                    Suggested Price
                  </span>
                  <span className="font-semibold text-green-600 dark:text-green-400">
                    ${(approval.proposed_action || approval.context_payload)
                      ?.suggested_price || 0}
                  </span>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "quote_draft" && (
              <div
                className="mb-4 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3"
                data-testid="quote-draft-card"
              >
                <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                  <svg
                    className="w-5 h-5"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                  Draft Quote:{" "}
                  {(approval.proposed_action || approval.context_payload)
                    .service || "Plumbing Fix"}{" "}
                  for Customer
                </div>
                <div className="text-xs text-[#0066FF] dark:text-blue-400 font-medium break-words">
                  {
                    (approval.proposed_action || approval.context_payload)
                      .customer_inquiry
                  }
                </div>
                <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-3 rounded-[8px] relative mt-2">
                  <div className="text-[10px] uppercase font-bold text-gray-500 mb-2">
                    AI Proposed Quote
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-xs text-gray-500">
                        Calculated Total:
                      </span>
                      <span className="text-xs font-semibold text-gray-900 dark:text-gray-100">
                        $
                        {
                          (approval.proposed_action || approval.context_payload)
                            .suggested_price
                        }
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-xs text-gray-500">
                        Scope of Work:
                      </span>
                      <span className="text-xs font-medium text-gray-800 dark:text-gray-200">
                        {
                          (approval.proposed_action || approval.context_payload)
                            .scope
                        }
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-xs text-gray-500">
                        Suggested Time:
                      </span>
                      <span className="text-xs font-medium text-gray-800 dark:text-gray-200">
                        {
                          (approval.proposed_action || approval.context_payload)
                            .suggested_time
                        }
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            )}
            {(approval.proposed_action || approval.context_payload)
              ?.feature_type === "newsletter_draft" ? (
              <div className="flex flex-col gap-3">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400 font-semibold">
                    Weekly Newsletter Draft Ready!
                  </span>
                  <span className="text-indigo-500 font-bold text-xs">
                    Review and send
                  </span>
                </div>

                <div className="bg-white/50 dark:bg-black/20 p-3 rounded-lg border border-gray-100 dark:border-gray-800">
                  <div className="flex items-center gap-2 mb-2">
                    <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                      Subject
                    </span>
                  </div>
                  <p className="text-sm font-medium text-gray-900 dark:text-gray-100 leading-snug">
                    {
                      (approval.proposed_action || approval.context_payload)
                        .subject
                    }
                  </p>
                </div>
                <div className="bg-white/50 dark:bg-black/20 p-3 rounded-lg border border-gray-100 dark:border-gray-800">
                  <div className="flex items-center gap-2 mb-2">
                    <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">
                      Preview
                    </span>
                  </div>
                  <p className="text-sm text-gray-700 dark:text-gray-300 italic line-clamp-3 leading-snug">
                    "
                    {
                      (approval.proposed_action || approval.context_payload)
                        .content_preview
                    }
                    "
                  </p>
                </div>
              </div>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "social_post_draft" ? (
              <div className="flex flex-col gap-3">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400 font-semibold">
                    New product detected!
                  </span>
                  <span className="text-pink-500 font-bold text-xs">
                    Schedule a post?
                  </span>
                </div>
                <div className="app-card dark:bg-gray-800 p-3 rounded-[8px] border border-pink-100 dark:border-pink-900/50">
                  <div className="text-[10px] uppercase font-bold text-gray-400 mb-2 flex items-center gap-1">
                    <span className="w-2 h-2 rounded-full bg-pink-500"></span>{" "}
                    Instagram / TikTok Draft
                  </div>
                  <div className="text-xs text-gray-700 dark:text-gray-300 italic line-clamp-3">
                    "
                    {(approval.proposed_action || approval.context_payload)
                      .instagram ||
                      (approval.proposed_action || approval.context_payload)
                        .tiktok ||
                      "Check out our new product!"}
                    "
                  </div>
                </div>
              </div>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "subscription_churn_risk" ? (
              <div className="flex flex-col gap-3">
                <div className="flex items-center gap-2 text-red-600 font-semibold text-sm">
                  ⚠️ High Churn Risk
                </div>
                <div className="text-sm font-medium text-gray-800 dark:text-gray-200 border-l-2 border-red-400 pl-2">
                  {approval.context_payload?.reason ||
                    approval.proposed_action?.reason ||
                    "Health score dropped due to inactivity."}
                </div>
                <div className="text-sm text-gray-600 dark:text-gray-400 italic">
                  "
                  {approval.proposed_action?.generated_response ||
                    approval.proposed_action?.message}
                  "
                </div>
              </div>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "subscription_replenishment" ? (
              <div className="flex flex-col gap-2">
                <div className="text-sm font-medium text-gray-800 dark:text-gray-200">
                  Autopilot Recommendation
                </div>
                <div className="text-sm text-gray-600 dark:text-gray-400">
                  {approval.proposed_action?.context ||
                    "Based on this customer's order history and the estimated consumption rate, they are due for a replenishment. Would you like me to generate a personalized checkout link and draft an email suggesting they refill?"}
                </div>
              </div>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "supply_order" ? (
              <>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Current Stock:
                  </span>
                  <span
                    className="font-semibold text-gray-800 dark:text-gray-200"
                    data-testid="supply-order-stock"
                  >
                    {
                      (approval.proposed_action || approval.context_payload)
                        .remaining_stock
                    }{" "}
                    units
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Est. Runout:
                  </span>
                  <span className="font-semibold text-gray-800 dark:text-gray-200">
                    {
                      (approval.proposed_action || approval.context_payload)
                        .est_runout_days
                    }{" "}
                    days
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Reorder Quantity:
                  </span>
                  <span
                    className="font-bold text-blue-600 dark:text-blue-400 text-base"
                    data-testid="supply-order-quantity"
                  >
                    {
                      (approval.proposed_action || approval.context_payload)
                        .suggested_reorder_quantity
                    }{" "}
                    Units
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm mb-3">
                  <span className="text-gray-500 dark:text-gray-400">
                    Vendor:
                  </span>
                  <span className="font-semibold text-gray-800 dark:text-gray-200">
                    {
                      (approval.proposed_action || approval.context_payload)
                        .vendor_name
                    }{" "}
                    (
                    {
                      (approval.proposed_action || approval.context_payload)
                        .vendor_contact
                    }
                    )
                  </span>
                </div>
                <div className="bg-gray-50 dark:bg-gray-800 p-3 rounded-[8px] border border-gray-200 dark:border-gray-700">
                  <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">
                    Drafted Message:
                  </div>
                  <div className="text-sm text-gray-800 dark:text-gray-200 italic font-medium">
                    "
                    {
                      (approval.proposed_action || approval.context_payload)
                        .draft_message
                    }
                    "
                  </div>
                </div>
              </>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "stockout_restock_and_price" ? (
              <>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Current Price:
                  </span>
                  <span className="font-semibold text-gray-400 dark:text-gray-500 line-through">
                    $
                    {Number(
                      (approval.proposed_action || approval.context_payload)
                        .old_price,
                    ).toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Suggested Price:
                  </span>
                  <span
                    className="font-bold text-green-600 dark:text-green-400 text-base"
                    data-testid="stockout-new-price"
                  >
                    $
                    {Number(
                      (approval.proposed_action || approval.context_payload)
                        .new_price,
                    ).toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Reorder Quantity:
                  </span>
                  <span
                    className="font-bold text-blue-600 dark:text-blue-400 text-base"
                    data-testid="stockout-reorder"
                  >
                    {
                      (approval.proposed_action || approval.context_payload)
                        .suggested_reorder_quantity
                    }{" "}
                    Units
                  </span>
                </div>
                <div className="text-sm font-medium text-gray-800 dark:text-gray-200 mt-2">
                  {
                    (approval.proposed_action || approval.context_payload)
                      .message
                  }
                </div>
              </>
            ) : approval.proposed_action?.action_type ===
                "Daily Prep Checklist" ||
              approval.context_payload?.feature_type ===
                "daily_prep_checklist" ? (
              <div className="flex flex-col sm:flex-row gap-3 w-full">
                <button
                  onClick={() =>
                    handleDecision(
                      approval.id,
                      true,
                      undefined,
                      approval.event_source,
                    )
                  }
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-indigo-600 text-white font-medium hover:bg-indigo-700 transition-all duration-200 shadow-md flex items-center justify-center"
                  aria-label="Mark Complete"
                  data-testid="feed-approve-btn"
                >
                  Mark Complete
                </button>
                <button
                  onClick={() =>
                    handleDecision(
                      approval.id,
                      true,
                      "Assign to Staff",
                      approval.event_source,
                    )
                  }
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                  aria-label="Assign to Staff"
                  data-testid="feed-assign-btn"
                >
                  Assign to Staff
                </button>
                <button
                  onClick={() =>
                    handleDecision(
                      approval.id,
                      false,
                      undefined,
                      approval.event_source,
                    )
                  }
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                  aria-label="Dismiss task"
                  data-testid="feed-dismiss-btn"
                  disabled={loadingAction !== null}
                >
                  {isActionLoading("dismiss") ? (
                    <span className="animate-pulse">Loading...</span>
                  ) : (
                    "Dismiss"
                  )}
                </button>
              </div>
            ) : (approval.proposed_action || approval.context_payload)?.context
                ?.smart_pricing === true ? (
              <>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Current Price:
                  </span>
                  <span className="font-semibold text-gray-400 dark:text-gray-500 line-through">
                    $
                    {Number(
                      (approval.proposed_action || approval.context_payload)
                        .context.old_price,
                    ).toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm mb-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Suggested Price:
                  </span>
                  <span
                    className="font-bold text-green-600 dark:text-green-400 text-base"
                    data-testid="smart-pricing-new-price"
                  >
                    $
                    {Number(
                      (approval.proposed_action || approval.context_payload)
                        .context.new_price,
                    ).toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Sales Projection:
                  </span>
                  <span
                    className="font-semibold text-indigo-600 dark:text-indigo-400"
                    data-testid="smart-pricing-sales-projection"
                  >
                    {
                      (approval.proposed_action || approval.context_payload)
                        .context.sales_projection
                    }
                  </span>
                </div>
              </>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "incident_resolution" ? (
              <div className="flex flex-col gap-2">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Proposed Actions:
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    {
                      (
                        (approval.proposed_action || approval.context_payload)
                          .actions || []
                      ).length
                    }{" "}
                    steps
                  </span>
                </div>
                <div className="w-full h-px bg-gray-200 dark:bg-gray-700 my-1"></div>
                {(
                  (approval.proposed_action || approval.context_payload)
                    .actions || []
                ).map((action: any, idx: number) => (
                  <div key={idx} className="flex flex-col mb-2">
                    <span className="text-xs font-semibold text-gray-700 dark:text-gray-300">
                      {action.action}
                    </span>
                    <span className="text-xs text-gray-500">
                      {action.details}
                    </span>
                  </div>
                ))}
              </div>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "ambassador_reply" ? (
              <div className="flex flex-col gap-2">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Context:
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    {(approval.proposed_action || approval.context_payload)
                      .source || "Message"}
                  </span>
                </div>
                <div className="flex flex-col text-sm mt-1">
                  <span className="text-gray-500 dark:text-gray-400">
                    Draft:
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100 line-clamp-2 mt-1">
                    {
                      (approval.proposed_action || approval.context_payload)
                        .generated_response
                    }
                  </span>
                </div>
              </div>
            ) : (approval.proposed_action || approval.context_payload)
                ?.feature_type === "quote_draft" ? (
              <div className="flex flex-col gap-2">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Context:
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    {(approval.proposed_action || approval.context_payload)
                      .customer_inquiry || "Client Inquiry"}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Scope:
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    {(approval.proposed_action || approval.context_payload)
                      .scope ||
                      (approval.proposed_action || approval.context_payload)
                        .service}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Timeline:
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    {(approval.proposed_action || approval.context_payload)
                      .suggested_time || "TBD"}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Price:
                  </span>
                  <span className="font-semibold text-green-600 dark:text-green-400">
                    $
                    {Number(
                      (approval.proposed_action || approval.context_payload)
                        .suggested_price ||
                        (approval.proposed_action || approval.context_payload)
                          .price ||
                        0,
                    ).toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-gray-500 dark:text-gray-400">
                    Required Deposit:
                  </span>
                  <span className="font-semibold text-gray-900 dark:text-gray-100">
                    $
                    {Number(
                      (approval.proposed_action || approval.context_payload)
                        .suggested_price ||
                        (approval.proposed_action || approval.context_payload)
                          .price ||
                        0,
                    ) * 0.2}
                  </span>
                </div>
                {((approval.proposed_action || approval.context_payload)
                  .proposed_slot_id ||
                  (approval.proposed_action || approval.context_payload)
                    .suggested_time) && (
                  <div className="flex justify-between items-center text-sm mt-1 pt-1 border-t border-gray-200 dark:border-gray-700">
                    <span className="text-gray-500 dark:text-gray-400">
                      Provisional Slot Held:
                    </span>
                    <span className="font-semibold text-[#0066FF] dark:text-blue-400">
                      Yes (
                      {(approval.proposed_action || approval.context_payload)
                        .suggested_time || "Pending"}
                      )
                    </span>
                  </div>
                )}
              </div>
            ) : (
              <>
                {(approval.proposed_action || approval.context_payload)?.context
                  ?.weekly_health_report === true ? (
                  <div className="flex flex-col gap-2">
                    <div className="text-sm text-gray-700 dark:text-gray-300">
                      <span className="font-semibold">Summary:</span>{" "}
                      {
                        (approval.proposed_action || approval.context_payload)
                          .context.summary
                      }
                    </div>
                    <div className="text-sm text-indigo-600 dark:text-indigo-400 font-medium">
                      <span className="font-semibold text-gray-700 dark:text-gray-300">
                        Suggestion:
                      </span>{" "}
                      {
                        (approval.proposed_action || approval.context_payload)
                          .context.actionable_suggestion
                      }
                    </div>
                  </div>
                ) : (
                  <>
                    {(approval.proposed_action || approval.context_payload)
                      ?.context?.abandoned_carts_count !== undefined && (
                      <div className="flex justify-between items-center text-sm">
                        <span className="text-gray-500 dark:text-gray-400">
                          Abandoned Carts:
                        </span>
                        <span className="font-semibold text-gray-900 dark:text-gray-100">
                          {
                            (
                              approval.proposed_action ||
                              approval.context_payload
                            ).context.abandoned_carts_count
                          }
                        </span>
                      </div>
                    )}
                    {(approval.proposed_action || approval.context_payload)
                      ?.context?.potential_revenue !== undefined && (
                      <div className="flex justify-between items-center text-sm">
                        <span className="text-gray-500 dark:text-gray-400">
                          Potential Revenue:
                        </span>
                        <span className="font-semibold text-green-600 dark:text-green-400">
                          $
                          {Number(
                            (
                              approval.proposed_action ||
                              approval.context_payload
                            ).context.potential_revenue,
                          ).toFixed(2)}
                        </span>
                      </div>
                    )}
                    {(approval.proposed_action || approval.context_payload)
                      ?.remaining_stock !== undefined && (
                      <div className="flex flex-col gap-2">
                        <div className="flex justify-between items-center text-sm">
                          <span className="text-gray-500 dark:text-gray-400">
                            Product ID:
                          </span>
                          <span className="font-semibold text-gray-900 dark:text-gray-100">
                            {
                              (
                                approval.proposed_action ||
                                approval.context_payload
                              ).product_id
                            }
                          </span>
                        </div>
                        <div className="flex justify-between items-center text-sm">
                          <span className="text-gray-500 dark:text-gray-400">
                            Remaining Stock:
                          </span>
                          <span className="font-semibold text-red-600 dark:text-red-400">
                            {
                              (
                                approval.proposed_action ||
                                approval.context_payload
                              ).remaining_stock
                            }
                          </span>
                        </div>
                        <div className="flex justify-between items-center text-sm">
                          <span className="text-gray-500 dark:text-gray-400">
                            Alert Message:
                          </span>
                          <span className="font-semibold text-gray-900 dark:text-gray-100">
                            {
                              (
                                approval.proposed_action ||
                                approval.context_payload
                              ).message
                            }
                          </span>
                        </div>
                      </div>
                    )}
                  </>
                )}
              </>
            )}
          </div>
        )}
      </div>

      <div className="flex flex-col gap-3 w-full mt-2">
        {(approval.proposed_action || approval.context_payload)
          ?.feature_type === "onboarding_welcome" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <a
              href="/storefront-builder"
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Review Storefront"
              data-testid="review-storefront-btn"
            >
              Review Storefront
            </a>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss"
              data-testid="dismiss-onboarding-welcome"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "booking_reengagement" ? (
          editingId === approval.id ? (
            <div className="flex flex-col gap-3 w-full">
              <textarea
                className="w-full min-h-[44px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
                rows={4}
                value={editContent}
                onChange={(e) => setEditContent(e.target.value)}
                data-testid="edit-booking-reengagement-textarea"
                autoFocus
              />
              <div className="flex gap-3">
                <button
                  onClick={() => {
                    handleDecision(
                      approval.id,
                      true,
                      editContent,
                      approval.event_source,
                    );
                    setEditingId(null);
                  }}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                  data-testid="save-booking-reengagement"
                  disabled={loadingAction !== null}
                >
                  {isActionLoading("approve") ? (
                    <span className="animate-pulse">Loading...</span>
                  ) : (
                    "Save & Approve"
                  )}
                </button>
                <button
                  onClick={() => setEditingId(null)}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                  data-testid="cancel-edit-booking-reengagement"
                >
                  Cancel
                </button>
              </div>
            </div>
          ) : (
            <>
              <div className="p-3 bg-white/50 dark:bg-gray-800/50 rounded-lg mb-3 border border-gray-200 dark:border-gray-700 backdrop-blur-[10px]">
                <p className="text-sm font-medium text-gray-900 dark:text-white mb-1">
                  Drafted Re-engagement Message
                </p>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  {approval.proposed_action?.draft_action ||
                    approval.proposed_action?.message}
                </p>
              </div>
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    true,
                    undefined,
                    approval.event_source,
                  )
                }
                className="w-full min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-500 text-white font-medium hover:bg-green-600 transition-all duration-200 shadow-md flex items-center justify-center mb-3"
                aria-label={`Approve Re-engagement for ${approval.context_payload?.customer_name || "Customer"}`}
                data-testid="approve-booking-reengagement"
                disabled={loadingAction !== null}
              >
                {isActionLoading("approve") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  `Approve Re-engagement for ${approval.context_payload?.customer_name || "Customer"}`
                )}
              </button>
              <div className="flex flex-col sm:flex-row gap-3 w-full">
                <button
                  onClick={() => {
                    setEditingId(approval.id);
                    setEditContent(
                      approval.proposed_action?.draft_action ||
                        approval.proposed_action?.message ||
                        "",
                    );
                  }}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                  aria-label="Edit booking re-engagement"
                  data-testid="edit-booking-reengagement"
                >
                  Edit
                </button>
                <button
                  onClick={() =>
                    handleDecision(
                      approval.id,
                      false,
                      undefined,
                      approval.event_source,
                    )
                  }
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                  aria-label="Reject booking re-engagement"
                  data-testid="reject-booking-reengagement"
                  disabled={loadingAction !== null}
                >
                  {isActionLoading("dismiss") ? (
                    <span className="animate-pulse">Loading...</span>
                  ) : (
                    "Deny"
                  )}
                </button>
              </div>
            </>
          )
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "booking_draft" ? (
          editingId === approval.id ? (
            <div className="flex flex-col gap-3 w-full">
              <textarea
                className="w-full min-h-[44px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
                rows={4}
                value={editContent}
                onChange={(e) => setEditContent(e.target.value)}
                data-testid="edit-booking-draft-textarea"
                autoFocus
              />
              <div className="flex gap-3">
                <button
                  onClick={() => {
                    handleDecision(
                      approval.id,
                      true,
                      editContent,
                      approval.event_source,
                    );
                    setEditingId(null);
                  }}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                  data-testid="save-booking-draft"
                  disabled={loadingAction !== null}
                >
                  {isActionLoading("approve") ? (
                    <span className="animate-pulse">Loading...</span>
                  ) : (
                    "Save & Approve"
                  )}
                </button>
                <button
                  onClick={() => setEditingId(null)}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                  data-testid="cancel-edit-booking-draft"
                >
                  Cancel
                </button>
              </div>
            </div>
          ) : (
            <>
              <div className="p-3 bg-white/50 dark:bg-gray-800/50 rounded-lg mb-3 border border-gray-200 dark:border-gray-700 backdrop-blur-[10px]">
                <p className="text-sm font-medium text-gray-900 dark:text-white mb-1">
                  Audio Summary
                </p>
                <div className="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 bg-white dark:bg-gray-800 p-2 rounded border border-gray-200 dark:border-gray-700">
                  <span className="text-lg">▶️</span>
                  <span>
                    0:10 AI Summary (
                    {
                      (approval.proposed_action || approval.context_payload)
                        ?.caller_phone
                    }
                    )
                  </span>
                </div>
              </div>
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    true,
                    undefined,
                    approval.event_source,
                  )
                }
                className="w-full min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3"
                aria-label="Approve Route & Send Confirmation"
                data-testid="approve-booking-draft"
              >
                Approve Route & Send Confirmation
              </button>
              <div className="flex flex-col sm:flex-row gap-3 w-full">
                <button
                  onClick={() => {
                    setEditingId(approval.id);
                    setEditContent(
                      (approval.proposed_action || approval.context_payload)
                        ?.summary || "",
                    );
                  }}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                  aria-label="Edit booking draft"
                  data-testid="edit-booking-draft"
                >
                  Edit
                </button>
                <button
                  onClick={() =>
                    handleDecision(
                      approval.id,
                      false,
                      undefined,
                      approval.event_source,
                    )
                  }
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                  aria-label="Reject booking draft"
                  data-testid="reject-booking-draft"
                  disabled={loadingAction !== null}
                >
                  {isActionLoading("dismiss") ? (
                    <span className="animate-pulse">Loading...</span>
                  ) : (
                    "Deny"
                  )}
                </button>
              </div>
            </>
          )
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "incident_resolution" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-red-600 text-white font-medium hover:bg-red-700 transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Execute Plan"
              data-testid="approve-incident-resolution"
            >
              Execute Plan
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss Plan"
              data-testid="dismiss-incident-resolution"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "subscription_churn_risk" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full sm:flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-red-600 text-white font-bold hover:bg-red-700 transition-all duration-200 shadow-md flex items-center justify-center transform active:scale-95"
              aria-label="Approve Win-Back Offer"
              data-testid={`action-card-approve-${approval.id}`}
            >
              Approve & Send Offer
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full sm:w-auto min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-6 rounded-[8px] glassmorphism bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 font-semibold hover:bg-gray-200 dark:hover:bg-gray-700 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss"
              data-testid={`action-card-dismiss-${approval.id}`}
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "subscription_replenishment" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full sm:flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-blue-600 transition-all duration-200 shadow-sm flex items-center justify-center"
              aria-label="Generate & Send Email"
              data-testid="approve-subscription-replenishment"
            >
              Generate & Send Email
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full sm:w-auto min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-6 rounded-[8px] bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 font-medium hover:bg-gray-200 dark:hover:bg-gray-700 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "subscription_replenishment" ? (
          <div className="flex flex-col gap-2">
            <div className="flex justify-between items-center">
              <span className="text-gray-500 dark:text-gray-400">Action:</span>
              <span className="font-medium">Send Check-in Email</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-gray-500 dark:text-gray-400">Offer:</span>
              <span className="font-medium">1-Click Repurchase Link</span>
            </div>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "supply_order" ? (
          <>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-500 text-white font-medium hover:bg-green-600 transition-all duration-200 shadow-md flex items-center justify-center mb-3"
              aria-label="Approve & Send"
              data-testid="approve-supply-order"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve & Send"
              )}
            </button>
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => {
                  setEditingId(approval.id);
                  setEditContent(
                    (approval.proposed_action || approval.context_payload)
                      .draft_message,
                  );
                }}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Edit message"
                data-testid="edit-supply-order"
              >
                Edit
              </button>
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    false,
                    undefined,
                    approval.event_source,
                  )
                }
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Deny supply order"
                data-testid="reject-supply-order"
                disabled={loadingAction !== null}
              >
                {isActionLoading("dismiss") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  "Deny"
                )}
              </button>
            </div>
          </>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "newsletter_draft" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-indigo-600 text-white font-medium hover:bg-indigo-700 transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve & Send"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve & Send"
              )}
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Skip this week"
            >
              Skip this week
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "social_post_draft" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-gradient-to-r from-pink-500 to-indigo-500 text-white font-medium hover:from-pink-600 hover:to-indigo-600 transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Schedule Posts"
              data-testid="approve-social-post"
            >
              Schedule Posts
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss proposal"
              data-testid="dismiss-social-post"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "supply_order" ? (
          <>
            <div className="flex justify-between items-center text-sm mb-1">
              <span className="text-gray-500 dark:text-gray-400">
                Current Stock:
              </span>
              <span
                className="font-semibold text-gray-800 dark:text-gray-200"
                data-testid="supply-order-stock"
              >
                {
                  (approval.proposed_action || approval.context_payload)
                    .remaining_stock
                }{" "}
                units
              </span>
            </div>
            <div className="flex justify-between items-center text-sm mb-1">
              <span className="text-gray-500 dark:text-gray-400">
                Est. Runout:
              </span>
              <span className="font-semibold text-gray-800 dark:text-gray-200">
                {
                  (approval.proposed_action || approval.context_payload)
                    .est_runout_days
                }{" "}
                days
              </span>
            </div>
            <div className="flex justify-between items-center text-sm mb-1">
              <span className="text-gray-500 dark:text-gray-400">
                Reorder Quantity:
              </span>
              <span
                className="font-bold text-blue-600 dark:text-blue-400 text-base"
                data-testid="supply-order-quantity"
              >
                {
                  (approval.proposed_action || approval.context_payload)
                    .suggested_reorder_quantity
                }{" "}
                Units
              </span>
            </div>
            <div className="flex justify-between items-center text-sm mb-3">
              <span className="text-gray-500 dark:text-gray-400">Vendor:</span>
              <span className="font-semibold text-gray-800 dark:text-gray-200">
                {
                  (approval.proposed_action || approval.context_payload)
                    .vendor_name
                }{" "}
                (
                {
                  (approval.proposed_action || approval.context_payload)
                    .vendor_contact
                }
                )
              </span>
            </div>
            <div className="bg-gray-50 dark:bg-gray-800 p-3 rounded-[8px] border border-gray-200 dark:border-gray-700">
              <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">
                Drafted Message:
              </div>
              <div className="text-sm text-gray-800 dark:text-gray-200 italic font-medium">
                "
                {
                  (approval.proposed_action || approval.context_payload)
                    .draft_message
                }
                "
              </div>
            </div>
          </>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "stockout_restock_and_price" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve"
              data-testid="approve-stockout"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve"
              )}
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss"
              data-testid="dismiss-stockout"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "invoice_draft" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              disabled={isActionLoading(approval.id)}
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center opacity-100"
              data-testid="feed-approve-btn"
            >
              {isActionLoading(approval.id) ? (
                <span className="flex items-center gap-2">
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                  Processing...
                </span>
              ) : (
                "Approve & Send"
              )}
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              disabled={isActionLoading(approval.id)}
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
              data-testid="feed-dismiss-btn"
            >
              Dismiss
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "review" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "order" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "triage" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "task" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "proactive_ops" ||
          (approval.proposed_action || approval.context_payload)
            ?.feature_type === "invoice_followup" ? null : (
            approval.proposed_action || approval.context_payload
          )?.feature_type === "ambassador_reply" ? (
          editingId === approval.id ? (
            <div className="flex flex-col gap-3 w-full">
              <textarea
                className="w-full min-h-[44px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
                rows={4}
                value={editContent}
                onChange={(e) => setEditContent(e.target.value)}
                data-testid="edit-ambassador-reply-textarea"
                autoFocus
              />
              <div className="flex gap-3">
                <button
                  onClick={() => {
                    handleDecision(
                      approval.id,
                      true,
                      editContent,
                      approval.event_source,
                    );
                    setEditingId(null);
                  }}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                  data-testid="save-send-ambassador-reply"
                >
                  Save & Send
                </button>
                <button
                  onClick={() => setEditingId(null)}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                  data-testid="cancel-edit-ambassador-reply"
                >
                  Cancel
                </button>
              </div>
            </div>
          ) : (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    true,
                    undefined,
                    approval.event_source,
                  )
                }
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                aria-label="Approve & Send Draft"
                data-testid="approve-ambassador-reply"
              >
                ✨ Approve & Send Draft
              </button>
              <button
                onClick={() => {
                  setEditingId(approval.id);
                  setEditContent(
                    (approval.proposed_action || approval.context_payload)
                      ?.generated_response ||
                      (approval.proposed_action || approval.context_payload)
                        ?.draft_reply ||
                      "",
                  );
                }}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Edit Draft"
                data-testid="edit-ambassador-reply"
              >
                Edit
              </button>
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    false,
                    undefined,
                    approval.event_source,
                  )
                }
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Dismiss Draft"
                data-testid="dismiss-ambassador-reply"
                disabled={loadingAction !== null}
              >
                {isActionLoading("dismiss") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  "Dismiss"
                )}
              </button>
            </div>
          )
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "quote_draft" ? (
          editingId === approval.id ? (
            <div className="flex flex-col gap-3 w-full">
              <div className="flex flex-col gap-1">
                <label className="text-xs text-gray-500 font-semibold">
                  Total Price ($)
                </label>
                <input
                  type="number"
                  className="w-full p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all"
                  value={editQuotePrice}
                  onChange={(e) => setEditQuotePrice(e.target.value)}
                  data-testid="edit-quote-price"
                  autoFocus
                />
              </div>
              <div className="flex flex-col gap-1">
                <label className="text-xs text-gray-500 font-semibold">
                  Scope of Work
                </label>
                <textarea
                  className="w-full min-h-[44px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
                  rows={3}
                  value={editQuoteScope}
                  onChange={(e) => setEditQuoteScope(e.target.value)}
                  data-testid="edit-quote-scope"
                />
              </div>
              <div className="flex gap-3 mt-2">
                <button
                  onClick={() => {
                    handleDecision(
                      approval.id,
                      true,
                      JSON.stringify({
                        price: parseFloat(editQuotePrice),
                        scope: editQuoteScope,
                      }),
                      approval.event_source,
                    );
                    setEditingId(null);
                  }}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                  data-testid="modal-approve-btn"
                  disabled={loadingAction !== null}
                >
                  {isActionLoading("approve") ? (
                    <span className="animate-pulse">Loading...</span>
                  ) : (
                    "Approve & Send"
                  )}
                </button>
                <button
                  onClick={() => setEditingId(null)}
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                  data-testid="cancel-edit-quote"
                >
                  Cancel
                </button>
              </div>
            </div>
          ) : (
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    true,
                    undefined,
                    approval.event_source,
                  )
                }
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                aria-label="Approve & Send"
                data-testid="approve-quote-draft"
                disabled={loadingAction !== null}
              >
                {isActionLoading("approve") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  "Approve & Send"
                )}
              </button>
              <button
                onClick={() => {
                  window.location.href = `/quoting?id=${(approval.proposed_action || approval.context_payload)?.quote_id || approval.id}`;
                }}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Edit Draft"
                data-testid="edit-quote-draft"
              >
                Edit
              </button>
            </div>
          )
        ) : (approval.proposed_action || approval.context_payload)?.context
            ?.smart_pricing === true ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve & Run Sale"
              data-testid="approve-run-sale"
            >
              Approve & Run Sale
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss proposal"
              data-testid="feed-dismiss-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : ((approval as any).action_type || "") ===
          "Review Proposed Win-back" ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve"
              data-testid="feed-approve-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve"
              )}
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss"
              data-testid="feed-dismiss-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("reject") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)?.context
            ?.weekly_health_report === true ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            {!isDraftExpanded ? (
              <button
                onClick={() => setIsDraftExpanded(true)}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-all duration-200 shadow-md flex items-center justify-center"
                aria-label="Draft it"
                data-testid="feed-approve-btn"
              >
                Yes, draft it!
              </button>
            ) : (
              <div className="flex flex-col gap-2 w-full">
                <div className="p-3 bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 text-sm">
                  <p className="font-medium text-gray-700 dark:text-gray-300">
                    Drafted Content:
                  </p>
                  <p className="mt-1 text-gray-600 dark:text-gray-400">
                    {(approval.proposed_action || approval.context_payload)
                      .context.actionable_suggestion ||
                      "Here is the drafted content..."}
                  </p>
                </div>
                <button
                  onClick={() =>
                    handleDecision(
                      approval.id,
                      true,
                      undefined,
                      approval.event_source,
                    )
                  }
                  className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-all duration-200 shadow-md flex items-center justify-center"
                  aria-label="Approve & Send"
                  data-testid="feed-approve-btn"
                  disabled={loadingAction !== null}
                >
                  {isActionLoading("approve") ? (
                    <span className="animate-pulse">Loading...</span>
                  ) : (
                    "Approve & Send"
                  )}
                </button>
              </div>
            )}
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss proposal"
              data-testid="feed-dismiss-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.remaining_stock !== undefined ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-amber-500 text-white font-medium hover:bg-amber-600 transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve Restock"
              data-testid="feed-approve-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve Restock"
              )}
            </button>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  false,
                  undefined,
                  approval.event_source,
                )
              }
              className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss restock"
              data-testid="feed-dismiss-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("dismiss") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Dismiss"
              )}
            </button>
          </div>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "create_product" ? (
          <>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-500 text-white font-medium hover:bg-green-600 transition-all duration-200 shadow-md flex items-center justify-center mb-3"
              aria-label="Approve & Create"
              data-testid="feed-approve-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve & Create"
              )}
            </button>
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    false,
                    undefined,
                    approval.event_source,
                  )
                }
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Ask Agent to Adjust"
                data-testid="feed-dismiss-btn"
                disabled={loadingAction !== null}
              >
                {isActionLoading("dismiss") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  "Ask Agent to Adjust"
                )}
              </button>
            </div>
          </>
        ) : (approval.proposed_action || approval.context_payload)
            ?.feature_type === "quote_draft" ? (
          <>
            <button
              onClick={() =>
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-500 text-white font-medium hover:bg-green-600 transition-all duration-200 shadow-md flex items-center justify-center mb-3"
              aria-label="Approve & Send"
              data-testid="feed-approve-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve & Send"
              )}
            </button>
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <a
                href={`/quotes/${(approval.proposed_action || approval.context_payload)?.quote_id || approval.id}`}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Edit Draft"
                data-testid="edit-proposal"
              >
                Edit Draft
              </a>
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    false,
                    undefined,
                    approval.event_source,
                  )
                }
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Ask Agent to Adjust"
                data-testid="feed-dismiss-btn"
                disabled={loadingAction !== null}
              >
                {isActionLoading("dismiss") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  "Ask Agent to Adjust"
                )}
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
                  handleDecision(
                    approval.id,
                    true,
                    editContent,
                    approval.event_source,
                  );
                  setEditingId(null);
                }}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                data-testid="save-proposal"
                disabled={loadingAction !== null}
              >
                {isActionLoading("approve") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  "Save & Approve"
                )}
              </button>
              <button
                onClick={() => setEditingId(null)}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
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
                handleDecision(
                  approval.id,
                  true,
                  undefined,
                  approval.event_source,
                )
              }
              className="w-full min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] bg-green-500 text-white font-medium hover:bg-green-600 transition-all duration-200 shadow-md flex items-center justify-center mb-3"
              aria-label="Approve proposal"
              data-testid="feed-approve-btn"
              disabled={loadingAction !== null}
            >
              {isActionLoading("approve") ? (
                <span className="animate-pulse">Loading...</span>
              ) : (
                "Approve"
              )}
            </button>
            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => {
                  setEditingId(approval.id);
                  const textToEdit =
                    (approval.proposed_action || approval.context_payload)
                      ?.generated_response ||
                    (approval.proposed_action || approval.context_payload)
                      ?.draft_reply ||
                    approval.context_payload?.description ||
                    approval.proposed_action?.message ||
                    approval.proposed_action?.action_type ||
                    approval.event_source;
                  setEditContent(textToEdit || "");
                }}
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Edit proposal"
                data-testid="edit-proposal"
              >
                Edit
              </button>
              <button
                onClick={() =>
                  handleDecision(
                    approval.id,
                    false,
                    undefined,
                    approval.event_source,
                  )
                }
                className="flex-1 min-h-[44px] min-w-[44px] max-w-full overflow-hidden px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                aria-label="Reject proposal"
                data-testid="feed-dismiss-btn"
                disabled={loadingAction !== null}
              >
                {isActionLoading("dismiss") ? (
                  <span className="animate-pulse">Loading...</span>
                ) : (
                  "Deny"
                )}
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
};
