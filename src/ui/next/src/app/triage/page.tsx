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
  action_type?: string;
  action_payload?: string;
  status?: string;
  created_at: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return (
    localStorage.getItem("tenant_id") ||
    localStorage.getItem("tenant") ||
    "default"
  );
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
      const res = await fetch(
        `/api/ui/triage?tenant_id=${encodeURIComponent(tenantId())}`,
      );
      if (!res.ok)
        throw new Error("Failed to load triage items from the database");
      const data = await res.json();
      const rows = Array.isArray(data)
        ? data
        : Array.isArray(data?.items)
          ? data.items
          : [];
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
  const urgentCount = items.filter((item) =>
    ["urgent", "high"].includes((item.priority || "").toLowerCase()),
  ).length;

  async function handleDecision(id: string, approved: boolean) {
    try {
      setActionStatus(approved ? "Approving..." : "Dismissing...");
      const res = await fetch(
        `/api/ui/triage/action?tenant_id=${encodeURIComponent(tenantId())}`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ triage_item_id: id, approved }),
        },
      );
      if (!res.ok) throw new Error("Failed to update action");

      setActionStatus(approved ? "Approved!" : "Dismissed.");

      // Optimistic UI update
      const newItems = items.filter((i) => i.id !== id);
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
      statusItems={[
        {
          label: "Active",
          value: String(activeCount),
          tone: activeCount > 0 ? "warn" : "good",
        },
        {
          label: "Urgent",
          value: String(urgentCount),
          tone: urgentCount > 0 ? "bad" : "neutral",
        },
      ]}
    >
      {actionStatus && (
        <div className="mb-4 app-badge good" role="status">
          {actionStatus}
        </div>
      )}

      <div className="mb-6 p-6 glassmorphism border border-white/40 dark:border-white/10 shadow-sm">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
          Action Center
        </h2>
        <p className="text-gray-600 dark:text-gray-400">
          Review AI-prepared actions and reply drafts across all channels.
        </p>
      </div>

      <div className="flex flex-col lg:flex-row gap-6 w-full max-w-full">
        <section className="flex-[1.5] w-full app-panel glassmorphism shadow-sm flex flex-col">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Triage Queue</div>
            </div>
          </div>
          <div id="triage-list" className="app-list">
            {error && <div className="app-empty">{error}</div>}
            {loading ? (
              <div className="p-6 space-y-4">
                <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-3/4"></div>
                <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-1/2"></div>
                <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-5/6"></div>
              </div>
            ) : !error && items.length === 0 ? (
              <div className="app-empty flex flex-col items-center justify-center py-12">
                <div className="text-4xl mb-4">✨</div>
                <div className="text-lg font-medium text-gray-900 dark:text-white">
                  All caught up!
                </div>
                <div className="text-sm text-gray-500 mt-2">
                  {loading
                    ? "Loading triage items..."
                    : "No triage items need your attention right now. Great job!"}
                </div>
              </div>
            ) : (
              items.map((item) => (
                <button
                  key={item.id}
                  type="button"
                  data-testid={`triage-card-${item.id}`}
                  onClick={() => setSelectedId(item.id)}
                  className="app-list-item w-full text-left min-h-44px rounded-[16px] transition-colors"
                  style={{
                    background:
                      selected?.id === item.id
                        ? "rgba(0, 102, 255, 0.05)"
                        : "transparent",
                  }}
                >
                  <div className="min-w-0">
                    <div className="app-list-title font-inter text-[#1D1D1F] dark:text-[#F5F5F7]">
                      {item.source || "Unknown Source"}
                    </div>
                    <div className="app-list-subtitle truncate font-inter text-gray-600 dark:text-gray-400">
                      {item.context || "No context provided"}
                    </div>
                  </div>
                  <span className={`app-badge ${badgeTone(item.priority)}`}>
                    {item.priority || "Normal"}
                  </span>
                </button>
              ))
            )}
          </div>
        </section>

        <section className="flex-[0.8] w-full app-panel glassmorphism shadow-sm flex flex-col">
          <div className="app-panel-header">
            <div className="app-panel-title">Triage Detail</div>
          </div>
          {loading ? (
            <div className="p-6 space-y-4">
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-3/4"></div>
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-1/2"></div>
              <div className="h-20 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-full"></div>
            </div>
          ) : !selected ? (
            <div className="app-empty">Select a triage item to review it.</div>
          ) : (
            <div className="app-panel-body p-6 flex flex-col gap-6 w-full">
              {/* Action Card Container */}
              <div className="w-full glassmorphism border border-white/40 dark:border-white/10 rounded-[16px] shadow-sm overflow-hidden flex flex-col">
                {/* Header Context */}
                <div className="p-4 border-b border-gray-200 dark:border-white/10 bg-white/30 dark:bg-black/10">
                  <div className="flex justify-between items-start mb-2">
                    <div className="app-metric-label flex items-center gap-2">
                      <span className="text-xl">✨</span>
                      <span className="font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">
                        Agent Context
                      </span>
                    </div>
                    <span
                      className={`app-badge ${badgeTone(selected.priority)}`}
                    >
                      {selected.priority || "Normal"}
                    </span>
                  </div>
                  <div className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] whitespace-pre-wrap break-words">
                    {selected.context || "No context"}
                  </div>
                </div>

                {/* Draft Proposal */}
                {selected.action_type && (
                  <div className="p-4 bg-[#0066FF]/5 dark:bg-[#0066FF]/10 flex flex-col gap-2">
                    <div className="text-xs uppercase tracking-wider font-semibold text-[#0066FF] dark:text-[#3388FF]">
                      Proposed Action: {selected.action_type}
                    </div>
                    <div className="rounded-[12px] border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/80 dark:bg-black/40 p-4 text-sm leading-6 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium whitespace-pre-wrap break-words">
                      {selected.action_payload || "No specific payload"}
                    </div>
                  </div>
                )}

                {/* Meta details */}
                <div className="px-4 py-3 grid grid-cols-2 gap-3 bg-white/20 dark:bg-black/5 text-xs text-gray-500 dark:text-gray-400">
                  <div>
                    <span className="font-semibold text-gray-600 dark:text-gray-300">
                      Source:
                    </span>{" "}
                    {selected.source || "Unknown source"}
                  </div>
                  <div className="text-right">
                    {new Date(
                      selected.created_at || Date.now(),
                    ).toLocaleString()}
                  </div>
                </div>
              </div>

              {/* Action Buttons */}
              <div className="flex flex-col sm:flex-row gap-3 w-full">
                <button
                  className="w-full flex-1 min-h-44px min-w-[44px] px-4 rounded-[16px] font-medium transition-transform active:scale-[0.98] shadow-md flex items-center justify-center cursor-pointer text-white"
                  style={{ background: "#0066FF" }}
                  data-testid="approve-btn"
                  onClick={() => handleDecision(selected.id, true)}
                >
                  ✨ Approve &amp; Execute
                </button>
                <button
                  className="w-full flex-1 min-h-44px min-w-[44px] px-4 rounded-[16px] border border-gray-200 dark:border-white/10 text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 hover:bg-white/80 dark:hover:bg-black/40 font-medium transition-all active:scale-[0.98] flex items-center justify-center cursor-pointer"
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
    </AppShell>
  );
}
