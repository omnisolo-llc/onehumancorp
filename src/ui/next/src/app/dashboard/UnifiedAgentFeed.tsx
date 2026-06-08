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

export function UnifiedAgentFeed() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">("proposals");
  const [activities, setActivities] = useState<ApprovalRequest[]>([]);
  const [activityLoading, setActivityLoading] = useState(false);

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
          fetch(`/api/agents/approvals/activity?tenant_id=${tenant}`, {
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
          if (activityData.pending_approvals) {
            setActivities(activityData.pending_approvals);
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
                className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4"
              >
                <div className="flex flex-col gap-1">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                      {approval.department.replace('_', ' ')}
                    </span>
                    {approval.action_risk === 'HIGH' && (
                      <span className="text-xs font-bold uppercase tracking-wider text-red-600 bg-red-50 px-2 py-1 rounded-md">
                        Requires Review
                      </span>
                    )}
                  </div>
                  <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1">
                    {approval.description}
                  </h3>
                  {(approval.payload?.context || approval.payload?.remaining_stock !== undefined || approval.payload?.feature_type === "quote_draft") && (
                    <div className="mt-2 flex flex-col gap-1 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
                      {approval.payload?.feature_type === "quote_draft" ? (
                        <div className="mb-2 p-2 rounded-xl bg-blue-50 dark:bg-blue-900/20 border border-blue-100 dark:border-blue-800 flex flex-col gap-2" data-testid="draft-quote-card">
                          <div className="flex items-center gap-2 text-blue-800 dark:text-blue-400 font-semibold text-sm">
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                            </svg>
                            Draft Quote: {approval.payload.service || 'Plumbing Fix'} for Customer
                          </div>
                          <div className="text-xs text-blue-700 dark:text-blue-300 font-medium">
                            {approval.payload.customer_inquiry}
                          </div>
                          <div className="bg-white dark:bg-gray-800 p-3 rounded-lg border border-blue-100 dark:border-blue-800 relative mt-1">
                            <div className="text-[10px] uppercase font-bold text-gray-400 dark:text-gray-500 mb-2">
                              AI Proposed Quote
                            </div>
                            <div className="space-y-1">
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500 dark:text-gray-400">Calculated Total:</span>
                                <span className="text-xs font-semibold text-gray-900 dark:text-gray-100">${approval.payload.suggested_price}</span>
                              </div>
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500 dark:text-gray-400">Scope of Work:</span>
                                <span className="text-xs font-medium text-gray-800 dark:text-gray-200 text-right ml-2">{approval.payload.scope}</span>
                              </div>
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500 dark:text-gray-400">Suggested Time:</span>
                                <span className="text-xs font-medium text-gray-800 dark:text-gray-200">{approval.payload.suggested_time}</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      ) : approval.payload?.context?.smart_pricing === true ? (
                        <>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Current Price:</span>
                            <span className="font-semibold text-gray-400 dark:text-gray-500 line-through">
                              ${Number(approval.payload.context.old_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Suggested Price:</span>
                            <span className="font-bold text-green-600 dark:text-green-400 text-base" data-testid="smart-pricing-new-price">
                              ${Number(approval.payload.context.new_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Sales Projection:</span>
                            <span className="font-semibold text-indigo-600 dark:text-indigo-400" data-testid="smart-pricing-sales-projection">
                              {approval.payload.context.sales_projection}
                            </span>
                          </div>
                        </>
                      ) : (
                        <>
                          {approval.payload?.context?.weekly_health_report === true ? (
                            <div className="flex flex-col gap-2">
                              <div className="text-sm text-gray-700 dark:text-gray-300">
                                <span className="font-semibold">Summary:</span> {approval.payload.context.summary}
                              </div>
                              <div className="text-sm text-indigo-600 dark:text-indigo-400 font-medium">
                                <span className="font-semibold text-gray-700 dark:text-gray-300">Suggestion:</span> {approval.payload.context.actionable_suggestion}
                              </div>
                            </div>
                          ) : (
                            <>
                              {approval.payload?.context?.abandoned_carts_count !== undefined && (
                                <div className="flex justify-between items-center text-sm">
                                  <span className="text-gray-500 dark:text-gray-400">Abandoned Carts:</span>
                                  <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.context.abandoned_carts_count}</span>
                                </div>
                              )}
                              {approval.payload?.context?.potential_revenue !== undefined && (
                                <div className="flex justify-between items-center text-sm">
                                  <span className="text-gray-500 dark:text-gray-400">Potential Revenue:</span>
                                  <span className="font-semibold text-green-600 dark:text-green-400">
                                    ${Number(approval.payload.context.potential_revenue).toFixed(2)}
                                  </span>
                                </div>
                              )}
                              {approval.payload?.remaining_stock !== undefined && (
                                <div className="flex flex-col gap-2">
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Product ID:</span>
                                    <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.product_id}</span>
                                  </div>
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Remaining Stock:</span>
                                    <span className="font-semibold text-red-600 dark:text-red-400">{approval.payload.remaining_stock}</span>
                                  </div>
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Alert Message:</span>
                                    <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.message}</span>
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
                  {approval.payload?.feature_type === "quote_draft" ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md flex items-center justify-center"
                        aria-label="Approve & Send Proposal"
                        data-testid="approve-send"
                      >
                        Approve & Send Proposal
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                        aria-label="Edit Draft"
                        data-testid="edit-draft"
                      >
                        Edit Draft
                      </button>
                    </div>
                  ) : approval.payload?.context?.smart_pricing === true ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md flex items-center justify-center"
                        aria-label="Approve & Run Sale"
                        data-testid="approve-run-sale"
                      >
                        Approve & Run Sale
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                        aria-label="Dismiss proposal"
                        data-testid="dismiss-sale"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : approval.payload?.context?.weekly_health_report === true ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-colors shadow-md flex items-center justify-center"
                        aria-label="Draft it"
                        data-testid="approve-draft"
                      >
                        Yes, draft it!
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                        aria-label="Dismiss proposal"
                        data-testid="dismiss-draft"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : approval.payload?.remaining_stock !== undefined ? (
                    <div className="flex gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-amber-500 text-white font-medium hover:bg-amber-600 transition-colors shadow-md flex items-center justify-center"
                        aria-label="Approve Restock"
                        data-testid="approve-restock"
                      >
                        Approve Restock
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                        aria-label="Dismiss restock"
                        data-testid="dismiss-restock"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (
                    <>
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="w-full min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md"
                        aria-label="Approve proposal"
                        data-testid="approve-proposal"
                      >
                        Approve
                      </button>
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => {}}
                          className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                          aria-label="Edit proposal"
                          data-testid="edit-proposal"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                          aria-label="Reject proposal"
                          data-testid="reject-proposal"
                        >
                          Decline
                        </button>
                      </div>
                    </>
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
            {activities.map((activity) => (
              <div
                key={activity.id}
                className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-3 opacity-90"
              >
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                    {activity.department.replace('_', ' ')}
                  </span>
                  <span className={`text-xs font-bold uppercase tracking-wider px-2 py-1 rounded-md ${
                    activity.status === 'Approved' ? 'text-green-600 bg-green-50 dark:text-green-400 dark:bg-green-900/30' :
                    activity.status === 'Rejected' ? 'text-red-600 bg-red-50 dark:text-red-400 dark:bg-red-900/30' :
                    'text-gray-600 bg-gray-50 dark:text-gray-400 dark:bg-gray-800'
                  }`}>
                    {activity.status}
                  </span>
                </div>
                <h3 className="text-md font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                  {activity.description}
                </h3>
              </div>
            ))}
          </>
        )}
      </div>
    </section>
  );
}
