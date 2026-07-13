"use client";
import DOMPurify from 'isomorphic-dompurify';

import { useEffect, useMemo, useState } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { useQuery } from "@powersync/react";
import { PowerSyncProvider } from "../../lib/powersync/PowerSyncProvider";

type Message = {
  id: string;
  source?: string;
  content?: string;
  original_content?: string;
  translated_from_language?: string;
  draft_reply?: string;
  status?: string;
  sender_id?: string;
  customer_id?: string;
  created_at?: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved", "auto_replied"].includes(normalized)) return "good";
  if (["open", "pending", ""].includes(normalized)) return "warn";
  if (["failed", "blocked"].includes(normalized)) return "bad";
  return "";
}


function renderMessageContent(content: string) {
  if (!content) return "Empty message";

  // Basic XSS mitigation - encode HTML tags
  let safeContent = content.replace(/</g, "&lt;").replace(/>/g, "&gt;");

  // Format Twilio Media tags e.g. [Media: image/jpeg - https://example.com/image.jpg]
  safeContent = safeContent.replace(/\[Media:\s*([^-]+?)\s*-\s*(https?:\/\/[^\]]+)\]/g, (match, type, url) => {
    if (type.startsWith('image/')) {
      // url is sanitized via DOMPurify below
      return `<div class="my-2"><img src="${url}" alt="${type}" class="max-w-full h-auto rounded-md shadow-sm" style="max-height: 300px;" /></div>`;
    }
    return `<div class="my-2"><a href="${url}" target="_blank" rel="noopener noreferrer" class="text-blue-600 underline">Attached Media (${type})</a></div>`;
  });

  // Format standard Markdown images e.g. ![Alt text](https://example.com/image.jpg)
  safeContent = safeContent.replace(/!\[([^\]]*)\]\((https?:\/\/[^)]+)\)/g, (match, alt, url) => {
    return `<div class="my-2"><img src="${url}" alt="${alt}" class="max-w-full h-auto rounded-md shadow-sm" style="max-height: 300px;" /></div>`;
  });

  // Format basic line breaks
  safeContent = safeContent.replace(/\n/g, '<br />');

  // Sanitize the final HTML using DOMPurify
  return DOMPurify.sanitize(safeContent);
}

function formatStatus(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (normalized === "auto_replied") return "✨ AI Handled";
  return status || "Open";
}

function CustomerContextCard({ customerId, tenantId }: { customerId: string; tenantId: string }) {
  const [summary, setSummary] = useState<any>(null);

  useEffect(() => {
    async function fetchSummary() {
      try {
        const res = await fetch(`/api/memory/summary/${tenantId}/${customerId}`);
        if (res.ok) {
          const data = await res.json();
          setSummary(data);
        }
      } catch (err) {
        console.error("Failed to fetch customer memory summary:", err);
      }
    }
    fetchSummary();
  }, [customerId, tenantId]);

  if (!summary) return null;
  if (summary.total_interactions === 0 && summary.segments.length === 0) return null;

  return (
    <div className="mt-4 rounded-xl border border-gray-100 bg-blue-50/50 p-4 dark:border-white/10 dark:bg-blue-900/10">
      <div className="mb-2 flex items-center justify-between">
        <h3 className="text-sm font-semibold text-blue-900 dark:text-blue-100">Unified Customer Memory</h3>
        <span className="app-badge good">{summary.total_interactions} interactions</span>
      </div>
      {summary.segments.length > 0 && (
        <div className="mb-2 text-xs text-gray-700 dark:text-gray-300">
          <span className="font-semibold text-gray-900 dark:text-white">Segments: </span>
          {summary.segments.join(", ")}
        </div>
      )}
      {summary.preferences.length > 0 && (
        <div className="mb-2 text-xs text-gray-700 dark:text-gray-300">
          <span className="font-semibold text-gray-900 dark:text-white">Preferences: </span>
          {summary.preferences.join(", ")}
        </div>
      )}
      <div className="text-xs text-gray-600 dark:text-gray-400">
        {summary.summary}
      </div>
    </div>
  );
}

