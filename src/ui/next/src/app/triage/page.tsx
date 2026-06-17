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

      <div className="flex flex-col gap-6 w-full max-w-full pb-8">
        {error && <div className="app-empty">{error}</div>}
        {loading ? (
          <div className="space-y-4">
            <div className="glassmorphism p-6 rounded-[16px] shadow-sm animate-pulse h-48"></div>
            <div className="glassmorphism p-6 rounded-[16px] shadow-sm animate-pulse h-48"></div>
            <div className="glassmorphism p-6 rounded-[16px] shadow-sm animate-pulse h-48"></div>
          </div>
        ) : !error && items.length === 0 ? (
          <div className="app-empty flex flex-col items-center justify-center py-12 glassmorphism rounded-[16px] shadow-sm">
            <div className="text-4xl mb-4">✨</div>
            <div className="text-lg font-medium text-gray-900 dark:text-white">
              All caught up!
            </div>
            <div className="text-sm text-gray-500 mt-2 text-center max-w-xs">
              {loading
                ? "Loading triage items..."
                : "No triage items need your attention right now. Great job!"}
            </div>
          </div>
        ) : (
          <div className="flex flex-col gap-4">
            {items.map((item) => (
              <div
                key={item.id}
                data-testid={`triage-card-${item.id}`}
                className="w-full glassmorphism border border-white/40 dark:border-white/10 rounded-[16px] shadow-sm overflow-hidden flex flex-col transition-all"
              >
                {/* Header Context */}
                <div className="p-4 border-b border-gray-200 dark:border-white/10 bg-white/30 dark:bg-black/10">
                  <div className="flex justify-between items-start mb-2">
                    <div className="app-metric-label flex items-center gap-2">
                      <span className="text-xl">✨</span>
                      <span className="font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">
                        {item.source || "Unknown Source"}
                      </span>
                    </div>
                    <span className={`app-badge ${badgeTone(item.priority)}`}>
                      {item.priority || "Normal"}
                    </span>
                  </div>
                  <div className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7] whitespace-pre-wrap break-words mt-1">
                    {item.context || "No context"}
                  </div>
                </div>

                {/* Draft Proposal */}
                {item.action_type && (
                  <div className="p-4 bg-[#0066FF]/5 dark:bg-[#0066FF]/10 flex flex-col gap-2">
                    <div className="text-xs uppercase tracking-wider font-semibold text-[#0066FF] dark:text-[#3388FF]">
                      Proposed Action: {item.action_type}
                    </div>
                    <div className="rounded-[12px] border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/80 dark:bg-black/40 p-4 text-sm leading-6 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium whitespace-pre-wrap break-words">
                      {item.action_payload || "No specific payload"}
                    </div>
                  </div>
                )}

                {/* Action Buttons */}
                <div className="p-4 bg-white/20 dark:bg-black/5 flex flex-col sm:flex-row gap-3 w-full border-t border-gray-100 dark:border-white/5">
                  <button
                    className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[12px] font-medium transition-transform active:scale-[0.98] shadow-md flex items-center justify-center cursor-pointer text-white"
                    style={{ background: "#0066FF" }}
                    data-testid="approve-btn"
                    onClick={() => handleDecision(item.id, true)}
                  >
                    ✨ Approve
                  </button>
                  <button
                    className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[12px] border border-gray-200 dark:border-white/10 text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 hover:bg-white/80 dark:hover:bg-black/40 font-medium transition-all active:scale-[0.98] flex items-center justify-center cursor-pointer"
                    data-testid="dismiss-btn"
                    onClick={() => handleDecision(item.id, false)}
                  >
                    Dismiss
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </AppShell>
  );
}
