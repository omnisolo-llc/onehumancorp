"use client";

import { useEffect, useState } from "react";
import { InterventionPanel } from "@/components/InterventionPanel";

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
  const [activeIntervention, setActiveIntervention] = useState<{
    taskId: string;
    toolCallId: string;
    reason: string;
  } | null>(null);
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

    async function fetchFeed() {
      try {
        const tenant = tenantId();
        // Fetch proposals
        const res = await fetch(`/api/agents/approvals?tenant_id=${tenant}`, {
          headers: {
            "x-tenant-id": tenant,
            "x-user-id": "default",
          },
        });

        if (!res.ok) {
          throw new Error("Failed to load agent feed");
        }

        const data: ApprovalsResponse = await res.json();
        if (mounted && data.pending_approvals) {
          setApprovals(data.pending_approvals);
        }
      } catch (err: any) {
        if (mounted) {
          setError(err.message || "Failed to load feed");
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    }

    async function fetchActivity() {
      try {
        setActivityLoading(true);
        const tenant = tenantId();
        const res = await fetch(`/api/agents/approvals/activity?tenant_id=${tenant}`, {
          headers: {
            "x-tenant-id": tenant,
            "x-user-id": "default",
          },
        });

        if (res.ok) {
          const data: ApprovalsResponse = await res.json();
          if (mounted && data.pending_approvals) {
            setActivities(data.pending_approvals);
          }
        }
      } catch (err: any) {
        console.error("Failed to load activity", err);
      } finally {
        if (mounted) {
          setActivityLoading(false);
        }
      }
    }

    fetchFeed();
    fetchActivity();
    return () => { mounted = false; };
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    const approval = approvals.find(a => a.id === id);

    // Check if this is a user intervention request (stored as a special status or payload)
    if (approval?.status === "USER_INTERVENTION_REQUIRED" || approval?.payload?.is_intervention) {
        setActiveIntervention({
            taskId: approval.payload?.task_id || id,
            toolCallId: approval.payload?.tool_call_id || "unknown",
            reason: approval.description
        });
        return;
    }

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

  const handleResolveIntervention = async (input: string, type: string) => {
    if (!activeIntervention) return;

    try {
      const res = await fetch("/api/agents/approvals/resolve", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": tenantId(),
        },
        body: JSON.stringify({
          task_id: activeIntervention.taskId,
          tool_call_id: activeIntervention.toolCallId,
          input,
          resolution_type: type
        }),
      });

      if (!res.ok) {
        throw new Error("Failed to send response to agent");
      }

      // Success! Clear the intervention and the approval item
      setApprovals(prev => prev.filter(app =>
        !(app.payload?.task_id === activeIntervention.taskId &&
          app.payload?.tool_call_id === activeIntervention.toolCallId)
      ));
      setActiveIntervention(null);
    } catch (err: any) {
      throw err;
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
      {activeIntervention && (
        <InterventionPanel
          taskId={activeIntervention.taskId}
          toolCallId={activeIntervention.toolCallId}
          reason={activeIntervention.reason}
          onResolve={handleResolveIntervention}
          onClose={() => setActiveIntervention(null)}
        />
      )}
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
              <div className="w-full p-6 glassmorphism rounded-[16px] text-center">
                <div className="text-3xl mb-2">✨</div>
                <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  Your agents are currently monitoring the business.
                </p>
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
                  {approval.payload?.context && (
                    <div className="mt-2 flex flex-col gap-1 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
                      {approval.payload.context.abandoned_carts_count !== undefined && (
                        <div className="flex justify-between items-center text-sm">
                          <span className="text-gray-500 dark:text-gray-400">Abandoned Carts:</span>
                          <span className="font-semibold text-gray-900 dark:text-gray-100">{approval.payload.context.abandoned_carts_count}</span>
                        </div>
                      )}
                      {approval.payload.context.potential_revenue !== undefined && (
                        <div className="flex justify-between items-center text-sm">
                          <span className="text-gray-500 dark:text-gray-400">Potential Revenue:</span>
                          <span className="font-semibold text-green-600 dark:text-green-400">
                            ${Number(approval.payload.context.potential_revenue).toFixed(2)}
                          </span>
                        </div>
                      )}
                    </div>
                  )}
                </div>

                <div className="flex flex-col gap-3 w-full mt-2">
                  <button
                    onClick={() => handleDecision(approval.id, true)}
                    className="w-full min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md"
                    aria-label="Approve proposal"
                  >
                    Approve
                  </button>
                  <div className="flex gap-3 w-full">
                    <button
                      onClick={() => {}}
                      className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                      aria-label="Edit proposal"
                    >
                      Edit
                    </button>
                    <button
                      onClick={() => handleDecision(approval.id, false)}
                      className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                      aria-label="Reject proposal"
                    >
                      Decline
                    </button>
                  </div>
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
