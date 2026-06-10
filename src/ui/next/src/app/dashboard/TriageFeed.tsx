"use client";

import { useEffect, useState } from "react";

type TriageItem = {
  id: string;
  tenant_id: string;
  customer_id?: string;
  source?: string;
  priority?: string;
  context?: string;
  status?: string;
  created_at?: string;
  action_type?: string;
  action_payload?: string;
};

function badgeTone(priority?: string) {
  const normalized = (priority || "").toLowerCase();
  if (["urgent", "high"].includes(normalized)) return "bad";
  if (["action needed", "medium"].includes(normalized)) return "warn";
  if (["fyi", "low"].includes(normalized)) return "good";
  return "neutral";
}

export function TriageFeed({ tenantId }: { tenantId: string }) {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    loadItems();
  }, [tenantId]);

  async function loadItems() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/ui/triage?tenant_id=${encodeURIComponent(tenantId)}`);
      if (!res.ok) throw new Error("Failed to load triage items from the database");
      const data = await res.json();
      const rows = Array.isArray(data) ? data : [];
      setItems(rows);
    } catch (e: any) {
      setError(e?.message || "Failed to load triage items");
    } finally {
      setLoading(false);
    }
  }

  async function handleDecision(id: string, approved: boolean) {
    try {
      setActionStatus(approved ? "Approving..." : "Dismissing...");
      const res = await fetch(`/api/ui/triage/action?tenant_id=${encodeURIComponent(tenantId)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved })
      });
      if (!res.ok) throw new Error("Failed to update action");

      setActionStatus(approved ? "Approved!" : "Dismissed.");

      // Optimistic UI update
      const newItems = items.filter(i => i.id !== id);
      setItems(newItems);

      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    }
  }

  if (loading) {
    return <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 text-gray-500">Loading triage items...</div>;
  }

  if (items.length === 0) {
    return null;
  }

  return (
    <div className="mb-6">
      <div className="mb-4 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Needs Your Attention</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
      </div>

      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

      <div className="flex flex-col gap-4">
        {error && <div className="app-empty">{error}</div>}
        {!error && items.map((item) => (
          <div
            key={item.id}
            data-testid={`triage-card-${item.id}`}
            className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4"
          >
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold font-outfit uppercase tracking-wider text-[#1D1D1F] dark:text-[#F5F5F7]">
                {item.source || "Unknown Source"}
              </span>
              <span className={`app-badge ${badgeTone(item.priority)}`}>
                {item.priority || "Normal"}
              </span>
            </div>

            <div className="rounded-md border border-gray-200 dark:border-white/10 bg-white/50 dark:bg-black/20 p-3 text-sm leading-6 text-[#1D1D1F] dark:text-[#F5F5F7]">
              {item.context || "No context provided"}
            </div>

            {item.action_type && (
              <div>
                <div className="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1">
                  Proposed Action: {item.action_type}
                </div>
                <div className="rounded-md border border-blue-200 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
                  {item.action_payload || "No specific payload"}
                </div>
              </div>
            )}

            <div className="flex flex-col sm:flex-row gap-3 pt-2">
              <button
                className="app-btn-primary flex-1 min-h-[44px]"
                data-testid="approve-btn"
                onClick={() => handleDecision(item.id, true)}
              >
                ✨ Approve &amp; Execute
              </button>
              <button
                className="px-4 py-2 rounded-[8px] border border-gray-300 dark:border-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 hover:bg-white/80 dark:hover:bg-black/40 flex-1 min-h-[44px] font-medium transition-colors"
                data-testid="dismiss-btn"
                onClick={() => handleDecision(item.id, false)}
              >
                Dismiss
              </button>
            </div>

            <div className="text-right text-xs text-gray-500 font-inter">
              {new Date(item.created_at || Date.now()).toLocaleString()}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
