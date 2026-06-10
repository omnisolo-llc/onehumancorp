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
  sender_id?: string;
  created_at?: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved"].includes(normalized)) return "good";
  if (["open", "pending", ""].includes(normalized)) return "warn";
  if (["failed", "blocked"].includes(normalized)) return "bad";
  return "";
}

export default function InboxPage() {
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
        if (!res.ok) throw new Error("Failed to load inbox messages from the database");
        const data = await res.json();
        const rows = Array.isArray(data) ? data : [];
        setMessages(rows);
        setSelectedId(rows[0]?.id || null);
        setShowOriginal(false);
      } catch (e: any) {
        setError(e?.message || "Failed to load inbox");
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

  const openCount = messages.filter((message) => !["closed", "resolved"].includes((message.status || "").toLowerCase())).length;

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
        setMessages((prev) => prev.map((m) => m.id === inboxMessageId ? { ...m, status: "sent" } : m));
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
      title="Inbox"
      subtitle="Database-backed customer conversations and generated drafts."
      statusItems={[
        { label: "Messages", value: String(messages.length), tone: messages.length > 0 ? "good" : "neutral" },
        { label: "Open", value: String(openCount), tone: openCount > 0 ? "warn" : "good" },
      ]}
      actions={[{ label: "Audit", href: "/agent-audit-dashboard" }]}
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}
      <div className="app-grid two">
        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Message Queue</div>
              <div className="app-list-subtitle">Loaded from `/api/ui/inbox/messages`.</div>
            </div>
          </div>
          <div id="messages-list" className="app-list">
            {error && <div className="app-empty">{error}</div>}
            {!error && messages.length === 0 ? (
              <div className="app-empty">{loading ? "Loading inbox messages from the database..." : "No inbox message rows found for this tenant."}</div>
            ) : messages.map((message) => (
              <button
                key={message.id}
                type="button"
                onClick={() => {
                  setSelectedId(message.id);
                  setShowOriginal(false);
                }}
                className="app-list-item w-full text-left"
                style={{ background: selected?.id === message.id ? "#f8fafc" : "transparent" }}
              >
                <div className="min-w-0">
                  <div className="app-list-title">{message.source || "Unknown source"}</div>
                  <div className="app-list-subtitle truncate">{message.content || "Empty message"}</div>
                </div>
                <span className={`app-badge ${badgeTone(message.status)}`}>{message.status || "Open"}</span>
              </button>
            ))}
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div className="app-panel-title">Conversation Detail</div>
          </div>
          {!selected ? (
            <div className="app-empty">Select a database-backed message to inspect it.</div>
          ) : (
            <div className="app-panel-body">
              <div className="mb-4">
                <div className="app-metric-label">Source</div>
                <div className="mt-1 text-sm font-semibold text-gray-900">{selected.source || "Unknown source"}</div>
              </div>
              <div className="mb-4">
                <div className="app-metric-label">Sender ID</div>
                <div className="mt-1 text-sm font-semibold text-gray-900">{selected.sender_id || "Unknown"}</div>
              </div>
              <div className="mb-4">
                <div className="flex items-center justify-between gap-3">
                  <div className="app-metric-label">Customer Message</div>
                  {selected.original_content && selected.original_content !== selected.content && (
                    <button
                      type="button"
                      className="app-badge"
                      onClick={() => setShowOriginal((value) => !value)}
                    >
                      {showOriginal ? "Translated" : `Original ${selected.translated_from_language || ""}`.trim()}
                    </button>
                  )}
                </div>
                <div className="mt-2 rounded-md border border-gray-200 bg-gray-50 p-3 text-sm leading-6 text-gray-800">
                  {(showOriginal ? selected.original_content : selected.content) || "Empty message"}
                </div>
              </div>
              <div className="mb-4">
                <div className="app-metric-label">Draft Reply</div>
                <div className="mt-2 rounded-md border border-gray-200 bg-white p-3 text-sm leading-6 text-gray-800">
                  {selected.draft_reply || "No draft reply stored for this message."}
                </div>
              </div>
              <div className="grid grid-cols-2 gap-3">
                <div className="app-card">
                  <div className="app-metric-label">Status</div>
                  <div className="mt-2"><span className={`app-badge ${badgeTone(selected.status)}`}>{selected.status || "Open"}</span></div>
                </div>
                <div className="app-card">
                  <div className="app-metric-label">Created</div>
                  <div className="mt-2 text-sm font-semibold text-gray-900">{selected.created_at || "Unknown"}</div>
                </div>
              </div>
              {badgeTone(selected.status) === "warn" && (
                <div className="mt-6">
                  <button
                    className="app-btn-primary w-full"
                    onClick={() => handleApproveAndSend(selected.id)}
                  >✨ Approve &amp; Send Draft</button>
                </div>
              )}
            </div>
          )}
        </section>
      </div>
    </AppShell>
  );
}
