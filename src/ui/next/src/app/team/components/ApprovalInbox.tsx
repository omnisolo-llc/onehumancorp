"use client";

import { useState } from "react";
import { ApprovalRequest } from "../page";

type Props = {
  departmentId: string;
  departmentName: string;
  approvals: ApprovalRequest[];
  onBack: () => void;
  onApprove: (id: string, editedPayload?: import('@/lib/agent-feed-types').ActionPayload) => Promise<boolean>;
  onReject: (id: string) => Promise<boolean>;
  blocked: (id: string) => boolean;
  status: React.ReactNode;
  readState: 'loading' | 'unavailable' | 'ready';
};

export default function ApprovalInbox({
  departmentName,
  approvals,
  onBack,
  onApprove,
  onReject,
  blocked,
  status,
  readState,
}: Props) {
  const [selectedReview, setSelectedReview] = useState<ApprovalRequest | null>(
    null,
  );
  const [editedQuote, setEditedQuote] = useState<{ suggested_price: string, scope: string } | null>(null);
  const [editedDraft, setEditedDraft] = useState<string | null>(null);

  const extractPayload = (request: ApprovalRequest) => ({ desc: request.description, payload: request.payload });

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-full sm:w-[375px] max-w-[375px] min-h-[812px] bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-2xl overflow-hidden flex flex-col relative border border-white/40 rounded-3xl glassmorphism">
        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button
            onClick={onBack}
            className="w-[44px] h-[44px] flex items-center justify-center rounded-full bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm border border-gray-100 text-gray-500 hover:text-gray-900 transition-colors"
          >
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
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </button>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">
              {departmentName}
            </h1>
            <p className="text-gray-500 text-xs font-medium uppercase tracking-wider mt-1">
              Approval Inbox
            </p>
          </div>
        </div>

        <div className="px-6 py-4 border-b border-gray-100 text-sm text-gray-600">
          <p>Department policy cannot be verified or changed in this review view.</p>
          <button type="button" disabled aria-label="Department policy unavailable">Policy unavailable</button>
        </div>

        {status}
        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 space-y-4 hide-scrollbar">
          {readState !== 'ready' ? (
            <p role="status">{readState === 'loading' ? 'Refreshing recorded approvals…' : 'The approval list could not be verified.'}</p>
          ) : approvals.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-64 text-center px-8">
              <div className="w-16 h-16 bg-green-50 text-[#34C759] rounded-full flex items-center justify-center mb-4">
                <svg
                  className="w-8 h-8"
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
              </div>
              <h3 className="font-outfit font-bold text-gray-900 text-lg mb-2">
                All Caught Up!
              </h3>
              <p className="text-sm text-gray-500">
                There are no pending actions requiring your review.
              </p>
            </div>
          ) : (
            approvals.map((req) => {
              const { desc, payload } = extractPayload(req);
              return (
                <div
                  key={req.id}
                  className="glassmorphism rounded-2xl p-5 border border-white/40 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] transition-all duration-300"
                >
                  <div className="flex items-center gap-2 mb-3">
                    <span
                      className={`px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider ${
                        req.action_risk?.toLowerCase() === "high"
                          ? "bg-orange-100 text-orange-700"
                          : "bg-blue-100 text-blue-700"
                      }`}
                    >
                      {req.action_risk
                        ? req.action_risk.charAt(0).toUpperCase() +
                          req.action_risk.slice(1).toLowerCase()
                        : "Unknown"}{" "}
                      Risk
                    </span>
                    <span className="text-xs text-gray-400 font-medium">
                      {req.status}
                    </span>
                  </div>

                  <p className="text-gray-800 text-sm leading-relaxed mb-6 font-medium">
                    {desc}
                  </p>

                  {req.payload && !['ambassador_reply', 'lead_recovery', 'case_study', 'social_post_draft', 'quote_draft', 'low_stock_restock', 'abandoned_cart'].includes(req.payload.feature_type ?? '') && (
                    <pre className="mb-4 whitespace-pre-wrap break-words text-xs" aria-label="Recorded action payload">{JSON.stringify(req.payload, null, 2)}</pre>
                  )}

                  {req.payload?.feature_type === "ambassador_reply" && (
                    <div className="mb-6 p-4 rounded-xl glassmorphism border border-blue-100 flex flex-col gap-3">
                      <div className="flex items-center gap-2 text-blue-800 font-semibold text-sm">
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
                            d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"
                          />
                        </svg>
                        Customer Inquiry
                      </div>

                      <div className="app-card p-3 rounded-lg border border-blue-100 text-xs text-gray-700 italic">
                        "{req.payload.original_message}"
                      </div>

                      <div className="text-blue-800 font-semibold text-sm mt-2 flex items-center gap-2">
                        <svg
                          className="w-4 h-4"
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
                        Draft Reply
                      </div>
                      <div className="bg-[#0071E3] p-3 rounded-lg text-xs text-white shadow-inner">
                        {req.payload.generated_response}
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "lead_recovery" && (
                    <div className="mb-6 p-4 rounded-xl bg-orange-50 border border-orange-100 flex flex-col gap-3">
                      <div className="flex items-center gap-2 text-orange-800 font-semibold text-sm">
                        <svg
                          className="w-5 h-5 text-orange-600"
                          fill="none"
                          stroke="currentColor"
                          viewBox="0 0 24 24"
                        >
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        Missed Lead Detected
                      </div>
                      <div className="text-xs text-orange-700 font-medium">
                        {payload?.description || "No lead context was returned."}
                      </div>

                      <div className="app-card p-3 rounded-lg border border-orange-100 relative mt-2">
                        <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 absolute top-2 right-2">
                          AI Draft
                        </div>
                        <p className="text-xs text-gray-700 italic">
                          "{payload?.draft_reply || 'No reply draft was returned.'}"
                        </p>
                      </div>

                      <div className="flex gap-2 mt-1">
                        <span className="text-[10px] bg-orange-100 text-orange-700 px-2 py-1 rounded font-medium">
                          Customer Relationship
                        </span>
                        <span className="text-[10px] bg-gray-100 text-gray-600 px-2 py-1 rounded font-medium">
                          SMS
                        </span>
                      </div>
                    </div>
                  )}




                  {req.payload?.feature_type === "case_study" && (
                    <div className="mb-6 p-4 rounded-xl glassmorphism border border-blue-100 flex flex-col gap-3">
                      <div className="flex items-center gap-2 text-blue-800 font-semibold text-sm">
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
                            d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                          />
                        </svg>
                        Portfolio Post Drafted
                      </div>
                      <div className="text-xs text-blue-700">
                        Based on your recently completed job:{" "}
                        {req.payload.service_name}
                      </div>

                      <div className="app-card rounded-lg border border-blue-100 overflow-hidden shadow-sm">
                        {req.payload.media_url && (
                          <div className="w-full h-40 bg-gray-100 relative">
                            <img
                              src={req.payload.media_url}
                              alt="Project photo"
                              className="w-full h-full object-cover"
                            />
                          </div>
                        )}
                        <div className="p-3">
                          <div className="text-[10px] uppercase font-bold text-gray-400 mb-1">
                            Generated Description
                          </div>
                          <p className="text-xs text-gray-700 italic">
                            "{req.payload.draft_copy}"
                          </p>
                        </div>
                      </div>
                    </div>
                  )}


                  {req.payload?.feature_type === "social_post_draft" && (
                    <div className="mb-6 p-4 rounded-xl bg-pink-50 border border-pink-100 flex flex-col gap-3">
                      <div className="flex items-center gap-2 text-pink-800 font-semibold text-sm">
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
                            d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"
                          />
                        </svg>
                        Social Post Drafted
                      </div>
                      <div className="text-xs text-pink-700">
                        Based on your new product:{" "}
                        <span className="font-semibold">{req.payload.product_name}</span>
                      </div>

                      <div className="app-card rounded-lg border border-pink-100 overflow-hidden shadow-sm flex flex-col gap-2">
                        {req.payload.image_url && (
                          <div className="w-full h-40 bg-gray-100 relative">
                            <img
                              src={req.payload.image_url}
                              alt="Social post media"
                              className="w-full h-full object-cover"
                            />
                          </div>
                        )}
                        {req.payload.tiktok && (
                          <div className="p-3 border-b border-pink-50 last:border-b-0">
                            <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 flex items-center gap-1">
                              TikTok
                            </div>
                            <p className="text-xs text-gray-700 italic">
                              "{req.payload.tiktok}"
                            </p>
                          </div>
                        )}
                        {req.payload.instagram && (
                          <div className="p-3 border-b border-pink-50 last:border-b-0">
                            <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 flex items-center gap-1">
                              Instagram
                            </div>
                            <p className="text-xs text-gray-700 italic">
                              "{req.payload.instagram}"
                            </p>
                          </div>
                        )}
                        {req.payload.facebook && (
                          <div className="p-3 border-b border-pink-50 last:border-b-0">
                            <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 flex items-center gap-1">
                              Facebook
                            </div>
                            <p className="text-xs text-gray-700 italic">
                              "{req.payload.facebook}"
                            </p>
                          </div>
                        )}
                        {req.payload.draft_copy && (
                          <div className="p-3 border-b border-pink-50 last:border-b-0">
                            <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 flex items-center gap-1">
                              Draft Copy
                            </div>
                            <p className="text-xs text-gray-700 italic">
                              "{req.payload.draft_copy}"
                            </p>
                          </div>
                        )}
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "quote_draft" && (
                    <div className="mb-6 p-4 rounded-xl glassmorphism border border-white/40 flex flex-col gap-3 max-w-full break-words" data-testid="quote-draft-card">
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
                        Action Required: Approve Estimate for {req.payload.service}
                      </div>
                      <div className="text-xs text-[#0066FF] font-medium">
                        {req.payload.customer_inquiry}
                      </div>
                      {req.payload.suggested_time && (
                        <div className="text-xs text-orange-600 font-semibold bg-orange-50 p-2 rounded-lg border border-orange-100 mt-1">
                           🗓️ Proposed Time: {req.payload.suggested_time}
                           {req.payload.proposed_slot_id ? ' (Slot Temporarily Locked)' : ''}
                        </div>
                      )}

                      <div className="glassmorphism p-3 rounded-lg border border-white/40 relative mt-2">
                        <div className="text-[10px] uppercase font-bold text-gray-500 mb-2">
                          AI Proposed Quote
                        </div>
                        <div className="space-y-2">
                          <div className="flex justify-between">
                            <span className="text-xs text-gray-500">Calculated Total:</span>
                            <span className="text-xs font-semibold text-gray-900">${req.payload.suggested_price}</span>
                          </div>
                          <div className="flex justify-between">
                            <span className="text-xs text-gray-500">Scope of Work:</span>
                            <span className="text-xs font-medium text-gray-800">{req.payload.scope}</span>
                          </div>
                          {req.payload.suggested_time && (
                            <div className="flex justify-between">
                              <span className="text-xs text-gray-500">Suggested Time:</span>
                              <span className="text-xs font-medium text-gray-800">{req.payload.suggested_time}</span>
                            </div>
                          )}
                        </div>
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "low_stock_restock" && (
                    <div className="mb-6 p-4 rounded-xl bg-orange-50 border border-orange-100 flex flex-col gap-3">
                      <div className="flex items-center gap-2 text-orange-800 font-semibold text-sm">
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
                        Low Stock Alert
                      </div>
                      <div className="text-xs text-orange-700 font-medium">
                        Inventory for <span className="font-semibold">{req.payload.product_id}</span> is critically low.
                      </div>

                      <div className="app-card p-3 rounded-lg border border-orange-100 relative mt-2">
                        <div className="space-y-2">
                          <div className="flex justify-between">
                            <span className="text-xs text-gray-500">Remaining Stock:</span>
                            <span className="text-xs font-semibold text-orange-600">{req.payload.remaining_stock}</span>
                          </div>
                          <div className="text-xs text-gray-700 italic mt-2">
                            System suggests: "{req.payload.suggested_action}"
                          </div>
                        </div>
                      </div>
                    </div>
                  )}

                  {req.payload?.feature_type === "abandoned_cart" && (
                    <div className="mb-6 p-4 rounded-xl bg-rose-50 border border-rose-100 flex flex-col gap-3">
                      <div className="flex items-center gap-2 text-rose-800 font-semibold text-sm">
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
                            d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"
                          />
                        </svg>
                        Abandoned Cart Detected
                      </div>
                      <div className="text-xs text-rose-700 font-medium">
                        An abandoned-cart event returned by the backend requires review.
                      </div>

                      <div className="app-card p-3 rounded-lg border border-rose-100 relative">
                        <div className="text-[10px] uppercase font-bold text-gray-400 mb-1 absolute top-2 right-2">
                          AI Draft
                        </div>
                        <p className="text-xs text-gray-700 italic">
                          {typeof req.payload.generated_response === "string"
                            ? req.payload.generated_response
                            : "No recovery draft was returned."}
                        </p>
                      </div>
                    </div>
                  )}

                  <div className="flex gap-3">
                    <button
                      disabled={blocked(req.id)}
                      onClick={() => {
                        if (payload && (payload.original_message || payload.feature_type === "quote_draft" || payload.feature_type === "ambassador_reply")) {
                          setSelectedReview(req);
                          if (payload.feature_type === "quote_draft") {
                            setEditedQuote({
                              suggested_price: String(payload.suggested_price || ''),
                              scope: payload.scope || ''
                            });
                          } else if (payload.feature_type === "ambassador_reply") {
                            setEditedDraft(payload.generated_response || payload.draft_reply || '');
                          }
                        } else {
                          onReject(req.id);
                        }
                      }}
                      className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all min-h-[44px] min-w-[44px] w-full"
                    >
                      {payload && (payload.original_message || payload.feature_type === "quote_draft" || payload.feature_type === "ambassador_reply")
                        ? "Edit"
                        : "Reject / Edit"}
                    </button>
                    <button
                      disabled={blocked(req.id)}
                      onClick={() => { void onApprove(req.id); }}
                      className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-[#0066FF] text-white hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all min-h-[44px] min-w-[44px] w-full"
                    >
                      Record approval
                    </button>
                  </div>
                </div>
              );
            })
          )}
        </div>

        {/* Review Modal */}
        {selectedReview && (
          <div className="absolute inset-0 bg-black/40 z-50 flex flex-col justify-end">
            <div
              className="app-card rounded-t-3xl p-6 shadow-2xl transition-transform duration-300"
              style={{
                animation: "slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1)",
              }}
            >
              <h2 className="text-xl font-bold mb-4 font-outfit text-gray-900">
                Review Draft
              </h2>

              <div className="mb-4">
                <p className="text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">
                  Context
                </p>
                <div className="bg-gray-50 p-3 rounded-xl border border-gray-100 text-sm text-gray-700">
                  {extractPayload(selectedReview).payload
                    ?.original_message || extractPayload(selectedReview).payload?.customer_inquiry || "N/A"}
                </div>
              </div>

              {extractPayload(selectedReview).payload?.feature_type === "quote_draft" ? (
                <div className="mb-6 space-y-4">
                  <div>
                    <label className="block text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">Suggested Price ($)</label>
                    <input
                      type="number"
                      value={editedQuote?.suggested_price || ''}
                      onChange={(e) => setEditedQuote(prev => prev ? { ...prev, suggested_price: e.target.value } : null)}
                      className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 bg-white"
                      data-testid="edit-quote-price"
                    />
                  </div>
                  <div>
                    <label className="block text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">Scope of Work</label>
                    <textarea
                      value={editedQuote?.scope || ''}
                      onChange={(e) => setEditedQuote(prev => prev ? { ...prev, scope: e.target.value } : null)}
                      rows={3}
                      className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 bg-white resize-none"
                      data-testid="edit-quote-scope"
                    />
                  </div>
                </div>
              ) : extractPayload(selectedReview).payload?.feature_type === "ambassador_reply" ? (
                <div className="mb-6">
                  <p className="text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">
                    Edit Draft Reply
                  </p>
                  <textarea
                    value={editedDraft || ''}
                    onChange={(e) => setEditedDraft(e.target.value)}
                    rows={4}
                    className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50 bg-white resize-none"
                    data-testid="edit-ambassador-draft"
                  />
                </div>
              ) : (
                <div className="mb-6">
                  <p className="text-xs text-gray-500 font-medium uppercase tracking-wider mb-1">
                    Draft
                  </p>
                  <div className="glassmorphism p-3 rounded-xl border border-blue-100 text-sm text-gray-800 italic relative">
                    {extractPayload(selectedReview).payload
                      ?.generated_response || extractPayload(selectedReview).payload?.draft_reply || "N/A"}
                  </div>
                </div>
              )}

              <div className="flex gap-3">
                <button
                  disabled={blocked(selectedReview.id)}
                  onClick={async () => {
                    if (!await onReject(selectedReview.id)) return;
                    setSelectedReview(null);
                    setEditedQuote(null);
                    setEditedDraft(null);
                  }}
                  className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 min-h-[44px] min-w-[44px]"
                >
                  Discard
                </button>
                <button
                  onClick={() => {
                    setSelectedReview(null);
                    setEditedQuote(null);
                    setEditedDraft(null);
                  }}
                  className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 min-h-[44px] min-w-[44px]"
                >
                  Cancel
                </button>
                <button
                  disabled={blocked(selectedReview.id)}
                  onClick={async () => {
                    let recorded: boolean;
                    if (extractPayload(selectedReview).payload?.feature_type === "quote_draft" && editedQuote) {
                      recorded = await onApprove(selectedReview.id, {
                         ...extractPayload(selectedReview).payload,
                         suggested_price: parseFloat(editedQuote.suggested_price) || 0,
                         scope: editedQuote.scope
                      });
                    } else if (extractPayload(selectedReview).payload?.feature_type === "ambassador_reply" && editedDraft !== null) {
                      recorded = await onApprove(selectedReview.id, {
                         ...extractPayload(selectedReview).payload,
                         generated_response: editedDraft,
                         draft_reply: editedDraft,
                      });
                    } else {
                      recorded = await onApprove(selectedReview.id);
                    }
                    if (recorded) { setSelectedReview(null); setEditedQuote(null); setEditedDraft(null); }
                  }}
                  className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-[#0066FF] text-white hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all min-h-[44px] min-w-[44px] w-full"
                  data-testid="modal-approve-btn"
                >
                  Record approval
                </button>
              </div>
            </div>
          </div>
        )}
        <style
          dangerouslySetInnerHTML={{
            __html: `
          @keyframes slideUp {
            from { transform: translateY(100%); }
            to { transform: translateY(0); }
          }
        `,
          }}
        />
      </div>
    </div>
  );
}
