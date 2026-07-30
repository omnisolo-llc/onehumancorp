"use client";

import { useEffect, useState } from "react";
import { WithTooltip } from "../../components/TooltipRegistry";

type TriageItem = {
  id: string;
  tenant_id: string;
  source?: string;
  priority?: string;
  context?: string;
  action_type?: string;
  action_payload?: string;
  status?: string;
  created_at: string;
};

export function AssistantInsightsWidget({ tenant }: { tenant: string }) {
  const [loading, setLoading] = useState(true);
  const [actions, setActions] = useState<TriageItem[]>([]);
  const [approvingId, setApprovingId] = useState<string | null>(null);

  useEffect(() => {
    async function loadInsights() {
      try {
        const res = await fetch(`/api/v1/ui/triage?tenant_id=${encodeURIComponent(tenant)}`);
        if (res.ok) {
          const data = await res.json();
          const items = Array.isArray(data) ? data : (Array.isArray(data?.items) ? data.items : []);

          // Try to find actions that need approval, default to top 3 if none specific
          const pending = items.filter((i: TriageItem) => i.status !== 'resolved' && i.status !== 'dismissed');
          setActions(pending.slice(0, 3));
        }
      } catch (e) {
        console.error("Failed to load assistant insights", e);
      } finally {
        setLoading(false);
      }
    }
    loadInsights();
  }, [tenant]);

  const handleApprove = async (id: string) => {
    setApprovingId(id);
    try {
      const res = await fetch(`/api/v1/ui/triage/action?tenant_id=${encodeURIComponent(tenant)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved: true })
      });

      if (!res.ok) throw new Error("Failed to approve action");

      // Remove approved item from list
      setActions(prev => prev.filter(a => a.id !== id));
    } catch (e) {
      console.error(e);
    } finally {
      setApprovingId(null);
    }
  };

  if (loading) {
    return (
      <div className="p-4 mb-6 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:border-white/10 shadow-sm animate-pulse w-full">
        <div className="h-5 bg-gray-200 dark:bg-gray-700 rounded w-1/3 mb-4"></div>
        <div className="space-y-3">
          <div className="h-16 bg-gray-200 dark:bg-gray-700 rounded-xl w-full"></div>
        </div>
      </div>
    );
  }

  if (actions.length === 0) {
    return null; // Don't show if no insights
  }

  return (
    <section className="mb-6 w-full" data-testid="assistant-insights-widget">
      <div className="flex items-center gap-2 mb-3 px-1">
        <span className="text-xl">✨</span>
        <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
          Assistant Insights
        </h2>
        <span className="text-[10px] font-bold uppercase text-white bg-indigo-500 px-2 py-0.5 rounded-full">AI</span>
      </div>

      <div className="flex flex-col gap-3">
        {actions.map((action) => (
          <div
            key={action.id}
            className="rounded-[16px] p-4 bg-white/70 backdrop-blur-xl border border-white/50 shadow-sm dark:bg-[#1C1C1E]/70 dark:border-white/10 flex flex-col sm:flex-row gap-4 sm:items-center justify-between"
          >
            <div className="flex-1 min-w-0">
              <div className="flex items-center gap-2 mb-1">
                <span className="text-xs uppercase tracking-wider font-semibold text-indigo-600 dark:text-indigo-400">
                  Next Best Action
                </span>
                {action.priority === 'High' && (
                  <span className="w-2 h-2 rounded-full bg-red-500"></span>
                )}
              </div>
              <p className="text-sm font-medium text-gray-900 dark:text-gray-100 leading-snug">
                {action.context || "Automated task suggestion ready for review."}
              </p>
              {action.source && (
                <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  Source: {action.source}
                </p>
              )}
            </div>

            <div className="flex-shrink-0 flex w-full sm:w-auto">
              <button
                onClick={() => handleApprove(action.id)}
                disabled={approvingId === action.id}
                className="w-full sm:w-auto min-h-[44px] px-6 rounded-xl bg-[#0066FF] hover:bg-[#0052CC] text-white text-sm font-semibold shadow-sm transition-all disabled:opacity-70 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                data-testid={`approve-action-${action.id}`}
              >
                {approvingId === action.id ? (
                  <span className="inline-block w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
                ) : (
                  <>
                    <span>Approve & Send</span>
                    <span className="text-lg leading-none">→</span>
                  </>
                )}
              </button>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
