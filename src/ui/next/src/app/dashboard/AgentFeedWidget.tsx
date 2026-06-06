"use client";
import React, { useEffect, useState } from "react";

export interface ActionCardData {
  id: string;
  tenant_id: string;
  agent_id: string;
  action_type: string;
  payload: any;
  status: string;
}

export interface AgentFeedResponse {
  pending_actions: ActionCardData[];
}

function getTenantId() {
  if (typeof window !== "undefined") {
    return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "storefront";
  }
  return "storefront";
}

export function AgentFeedWidget() {
  const [actions, setActions] = useState<ActionCardData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let mounted = true;

    async function fetchFeed() {
      try {
        setLoading(true);
        const tenant = getTenantId();
        const res = await fetch(`/api/agent-feed?tenant_id=${tenant}`, {
          headers: {
            "x-tenant-id": tenant,
            "x-user-id": "default",
          },
        });

        if (res.ok) {
          const data: AgentFeedResponse = await res.json();
          if (mounted && data.pending_actions) {
            setActions(data.pending_actions);
          }
        } else {
            if (mounted) setError("Failed to fetch feed");
        }
      } catch (err: any) {
        if (mounted) setError(err.message || "Failed to load feed");
      } finally {
        if (mounted) setLoading(false);
      }
    }

    fetchFeed();
    return () => { mounted = false; };
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    // Optimistic UI update
    setActions(prev => prev.filter(app => app.id !== id));
    // Actually the endpoint to decide on an action hasn't been explicitly requested,
    // but the mobile feed UI should have prominent 1-tap "Approve" or "Dismiss" buttons.
    // For now we just dismiss it from the UI.
  };

  if (error) {
    return (
      <div className="w-full mb-6 p-4 glassmorphism rounded-[16px] border border-red-500/50 bg-red-500/10 text-red-500 text-center">
        {error}
      </div>
    );
  }

  return (
    <section className="mb-6 w-full" aria-label="Proactive Agent Feed">
      <div className="mb-4 flex items-center border-b border-gray-200 dark:border-gray-700">
        <h2 className="py-3 text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
          Agent Feed
        </h2>
      </div>

      <div className="flex flex-col gap-4">
        {loading && (
          <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
            Loading Agent Feed...
          </div>
        )}
        {!loading && actions.length === 0 && (
          <div className="w-full p-6 glassmorphism rounded-[16px] text-center">
            <div className="text-3xl mb-2">✨</div>
            <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
            <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Your agents are currently monitoring the business.
            </p>
          </div>
        )}
        {actions.map((action) => (
          <div
            key={action.id}
            className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4"
          >
            <div className="flex flex-col gap-1">
              <div className="flex items-center gap-2">
                <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                  {action.agent_id}
                </span>
                <span className="text-xs font-bold uppercase tracking-wider text-blue-600 bg-blue-50 px-2 py-1 rounded-md">
                  {action.action_type.replace('_', ' ')}
                </span>
              </div>
              <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1">
                {action.payload?.description || action.action_type}
              </h3>
            </div>

            <div className="flex gap-3 w-full mt-2">
              <button
                onClick={() => handleDecision(action.id, true)}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-md"
                aria-label="Approve action"
              >
                Approve
              </button>
              <button
                onClick={() => handleDecision(action.id, false)}
                className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
                aria-label="Dismiss action"
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
