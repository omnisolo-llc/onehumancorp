"use client";

import { useEffect, useState } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";

type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
  payload?: any;
};

type ApprovalsResponse = {
  pending_approvals: ApprovalRequest[];
  next_cursor?: string | null;
};

type OHCLedgerEntry = {
  id: string;
  tenant_id: string;
  event_type: string;
  department: string;
  payload: any;
  created_at: string;
};

type LedgerResponse = {
  entries: OHCLedgerEntry[];
};

export function UnifiedAgentFeed() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">("proposals");
  const [activities, setActivities] = useState<OHCLedgerEntry[]>([]);
  const [activityLoading, setActivityLoading] = useState(false);
  const [expandedActionId, setExpandedActionId] = useState<string | null>(null);

  const tenantId = () => {
    if (typeof window === "undefined") return "default";
    return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
  };

  useEffect(() => {
    let mounted = true;

    async function fetchAll() {
      try {
        setLoading(true);
        setActivityLoading(true);
        const tenant = tenantId();

        const [feedRes, activityRes] = await Promise.all([
          fetch(`/api/agents/approvals?tenant_id=${tenant}`, {
            headers: {
              "x-tenant-id": tenant,
              "x-user-id": "default",
            },
          }),
          fetch(`/api/agents/approvals/ledger?tenant_id=${tenant}`, {
            headers: {
              "x-tenant-id": tenant,
              "x-user-id": "default",
            },
          })
        ]);

        if (!feedRes.ok) {
          throw new Error("Failed to load agent feed");
        }

        const [feedData, activityData] = await Promise.all([
          feedRes.json(),
          activityRes.ok ? activityRes.json() : Promise.resolve({ pending_approvals: [] })
        ]);

        if (mounted) {
          if (feedData.pending_approvals) {
            setApprovals(feedData.pending_approvals);
          }
          if (activityData.entries) {
            setActivities(activityData.entries);
          }
        }
      } catch (err: any) {
        if (mounted) {
          setError(err.message || "Failed to load feed");
        }
        console.error("Failed to load activity", err);
      } finally {
        if (mounted) {
          setLoading(false);
          setActivityLoading(false);
        }
      }
    }

    fetchAll();
    return () => { mounted = false; };
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    // Optimistic UI update
    setApprovals(prev => prev.filter(app => app.id !== id));

    try {
      const tenant = tenantId();
      const res = await fetch(`/api/agents/approvals/${id}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": tenant,
          "x-user-id": "default",
        },
        body: JSON.stringify({ approved }),
      });

      if (!res.ok) {
        // If it fails, we might want to fetch again to restore state
        const refreshRes = await fetch(`/api/agents/approvals?tenant_id=${tenant}`, {
            headers: { "x-tenant-id": tenant, "x-user-id": "default" }
        });
        if (refreshRes.ok) {
            const data: ApprovalsResponse = await refreshRes.json();
            setApprovals(data.pending_approvals);
        }
        throw new Error("Failed to submit decision");
      }
    } catch (err: any) {
      setError(err.message || "Action failed");
    }
  };



  if (error) {
    return (
      <div className="w-full mb-6 p-4 glassmorphism rounded-[16px] border border-red-500/50 bg-red-500/10 text-red-500 text-center">
        {error}
      </div>
    );
  }

  return (
    <section className="mb-6 w-full" aria-label="Unified Agent Feed">
      <div className="mb-4 flex items-center border-b border-gray-200 dark:border-gray-700">
        <button
          onClick={() => setActiveTab("proposals")}
          className={`flex-1 py-3 text-center text-sm font-semibold transition-colors ${
            activeTab === "proposals"
              ? "border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Proposals ({approvals.length})
        </button>
        <button
          onClick={() => setActiveTab("activity")}
          className={`flex-1 py-3 text-center text-sm font-semibold transition-colors ${
            activeTab === "activity"
              ? "border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Activity Feed
        </button>
      </div>

      <div className="flex flex-col gap-4">
        {activeTab === "proposals" && (
          <>
            {loading && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Agent Proposals...
              </div>
            )}
            {!loading && approvals.length === 0 && (
              <div className="w-full flex flex-col items-center gap-6 p-6 glassmorphism rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm opacity-90 text-center">
                <div className="text-3xl mb-2">✨</div>
                <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  Your agents are currently monitoring the business. While you're here, why not help us grow?
                </p>
                <div className="w-full max-w-md text-left">
                   <GrowthReferralWidget />
                </div>
              </div>
            )}
            {approvals.map((approval) => (
              <div
                key={approval.id}
                className="glassmorphism p-5 rounded-[20px] border border-white/40 dark:border-white/10 shadow-lg flex flex-col gap-4 overflow-hidden"
              >
                <div className="flex flex-col gap-2">
                  <div className="flex items-center justify-between mb-1">
                    <span className={`text-[10px] font-bold uppercase tracking-[0.12em] px-2 py-0.5 rounded-md ${
                      approval.department.toLowerCase().includes('operations') ? 'bg-blue-500/10 text-blue-600 dark:text-blue-400 border border-blue-200/50 dark:border-blue-800/50' :
                      approval.department.toLowerCase().includes('marketing') ? 'bg-purple-500/10 text-purple-600 dark:text-purple-400 border border-purple-200/50 dark:border-purple-800/50' :
                      'bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 border border-indigo-200/50 dark:border-indigo-800/50'
                    }`}>
                      {approval.department.replace(/([A-Z])/g, ' $1').trim()} Agent
                    </span>
                    {approval.action_risk === 'HIGH' && (
                      <div className="flex items-center gap-1.5 px-2 py-0.5 bg-amber-50 dark:bg-amber-900/20 rounded-md border border-amber-100 dark:border-amber-900/30">
                         <div className="w-1.5 h-1.5 rounded-full bg-amber-500" />
                         <span className="text-[9px] font-black text-amber-700 dark:text-amber-400 uppercase tracking-tighter">Needs Review</span>
                      </div>
                    )}
                  </div>
                  <h3 className="text-[19px] font-bold text-[#1D1D1F] dark:text-[#F5F5F7] leading-[1.3] font-outfit">
                    {approval.description}
                  </h3>

                  {/* Expanded Content Area */}
                  {expandedActionId === approval.id && (
                    <div className="mt-2 animate-in fade-in slide-in-from-top-2 duration-300">
                      {approval.payload?.feature_type === 'promo_advisory' && (
                        <div className="bg-white/50 dark:bg-black/20 rounded-xl p-4 border border-white/20 dark:border-white/5 shadow-inner">
                           <div className="text-[11px] font-bold text-gray-400 uppercase mb-2">Generated Draft</div>
                           <pre className="text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap font-sans italic leading-relaxed">
                             "{approval.payload.draft_content}"
                           </pre>
                        </div>
                      )}
                      {approval.payload?.feature_type === 'social_post' && (
                        <div className="bg-white/50 dark:bg-black/20 rounded-xl overflow-hidden border border-white/20 dark:border-white/5 shadow-inner">
                           <img src={approval.payload.image_url} alt="Social post preview" className="w-full h-48 object-cover" />
                           <div className="p-3">
                             <div className="text-[11px] font-bold text-gray-400 uppercase mb-1">Instagram Caption</div>
                             <p className="text-sm text-gray-700 dark:text-gray-300 leading-snug">
                               {approval.payload.caption}
                             </p>
                           </div>
                        </div>
                      )}
                      {approval.payload?.feature_type === "quote_draft" && (
                        <div className="bg-blue-50/50 dark:bg-blue-900/10 border border-blue-100 dark:border-blue-900/30 rounded-xl p-4 flex flex-col gap-3">
                          <div className="flex items-center gap-2 text-blue-800 dark:text-blue-300 font-semibold text-sm">
                            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                            </svg>
                            Draft Quote: {approval.payload.service || 'Plumbing Fix'}
                          </div>
                          <div className="text-xs text-blue-700 dark:text-blue-400 font-medium">
                            {approval.payload.customer_inquiry}
                          </div>
                          <div className="bg-white dark:bg-gray-800 p-3 rounded-lg border border-blue-100 dark:border-blue-900/50">
                            <div className="space-y-2">
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500">Calculated Total:</span>
                                <span className="text-xs font-semibold text-gray-900 dark:text-gray-100">${approval.payload.suggested_price}</span>
                              </div>
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500">Scope of Work:</span>
                                <span className="text-xs font-medium text-gray-800 dark:text-gray-200">{approval.payload.scope}</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      )}
                      {approval.payload?.feature_type === 'stockout_restock_and_price' ? (
                        <div className="bg-amber-50/50 dark:bg-amber-900/10 border border-amber-100 dark:border-amber-900/30 rounded-xl p-4 flex flex-col gap-2">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Current Price:</span>
                            <span className="font-semibold text-gray-400 dark:text-gray-500 line-through">
                               ${Number(approval.payload.old_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Suggested Price:</span>
                            <span className="font-bold text-green-600 dark:text-green-400 text-base" data-testid="stockout-new-price">
                               ${Number(approval.payload.new_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Reorder Quantity:</span>
                            <span className="font-bold text-blue-600 dark:text-blue-400 text-base" data-testid="stockout-reorder">
                               {approval.payload.suggested_reorder_quantity} Units
                            </span>
                          </div>
                          <div className="text-sm font-medium text-gray-800 dark:text-gray-200 mt-2">
                            {approval.payload.message}
                          </div>
                        </div>
                      ) : approval.payload?.context?.smart_pricing === true ? (
                        <div className="bg-green-50/50 dark:bg-green-900/10 border border-green-100 dark:border-green-900/30 rounded-xl p-4 flex flex-col gap-2">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Current Price:</span>
                            <span className="font-semibold text-gray-400 dark:text-gray-500 line-through">
                              ${Number(approval.payload.context.old_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Suggested Price:</span>
                            <span className="font-bold text-green-600 dark:text-green-400 text-base">
                              ${Number(approval.payload.context.new_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Sales Projection:</span>
                            <span className="font-semibold text-indigo-600 dark:text-indigo-400">
                              {approval.payload.context.sales_projection}
                            </span>
                          </div>
                        </div>
                      ) : null}
                      {approval.payload?.context?.weekly_health_report === true && (
                        <div className="bg-indigo-50/50 dark:bg-indigo-900/10 border border-indigo-100 dark:border-indigo-900/30 rounded-xl p-4 flex flex-col gap-2">
                          <div className="text-sm text-gray-700 dark:text-gray-300">
                            <span className="font-semibold">Summary:</span> {approval.payload.context.summary}
                          </div>
                          <div className="text-sm text-indigo-600 dark:text-indigo-400 font-medium">
                            <span className="font-semibold text-gray-700 dark:text-gray-300">Suggestion:</span> {approval.payload.context.actionable_suggestion}
                          </div>
                        </div>
                      )}
                      {approval.payload?.remaining_stock !== undefined && (
                        <div className="bg-amber-50/50 dark:bg-amber-900/10 border border-amber-100 dark:border-amber-900/30 rounded-xl p-4 flex flex-col gap-2">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Product:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.product_id}</span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Stock Remaining:</span>
                            <span className="font-semibold text-red-600 dark:text-red-400">{approval.payload.remaining_stock}</span>
                          </div>
                        </div>
                      )}
                    </div>
                  )}

                  {/* Collapsed Preview / Simple Payload */}
                  {expandedActionId !== approval.id && (approval.payload?.context || approval.payload?.remaining_stock !== undefined || approval.payload?.feature_type) && (
                    <div className="mt-1">
                       {/* Simplified preview text for non-expanded cards */}
                       {approval.payload?.feature_type === "fulfillment_batch" && (
                         <div className="text-sm text-gray-500 dark:text-gray-400">
                            Fulfilling these will update stock and notify customers.
                         </div>
                       )}
                       {approval.payload?.feature_type === "quote_draft" && (
                         <div className="text-sm text-gray-500 dark:text-gray-400">
                            Proposed Quote: ${approval.payload.suggested_price}
                         </div>
                       )}
                       {approval.payload?.context?.smart_pricing === true && (
                         <div className="text-sm text-green-600 dark:text-green-400 font-medium">
                            Price optimization suggested.
                         </div>
                       )}
                       {approval.payload?.feature_type === 'stockout_restock_and_price' && (
                         <div className="text-sm text-amber-600 dark:text-amber-400 font-medium">
                            Stockout reorder & price adjustment.
                         </div>
                       )}
                    </div>
                  )}
                </div>

                <div className="flex flex-col gap-3 w-full">
                  {approval.payload?.feature_type === "fulfillment_batch" ? (
                    <button
                      onClick={() => handleDecision(approval.id, true)}
                      className="w-full min-h-[56px] px-6 rounded-2xl bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-black font-bold text-lg hover:scale-[1.02] active:scale-[0.98] transition-all shadow-xl flex items-center justify-center gap-2"
                      aria-label="Fulfill Now"
                      data-testid="approve-fulfillment"
                    >
                      <span>Fulfill Now</span>
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M5 13l4 4L19 7" />
                      </svg>
                    </button>
                  ) : approval.payload?.feature_type === 'promo_advisory' || approval.payload?.context?.weekly_health_report === true ? (
                    <div className="flex flex-col gap-3 w-full">
                      {expandedActionId === approval.id ? (
                        <button
                          onClick={() => handleDecision(approval.id, true)}
                          className="w-full min-h-[56px] px-6 rounded-2xl bg-[#0066FF] text-white font-bold text-lg hover:bg-[#0052CC] transition-all shadow-xl flex items-center justify-center"
                          aria-label={approval.payload?.context?.weekly_health_report === true ? "Approve" : "Approve & Send"}
                          data-testid="approve-send-promo"
                        >
                          {approval.payload?.context?.weekly_health_report === true ? "Approve" : "Approve & Send"}
                        </button>
                      ) : (
                        <button
                          onClick={() => setExpandedActionId(approval.id)}
                          className="w-full min-h-[56px] px-6 rounded-2xl bg-[#0066FF] text-white font-bold text-lg hover:bg-[#0052CC] transition-all shadow-xl flex items-center justify-center"
                          aria-label="Yes, draft it"
                          data-testid="approve-draft"
                        >
                          Yes, draft it
                        </button>
                      )}
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="w-full min-h-[48px] px-6 rounded-2xl border border-gray-200 dark:border-gray-800 text-gray-500 font-semibold text-sm hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors"
                        aria-label="Discard"
                      >
                        Discard
                      </button>
                    </div>
                  ) : approval.payload?.feature_type === 'social_post' ? (
                    <div className="flex flex-col gap-3 w-full">
                       <button
                          onClick={() => expandedActionId === approval.id ? handleDecision(approval.id, true) : setExpandedActionId(approval.id)}
                          className="w-full min-h-[56px] px-6 rounded-2xl bg-[#FF3B30] text-white font-bold text-lg hover:opacity-90 transition-all shadow-xl flex items-center justify-center"
                          aria-label={expandedActionId === approval.id ? "Approve & Post" : "Review Post"}
                          data-testid="approve-social-post"
                        >
                          {expandedActionId === approval.id ? "Approve & Post" : "Review Post"}
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="w-full min-h-[48px] px-6 rounded-2xl border border-gray-200 dark:border-gray-800 text-gray-500 font-semibold text-sm hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors"
                          aria-label="Discard"
                        >
                          Discard
                        </button>
                    </div>
                  ) : approval.payload?.feature_type === 'stockout_restock_and_price' || approval.payload?.remaining_stock !== undefined || approval.payload?.context?.smart_pricing === true ? (
                    <div className="flex flex-col gap-3 w-full">
                       <button
                          onClick={() => expandedActionId === approval.id ? handleDecision(approval.id, true) : setExpandedActionId(approval.id)}
                          className="w-full min-h-[56px] px-6 rounded-2xl bg-[#0066FF] text-white font-bold text-lg hover:bg-[#0052CC] transition-all shadow-xl flex items-center justify-center"
                          aria-label={expandedActionId === approval.id ? "Approve" : "Review Proposal"}
                          data-testid="approve-stockout"
                        >
                          {expandedActionId === approval.id ? "Approve" : "Review Proposal"}
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="w-full min-h-[48px] px-6 rounded-2xl border border-gray-200 dark:border-gray-800 text-gray-500 font-semibold text-sm hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors"
                          aria-label="Dismiss"
                        >
                          Dismiss
                        </button>
                    </div>
                  ) : (
                    // Default Fallback Massive Button
                    <div className="flex flex-col gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="w-full min-h-[56px] px-6 rounded-2xl bg-[#0066FF] text-white font-bold text-lg hover:bg-[#0052CC] transition-all shadow-xl flex items-center justify-center"
                        aria-label="Approve"
                        data-testid="approve-proposal"
                      >
                        Approve
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="w-full min-h-[48px] px-6 rounded-2xl border border-gray-200 dark:border-gray-800 text-gray-500 font-semibold text-sm hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors"
                        aria-label="Dismiss"
                      >
                        Dismiss
                      </button>
                    </div>
                  )}
                </div>
              </div>
            ))}
          </>
        )}

        {activeTab === "activity" && (
          <>
            {activityLoading && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Activity Feed...
              </div>
            )}
            {!activityLoading && activities.length === 0 && (
              <div className="w-full p-6 glassmorphism rounded-[16px] text-center">
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  No recent activity found.
                </p>
              </div>
            )}
            <div className="flex flex-col gap-3 min-w-[320px] max-w-full">
            {activities.map((activity) => (
              <div
                key={activity.id}
                className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-3 opacity-90 min-h-[44px]"
              >
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold font-outfit uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                    {activity.department.replace('_', ' ')}
                  </span>
                  <span className="text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-md text-green-600 bg-green-50 dark:text-green-400 dark:bg-green-900/30">
                    APPROVED
                  </span>
                </div>
                <h3 className="text-md font-semibold font-inter text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                  {(() => {
                    try {
                      const p = typeof activity.payload === 'string' ? JSON.parse(activity.payload) : activity.payload;
                      return p?.original_payload?.description || 'Action completed';
                    } catch (e) {
                      return 'Action completed';
                    }
                  })()}
                </h3>
                <span className="text-xs text-gray-500 font-inter">{new Date(activity.created_at).toLocaleString()}</span>
              </div>
            ))}
            </div>
          </>
        )}
      </div>
    </section>
  );
}
