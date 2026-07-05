
"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";
import { SyncManager } from "../../lib/sync/SyncManager";
import { getActions } from "../utils/offlineQueue";


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
  const urlParams = new URLSearchParams(window.location.search);
  const urlTenant = urlParams.get("tenant_id");
  if (urlTenant) return urlTenant;
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
  const [isOffline, setIsOffline] = useState(false);
  const [offlineActionsCount, setOfflineActionsCount] = useState(0);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editValue, setEditValue] = useState<string>("");
  const [selectedItemId, setSelectedItemId] = useState<string | null>(null);


  useEffect(() => {
    loadItems();

    const updateOfflineCount = async () => {
      try {
        const actions = await getActions();
        setOfflineActionsCount(actions.length);
      } catch (err) {
        console.warn("Failed to fetch offline actions count:", err);
      }
    };
    updateOfflineCount();

    setIsOffline(!navigator.onLine);

    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    const handleQueueUpdated = () => updateOfflineCount();
    window.addEventListener('ohc_queue_updated', handleQueueUpdated);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
      window.removeEventListener('ohc_queue_updated', handleQueueUpdated);
    };
  }, []);


  async function loadItems() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(
        `/api/triage/pending?tenant_id=${encodeURIComponent(tenantId())}`,
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

  async function handleDecision(id: string, approved: boolean, edited_payload?: string) {
    if (isOffline) {
      await SyncManager.getInstance().enqueue({
        id: crypto.randomUUID ? crypto.randomUUID() : Date.now().toString(),
        type: 'triage_action',
        payload: { triage_item_id: id, approved, edited_payload },
        timestamp: Date.now()
      });
      const newItems = items.filter((i) => i.id !== id);
      setItems(newItems);
      setActionStatus(approved ? "Approved offline." : "Dismissed offline.");
      setTimeout(() => setActionStatus(""), 3000);
      return;
    }

    try {
      setProcessingId(id);
      setActionStatus(approved ? "Approving..." : "Dismissing...");
      const res = await fetch(
        `/api/triage/action?tenant_id=${encodeURIComponent(tenantId())}`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ triage_item_id: id, approved, edited_payload }),
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
      setEditingId(null);
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
      {isOffline && (
        <div className="mb-4 w-full p-2 glassmorphism rounded-[8px] bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
          <span>📡</span> You are offline. Actions will sync when online.
        </div>
      )}
      {offlineActionsCount > 0 && (
        <div className="mb-4 w-full p-2 glassmorphism rounded-[8px] bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
          <span>🔄</span> Pending Sync ({offlineActionsCount})
        </div>
      )}
      {actionStatus && (
        <div id="action-status" className="mb-4 app-badge good" role="status">
          {actionStatus}
        </div>
      )}

      <div className="flex flex-col gap-4 w-full max-w-full pb-20">
        <div className="flex justify-end px-1">
          <button
            onClick={async () => {
              setLoading(true);
              try {
                await fetch(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId())}`, {
                  method: "POST",
                  headers: { "Content-Type": "application/json" },
                  body: JSON.stringify({
                    source: "SMS (Missed Call)",
                    priority: "urgent",
                    context: "Leaky pipe under sink, can you fix?",
                    action_type: "Draft Reply",
                    action_payload: "Hi! I am currently on a job but can fix this today. Can you send a photo of the leak?",
                    customer_id: "Unknown Caller"
                  }),
                });
                await loadItems();
              } catch (err) {
                console.error("Failed to simulate missed lead", err);
              } finally {
                setLoading(false);
              }
            }}
            data-testid="simulate-missed-lead-btn"
            className="text-xs bg-[#0066FF]/10 text-[#0066FF] dark:text-[#3388FF] px-3 py-1.5 rounded-full font-medium hover:bg-[#0066FF]/20 transition-colors flex items-center gap-1 shadow-sm border border-[#0066FF]/20 min-h-[44px] min-w-[44px]"
          >
            <span>📱</span> Simulate Missed Call
          </button>
        </div>

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
            const isSelected = selectedItemId === item.id;

            return (
              <div
                key={item.id}
                data-testid={`triage-card-${item.id}`}
                className="ohc-card w-full glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[24px] shadow-sm flex flex-col mb-4 overflow-hidden transition-all duration-300"
              >
                {/* Header Context */}
                <div
                  className="p-5 border-b border-[rgba(255,255,255,0.2)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] backdrop-blur-[30px] backdrop-saturate-[210%] cursor-pointer"
                  onClick={() => {
                    if (isSelected) {
                        setSelectedItemId(null);
                        setEditingId(null);
                    } else {
                        setSelectedItemId(item.id);
                        setEditingId(null);
                    }
                  }}
                  data-testid={`triage-card-header-${item.id}`}
                >
                  <div className="flex justify-between items-start mb-3">
                    <div className="flex items-center gap-2">
                      <span className="text-xl">{getSourceIcon(item.source || "")}</span>
                      <span className="font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-sm">
                        {item.customer_id || item.source || "Unknown Source"}
                      </span>
                    </div>
                    <span className={`app-badge ${badgeTone(item.priority)}`}>
                      {item.priority || "Normal"}
                    </span>
                  </div>
                  <div className="text-[15px] font-medium text-gray-900 dark:text-white leading-snug break-words line-clamp-2">
                    {item.context || "No context provided"}
                  </div>
                  {!isSelected && item.action_type && (
                     <div className="mt-2 text-[12px] text-[#0066FF] dark:text-[#3388FF] font-medium flex items-center gap-1">
                        <span>✨</span> AI Drafted: {item.action_type} (Tap to review)
                     </div>
                  )}
                </div>

                {/* Slide-in / Expanded Detail View */}
                {isSelected && (
                  <div className="animate-in slide-in-from-top-2 duration-200 fade-in">
                    {item.action_type && (
                      <div className="p-5 bg-[#0066FF]/10 dark:bg-[#0066FF]/20 backdrop-blur-[30px] saturate-[210%] flex flex-col gap-2">
                        <div className="text-[11px] uppercase tracking-wider font-bold text-[#0066FF] dark:text-[#3388FF]">
                          Proposed Action: {item.action_type}
                        </div>
                        <div className="proposed-action border border-[#0066FF]/20 dark:border-[#0066FF]/30 bg-white/50 dark:bg-black/30 backdrop-blur-[30px] saturate-[210%] p-4 text-[13px] leading-relaxed text-gray-900 dark:text-white whitespace-pre-wrap break-words">
                          {item.action_payload || "No specific payload"}
                        </div>
                      </div>
                    )}

                    {/* Meta Details */}
                    <div className="px-5 py-3 flex justify-between bg-white/30 dark:bg-black/30 backdrop-blur-[30px] saturate-[210%] text-[11px] text-gray-500 dark:text-gray-400">
                      <span>{new Date(item.created_at || Date.now()).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                      <span>{new Date(item.created_at || Date.now()).toLocaleDateString()}</span>
                    </div>

                    {/* Action Buttons */}
                    {editingId === item.id ? (
                      <div className="p-5 flex flex-col gap-3 border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%]">
                        <textarea
                          value={editValue}
                          onChange={(e) => setEditValue(e.target.value)}
                          className="w-full min-h-[88px] text-[13px] text-gray-900 dark:text-white bg-white/80 dark:bg-gray-800/80 border border-gray-300 dark:border-gray-600 rounded-[12px] p-3 focus:outline-none focus:ring-2 focus:ring-[#0066FF] shadow-inner resize-y"
                          data-testid={`triage-edit-textarea-${item.id}`}
                          placeholder="Edit the draft payload..."
                        />
                        <div className="flex flex-col sm:flex-row gap-3 w-full pt-2">
                          <button
                            onClick={() => handleDecision(item.id, true, editValue)}
                            disabled={isProcessing}
                            className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                            data-testid={`triage-save-btn-${item.id}`}
                          >
                            {isProcessing ? "Processing..." : "Save & Send"}
                          </button>
                          <button
                            onClick={() => {
                              setEditingId(null);
                              setEditValue("");
                            }}
                            disabled={isProcessing}
                            className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                            data-testid={`triage-cancel-btn-${item.id}`}
                          >
                            Cancel
                          </button>
                        </div>
                      </div>
                    ) : (
                      <div className="p-5 pt-2 flex flex-col sm:flex-row gap-3 w-full border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%]">
                        {item.action_type ? (
                          <>
                            <button
                              disabled={isProcessing}
                              className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                              data-testid={`triage-review-btn-${item.id}`}
                              onClick={() => {
                                setEditingId(item.id);
                                setEditValue(item.action_payload || "");
                              }}
                            >
                              Review Draft
                            </button>
                            <button
                              disabled={isProcessing}
                              className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                              data-testid={`triage-approve-${item.id}`}
                              onClick={() => handleDecision(item.id, true)}
                            >
                              {isProcessing ? "Processing..." : "Approve as-is"}
                            </button>
                          </>
                        ) : (
                          <button
                            disabled={isProcessing}
                            className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center disabled:opacity-50"
                            data-testid={`triage-approve-${item.id}`}
                            onClick={() => handleDecision(item.id, true)}
                          >
                            {isProcessing ? "Processing..." : "Approve & Send"}
                          </button>
                        )}
                        <button
                          disabled={isProcessing}
                          className="w-full flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 bg-white/50 dark:bg-black/50 backdrop-blur-[30px] saturate-[210%] text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-white/70 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center disabled:opacity-50 shadow-sm"
                          data-testid={`triage-dismiss-${item.id}`}
                          onClick={() => handleDecision(item.id, false)}
                        >
                          Dismiss
                        </button>
                      </div>
                    )}
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>
    </AppShell>
  );
}
