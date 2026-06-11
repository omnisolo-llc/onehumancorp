"use client";

import { useEffect, useState } from "react";

type AgentFeedItem = {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload: any;
  proposed_action: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
};

export function ProactiveInsightsFeed() {
  const [items, setItems] = useState<AgentFeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  const tenantId = () => {
    if (typeof window === "undefined") return "default";
    return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
  };

  useEffect(() => {
    let mounted = true;

    async function fetchInsights() {
      try {
        setLoading(true);
        const tenant = tenantId();
        const res = await fetch(`/api/agent-feed?tenant_id=${tenant}`, {
          headers: {
            "x-tenant-id": tenant,
            "x-user-id": "default",
          },
        });

        if (!res.ok) {
          throw new Error("Failed to load agent feed");
        }

        const data = await res.json();
        if (mounted && data?.items) {
          setItems(
            data.items.filter(
              (i: any) =>
                i.event_source === "Proactive Context Agent" &&
                i.lifecycle_state === "PENDING_APPROVAL"
            )
          );
        }
      } catch (err: any) {
        if (mounted) {
          setError(err.message || "Failed to load proactive insights");
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    }

    fetchInsights();

    return () => {
      mounted = false;
    };
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    setItems((prev) => prev.filter((app) => app.id !== id));

    const tenant = tenantId();
    try {
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
        throw new Error("Failed to submit decision");
      }
    } catch (err: any) {
      console.error(err);
      // Fallback
    }
  };

  if (loading) return null;
  if (items.length === 0) return null;

  return (
    <section className="mb-6 w-full mx-auto" aria-label="Proactive Insights Feed">
      <div className="flex flex-col gap-3 min-w-[320px] max-w-full">
        {items.map((approval) => (
          <div
            key={approval.id}
            className="glassmorphism p-5 rounded-[16px] border border-blue-400 dark:border-blue-500/30 shadow-md bg-blue-50/80 dark:bg-blue-900/10 flex flex-col gap-3 opacity-95 min-h-[44px]"
            data-testid={`proactive-insight-${approval.id}`}
          >
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold font-outfit uppercase tracking-wider text-blue-600 dark:text-blue-400 bg-blue-100 dark:bg-blue-900/40 px-2 py-1 rounded-md flex items-center gap-1">
                <span>⚡</span> Needs Attention Today
              </span>
            </div>
            <h3 className="text-md font-semibold font-inter text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
              {approval.proposed_action?.payload?.message || "Operational anomaly detected. Please review."}
            </h3>

            <div className="text-sm text-gray-700 dark:text-gray-300 mt-1 mb-2">
               Action: <span className="font-semibold">{approval.proposed_action?.action_type || "Review"}</span>
            </div>

            <div className="flex flex-col sm:flex-row gap-3 w-full">
              <button
                onClick={() => handleDecision(approval.id, true)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-blue-600 text-white font-medium hover:bg-blue-700 transition-all duration-200 shadow-sm flex items-center justify-center"
                aria-label="Approve Insight Action"
                data-testid="approve-insight"
              >
                Approve
              </button>
              <button
                onClick={() => handleDecision(approval.id, false)}
                className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/50 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center bg-transparent"
                aria-label="Dismiss Insight"
                data-testid="dismiss-insight"
              >
                Dismiss
              </button>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
