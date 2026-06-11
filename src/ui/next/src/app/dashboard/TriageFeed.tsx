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

export function TriageFeed({ tenantId, initialItems }: { tenantId: string, initialItems?: TriageItem[] }) {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    if (initialItems && initialItems.length > 0) {
      setItems(initialItems);
      if (!selectedId) setSelectedId(initialItems[0].id);
      setLoading(false);
    } else {
      loadItems();
    }
  }, [tenantId, initialItems]);

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

  const proactiveItems = items.filter(item => item.source === "Proactive Context Agent");
  const regularItems = items.filter(item => item.source !== "Proactive Context Agent");

  return (
    <div className="mb-6">
      {proactiveItems.map((item) => (
        <div key={item.id} className="mb-6 p-6 rounded-[16px] glassmorphism border border-orange-400/50 dark:border-orange-500/30 bg-orange-50/50 dark:bg-orange-900/10 shadow-lg relative overflow-hidden">
          <div className="absolute top-0 left-0 w-1 h-full bg-orange-500"></div>
          <div className="flex justify-between items-start mb-3">
            <div>
              <h2 className="text-xl font-bold font-outfit text-orange-900 dark:text-orange-100 flex items-center gap-2">
                <span className="text-2xl">✨</span> Needs Attention Today
              </h2>
              <p className="text-orange-800/80 dark:text-orange-200/80 mt-1 text-sm font-medium">{item.context}</p>
            </div>
            <span className={`app-badge ${badgeTone(item.priority)}`}>{item.priority || "High"}</span>
          </div>

          {item.action_type && (
            <div className="mt-4 mb-5 p-4 rounded-xl bg-white/60 dark:bg-black/40 border border-orange-200 dark:border-orange-900/50">
              <div className="text-xs uppercase tracking-wider font-semibold text-orange-800 dark:text-orange-300 mb-1">Suggested Action: {item.action_type}</div>
              <div className="text-sm font-medium text-gray-900 dark:text-gray-100">{item.action_payload}</div>
            </div>
          )}

          <div className="flex gap-3 mt-2">
            <button
              onClick={() => handleDecision(item.id, true)}
              className="px-6 py-2.5 rounded-[16px] bg-orange-500 hover:bg-orange-600 text-white font-medium shadow-sm transition-colors"
            >
              Approve & Execute
            </button>
            <button
              onClick={() => handleDecision(item.id, false)}
              className="px-6 py-2.5 rounded-[16px] bg-white/50 dark:bg-black/30 border border-orange-200 dark:border-orange-900/30 hover:bg-white/80 dark:hover:bg-black/50 text-orange-900 dark:text-orange-100 font-medium transition-colors"
            >
              Dismiss
            </button>
          </div>
        </div>
      ))}

      <div className="mb-4 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Unified Agent Feed</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
      </div>

      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

      <div className="app-grid two" style={{ display: regularItems.length > 0 ? "grid" : "none" }}>
        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Triage Queue</div>
            </div>
          </div>
          <div id="triage-list" className="app-list">
            {error && <div className="app-empty">{error}</div>}
            {!error && regularItems.map((item) => (
              <button
                key={item.id}
                type="button"
                data-testid={`triage-card-${item.id}`}
                onClick={() => setSelectedId(item.id)}
                className="app-list-item w-full text-left min-h-[44px]"
                style={{ background: selected?.id === item.id ? "rgba(255, 255, 255, 0.1)" : "transparent" }}
              >
                <div className="min-w-0">
                  <div className="app-list-title">{item.source || "Unknown Source"}</div>
                  <div className="app-list-subtitle truncate">{item.context || "No context provided"}</div>
                </div>
                <span className={`app-badge ${badgeTone(item.priority)}`}>{item.priority || "Normal"}</span>
              </button>
            ))}
          </div>
        </section>

        <section className="app-panel glassmorphism border border-white/40 dark:border-white/10">
          <div className="app-panel-header">
            <div className="app-panel-title">Triage Detail</div>
          </div>
          {!selected ? (
            <div className="app-empty">Select a triage item to review it.</div>
          ) : (
            <div className="app-panel-body">
              <div className="mb-4">
                <div className="app-metric-label">Source</div>
                <div className="mt-1 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">{selected.source || "Unknown source"}</div>
              </div>
              <div className="mb-4">
                <div className="app-metric-label">Context</div>
                <div className="mt-2 rounded-md border border-gray-200 dark:border-white/10 bg-white/50 dark:bg-black/20 p-3 text-sm leading-6 text-[#1D1D1F] dark:text-[#F5F5F7]">
                  {selected.context || "No context"}
                </div>
              </div>
              {selected.action_type && (
                <div className="mb-6">
                  <div className="app-metric-label">Proposed Action: {selected.action_type}</div>
                  <div className="mt-2 rounded-md border border-blue-200 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
                    {selected.action_payload || "No specific payload"}
                  </div>
                </div>
              )}

              <div className="grid grid-cols-2 gap-3 mb-6">
                <div className="app-card bg-white/30 dark:bg-black/30 border border-white/40 dark:border-white/10">
                  <div className="app-metric-label">Priority</div>
                  <div className="mt-2"><span className={`app-badge ${badgeTone(selected.priority)}`}>{selected.priority || "Normal"}</span></div>
                </div>
                <div className="app-card bg-white/30 dark:bg-black/30 border border-white/40 dark:border-white/10">
                  <div className="app-metric-label">Created</div>
                  <div className="mt-2 text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">{new Date(selected.created_at || Date.now()).toLocaleString()}</div>
                </div>
              </div>

              <div className="flex flex-col sm:flex-row gap-3">
                <button
                  className="app-btn-primary flex-1 min-h-[44px]"
                  data-testid="approve-btn"
                  onClick={() => handleDecision(selected.id, true)}
                >
                  ✨ Approve &amp; Execute
                </button>
                <button
                  className="px-4 py-2 rounded-[16px] border border-white/40 dark:border-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 hover:bg-white/80 dark:hover:bg-black/40 flex-1 min-h-[44px] font-medium transition-colors backdrop-blur-md"
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
