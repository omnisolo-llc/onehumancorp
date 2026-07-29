"use client";

import { Fragment, useEffect, useMemo, useState, useRef, type ReactNode } from "react";
import { useRouter } from "next/navigation";
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
  customer_id?: string;
  created_at?: string;
  conversation_id?: string;
};

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved", "auto_replied"].includes(normalized)) return "good";
  if (["open", "pending", ""].includes(normalized)) return "warn";
  if (["failed", "blocked"].includes(normalized)) return "bad";
  return "";
}


function normalizeExternalHttpUrl(value: string) {
  try {
    const url = new URL(value);
    return url.protocol === "http:" || url.protocol === "https:" ? url.toString() : null;
  } catch {
    return null;
  }
}

function textWithLineBreaks(value: string, keyPrefix: string): ReactNode[] {
  return value.split("\n").flatMap((line, index) => [
    index > 0 ? <br key={`${keyPrefix}-break-${index}`} /> : null,
    <Fragment key={`${keyPrefix}-line-${index}`}>{line}</Fragment>,
  ]);
}

function renderMessageContent(content: string): ReactNode {
  if (!content) return "Empty message";

  const tokenPattern = /\[Media:\s*(.+?)\s+-\s+(https?:\/\/[^\]\s]+)\]|!\[([^\]]*)\]\((https?:\/\/[^)\s]+)\)/g;
  const nodes: ReactNode[] = [];
  let cursor = 0;
  let tokenIndex = 0;

  for (const match of content.matchAll(tokenPattern)) {
    const offset = match.index ?? cursor;
    nodes.push(...textWithLineBreaks(content.slice(cursor, offset), `text-${tokenIndex}`));

    const mediaType = match[1]?.trim();
    const rawUrl = match[2] ?? match[4];
    const url = rawUrl ? normalizeExternalHttpUrl(rawUrl) : null;
    const alt = mediaType ?? match[3] ?? "Attached image";

    if (!url) {
      nodes.push(...textWithLineBreaks(match[0], `invalid-${tokenIndex}`));
    } else if (!mediaType || mediaType.startsWith("image/")) {
      nodes.push(
        <span className="my-2 block" key={`image-${tokenIndex}`}>
          {/* eslint-disable-next-line @next/next/no-img-element -- customer media uses an external, runtime URL */}
          <img
            src={url}
            alt={alt}
            className="h-auto max-h-[300px] max-w-full rounded-md shadow-sm"
          />
        </span>,
      );
    } else {
      nodes.push(
        <span className="my-2 block" key={`attachment-${tokenIndex}`}>
          <a
            href={url}
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 underline"
          >
            Attached Media ({mediaType})
          </a>
        </span>,
      );
    }

    cursor = offset + match[0].length;
    tokenIndex += 1;
  }

  nodes.push(...textWithLineBreaks(content.slice(cursor), `text-${tokenIndex}`));
  return nodes;
}

function formatStatus(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (normalized === "auto_replied") return "✨ AI Handled";
  return status || "Open";
}

