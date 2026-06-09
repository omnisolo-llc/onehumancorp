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

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function badgeTone(priority?: string) {
  const normalized = (priority || "").toLowerCase();
  if (["urgent", "high"].includes(normalized)) return "bad";
  if (["action needed", "medium"].includes(normalized)) return "warn";
  if (["fyi", "low"].includes(normalized)) return "good";
  return "neutral";
}

export function TriageFeed() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    loadItems();
  }, []);

  async function loadItems() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/ui/triage?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load triage items");
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
      const res = await fetch(`/api/ui/triage/action?tenant_id=${encodeURIComponent(tenantId())}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved })
      });
      if (!res.ok) throw new Error("Failed to update action");

      setActionStatus(approved ? "Approved!" : "Dismissed.");
      setItems(prev => prev.filter(i => i.id !== id));
      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    }
  }

  if (!loading && !error && items.length === 0) {
    return (
      <section className="mb-6 w-full" aria-label="Triage Feed">
        <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
          <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Needs Your Attention</h2>
          <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
        </div>
        <div className="app-empty p-6 glassmorphism rounded-[16px] border border-white/40 dark:border-white/10 text-center text-gray-500">
          No items need your attention right now. Great job!
        </div>
      </section>
    );
  }

  return (
    <section className="mb-6 w-full" aria-label="Triage Feed">
      <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Needs Your Attention</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
        {actionStatus && <div className="mt-4 app-badge good" role="status">{actionStatus}</div>}
      </div>

      <div className="flex flex-col gap-4">
        {error && <div className="app-empty">{error}</div>}
        {items.map((item) => (
          <div
            key={item.id}
            data-testid={`triage-card-${item.id}`}
            className="glassmorphism rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm overflow-hidden"
          >
            <div className="w-full p-5 text-left flex flex-col gap-2">
              <div className="flex justify-between items-start w-full">
                <span className="text-xs font-bold uppercase tracking-wider text-[#0066FF] bg-[#0066FF]/10 px-2 py-1 rounded">
                  {item.source || "Unknown Source"}
                </span>
                <span className={`app-badge ${badgeTone(item.priority)}`}>{item.priority || "Normal"}</span>
              </div>
              <h3 className="text-[17px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit leading-tight mt-1">
                {item.context || "No context provided"}
              </h3>
            </div>

            <div className="p-5 pt-0 border-t border-white/10 dark:border-gray-700/50 mt-2 bg-gray-50/50 dark:bg-gray-800/30">
              <div className="mt-4 mb-4">
                <div className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2">
                  Proposed Action: {item.action_type || "Draft Reply"}
                </div>
                <div className="rounded-md border border-blue-200 dark:border-blue-900 bg-blue-50/80 dark:bg-blue-900/20 p-4 text-sm leading-6 text-blue-900 dark:text-blue-200 font-medium whitespace-pre-wrap">
                  {item.action_payload || "No specific payload"}
                </div>
              </div>

              <div className="flex flex-col sm:flex-row gap-3 mt-4">
                <button
                  className="flex-1 min-h-[44px] rounded-lg font-bold text-sm bg-[#0066FF] hover:bg-[#0052CC] text-white shadow-sm transition-transform active:scale-[0.98]"
                  data-testid="approve-btn"
                  onClick={() => handleDecision(item.id, true)}
                >
                  ✨ Approve &amp; Execute
                </button>
                <button
                  className="flex-1 min-h-[44px] rounded-lg font-bold text-sm bg-gray-200 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-700 text-[#1D1D1F] dark:text-[#F5F5F7] transition-transform active:scale-[0.98]"
                  data-testid="dismiss-btn"
                  onClick={() => handleDecision(item.id, false)}
                >
                  Dismiss
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
