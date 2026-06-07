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
  const urgentCount = messages.filter((message) => badgeTone(message.status) === "warn").length;
  const quoteReadyCount = messages.filter((message) => message.draft_reply && badgeTone(message.status) === "warn").length;

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
      title="Triage Feed"
      subtitle="Unified Omnichannel Inbox & Agent Triage."
      statusItems={[
        { label: "Urgent Inquiries", value: String(urgentCount), tone: urgentCount > 0 ? "warn" : "good" },
        { label: "Quote Ready to Send", value: String(quoteReadyCount), tone: quoteReadyCount > 0 ? "good" : "neutral" },
      ]}
      actions={[{ label: "Audit", href: "/agent-audit-dashboard" }]}
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}
      <div className="app-grid two">
        <section className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Triage Feed</div>
              <div className="app-list-subtitle">Prioritized incoming messages from all channels.</div>
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
                style={{
                  background: selected?.id === message.id ? "rgba(255, 255, 255, 0.65)" : "transparent",
                  backdropFilter: selected?.id === message.id ? "blur(30px) saturate(210%)" : "none",
                  border: selected?.id === message.id ? "1px solid rgba(255, 255, 255, 0.4)" : "none",
                  borderRadius: "8px",
                  marginBottom: "4px"
                }}
              >
                <div className="min-w-0">
                  <div className="app-list-title">{message.source || "Unknown channel"}</div>
                  <div className="app-list-subtitle truncate text-gray-500">{message.content || "Empty message"}</div>
                </div>
                <span className={`app-badge ${badgeTone(message.status)}`}>{message.status || "Open"}</span>
              </button>
            ))}
          </div>
        </section>

        <section className="app-panel">
          <div className="app-panel-header">
            <div className="app-panel-title">Thread View</div>
          </div>
          {!selected ? (
            <div className="app-empty">Select a database-backed message to inspect it.</div>
          ) : (
            <div className="app-panel-body flex flex-col gap-6">
              <div>
                <div className="flex items-center justify-between mb-2">
                  <div className="app-metric-label">Customer Inquiry ({selected.source})</div>
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
                <div className="rounded-2xl border border-gray-100 bg-white p-4 text-[15px] leading-relaxed text-gray-900 shadow-sm">
                  {(showOriginal ? selected.original_content : selected.content) || "Empty message"}
                </div>
                <div className="mt-2 text-xs text-gray-400 font-medium tracking-wide uppercase">
                  {selected.created_at || "Unknown date"}
                </div>
              </div>

              {selected.draft_reply && (
                <div>
                  <div className="app-metric-label mb-2 text-[#0066FF]">Agent Draft</div>
                  <div className="rounded-2xl border border-[#0066FF]/20 bg-[#0066FF]/[0.02] p-4 text-[15px] leading-relaxed text-gray-900 shadow-sm relative">
                    <textarea
                      className="w-full bg-transparent resize-none outline-none border-none p-0 m-0 focus:ring-0"
                      rows={5}
                      defaultValue={selected.draft_reply}
                    />
                  </div>
                </div>
              )}

              <div className="grid grid-cols-2 gap-3 mt-auto">
                <div className="app-card" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", border: "1px solid rgba(255, 255, 255, 0.4)", borderRadius: "16px" }}>
                  <div className="app-metric-label">Status</div>
                  <div className="mt-2"><span className={`app-badge ${badgeTone(selected.status)}`}>{selected.status || "Open"}</span></div>
                </div>
              </div>

              {badgeTone(selected.status) === "warn" && (
                <div className="mt-2">
                  <button
                    className="app-btn-primary w-full flex items-center justify-center gap-2 text-[17px] font-semibold tracking-wide"
                    style={{ minHeight: "56px", borderRadius: "14px" }}
                    onClick={() => handleApproveAndSend(selected.id)}
                  >
                    ✨ Approve &amp; Send Draft
                  </button>
                </div>
              )}
            </div>
          )}
        </section>
      </div>
    </AppShell>
  );
}
