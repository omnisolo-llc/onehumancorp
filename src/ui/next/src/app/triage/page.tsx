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
      statusItems={[
        { label: "Active", value: String(activeCount), tone: activeCount > 0 ? "warn" : "good" },
        { label: "Urgent", value: String(urgentCount), tone: urgentCount > 0 ? "bad" : "neutral" },
      ]}
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

      <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Unified Agent Feed</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
      </div>

      <div className="app-grid two grid grid-cols-1 lg:grid-cols-[1.5fr_0.8fr] gap-6">
        <section className="app-panel glassmorphism">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Triage Queue</div>
            </div>
          </div>
          <div id="triage-list" className="app-list">
            {error && <div className="app-empty">{error}</div>}
            {!error && items.length === 0 ? (
              <div className="app-empty">{loading ? "Loading triage items..." : "No items need your attention right now. Great job!"}</div>
            ) : items.map((item) => (
              <button
                key={item.id}
                type="button"
                data-testid={`triage-card-${item.id}`}
                onClick={() => setSelectedId(item.id)}
                className="app-list-item w-full text-left min-h-[44px]"
                style={{ background: selected?.id === item.id ? "#f8fafc" : "transparent" }}
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

        <section className="app-panel glassmorphism">
          <div className="app-panel-header">
            <div className="app-panel-title">Triage Detail</div>
          </div>
          {!selected ? (
            <div className="app-empty">Select a triage item to review it.</div>
          ) : (
            <div className="app-panel-body">
              <div className="mb-4">
                <div className="app-metric-label">Source</div>
                <div className="mt-1 text-sm font-semibold text-gray-900">{selected.source || "Unknown source"}</div>
              </div>
              <div className="mb-4">
                <div className="app-metric-label">Context</div>
                <div className="mt-2 rounded-md border border-gray-200 bg-gray-50 p-3 text-sm leading-6 text-gray-800">
                  {selected.context || "No context"}
                </div>
              </div>
              {selected.action_type === 'ProcessReturn' ? (
                <div className="mb-6">
                  <div className="app-metric-label">Proposed Action: Process Return</div>
                  <div className="mt-2 rounded-[8px] border border-[#0066FF]/20 bg-[#0066FF]/5 p-4">
                    {(() => {
                      try {
                        const payload = JSON.parse(selected.action_payload || "{}");
                        return (
                          <div className="space-y-3">
                            <div className="flex justify-between items-center pb-2 border-b border-[#0066FF]/10">
                              <span className="text-sm text-gray-600 dark:text-gray-400">Order ID</span>
                              <span className="font-semibold text-gray-900 dark:text-gray-100">{payload.order_id || 'Unknown'}</span>
                            </div>
                            <div className="flex justify-between items-center pb-2 border-b border-[#0066FF]/10">
                              <span className="text-sm text-gray-600 dark:text-gray-400">Type</span>
                              <span className="font-semibold text-gray-900 dark:text-gray-100">{payload.return_type || 'Refund'}</span>
                            </div>
                            <div className="flex justify-between items-center">
                              <span className="text-sm font-semibold text-gray-800 dark:text-gray-200">Amount</span>
                              <span className="text-lg font-bold text-green-600 dark:text-green-400">
                                ${(payload.amount_cents / 100).toFixed(2)}
                              </span>
                            </div>
                            <div className="mt-3 p-3 bg-white dark:bg-[#1D1D1F] rounded border border-gray-100 dark:border-gray-800 text-xs text-gray-600 dark:text-gray-400">
                              <p>✓ Operations Agent will restock Product {payload.product_id}</p>
                              <p>✓ Finance Agent will refund via Stripe</p>
                              <p>✓ Automated confirmation will be sent to customer</p>
                            </div>
                          </div>
                        );
                      } catch {
                        return <div className="text-sm text-gray-800">{selected.action_payload}</div>;
                      }
                    })()}
                  </div>
                </div>
              ) : selected.action_type ? (
                <div className="mb-6">
                  <div className="app-metric-label">Proposed Action: {selected.action_type}</div>
                  <div className="mt-2 rounded-[8px] border border-blue-200 bg-blue-50 p-4 text-sm leading-6 text-blue-900 font-medium">
                    {selected.action_payload || "No specific payload"}
                  </div>
                </div>
              ) : null}

              <div className="grid grid-cols-2 gap-3 mb-6">
                <div className="app-card glassmorphism">
                  <div className="app-metric-label">Priority</div>
                  <div className="mt-2"><span className={`app-badge ${badgeTone(selected.priority)}`}>{selected.priority || "Normal"}</span></div>
                </div>
                <div className="app-card glassmorphism">
                  <div className="app-metric-label">Created</div>
                  <div className="mt-2 text-sm font-semibold text-gray-900">{new Date(selected.created_at || Date.now()).toLocaleString()}</div>
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
                  className="px-4 py-2 rounded-md border border-gray-300 text-gray-700 bg-white hover:bg-gray-50 flex-1 min-h-[44px] font-medium transition-colors"
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