function InboxWorkspace({
  messages,
  sourceLabel,
}: {
  messages: Message[];
  sourceLabel: string;
}) {
  const router = useRouter();
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [showOriginal, setShowOriginal] = useState(false);
  const [actionStatus, setActionStatus] = useState("");
  const [manualReply, setManualReply] = useState("");


  const selected = useMemo(() => {
    if (messages.length === 0) return null;
    return messages.find((m) => m.id === selectedId) || messages[0];
  }, [messages, selectedId]);

  const [pendingApprovals, setPendingApprovals] = useState<any[]>([]);

  useEffect(() => {
    async function fetchApprovals() {
      try {
        const token = localStorage.getItem("token") || "";
        const res = await fetch(`/api/agents/approvals?limit=50`, {
          headers: { "Authorization": `Bearer ${token}` }
        });
        if (res.ok) {
          const data = await res.json();
          setPendingApprovals(data.pending_approvals || []);
        }
      } catch (e) {
        console.error(e);
      }
    }
    fetchApprovals();
  }, []);

  const activeApproval = useMemo(() => {
    if (!selected) return null;
    return pendingApprovals.find((a: any) => {
      try {
        const payload = typeof a.payload === 'string' ? JSON.parse(a.payload) : a.payload;
        return payload && payload.inbox_message_id === selected.id;
      } catch (e) {
        return false;
      }
    });
  }, [selected, pendingApprovals]);


  const openCount = messages.filter((message) => !["closed", "resolved"].includes((message.status || "").toLowerCase())).length;

  async function handleDraftQuoteWithAI(message: Message) {
    try {
      setActionStatus("Drafting quote with AI...");
      const res = await fetch("/api/quotes/draft_agent", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          inquiry: message.content || "",
          customer_id: message.customer_id || message.sender_id || "unknown",
          tenant_id: "t1" // Hardcoded fallback for now, normally from context
        }),
      });
      if (!res.ok) throw new Error("Failed to draft quote");
      const data = await res.json();
      if (data.id) {
        setActionStatus("Quote drafted successfully!");
        router.push(`/quotes/${data.id}`);
      }
    } catch (err: any) {
      setActionStatus(`Error drafting quote: ${err.message}`);
    } finally {
      setTimeout(() => setActionStatus(""), 3000);
    }
  }


  async function handleSendManualReply(inboxMessageId: string) {
    if (!manualReply.trim()) return;
    try {
      setActionStatus("Sending reply...");
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/ui/omni_inbox/action`, {
        method: "POST",
        headers: { "Content-Type": "application/json", "Authorization": `Bearer ${token}` },
        body: JSON.stringify({
          message_id: inboxMessageId,
          approved: true,
          edited_reply: manualReply
        })
      });
      if (res.ok) {
        setActionStatus("Manual reply sent.");
        setManualReply("");
      } else {
        setActionStatus("Failed to send manual reply.");
      }
    } catch (e) {
      console.error(e);
      setActionStatus("Error sending manual reply.");
    }
  }

  function handleAttachPhoto() {
    // Alert the user that this feature is not yet available, rather than inserting mock data.
    alert("Image uploading is currently unavailable. Please try again later.");
  }

  async function handleApproveAndSend(inboxMessageId: string) {
    try {
      const token = localStorage.getItem("token") || "";
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
        // Optimistic UI updates are handled by PowerSync once backend completes sync,
        // but we show the status to the user.
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
      title="Unified Inbox"
      subtitle="Local-first offline unified customer conversations and drafts."
      statusItems={[
        { label: "Messages", value: String(messages.length), tone: messages.length > 0 ? "good" : "neutral" },
        { label: "Open", value: String(openCount), tone: openCount > 0 ? "warn" : "good" },
      ]}
      actions={[{ label: "Audit", href: "/agent-audit-dashboard" }]}
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}
      <div className="w-full max-w-[375px] mx-auto md:max-w-none">
        <div className="app-grid two gap-4">
          <section className="app-panel glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] overflow-hidden">
            <div className="app-panel-header border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] p-4">
              <div>
                <div className="app-panel-title font-bold text-gray-900 dark:text-white">Message Queue</div>
                <div className="app-list-subtitle text-xs text-gray-500">{sourceLabel}</div>
              </div>
            </div>
            <div id="messages-list" className="app-list p-2">
              {messages.length === 0 ? (
                <div className="app-empty">No inbox messages found for this tenant.</div>
              ) : messages.map((message) => (
                <button
                  key={message.id}
                  type="button"
                  onClick={() => {
                    setSelectedId(message.id);
                    setShowOriginal(false);
                  }}
                  className={`app-list-item min-h-[44px] min-w-[44px] w-full text-left p-3 mb-2 rounded-[8px] transition-all backdrop-filter ${selected?.id === message.id ? "bg-white/60 dark:bg-black/20 shadow-sm" : "hover:bg-black/5 dark:hover:bg-white/5 bg-white/10"}`}
                >
                  <div className="min-w-0">
                    <div className="app-list-title">{message.source || "Unknown source"}</div>
                    <div className="app-list-subtitle truncate">{message.content || "Empty message"}</div>
                  </div>
                  <span className={`app-badge ${badgeTone(message.status)}`}>{formatStatus(message.status)}</span>
                </button>
              ))}
            </div>

                {/* Manual Reply Box */}
                {selected.status !== "resolved" && selected.status !== "dismissed" && (
                  <div className="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700">
                    <div className="app-metric-label mb-2">Manual Reply</div>
                    <textarea
                      className="w-full min-h-[100px] p-3 rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-zinc-800 text-gray-900 dark:text-gray-100 mb-3"
                      placeholder="Type your reply here..."
                      value={manualReply}
                      onChange={(e) => setManualReply(e.target.value)}
                    />
                    <div className="flex gap-3 mt-4 flex-wrap">
                      <button
                        onClick={() => handleSendManualReply(selected.id)}
                        className="app-btn-primary min-h-[44px] min-w-[44px] rounded-[8px]"
                        disabled={!manualReply.trim()}
                      >
                        Send Reply
                      </button>
                      <button
                        onClick={handleAttachPhoto}
                        className="app-btn-secondary flex items-center gap-2 min-h-[44px] min-w-[44px] rounded-[8px]"
                      >
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" /></svg>
                        Attach Photo
                      </button>
                    </div>
                  </div>
                )}
\n          </section>

          <section className="app-panel glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] overflow-hidden">
            <div className="app-panel-header border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] p-4">
              <div className="app-panel-title font-bold text-gray-900 dark:text-white">Conversation Detail</div>
            </div>
            {!selected ? (
              <div className="app-empty p-8 text-center text-gray-500">Select a database-backed message to inspect it.</div>
            ) : (
              <div className="app-panel-body p-5">
                <div className="mb-4 flex items-center justify-between">
                  <div>
                    <div className="app-metric-label">Source</div>
                    <div className="mt-1 text-sm font-semibold text-gray-900">{selected.source || "Unknown source"}</div>
                  </div>
                  {selected.sender_id && (
                    <div className="text-right">
                      <div className="app-metric-label">Sender</div>
                      <div className="mt-1 flex items-center gap-2">
                        <span className="text-sm font-semibold text-gray-900">{selected.sender_id}</span>
                        {selected.customer_id && (
                          <span className="app-badge good">Known Customer</span>
                        )}
                      </div>
                    </div>
                  )}
                </div>
                {selected.customer_id && (
                  <CustomerContextCard customerId={selected.customer_id} tenantId={tenantId()} />
                )}
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
                    <div dangerouslySetInnerHTML={{ __html: renderMessageContent((showOriginal ? selected.original_content : selected.content) || "Empty message") }} />
                  </div>
                </div>
                <div className="mb-4">
                  <div className="app-metric-label">Draft Reply</div>
                  <div className="mt-2 rounded-md border border-gray-200 bg-white p-3 text-sm leading-6 text-gray-800">
                    <div dangerouslySetInnerHTML={{ __html: renderMessageContent(selected.draft_reply || "No draft reply stored for this message.") }} />
                  </div>
                </div>
                <div className="grid grid-cols-2 gap-3">
                  <div className="app-card">
                    <div className="app-metric-label">Status</div>
                    <div className="mt-2"><span className={`app-badge ${badgeTone(selected.status)}`}>{formatStatus(selected.status)}</span></div>
                  </div>
                  <div className="app-card">
                    <div className="app-metric-label">Created</div>
                    <div className="mt-2 text-sm font-semibold text-gray-900">{selected.created_at || "Unknown"}</div>
                  </div>
                </div>
                {badgeTone(selected.status) === "warn" && (
                  <div className="mt-6">
                    {(() => {
                      let buttonText = "✨ Approve & Send Draft";
                      let parsedPayload = null;
                      if (activeApproval && activeApproval.payload) {
                        try {
                          parsedPayload = typeof activeApproval.payload === 'string' ? JSON.parse(activeApproval.payload) : activeApproval.payload;
                        } catch(e) {}
                        if (parsedPayload && parsedPayload.action_type === "Draft Quote") {
                           let amount = 0;
                           if (parsedPayload.total_amount_cents !== undefined && parsedPayload.total_amount_cents !== null) {
                              amount = parsedPayload.total_amount_cents / 100;
                           } else if (parsedPayload.total_amount !== undefined && parsedPayload.total_amount !== null) {
                              amount = parsedPayload.total_amount;
                           }
                           buttonText = `✨ Send quote for $${amount.toFixed(2)}`;
                        } else if (parsedPayload && parsedPayload.action_type === "Draft Booking") {
                           buttonText = "✨ Approve booking";
                        } else if (parsedPayload && parsedPayload.action_type === "Draft Reply") {
                           buttonText = "✨ Approve & Send Draft";
                        } else if (parsedPayload && parsedPayload.feature_type === "ambassador_reply") {
                           buttonText = "✨ Approve & Send Draft";
                        }
                      }
                      return (
                        <button
                          className="app-button primary w-full min-h-[44px] min-w-[44px] rounded-[8px] backdrop-filter bg-white/10"
                          onClick={() => handleApproveAndSend(selected.id)}
                        >
                          {buttonText}
                        </button>
                      );
                    })()}
                  </div>
                )}
                {!activeApproval && (
                  <div className="mt-4 flex flex-col gap-4">
                    <button
                      onClick={() => handleDraftQuoteWithAI(selected)}
                      className="app-button w-full min-h-[44px] min-w-[44px] rounded-[8px] bg-gradient-to-r from-purple-500 to-indigo-600 text-white font-bold shadow-lg hover:from-purple-600 hover:to-indigo-700 transition-all flex items-center justify-center gap-2"
                    >✨ Draft Quote with AI</button>
                  </div>
                )}
              </div>
            )}
          </section>
        </div>
      </div>
    </AppShell>
  );
}

function PowerSyncInboxContent() {
  const { data } = useQuery<Message>("SELECT * FROM omni_inbox_messages ORDER BY created_at DESC");
  return <InboxWorkspace messages={data || []} sourceLabel="Local database sync is active." />;
}

function ApiInboxFallback() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function loadMessages() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/ui/omni_inbox?tenant_id=${encodeURIComponent(tenantId())}`);
        if (!res.ok) throw new Error("Failed to load inbox messages");
        const data = await res.json();
        setMessages(Array.isArray(data) ? data : []);
      } catch (err: any) {
        setError(err?.message || "Failed to load inbox messages");
      } finally {
        setLoading(false);
      }
    }
    loadMessages();
  }, []);

  if (error) {
    return (
      <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
        <div className="app-panel">
          <div className="app-empty">{error}</div>
        </div>
      </AppShell>
    );
  }

  if (loading) {
    return (
      <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
        <div className="app-panel">
          <div className="app-empty">Loading inbox messages...</div>
        </div>
      </AppShell>
    );
  }

  return <InboxWorkspace messages={messages} sourceLabel="Live inbox messages for the current tenant." />;
}

export default function InboxPage() {
  return (
    <PowerSyncProvider
      fallback={(
        <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
          <div className="app-panel">
            <div className="app-empty">Loading local database...</div>
          </div>
        </AppShell>
      )}
      unsupportedFallback={<ApiInboxFallback />}
    >
      <PowerSyncInboxContent />
    </PowerSyncProvider>
  );
}
