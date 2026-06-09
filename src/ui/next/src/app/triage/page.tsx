"use client";

import { useEffect, useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";

type TriageItem = {
  id: string;
  source?: string;
  content?: string;
  priority?: string;
  draft_reply?: string;
  status?: string;
  created_at?: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function badgeTone(status?: string, priority?: string) {
  const normStatus = (status || "").toLowerCase();
  const normPriority = (priority || "").toLowerCase();
  if (["resolved", "sent"].includes(normStatus)) return "good";
  if (normPriority === "urgent") return "bad";
  if (normPriority === "action needed") return "warn";
  return "neutral";
}

export default function TriagePage() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    async function loadItems() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/ui/triage?tenant_id=${encodeURIComponent(tenantId())}`);
        if (!res.ok) throw new Error("Failed to load triage items from the database");
        const data = await res.json();
        const rows = Array.isArray(data) ? data : [];
        setItems(rows);
        setSelectedId(rows[0]?.id || null);
      } catch (e: any) {
        setError(e?.message || "Failed to load triage items");
      } finally {
        setLoading(false);
      }
    }
    loadItems();
  }, []);

  const selected = useMemo(
    () => items.find((item) => item.id === selectedId) || items[0],
    [items, selectedId],
  );

  const openCount = items.filter((item) => !["resolved", "closed"].includes((item.status || "").toLowerCase())).length;

  async function handleApproveAndSend(triageId: string) {
    try {
      const res = await fetch(`/api/ui/triage/${triageId}/approve`, {
        method: "POST",
      });

      if (res.ok) {
        setItems((prev) => prev.map((m) => m.id === triageId ? { ...m, status: "resolved" } : m));
        setActionStatus("Draft approved and sent.");
      } else {
        setActionStatus("Failed to approve and send message.");
      }
    } catch (e) {
      console.error(e);
      setActionStatus("Error approving message.");
    }
  }

  return (
    <AppShell
      title="Triage"
      subtitle="Unified omni-channel AI Inbox."
      statusItems={[
        { label: "Items", value: String(items.length), tone: items.length > 0 ? "good" : "neutral" },
        { label: "Action Needed", value: String(openCount), tone: openCount > 0 ? "warn" : "good" },
      ]}
      actions={[{ label: "Audit", href: "/agent-audit-dashboard" }]}
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}
      <div className="app-grid two">
        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Needs Your Attention</div>
              <div className="app-list-subtitle">Loaded from `/api/ui/triage`.</div>
            </div>
          </div>
          <div id="triage-list" className="app-list">
            {error && <div className="app-empty">{error}</div>}
            {!error && items.length === 0 ? (
              <div className="app-empty">{loading ? "Loading triage items from the database..." : "No triage items found for this tenant."}</div>
            ) : items.map((item) => (
              <button
                key={item.id}
                type="button"
                onClick={() => {
                  setSelectedId(item.id);
                }}
                className="app-list-item w-full text-left"
                style={{ background: selected?.id === item.id ? "#f8fafc" : "transparent" }}
              >
                <div className="min-w-0">
                  <div className="app-list-title">{item.source || "Unknown source"}</div>
                  <div className="app-list-subtitle truncate">{item.content || "Empty item"}</div>
                </div>
                <span className={`app-badge ${badgeTone(item.status, item.priority)}`}>{item.priority || item.status || "Open"}</span>
              </button>
            ))}
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div className="app-panel-title">Triage Detail</div>
          </div>
          {!selected ? (
            <div className="app-empty">Select a database-backed triage item to inspect it.</div>
          ) : (
            <div className="app-panel-body">
              <div className="mb-4">
                <div className="app-metric-label">Source</div>
                <div className="mt-1 text-sm font-semibold text-gray-900">{selected.source || "Unknown source"}</div>
              </div>
              <div className="mb-4">
                <div className="flex items-center justify-between gap-3">
                  <div className="app-metric-label">Customer Message</div>
                </div>
                <div className="mt-2 rounded-md border border-gray-200 bg-gray-50 p-3 text-sm leading-6 text-gray-800">
                  {selected.content || "Empty message"}
                </div>
              </div>
              <div className="mb-4">
                <div className="app-metric-label">Proposed Action / Draft Reply</div>
                <div className="mt-2 rounded-md border border-gray-200 bg-white p-3 text-sm leading-6 text-gray-800">
                  {selected.draft_reply || "No draft reply stored for this item."}
                </div>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div className="app-card">
                  <div className="app-metric-label">Status</div>
                  <div className="mt-2"><span className={`app-badge ${badgeTone(selected.status, selected.priority)}`}>{selected.status || "Open"}</span></div>
                </div>
                <div className="app-card">
                  <div className="app-metric-label">Created</div>
                  <div className="mt-2 text-sm font-semibold text-gray-900">{selected.created_at || "Unknown"}</div>
                </div>
              </div>
              {selected.status !== "resolved" && (
                <div className="mt-6">
                  <button
                    className="app-btn-primary w-full"
                    onClick={() => handleApproveAndSend(selected.id)}
                  >✨ Approve</button>
                </div>
              )}
            </div>
          )}
        </section>
      </div>
    </AppShell>
  );
}
