"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
// Assuming we don't need GrowthReferralWidget in this pure feed view, or we can add it at the bottom.

type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
  payload?: any;
};

export default function MobileAgentFeed() {
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
          }),
        ]);

        if (!feedRes.ok) throw new Error("Failed to load agent feed");
        const feedData = await feedRes.json();
        if (mounted) setApprovals(feedData.pending_approvals || []);

        if (activityRes.ok) {
           const activityData = await activityRes.json();
           if (mounted) setActivities(activityData.pending_approvals || []);
        }

      } catch (err) {
        console.error("Agent feed error:", err);
        if (mounted) setError("Could not load feed at this time.");
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

  const handleDecision = async (id: string, approve: boolean) => {
    try {
      const tenant = tenantId();
      setApprovals(current => current.filter(a => a.id !== id));

      const res = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          "x-tenant-id": tenant,
          "x-user-id": "default",
        },
        body: JSON.stringify({
          action: approve ? 'approve' : 'reject'
        })
      });

      if (!res.ok) {
        throw new Error('Failed to update decision');
      }

      // Refresh activity feed
      const refreshRes = await fetch(`/api/agents/approvals/activity?tenant_id=${tenant}`, {
         headers: {
            "x-tenant-id": tenant,
            "x-user-id": "default",
         },
      });
      if (refreshRes.ok) {
        const data = await refreshRes.json();
        setActivities(data.pending_approvals || []);
      }

    } catch (err) {
      console.error('Decision error:', err);
    }
  };

  return (
    <main className="min-h-[100dvh] bg-[#f4f6f8] dark:bg-[#111827] max-w-[375px] mx-auto overflow-x-hidden font-inter relative pb-20">

      {/* Premium Header */}
      <header className="sticky top-0 z-50 glassmorphism border-b border-white/40 dark:border-white/10 px-4 py-3 flex justify-between items-center">
         <div className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-full bg-indigo-100 dark:bg-indigo-900/50 flex items-center justify-center">
              🤖
            </div>
            <h1 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Unified Feed</h1>
         </div>
         <Link href="/dashboard" className="text-sm font-medium text-gray-500 hover:text-gray-800 dark:hover:text-gray-300">
           Dashboard
         </Link>
      </header>

      <section className="w-full px-4 pt-6" aria-label="Unified Agent Feed">
        <div className="flex items-center justify-between mb-4 bg-white dark:bg-gray-800 p-1 rounded-[12px] shadow-sm border border-gray-100 dark:border-gray-700">
          <button
            onClick={() => setActiveTab("proposals")}
            className={`flex-1 py-2 px-3 rounded-[8px] text-sm font-semibold transition-all ${
              activeTab === "proposals"
                ? "bg-indigo-50 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300 shadow-sm"
                : "text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700"
            }`}
          >
            Proposals ({approvals.length})
          </button>
          <button
            onClick={() => setActiveTab("activity")}
            className={`flex-1 py-2 px-3 rounded-[8px] text-sm font-semibold transition-all ${
              activeTab === "activity"
                ? "bg-indigo-50 dark:bg-indigo-900/40 text-indigo-700 dark:text-indigo-300 shadow-sm"
                : "text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700"
            }`}
          >
            Activity Feed
          </button>
        </div>

        <div className="flex flex-col gap-4 pb-8">
          {error && (
            <div className="p-4 bg-red-50 text-red-700 rounded-[12px] text-sm">
              {error}
            </div>
          )}

          {activeTab === "proposals" && (
            <>
              {loading && (
                <div className="w-full p-6 glassmorphism rounded-[16px] flex flex-col items-center justify-center text-gray-500 gap-3">
                  <div className="w-6 h-6 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
                  Loading Agent Proposals...
                </div>
              )}
              {!loading && approvals.length === 0 && (
                <div className="w-full p-8 glassmorphism rounded-[16px] text-center flex flex-col items-center">
                  <div className="w-16 h-16 bg-green-50 dark:bg-green-900/20 rounded-full flex items-center justify-center mb-4">
                    <svg className="w-8 h-8 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                  </div>
                  <h3 className="text-lg font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
                  <p className="text-sm text-gray-600 dark:text-gray-400 mt-2">
                    Your AI agents have no pending actions that require your approval right now.
                  </p>
                </div>
              )}
              {approvals.map((approval) => (
                <div
                  key={approval.id}
                  className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-lg flex flex-col gap-4 animate-fade-in hover:shadow-xl transition-shadow"
                >
                  <div className="flex items-center justify-between">
                    <span className="text-[10px] font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                      {approval.department.replace('_', ' ')}
                    </span>
                    <span className="text-[10px] font-bold uppercase tracking-wider text-amber-600 dark:text-amber-400 bg-amber-50 dark:bg-amber-900/30 px-2 py-1 rounded-md flex items-center gap-1">
                      <span className="w-1.5 h-1.5 rounded-full bg-amber-500 animate-pulse"></span>
                      Requires Review
                    </span>
                  </div>

                  <h3 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                    {approval.description}
                  </h3>

                  {approval.payload && (
                    <div className="bg-white/50 dark:bg-gray-900/50 rounded-[12px] p-4 text-sm text-gray-700 dark:text-gray-300 border border-white/20">
                      {approval.payload.context?.smart_pricing === true ? (
                        <div className="flex flex-col gap-2">
                          <p className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Smart Pricing Proposal:</p>
                          <p>Based on competitor analysis, suggest lowering the price of '{approval.payload.product_name}' by {approval.payload.discount_percentage}% for the weekend.</p>
                        </div>
                      ) : approval.payload.context?.weekly_health_report === true ? (
                         <div className="flex flex-col gap-2">
                          <p className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Advisory Action:</p>
                          <p>We noticed traffic dipped 12% this week. Would you like me to draft a quick 'weekend special' email to your subscribers?</p>
                        </div>
                      ) : approval.payload.remaining_stock !== undefined ? (
                        <div className="flex flex-col gap-2">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Item:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.item_name}</span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Current Stock:</span>
                            <span className="font-semibold text-red-600 dark:text-red-400">{approval.payload.remaining_stock}</span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Action:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">Order {approval.payload.suggested_reorder_qty} units</span>
                          </div>
                        </div>
                      ) : (
                        <>
                          {approval.payload.body ? (
                            <p className="italic border-l-2 border-indigo-300 pl-3">"{approval.payload.body}"</p>
                          ) : (
                            <>
                              {approval.payload.metric_name ? (
                                <div className="flex flex-col gap-2">
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Metric:</span>
                                    <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.metric_name}</span>
                                  </div>
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Change:</span>
                                    <span className="font-semibold text-red-600 dark:text-red-400">{approval.payload.drop_percentage}%</span>
                                  </div>
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Alert:</span>
                                    <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.message}</span>
                                  </div>
                                </div>
                              ) : null}
                            </>
                          )}
                        </>
                      )}
                    </div>
                  )}

                  <div className="flex flex-col gap-3 w-full mt-2">
                    {approval.payload?.context?.smart_pricing === true ? (
                      <div className="flex flex-col gap-3 w-full">
                        <button
                          onClick={() => handleDecision(approval.id, true)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md flex items-center justify-center"
                          aria-label="Approve & Run Sale"
                          data-testid="approve-run-sale"
                        >
                          Approve & Run Sale
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                          aria-label="Dismiss proposal"
                          data-testid="dismiss-sale"
                        >
                          Dismiss
                        </button>
                      </div>
                    ) : approval.payload?.context?.weekly_health_report === true ? (
                      <div className="flex flex-col gap-3 w-full">
                        <button
                          onClick={() => handleDecision(approval.id, true)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-colors shadow-md flex items-center justify-center"
                          aria-label="Draft it"
                          data-testid="approve-draft"
                        >
                          Yes, draft it!
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                          aria-label="Dismiss proposal"
                          data-testid="dismiss-draft"
                        >
                          Dismiss
                        </button>
                      </div>
                    ) : approval.payload?.remaining_stock !== undefined ? (
                      <div className="flex flex-col gap-3 w-full">
                        <button
                          onClick={() => handleDecision(approval.id, true)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] bg-amber-500 text-white font-medium hover:bg-amber-600 transition-colors shadow-md flex items-center justify-center"
                          aria-label="Approve Restock"
                          data-testid="approve-restock"
                        >
                          Approve Restock
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                          aria-label="Dismiss restock"
                          data-testid="dismiss-restock"
                        >
                          Dismiss
                        </button>
                      </div>
                    ) : approval.payload?.feature_type === 'quote_draft' ? (
                      <div className="flex flex-col gap-3 w-full">
                        <button
                          onClick={() => handleDecision(approval.id, true)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md flex items-center justify-center"
                          aria-label="Approve & Send Proposal"
                          data-testid="approve-proposal"
                        >
                          Approve & Send Proposal
                        </button>
                        <div className="flex gap-3 w-full">
                          <button
                            onClick={() => {}}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                            aria-label="Edit Draft"
                            data-testid="edit-proposal"
                          >
                            Edit
                          </button>
                          <button
                            onClick={() => handleDecision(approval.id, false)}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                            aria-label="Ask Agent to Adjust"
                            data-testid="reject-proposal"
                          >
                            Adjust
                          </button>
                        </div>
                      </div>
                    ) : (
                      <div className="flex flex-col gap-3 w-full">
                        <button
                          onClick={() => handleDecision(approval.id, true)}
                          className="w-full min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md flex items-center justify-center"
                          aria-label="Approve proposal"
                          data-testid="approve-proposal"
                        >
                          Approve
                        </button>
                        <div className="flex gap-3 w-full">
                          <button
                            onClick={() => {}}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                            aria-label="Edit proposal"
                            data-testid="edit-proposal"
                          >
                            Edit
                          </button>
                          <button
                            onClick={() => handleDecision(approval.id, false)}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors flex items-center justify-center"
                            aria-label="Reject proposal"
                            data-testid="reject-proposal"
                          >
                            Decline
                          </button>
                        </div>
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
              {activities.map((activity) => (
                <div
                  key={activity.id}
                  className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-3 opacity-90"
                >
                  <div className="flex items-center justify-between">
                    <span className="text-[10px] font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                      {activity.department.replace('_', ' ')}
                    </span>
                    <span className={`text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-md ${
                      activity.status === 'Approved' ? 'text-green-600 bg-green-50 dark:text-green-400 dark:bg-green-900/30' :
                      activity.status === 'Rejected' ? 'text-red-600 bg-red-50 dark:text-red-400 dark:bg-red-900/30' :
                      'text-gray-600 bg-gray-50 dark:text-gray-400 dark:bg-gray-800'
                    }`}>
                      {activity.status}
                    </span>
                  </div>
                  <h3 className="text-[15px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                    {activity.description}
                  </h3>
                </div>
              ))}
            </>
          )}
        </div>
      </section>

      {/* Bottom Nav Mock for Mobile Feel */}
      <nav className="fixed bottom-0 left-1/2 -translate-x-1/2 w-full max-w-[375px] h-16 bg-white/90 dark:bg-gray-900/90 backdrop-blur-md border-t border-gray-200 dark:border-gray-800 flex items-center justify-around z-50">
         <Link href="/dashboard" className="flex flex-col items-center gap-1 text-gray-400 hover:text-indigo-600">
           <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path></svg>
           <span className="text-[10px] font-medium">Hub</span>
         </Link>
         <Link href="/agent-feed" className="flex flex-col items-center gap-1 text-indigo-600">
           <div className="relative">
             <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
             {approvals.length > 0 && (
                <span className="absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full border-2 border-white dark:border-gray-900"></span>
             )}
           </div>
           <span className="text-[10px] font-medium">Inbox</span>
         </Link>
         <button className="flex flex-col items-center gap-1 text-gray-400 hover:text-indigo-600">
           <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 4v16m8-8H4"></path></svg>
           <span className="text-[10px] font-medium">Create</span>
         </button>
      </nav>
    </main>
  );
}
