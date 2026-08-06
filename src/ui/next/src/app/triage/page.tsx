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
    localStorage.getItem("business_display_name") ||
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
  if (s.includes("whatsapp")) return "💬";
  if (s.includes("sms")) return "📱";
  if (s.includes("email")) return "✉️";
  if (s.includes("web") || s.includes("chat")) return "🌐";
  return "📥";
};

export default function TriagePage() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [processingId, setProcessingId] = useState<string | null>(null);
  const [selectedItemId, setSelectedItemId] = useState<string | null>(null);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editValue, setEditValue] = useState("");

  const currentTenantId = tenantId();

  useEffect(() => {
    fetchTriageItems();

    // Set up a polling mechanism for realtime updates,
    // simulating WebSockets/Server-Sent Events for now
    const pollInterval = setInterval(() => {
        fetchTriageItems(false);
    }, 5000);

    return () => clearInterval(pollInterval);
  }, []);

  const fetchTriageItems = async (showLoading = true) => {
    if (showLoading) setLoading(true);
    try {
      // Look for triage items using unified triage API
      const res = await fetch(`/api/v1/inbox/${currentTenantId}/actions`);
      if (res.ok) {
        const data = await res.json();
        // Transform the UnifiedTriageAction to TriageItem format
        const transformedItems = (data || []).map((action: any) => ({
           id: action.id,
           tenant_id: action.tenant_id,
           customer_id: `Thread ${(action.thread_id || '').substring(0, 8)}`,
           source: 'Unified Inbox',
           priority: 'Action Needed',
           context: 'New message received in thread.',
           action_type: action.action_type,
           action_payload: action.action_payload,
           status: action.status,
           created_at: action.created_at || new Date().toISOString(),
        }));

        // Let's mix in offline queue tasks for demo purposes if nothing from backend
        let localActions = await getActions();
        const localTriage = localActions
            .filter((a: any) => a.action === 'CREATE_TRIAGE_TASK')
            .map((a: any) => ({
                id: a.id || `local-${Date.now()}`,
                tenant_id: currentTenantId,
                customer_id: 'Offline System',
                source: 'Local Operation',
                priority: 'High',
                context: a.payload.reason || 'Offline action needs review',
                created_at: new Date(a.timestamp).toISOString()
            }));

        // Use backend first, fallback to offline, then fallback to empty
        if (transformedItems.length > 0) {
            setItems([...transformedItems, ...localTriage]);
        } else {
            // Check old API for fallback
            const fallbackRes = await fetch(`/api/v1/dashboard/briefing?tenant_id=${currentTenantId}`);
            if (fallbackRes.ok) {
               const fallbackData = await fallbackRes.json();
               if (fallbackData.triage) {
                   setItems([...fallbackData.triage, ...localTriage]);
               } else {
                   setItems(localTriage);
               }
            } else {
               setItems(localTriage);
            }
        }
      } else {
        // Fallback for missing new API
        console.warn("Unified inbox action API missing, falling back to mock offline");
        const localActions = await getActions();
        const localTriage = localActions
            .filter((a: any) => a.action === 'CREATE_TRIAGE_TASK')
            .map((a: any) => ({
                id: a.id || `local-${Date.now()}`,
                tenant_id: currentTenantId,
                customer_id: 'Offline System',
                source: 'Local Operation',
                priority: 'High',
                context: a.payload.reason || 'Offline action needs review',
                created_at: new Date(a.timestamp).toISOString()
            }));
        setItems(localTriage);
      }
    } catch (e) {
      console.error("Failed to fetch triage items", e);
    } finally {
      if (showLoading) setLoading(false);
    }
  };

  const handleDecision = async (id: string, approved: boolean, updatedPayload?: string) => {
    setProcessingId(id);
    try {
      const resolution = approved ? (updatedPayload ? 'edited' : 'approved') : 'rejected';
      const res = await fetch(`/api/v1/inbox/${currentTenantId}/actions/${id}/resolve`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          tenant_id: currentTenantId,
          resolution,
          updated_payload: updatedPayload,
        }),
      });

      if (res.ok) {
        setItems((prev) => prev.filter((i) => i.id !== id));
        setSelectedItemId(null);
        setEditingId(null);
      }
    } catch (e) {
      console.error("Failed to process triage item", e);
    } finally {
      setProcessingId(null);
    }
  };

  return (
    <AppShell
      title="Work Triage"
      subtitle="Unified Omnichannel Inbox"
    >
      <div className="w-full max-w-[600px] mx-auto px-4 py-6 pb-24">
        {loading ? (
          <div className="flex flex-col gap-4">
            {[1, 2, 3].map((i) => (
              <div
                key={i}
                className="ohc-card w-full h-[120px] bg-white/40 dark:bg-black/20 rounded-[24px] animate-pulse"
              />
            ))}
          </div>
        ) : items.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-20 animate-in fade-in duration-500">
            <div className="text-[64px] mb-6 opacity-80">🎉</div>
            <h3 className="text-[20px] font-outfit font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center">
              Inbox Zero
            </h3>
            <div className="text-[15px] text-gray-500 dark:text-gray-400 text-center max-w-[280px]">
              Your AI assistant has handled all outstanding items. Take a breath, you're all caught up!
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
                          className="w-full min-h-[88px] text-[13px] text-gray-900 dark:text-white bg-white/80 dark:bg-gray-800/80 border border-gray-300 dark:border-gray-600 rounded-xl p-3 focus:outline-none focus:ring-2 focus:ring-[#0066FF] shadow-inner resize-y"
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
