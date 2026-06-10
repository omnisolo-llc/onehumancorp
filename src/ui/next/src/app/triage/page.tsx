"use client";

import { useEffect, useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";

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

export default function TriagePage() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
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
      if (!res.ok) throw new Error("Failed to load triage items from the database");
      const data = await res.json();
      const rows = Array.isArray(data) ? data : [];
      setItems(rows);
      if (!selectedId && rows.length > 0) {
        setSelectedId(rows[0].id);
      }
    } catch (e: any) {
      setError(e?.message || "Failed to load triage items");
    } finally {
      setLoading(false);
    }
  }

  const selected = useMemo(
    () => items.find((item) => item.id === selectedId) || items[0],
    [items, selectedId],
  );

  const activeCount = items.length;
  const urgentCount = items.filter(item => ["urgent", "high"].includes((item.priority || "").toLowerCase())).length;

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

      // Optimistic UI update
      const newItems = items.filter(i => i.id !== id);
      setItems(newItems);
      if (selectedId === id) {
        setSelectedId(newItems.length > 0 ? newItems[0].id : null);
      }

      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    }
  }

  return (
    <AppShell
      title="Work Triage"
      subtitle="AI-prioritized inbox and action center."
    >
      {actionStatus && (
        <div className="fixed top-4 left-1/2 -translate-x-1/2 z-50 mb-4 app-badge good shadow-lg" role="status">
          {actionStatus}
        </div>
      )}

      <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 shadow-sm">
        <h2 className="text-xl sm:text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 tracking-tight">
          Morning Briefing: {urgentCount} urgent messages, {activeCount} active items
        </h2>
        <p className="text-sm sm:text-base text-gray-600 dark:text-gray-400">
          Review AI-prepared actions and reply drafts across all channels.
        </p>
      </div>

      <div className="max-w-3xl mx-auto flex flex-col gap-4 relative">
        {error && <div className="app-empty glassmorphism">{error}</div>}
        {!error && items.length === 0 ? (
          <div className="app-empty glassmorphism py-12">
            {loading ? "Loading triage items..." : "No items need your attention right now. Great job!"}
          </div>
        ) : (
          items.map((item) => (
            <button
              key={item.id}
              type="button"
              data-testid={`triage-card-${item.id}`}
              onClick={() => setSelectedId(item.id)}
              className="w-full text-left p-4 rounded-2xl glassmorphism border border-white/40 dark:border-white/10 transition-all hover:bg-white/40 dark:hover:bg-white/5 active:scale-[0.98] min-h-[44px] shadow-sm flex flex-col sm:flex-row sm:items-center gap-3"
            >
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2 mb-1">
                  <div className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] truncate">{item.source || "Unknown Source"}</div>
                  <span className={`app-badge ${badgeTone(item.priority)} text-xs shrink-0`}>
                    {item.priority || "Normal"}
                  </span>
                </div>
                <div className="text-sm text-gray-600 dark:text-gray-400 truncate">
                  {item.context || "No context provided"}
                </div>
              </div>
              <div className="text-blue-600 dark:text-blue-400 text-sm font-medium shrink-0 flex items-center">
                Review <span className="ml-1 text-lg">›</span>
              </div>
            </button>
          ))
        )}

        {/* Half-sheet Modal */}
        {selected && (
          <div
            className="fixed inset-0 z-40 bg-black/20 dark:bg-black/40 backdrop-blur-sm transition-opacity flex items-end sm:items-center justify-center p-0 sm:p-4"
            onClick={() => setSelectedId(null)}
          >
            <div
              className="w-full max-w-lg bg-white/80 dark:bg-[#16161a]/80 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-t-3xl sm:rounded-3xl p-6 sm:p-8 shadow-2xl transition-transform transform-gpu overflow-y-auto max-h-[85vh]"
              onClick={(e) => e.stopPropagation()}
              data-testid="triage-modal"
            >
              <div className="flex justify-between items-start mb-6">
                <div>
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                      {selected.source || "Unknown source"}
                    </h3>
                    <span className={`app-badge ${badgeTone(selected.priority)} text-xs`}>
                      {selected.priority || "Normal"}
                    </span>
                  </div>
                  <div className="text-sm text-gray-500">
                    {new Date(selected.created_at || Date.now()).toLocaleString()}
                  </div>
                </div>
                <button
                  onClick={() => setSelectedId(null)}
                  className="w-8 h-8 flex items-center justify-center rounded-full bg-black/5 hover:bg-black/10 dark:bg-white/10 dark:hover:bg-white/20 transition-colors"
                  aria-label="Close modal"
                >
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M1 1L13 13M1 13L13 1" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                  </svg>
                </button>
              </div>

              <div className="mb-6">
                <div className="text-xs font-bold tracking-wider text-gray-500 uppercase mb-2">Context</div>
                <div className="rounded-xl border border-black/5 dark:border-white/5 bg-black/5 dark:bg-white/5 p-4 text-sm leading-relaxed text-[#1D1D1F] dark:text-[#F5F5F7]">
                  {selected.context || "No context"}
                </div>
              </div>

              {selected.action_type && (
                <div className="mb-8">
                  <div className="text-xs font-bold tracking-wider text-blue-600 dark:text-blue-400 uppercase mb-2">
                    Proposed Action: {selected.action_type}
                  </div>
                  <div className="rounded-xl border border-blue-200 dark:border-blue-900 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-relaxed text-blue-900 dark:text-blue-100 font-medium">
                    {selected.action_payload || "No specific payload"}
                  </div>
                </div>
              )}

              <div className="flex flex-col gap-3">
                <button
                  className="w-full min-h-[44px] rounded-xl bg-[#0066FF] hover:bg-[#005CE6] text-white font-semibold flex items-center justify-center gap-2 transition-colors shadow-sm"
                  data-testid="approve-btn"
                  onClick={() => handleDecision(selected.id, true)}
                >
                  ✨ Approve &amp; Send
                </button>
                <button
                  className="w-full min-h-[44px] rounded-xl border border-black/10 dark:border-white/10 bg-white/50 hover:bg-white/80 dark:bg-transparent dark:hover:bg-white/5 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-colors"
                  data-testid="dismiss-btn"
                  onClick={() => handleDecision(selected.id, false)}
                >
                  Dismiss
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </AppShell>
  );
}
