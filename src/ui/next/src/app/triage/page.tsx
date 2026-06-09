"use client";

import { useEffect, useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";

type Message = {
  id: string;
  source?: string;
  content?: string;
  original_content?: string;
  translated_from_language?: string;
  draft_reply?: string;
  status?: string;
  created_at?: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved"].includes(normalized)) return "good";
  if (["open", "pending", "action needed", ""].includes(normalized)) return "warn";
  if (["failed", "blocked", "urgent"].includes(normalized)) return "bad";
  return "";
}

export default function TriagePage() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [showOriginal, setShowOriginal] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    async function loadMessages() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/ui/inbox/messages?tenant_id=${encodeURIComponent(tenantId())}`);
        if (!res.ok) throw new Error("Failed to load triage items from the database");
        const data = await res.json();
        const rows = Array.isArray(data) ? data : [];
        setMessages(rows);
        setSelectedId(rows[0]?.id || null);
        setShowOriginal(false);
      } catch (e: any) {
        setError(e?.message || "Failed to load triage feed");
      } finally {
        setLoading(false);
      }
    }
    loadMessages();
  }, []);

  const selected = useMemo(
    () => messages.find((message) => message.id === selectedId) || messages[0],
    [messages, selectedId],
  );

  const openCount = messages.filter((message) => !["closed", "resolved", "sent"].includes((message.status || "").toLowerCase())).length;

  async function handleApproveAndSend(inboxMessageId: string) {
    try {
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/agents/approvals?limit=50`, {
        headers: { "Authorization": `Bearer ${token}` }
      });
      if (!res.ok) throw new Error("Failed to fetch approvals");
      const data = await res.json();
      const pendingApprovals = data.pending_approvals || [];

      const approval = pendingApprovals.find((a: any) => {
        try {
          const payload = typeof a.payload === 'string' ? JSON.parse(a.payload) : a.payload;
          return payload && payload.inbox_message_id === inboxMessageId;
        } catch (e) {
          return false;
        }
      });

      if (!approval) {
        setActionStatus("Could not find a pending approval for this message.");
        return;
      }

      const approveRes = await fetch(`/api/agents/approvals/${approval.id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
        body: JSON.stringify({ approved: true })
      });

      if (approveRes.ok) {
        setMessages((prev) => prev.map((m) => m.id === inboxMessageId ? { ...m, status: "resolved" } : m));
        setActionStatus("Proposed action approved.");
      } else {
        setActionStatus("Failed to approve and send message.");
      }
    } catch (e) {
      console.error(e);
      setActionStatus("Error approving action.");
    }
  }

  return (
    <AppShell
      title="Work Triage"
      subtitle="Your intelligent command center. See what needs attention today."
      statusItems={[
        { label: "Action Required", value: String(openCount), tone: openCount > 0 ? "warn" : "good" },
        { label: "Total Items", value: String(messages.length), tone: "neutral" },
      ]}
      actions={[{ label: "Agent Audit", href: "/agent-audit-dashboard" }]}
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}
      <div className="app-grid two">
        <section className="app-panel glassmorphism">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Needs Your Attention</div>
              <div className="app-list-subtitle">Prioritized feed by AI WorkTriage Agent.</div>
            </div>
          </div>
          <div id="triage-list" className="app-list">
            {error && <div className="app-empty text-red-500">{error}</div>}
            {!error && messages.length === 0 ? (
              <div className="app-empty">{loading ? "Analyzing incoming work..." : "You're all caught up! No triage items found."}</div>
            ) : messages.map((message) => (
              <button
                key={message.id}
                type="button"
                onClick={() => {
                  setSelectedId(message.id);
                  setShowOriginal(false);
                }}
                className={`app-list-item w-full text-left transition-all hover:scale-[1.01] ${selected?.id === message.id ? "bg-white/40 dark:bg-black/20 ring-1 ring-blue-500" : ""}`}
              >
                <div className="min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="text-sm font-semibold text-sky-700 dark:text-sky-300 bg-sky-50 dark:bg-sky-900/30 px-2 py-0.5 rounded-full">{message.source || "Unknown source"}</span>
                    {message.draft_reply && <span className="text-xs text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-0.5 rounded-full">✨ AI Drafted</span>}
                  </div>
                  <div className="app-list-title truncate font-medium">{message.content || "Empty message"}</div>
                  <div className="app-list-subtitle mt-1">
                      {["closed", "resolved", "sent"].includes((message.status || "").toLowerCase()) ? "Resolved" : "Action Suggested"}
                  </div>
                </div>
                <span className={`app-badge ${badgeTone(message.status)}`}>{message.status || "Urgent"}</span>
              </button>
            ))}
          </div>
        </section>

        <section className="app-panel glassmorphism">
          <div className="app-panel-header">
            <div className="app-panel-title text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Triage Detail</div>
          </div>
          {!selected ? (
            <div className="app-empty">Select a triage item to review the AI's proposed action.</div>
          ) : (
            <div className="app-panel-body">
              <div className="mb-6">
                <div className="app-metric-label flex items-center gap-2">
                    <span>Context</span>
                    <span className="text-xs font-semibold text-gray-500 dark:text-gray-400">{selected.source}</span>
                </div>
                <div className="mt-2 rounded-[12px] border border-white/40 dark:border-white/10 bg-white/60 dark:bg-black/30 p-4 text-sm leading-6 text-gray-800 dark:text-gray-200">
                  {(showOriginal ? selected.original_content : selected.content) || "Empty message"}
                </div>
              </div>
              <div className="mb-6">
                <div className="app-metric-label flex items-center gap-2">
                    <span>Proposed Action</span>
                    <span className="text-xs text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-0.5 rounded-full">✨ Prepared by WorkTriage Agent</span>
                </div>
                <div className="mt-2 rounded-[12px] border border-blue-200 dark:border-blue-900/50 bg-blue-50/50 dark:bg-blue-900/20 p-4 text-sm leading-6 text-gray-800 dark:text-gray-200">
                  {selected.draft_reply || "No draft reply stored. Awaiting manual review."}
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4 mb-6">
                <div className="app-card bg-white/50 dark:bg-black/20">
                  <div className="app-metric-label">Priority</div>
                  <div className="mt-2"><span className={`app-badge ${badgeTone(selected.status)}`}>{selected.status || "High"}</span></div>
                </div>
                <div className="app-card bg-white/50 dark:bg-black/20">
                  <div className="app-metric-label">Received</div>
                  <div className="mt-2 text-sm font-semibold text-gray-900 dark:text-gray-100">{selected.created_at || "Just now"}</div>
                </div>
              </div>
              {badgeTone(selected.status) === "warn" || badgeTone(selected.status) === "bad" ? (
                <div className="mt-4 flex gap-3">
                  <button
                    className="app-btn-primary w-full flex-1 justify-center py-3 text-base font-semibold"
                    onClick={() => handleApproveAndSend(selected.id)}
                  >✨ Approve Draft</button>
                  <button className="app-btn-secondary py-3 px-4 rounded-[8px]">Edit</button>
                </div>
              ) : null}
            </div>
          )}
        </section>
      </div>
    </AppShell>
  );
}
