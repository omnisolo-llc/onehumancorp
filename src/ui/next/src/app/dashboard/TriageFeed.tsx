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


      <div className="flex flex-col gap-4" style={{ display: regularItems.length > 0 ? "flex" : "none" }}>
        {error && <div className="app-empty">{error}</div>}
        {!error && regularItems.map((item) => (
          <div
            key={item.id}
            data-testid={`triage-card-${item.id}`}
            className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4 overflow-hidden transition-all cursor-pointer"
            onClick={() => setSelectedId(selectedId === item.id ? null : item.id)}
          >
            <div className="flex flex-col gap-1 pointer-events-none">
              <div className="flex justify-between items-start">
                <span className="text-xs font-bold uppercase tracking-wider text-blue-600 bg-blue-100 dark:bg-blue-900 dark:text-blue-300 px-2 py-1 rounded">
                  {item.source || "Unknown Source"}
                </span>
                <span className={`app-badge ${badgeTone(item.priority)}`}>{item.priority || "Normal"}</span>
              </div>
              <h3 className="text-[17px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit mt-2 leading-tight line-clamp-2">
                {item.context || "No context provided"}
              </h3>
            </div>

            {selectedId === item.id && (
              <div className="mt-2 pt-4 border-t border-gray-200 dark:border-gray-700/50 flex flex-col gap-4" onClick={(e) => e.stopPropagation()}>
                {item.action_type && (
                  <div className="rounded-md border border-blue-200 dark:border-blue-900/30 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-6 text-blue-900 dark:text-blue-100 font-medium">
                    <div className="text-xs uppercase tracking-wider font-semibold text-blue-800 dark:text-blue-300 mb-1">Proposed Action: {item.action_type}</div>
                    {item.action_payload || "No specific payload"}
                  </div>
                )}

                <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
                  <button
                    className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] hover:bg-[#0052CC] text-white font-medium shadow-sm transition-colors flex items-center justify-center"
                    data-testid="approve-btn"
                    onClick={(e) => { e.stopPropagation(); handleDecision(item.id, true); }}
                  >
                    ✨ Approve &amp; Execute
                  </button>
                  <button
                    className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 hover:bg-gray-100 dark:hover:bg-gray-800 font-medium transition-colors flex items-center justify-center"
                    data-testid="dismiss-btn"
                    onClick={(e) => { e.stopPropagation(); handleDecision(item.id, false); }}
                  >
                    Dismiss
                  </button>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
