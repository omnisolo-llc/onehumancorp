
"use client";

import { useEffect, useState } from "react";
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

const getSourceIcon = (source: string) => {
  const s = source.toLowerCase();
  if (s.includes("instagram")) return "📸";
  if (s.includes("email")) return "📧";
  if (s.includes("booking") || s.includes("calendar")) return "📅";
  if (s.includes("payment") || s.includes("stripe")) return "💳";
  if (s.includes("alert") || s.includes("inventory")) return "⚠️";
  return "✉️";
};

export default function TriagePage() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");
  const [processingId, setProcessingId] = useState<string | null>(null);

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
    } catch (e: any) {
      setError(e?.message || "Failed to load triage items");
    } finally {
      setLoading(false);
    }
  }

  const activeCount = items.length;
  const urgentCount = items.filter((item) =>
    ["urgent", "high"].includes((item.priority || "").toLowerCase()),
  ).length;

  async function handleDecision(id: string, approved: boolean) {
    try {
      setProcessingId(id);
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

      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    } finally {
      setProcessingId(null);
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
        <div id="action-status" className="mb-4 app-badge good" role="status">
          {actionStatus}
        </div>
      )}

      <div className="flex flex-col gap-4 w-full max-w-full pb-20">
        {error && <div className="app-empty">{error}</div>}
        {loading ? (
          <div className="p-6 space-y-4">
            <div className="h-20 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-full"></div>
            <div className="h-20 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-full"></div>
          </div>
        ) : !error && items.length === 0 ? (
          <div className="app-empty flex flex-col items-center justify-center py-12" data-testid="triage-feed-empty">
            <div className="text-4xl mb-4">✨</div>
            <div className="text-lg font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">
              All caught up! You're a hero.
            </div>
            <div className="text-sm text-gray-500 mt-2 text-center">
              Your AI assistant has handled all outstanding items. Great job!
            </div>
          </div>
        ) : (
          items.map((item) => {
            const isProcessing = processingId === item.id;

            return (
              <div
                key={item.id}
                data-testid={`triage-card-${item.id}`}
                className="w-full bg-white/60 dark:bg-black/60 backdrop-blur-md border border-white/40 dark:border-white/10 rounded-[24px] shadow-sm flex flex-col mb-4 overflow-hidden"
              >
                {/* Header Context */}
                <div className="p-5 border-b border-gray-200 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-sm">
                  <div className="flex justify-between items-start mb-3">
                    <div className="flex items-center gap-2">
                      <span className="text-xl">{getSourceIcon(item.source || "")}</span>
                      <span className="font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-sm">
                        {item.source || "Unknown Source"}
                      </span>
                    </div>
                    <span className={`app-badge ${badgeTone(item.priority)}`}>
                      {item.priority || "Normal"}
                    </span>
                  </div>
                  <div className="text-[15px] font-medium text-gray-900 dark:text-white leading-snug break-words">
                    {item.context || "No context provided"}
                  </div>
                </div>

                {/* Draft Proposal */}
                {item.action_type && (
                  <div className="p-5 bg-[#0066FF]/10 dark:bg-[#0066FF]/20 backdrop-blur-sm flex flex-col gap-2">
                    <div className="text-[11px] uppercase tracking-wider font-bold text-[#0066FF] dark:text-[#3388FF]">
                      Proposed Action: {item.action_type}
                    </div>
                    <div className="proposed-action rounded-[16px] border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/50 dark:bg-black/30 backdrop-blur-md p-4 text-[13px] leading-relaxed text-gray-900 dark:text-white whitespace-pre-wrap break-words">
                      {item.action_payload || "No specific payload"}
                    </div>
                  </div>
                )}

                {/* Meta Details */}
                <div className="px-5 py-3 flex justify-between bg-white/30 dark:bg-black/30 backdrop-blur-sm text-[11px] text-gray-500 dark:text-gray-400">
                  <span>{new Date(item.created_at || Date.now()).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                  <span>{new Date(item.created_at || Date.now()).toLocaleDateString()}</span>
                </div>

                {/* Action Buttons */}
                <div className="p-5 pt-2 flex flex-col sm:flex-row gap-3 w-full border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-sm">
                  <button
                    disabled={isProcessing}
                    className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                    data-testid="approve-btn"
                    onClick={() => handleDecision(item.id, true)}
                  >
                    {isProcessing ? "Processing..." : "Approve & Execute"}
                  </button>
                  <button
                    disabled={isProcessing}
                    className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-sm text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                    data-testid="dismiss-btn"
                    onClick={() => handleDecision(item.id, false)}
                  >
                    Dismiss
                  </button>
                </div>
              </div>
            );
          })
        )}
      </div>
    </AppShell>
  );
}