function CustomerContextCard({ customerId }: { customerId: string }) {
  const [summary, setSummary] = useState<any>(null);

  useEffect(() => {
    async function fetchSummary() {
      try {
        const res = await fetch(`/api/v1/memory/summary/${customerId}`);
        if (res.ok) {
          const data = await res.json();
          setSummary(data);
        }
      } catch (err) {
        console.error("Failed to fetch customer memory summary:", err);
      }
    }
    fetchSummary();
  }, [customerId]);

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
  sendMessage,
  sourceLabel,
}: {
  messages: Message[];
  sendMessage: (msg: string, conversation_id: string) => void;
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
        const res = await fetch(`/api/v1/agents/approvals?limit=50`);
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
  const unreadLeadsCount = messages.filter((message) => (message.status || "").toLowerCase() === "unread").length;

  async function handleDraftQuoteWithAI(message: Message) {
    try {
      setActionStatus("Drafting quote with AI...");
      const res = await fetch("/api/v1/quotes/draft_agent", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          inquiry: message.content || "",
          customer_id: message.customer_id || message.sender_id || "unknown",
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
    if (!manualReply.trim() || !selected?.conversation_id) return;
    sendMessage(manualReply, selected.conversation_id);
    setManualReply("");
  }

  async function handleApproveAndSend(inboxMessageId: string) {
    try {
      setActionStatus("Approving and sending...");

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

      const approveRes = await fetch(`/api/v1/agents/approvals/${approval.id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ approved: true })
      });

      if (approveRes.ok) {
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
      subtitle="Native Rust omnichannel chat engine unified conversations."
      metrics={[
        { label: "Messages", value: String(messages.length), tone: messages.length > 0 ? "good" : "neutral" },
        { label: "Unread Leads", value: String(unreadLeadsCount), tone: unreadLeadsCount > 0 ? "warn" : "neutral" },
      ]}
    >
      <div className="flex flex-col gap-6">
        <div className="flex items-center justify-between">
          <p className="text-sm font-medium text-gray-500">{sourceLabel}</p>
        </div>

        {actionStatus && (
          <div className="rounded border-l-4 border-blue-500 bg-blue-50 p-4 text-blue-700 shadow-sm transition-all">
            <p>{actionStatus}</p>
          </div>
        )}

        <div className="flex h-[700px] gap-6" data-testid="inbox-settled">
          <section className="app-panel flex w-[40%] min-w-[300px] flex-col overflow-hidden">
            <div className="app-panel-header border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] p-4">
              <div className="app-panel-title font-bold text-gray-900 dark:text-white">Message Queue</div>
            </div>
            {messages.length === 0 ? (
              <div className="app-empty p-8 text-center text-gray-500">Inbox is empty.</div>
            ) : (
              <ul className="flex-1 overflow-y-auto divide-y divide-[rgba(255,255,255,0.1)] dark:divide-[rgba(255,255,255,0.05)] bg-[rgba(255,255,255,0.1)] dark:bg-[rgba(0,0,0,0.1)]">
                {messages.map((message) => {
                  const isSelected = selected?.id === message.id;
                  const isUnread = (message.status || "").toLowerCase() === "unread";
                  const messagePreview = (message.content || "").substring(0, 60);

                  return (
                    <li key={message.id}>
                      <button
                        type="button"
                        onClick={() => setSelectedId(message.id)}
                        className={`w-full text-left p-4 hover:bg-[rgba(255,255,255,0.3)] dark:hover:bg-[rgba(255,255,255,0.05)] transition-colors ${
                          isSelected ? "bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(255,255,255,0.1)] border-l-4 border-blue-500" : ""
                        }`}
                      >
                        <div className="flex justify-between items-start mb-1">
                          <span className={`font-semibold text-sm ${isUnread ? "text-gray-900 dark:text-white font-bold" : "text-gray-700 dark:text-gray-300"}`}>
                            {message.sender_id || "Unknown Sender"}
                          </span>
                          <span className="text-xs text-gray-500">
                            {message.source}
                          </span>
                        </div>
                        <div className={`text-sm mb-2 ${isUnread ? "text-gray-800 dark:text-gray-200 font-medium" : "text-gray-600 dark:text-gray-400"}`}>
                          {messagePreview} {message.content && message.content.length > 60 && "..."}
                        </div>
                        <div className="flex justify-between items-center mt-2">
                           <span className={`app-badge ${badgeTone(message.status)} text-xs px-2 py-0.5 rounded-full font-medium`}>{formatStatus(message.status)}</span>
                        </div>
                      </button>
                    </li>
                  );
                })}
              </ul>
            )}
          </section>

          <section className="app-panel flex flex-1 flex-col overflow-hidden">
            <div className="app-panel-header border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] p-4">
              <div className="app-panel-title font-bold text-gray-900 dark:text-white">Conversation Detail</div>
            </div>
            {!selected ? (
              <div className="app-empty p-8 text-center text-gray-500">Select a message to inspect it.</div>
            ) : (
              <div className="app-panel-body p-5 flex-1 flex flex-col">
                <div className="flex-1 overflow-y-auto">
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
                      <CustomerContextCard customerId={selected.customer_id} />
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
                        <div>{renderMessageContent((showOriginal ? selected.original_content : selected.content) || "Empty message")}</div>
                      </div>
                    </div>

                    <div className="mb-4">
                      <div className="app-metric-label">Draft Reply</div>
                      <div className="mt-2 rounded-md border border-gray-200 bg-white p-3 text-sm leading-6 text-gray-800">
                        <div>{renderMessageContent(selected.draft_reply || "No draft reply stored for this message.")}</div>
                      </div>
                    </div>

                    <div className="grid grid-cols-2 gap-3 mb-6">
                      <div className="app-card">
                        <div className="app-metric-label">Status</div>
                        <div className="mt-2"><span className={`app-badge ${badgeTone(selected.status)}`}>{formatStatus(selected.status)}</span></div>
                      </div>
                      <div className="app-card">
                        <div className="app-metric-label">Created</div>
                        <div className="mt-2 text-sm font-semibold text-gray-900">{selected.created_at || "Unknown"}</div>
                      </div>
                    </div>
                </div>

                <div className="mt-4 border-t pt-4">
                  <div className="flex gap-2">
                    <input
                      type="text"
                      className="flex-1 app-input px-3 py-2 text-black bg-white rounded-md border"
                      placeholder="Type a manual reply..."
                      value={manualReply}
                      onChange={(e) => setManualReply(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter") handleSendManualReply(selected.id);
                      }}
                    />
                    <button
                      className="app-button primary px-4 py-2 bg-blue-600 text-white rounded-md min-h-[44px] min-w-[44px]"
                      onClick={() => handleSendManualReply(selected.id)}
                    >
                      Send
                    </button>
                  </div>
                </div>

                {badgeTone(selected.status) === "warn" && (
                  <div className="mt-4">
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

                      const isInventoryDeduction = selected.draft_reply?.includes("[Send & Deduct Inventory]");
                      if (isInventoryDeduction) {
                        return (
                          <button
                            className="app-button primary w-full min-h-[44px] min-w-[44px] backdrop-filter bg-white/10 glassmorphism shadow-lg bg-gradient-to-r from-green-500/80 to-emerald-600/80 text-white font-bold border border-white/20"
                            onClick={() => handleApproveAndSend(selected.id)}
                          >
                            ✨ Approve & Send (Deduct Inventory)
                          </button>
                        );
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

function InboxLoadingState() {
  return (
    <AppShell title="Unified Inbox" subtitle="Native Rust omnichannel chat engine unified conversations.">
      <div className="app-panel">
        <div className="app-empty">Loading inbox messages...</div>
      </div>
    </AppShell>
  );
}

export default function InboxPage() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    // Initial fetch from REST API if we wanted to get history
    // Since we are moving to native chat, let's load initial from WS replay or rest API
    async function loadMessages() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/v1/ui/omni_inbox`);
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

    // Connect to WebSocket
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/ws/chat`;
    const ws = new WebSocket(wsUrl);
    wsRef.current = ws;

    ws.onopen = () => {
      console.log('Connected to Chat WS');
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.action === "new_message" && data.message) {
          // Normalize incoming native chat message to UI Message type
          const newMsg: Message = {
            id: data.message.id,
            content: data.message.content,
            source: data.message.sender_type,
            status: 'open',
            sender_id: data.message.sender_id,
            conversation_id: data.message.conversation_id,
            created_at: data.message.created_at,
          };
          setMessages(prev => [newMsg, ...prev]);
        }
      } catch (e) {
        console.error("Failed to parse WS message", e);
      }
    };

    ws.onerror = (e) => {
      console.error('Chat WS error', e);
    };

    ws.onclose = () => {
      console.log('Chat WS closed');
    };

    return () => {
      ws.close();
    };
  }, []);

  const sendMessage = (content: string, conversation_id: string) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify({
        action: "send_message",
        conversation_id,
        content
      }));
    }
  };

  if (error) {
    return (
      <AppShell title="Unified Inbox" subtitle="Native Rust omnichannel chat engine unified conversations.">
        <div className="app-panel" data-testid="inbox-settled">
          <div className="app-empty">{error}</div>
        </div>
      </AppShell>
    );
  }

  if (loading) {
    return <InboxLoadingState />;
  }

  return <InboxWorkspace messages={messages} sendMessage={sendMessage} sourceLabel="Live inbox messages for the current tenant." />;
}
