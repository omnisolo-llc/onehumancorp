"use client";

import { useEffect, useMemo, useState } from "react";

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
  const [selectedId, setSelectedId] = useState<string | null>(null);
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
      if (selectedId === id) {
        setSelectedId(newItems.length > 0 ? newItems[0].id : null);
      }

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
      <div className="mb-4 p-6 rounded-[16px] glassmorphism bg-white/20 dark:bg-black/20 backdrop-blur-md border border-white/40 dark:border-white/10 shadow-sm">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Needs Your Attention</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
      </div>

      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

      <div className="app-grid two">
        <section className="app-panel glassmorphism bg-white/20 dark:bg-black/20 backdrop-blur-md border border-white/40 dark:border-white/10 shadow-sm rounded-[16px]">
          <div className="app-panel-header border-b border-white/20 dark:border-white/5 pb-4 mb-4 px-6 pt-6">
            <div>
              <div className="app-panel-title text-[#1D1D1F] dark:text-[#F5F5F7]">Triage Queue</div>
            </div>
          </div>
          <div id="triage-list" className="app-list px-6 pb-6">
            {error && <div className="app-empty text-red-500">{error}</div>}
            {!error && items.map((item) => (
              <button
                key={item.id}
                type="button"
                data-testid={`triage-card-${item.id}`}
                onClick={() => setSelectedId(item.id)}
                className="app-list-item w-full text-left min-h-[44px] rounded-lg p-3 transition-colors duration-200"
                style={{ background: selected?.id === item.id ? "rgba(0, 102, 255, 0.1)" : "transparent" }}
              >
                <div className="min-w-0">
                  <div className="app-list-title text-[#1D1D1F] dark:text-[#F5F5F7]">{item.source || "Unknown Source"}</div>
                  <div className="app-list-subtitle truncate text-gray-500 dark:text-gray-400">{item.context || "No context provided"}</div>
                </div>
                <span className={`app-badge ${badgeTone(item.priority)}`}>{item.priority || "Normal"}</span>
              </button>
            ))}
          </div>
        </section>

        <section className="app-panel glassmorphism bg-white/20 dark:bg-black/20 backdrop-blur-md border border-white/40 dark:border-white/10 shadow-sm rounded-[16px]">
          <div className="app-panel-header border-b border-white/20 dark:border-white/5 pb-4 mb-4 px-6 pt-6">
            <div className="app-panel-title text-[#1D1D1F] dark:text-[#F5F5F7]">Triage Detail</div>
          </div>
          {!selected ? (
            <div className="app-empty px-6 pb-6 text-gray-500">Select a triage item to review it.</div>
          ) : (
            <div className="app-panel-body px-6 pb-6">
              <div className="mb-4">
                <div className="app-metric-label">Source</div>
                <div className="mt-1 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">{selected.source || "Unknown source"}</div>
              </div>
              <div className="mb-4">
                <div className="app-metric-label">Context</div>
                <div className="mt-2 rounded-md border border-white/20 dark:border-white/10 bg-white/50 dark:bg-black/20 p-3 text-sm leading-6 text-[#1D1D1F] dark:text-[#F5F5F7]">
                  {selected.context || "No context"}
                </div>
              </div>
              {selected.action_type && (
                <div className="mb-6">
                  <div className="app-metric-label">Proposed Action: {selected.action_type}</div>
                  <div className="mt-2 rounded-md border border-blue-200/50 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
                    {selected.action_payload || "No specific payload"}
                  </div>
                </div>
              )}

              <div className="grid grid-cols-2 gap-3 mb-6">
                <div className="app-card bg-white/30 dark:bg-black/30 border border-white/40 dark:border-white/10 rounded-xl p-4">
                  <div className="app-metric-label">Priority</div>
                  <div className="mt-2"><span className={`app-badge ${badgeTone(selected.priority)}`}>{selected.priority || "Normal"}</span></div>
                </div>
                <div className="app-card bg-white/30 dark:bg-black/30 border border-white/40 dark:border-white/10 rounded-xl p-4">
                  <div className="app-metric-label">Created</div>
                  <div className="mt-2 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">{new Date(selected.created_at || Date.now()).toLocaleString()}</div>
                </div>
              </div>

              <div className="flex flex-col sm:flex-row gap-3">
                <button
                  className="w-full min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center flex-1"
                  data-testid="approve-btn"
                  onClick={() => handleDecision(selected.id, true)}
                >
                  ✨ Approve &amp; Execute
                </button>
                <button
                  className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                  data-testid="dismiss-btn"
                  onClick={() => handleDecision(selected.id, false)}
                >
                  Dismiss
                </button>
              </div>
            </div>
          )}
        </section>
      </div>
    </div>
  );
}
