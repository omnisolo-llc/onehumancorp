"use client";

import { useEffect, useState } from "react";

type TriageItem = {
  id: string;
  tenant_id: string;
  source?: string;
  priority?: string;
  context?: string;
  status?: string;
  action_type?: string;
  action_payload?: string;
};

export function ProactiveInsightsCard({ tenantId }: { tenantId: string }) {
  const [insight, setInsight] = useState<TriageItem | null>(null);
  const [loading, setLoading] = useState(true);
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    loadInsight();
  }, [tenantId]);

  async function loadInsight() {
    setLoading(true);
    try {
      const res = await fetch(`/api/ui/triage?tenant_id=${encodeURIComponent(tenantId)}`);
      if (res.ok) {
        const data = await res.json();
        const rows = Array.isArray(data) ? data : [];
        const proactiveInsight = rows.find(r => r.source === 'Proactive Context Agent');
        setInsight(proactiveInsight || null);
      }
    } catch (e) {
      console.error("Failed to load proactive insight", e);
    } finally {
      setLoading(false);
    }
  }

  async function handleDecision(id: string, approved: boolean) {
    try {
      setActionStatus(approved ? "Executing..." : "Dismissing...");
      const res = await fetch(`/api/ui/triage/action?tenant_id=${encodeURIComponent(tenantId)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved })
      });
      if (!res.ok) throw new Error("Failed to update action");

      setActionStatus(approved ? "Executed!" : "Dismissed.");
      setTimeout(() => {
        setActionStatus("");
        setInsight(null);
      }, 2000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    }
  }

  if (loading) return null; // Wait silently
  if (!insight) return null; // Don't show if there's no proactive insight

  return (
    <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10" style={{ background: "rgba(0, 102, 255, 0.1)" }}>
      <div className="flex justify-between items-start mb-2">
        <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] flex items-center">
          <span className="mr-2 text-2xl">✨</span> Needs Attention Today
        </h2>
        {actionStatus && <span className="app-badge good">{actionStatus}</span>}
      </div>

      <p className="text-sm text-gray-700 dark:text-gray-300 mb-4 font-inter">
        {insight.context || "You have tasks needing attention."}
      </p>

      {insight.action_type && (
        <div className="mb-4 p-4 rounded-md border border-blue-200 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
          {insight.action_payload}
        </div>
      )}

      <div className="flex flex-col sm:flex-row gap-3">
        <button
          className="app-btn-primary flex-1 min-h-[44px]"
          data-testid="approve-insight-btn"
          onClick={() => handleDecision(insight.id, true)}
          disabled={!!actionStatus}
        >
          {insight.action_type === 'Draft Approval' ? 'Approve Draft' : 'Approve Action'}
        </button>
        <button
          className="px-4 py-2 rounded-[16px] border border-white/40 dark:border-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 hover:bg-white/80 dark:hover:bg-black/40 flex-1 min-h-[44px] font-medium transition-colors backdrop-blur-md"
          data-testid="dismiss-insight-btn"
          onClick={() => handleDecision(insight.id, false)}
          disabled={!!actionStatus}
        >
          Dismiss
        </button>
      </div>
    </div>
  );
}
