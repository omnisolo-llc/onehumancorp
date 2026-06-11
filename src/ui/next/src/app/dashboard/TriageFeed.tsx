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

  return (
    <div className="mb-6">
      <div className="mb-4 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Unified Agent Feed</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
      </div>

      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

      <div className="flex flex-col gap-4">
        {items.map((item) => (
          <div key={item.id} data-testid={`triage-card-${item.id}`} className="p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 bg-white/60 dark:bg-black/40 shadow-sm relative overflow-hidden">
            <div className={`absolute top-0 left-0 w-1 h-full ${item.priority?.toLowerCase() === 'urgent' ? 'bg-red-500' : 'bg-[#0066FF]'}`}></div>
            <div className="flex justify-between items-start mb-4">
              <div className="flex flex-col">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-xs uppercase tracking-wider font-bold text-gray-500 dark:text-gray-400">
                    Drafted by {item.source || "Agent"}
                  </span>
                  <span className={`app-badge ${badgeTone(item.priority)}`}>{item.priority || "Normal"}</span>
                </div>
              </div>
            </div>

            <div className="mb-4">
              <div className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Context</div>
              <p className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] leading-relaxed">
                {item.context || "No context provided"}
              </p>
            </div>

            {item.action_type && (
              <div className="mb-5 p-4 rounded-xl bg-white/80 dark:bg-black/60 border border-gray-100 dark:border-gray-800">
                <div className="text-xs uppercase tracking-wider font-semibold text-blue-800 dark:text-blue-300 mb-1">Proposed Action: {item.action_type}</div>
                <div className="text-sm font-medium text-gray-900 dark:text-gray-100">{item.action_payload}</div>
              </div>
            )}

            <div className="flex flex-col sm:flex-row gap-3">
              <button
                onClick={() => handleDecision(item.id, true)}
                data-testid="approve-btn"
                className="flex-1 py-3 px-4 rounded-[12px] bg-[#0066FF] hover:bg-[#0052CC] text-white font-semibold text-sm shadow-sm active:scale-[0.98] transition-all min-h-[44px]"
              >
                ✨ Approve & Execute
              </button>
              <button
                onClick={() => handleDecision(item.id, false)}
                data-testid="dismiss-btn"
                className="flex-1 py-3 px-4 rounded-[12px] bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-700 dark:text-gray-200 font-semibold text-sm hover:bg-gray-50 dark:hover:bg-gray-700 active:scale-[0.98] transition-all min-h-[44px]"
              >
                Dismiss
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
