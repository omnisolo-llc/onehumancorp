"use client";

import { useEffect, useState } from "react";

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

  const tenantId = () => {
    if (typeof window === "undefined") return "default";
    return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
  };

  useEffect(() => {
    let mounted = true;
    async function fetchFeed() {
      try {
        const tenant = tenantId();
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

    fetchFeed();
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

  if (loading) {
    return (
      <div className="w-full mb-6 p-4 mac-glass-container rounded-[16px] text-center text-gray-500">
        Loading Agent Proposals...
      </div>
    );
  }

  if (error) {
    return (
      <div className="w-full mb-6 p-4 mac-glass-container rounded-[16px] border border-red-500/50 bg-red-500/10 text-red-500 text-center">
        {error}
      </div>
    );
  }

  if (approvals.length === 0) {
    return (
      <div className="w-full mb-6 p-6 mac-glass-container rounded-[16px] text-center">
        <div className="text-3xl mb-2">✨</div>
        <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
        <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
          Your agents are currently monitoring the business.
        </p>
      </div>
    );
  }

  return (
    <section className="mb-6 w-full" aria-label="Unified Agent Feed">
      <div className="mb-3 flex items-center justify-between">
        <h2 className="app-panel-title text-xl font-bold font-outfit">Agent Proposals</h2>
        <span className="app-badge">{approvals.length} Urgent</span>
      </div>

      <div className="flex flex-col gap-4">
        {approvals.map((approval) => (
          <div
            key={approval.id}
            className="mac-glass-container p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4"
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
            </div>

            <div className="flex gap-3 w-full mt-2">
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                aria-label="Reject proposal"
              >
                Dismiss
              </button>
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md"
                aria-label="Approve proposal"
              >
                Approve
              </button>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
