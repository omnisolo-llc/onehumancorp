"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';
import { useQuery, PowerSyncProvider } from '@powersync/react';
import { CustomerContextCard } from '../components/CustomerContextCard';

export interface Message {
  id: string;
  source: string;
  content: string;
  original_content?: string;
  translated_content?: string;
  translated_from_language?: string;
  draft_reply?: string;
  status: string;
  sender_id?: string;
  customer_id?: string;
  created_at: string;
  checkout_link?: string;
  proposed_product_id?: string;
}

function badgeTone(status: string) {
  switch (status.toLowerCase()) {
    case "unread": return "warn";
    case "resolved": return "good";
    case "auto_replied": return "good";
    case "auto_drafted": return "warn";
    case "draft_approved": return "good";
    case "pending_approval": return "warn";
    case "replied": return "good";
    default: return "neutral";
  }
}

function formatStatus(status: string) {
  return status.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
}

function InboxWorkspace({ messages, sourceLabel }: { messages: Message[], sourceLabel: string }) {
  const [selected, setSelected] = useState<Message | null>(null);
  const [activeApproval, setActiveApproval] = useState<any>(null);
  const [showOriginal, setShowOriginal] = useState(false);

  useEffect(() => {
    if (selected) {
      setShowOriginal(false);
      // Fetch any pending approval tasks for this message
      fetch(`/api/v1/inbox/approvals?message_id=${selected.id}`)
        .then(res => res.json())
        .then(data => {
            if (data && data.length > 0) {
               setActiveApproval(data[0]);
            } else {
               setActiveApproval(null);
            }
        })
        .catch(err => {
            console.error(err);
            setActiveApproval(null);
        });
    } else {
      setActiveApproval(null);
    }
  }, [selected]);

  const handleApproveAndSend = async (messageId: string) => {
    try {
      const response = await fetch('/api/v1/inbox/approve', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ message_id: messageId }),
      });

      if (!response.ok) {
        throw new Error('Failed to approve message');
      }

      // Optimistically update UI
      if (selected && selected.id === messageId) {
         setSelected({ ...selected, status: 'replied' });
         setActiveApproval(null);
      }
    } catch (error) {
      console.error('Error approving message:', error);
      alert('Failed to approve message. Please try again.');
    }
  };

  const handleDraftQuoteWithAI = async (message: Message) => {
      alert("Drafting quote with AI...");
  };

  const renderMessageContent = (content: string) => {
    if (!content) return null;
    return content.split('\n').map((line, i) => (
      <React.Fragment key={i}>
        {line}
        <br />
      </React.Fragment>
    ));
  };

  return (
    <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
      <div className="flex flex-col h-[calc(100vh-140px)] w-full max-w-7xl mx-auto gap-4">
        {/* Connection status strip */}
        <div className="flex items-center gap-2 px-4 py-2 bg-green-50/50 dark:bg-green-900/10 border border-green-200 dark:border-green-800 rounded-lg shadow-sm">
          <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
          <span className="text-xs font-medium text-green-700 dark:text-green-400">
            {sourceLabel}
          </span>
        </div>

        <div className="flex flex-1 gap-6 min-h-0 overflow-hidden">
          {/* List Sidebar */}
          <section className="w-full md:w-[400px] flex-shrink-0 flex flex-col app-panel overflow-hidden" data-testid="inbox-settled">
            <div className="app-panel-header border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] p-4 sticky top-0 z-10">
              <div className="flex justify-between items-center">
                <div className="app-panel-title font-bold text-gray-900 dark:text-white">Conversations</div>
                <span className="app-badge neutral">{messages.length}</span>
              </div>
            </div>
            <div className="flex-1 overflow-y-auto">
              {messages.length === 0 ? (
                <div className="app-empty p-8 text-center text-gray-500">No messages found.</div>
              ) : (
                <ul className="divide-y divide-gray-100 dark:divide-gray-800">
                  {messages.map((msg) => (
                    <li key={msg.id}>
                      <button
                        className={`w-full text-left p-4 hover:bg-black/5 dark:hover:bg-white/5 transition-colors focus:outline-none focus:bg-black/5 dark:focus:bg-white/5 ${selected?.id === msg.id ? 'bg-[#0066FF]/10 dark:bg-[#0071E3]/20' : ''}`}
                        onClick={() => setSelected(msg)}
                      >
                        <div className="flex justify-between items-start mb-1 gap-2">
                          <span className="font-semibold text-gray-900 dark:text-[#F5F5F7] text-sm truncate flex-1">
                            {msg.sender_id || "Unknown"}
                          </span>
                          <span className={`flex-shrink-0 app-badge ${badgeTone(msg.status)}`}>
                            {formatStatus(msg.status)}
                          </span>
                        </div>
                        <p className="text-xs text-gray-500 line-clamp-2 mt-1 font-medium">{msg.content}</p>
                        <div className="flex items-center justify-between mt-3 pt-3 border-t border-black/5 dark:border-white/5">
                           <span className="text-[10px] text-gray-400 font-medium uppercase tracking-wider">{msg.source}</span>
                           <span className="text-[10px] text-gray-400 font-medium">{msg.created_at ? new Date(msg.created_at).toLocaleDateString() : ""}</span>
                        </div>
                      </button>
                    </li>
                  ))}
                </ul>
              )}
            </div>
          </section>

          {/* Detail View */}
          <section className="hidden md:flex flex-1 flex-col app-panel overflow-hidden">
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
                  {selected.checkout_link && (
                    <div className="mt-3 flex items-center bg-white/60 dark:bg-black/20 border border-black/5 dark:border-white/5 rounded-lg p-3 backdrop-filter backdrop-blur-md">
                      <div className="bg-blue-600 text-white rounded w-8 h-8 flex items-center justify-center font-bold mr-3">🛍️</div>
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-semibold text-gray-900 dark:text-white m-0">Product: {selected.proposed_product_id || "Checkout Link"}</p>
                        <a href={selected.checkout_link} target="_blank" rel="noreferrer" className="text-xs text-blue-600 dark:text-blue-400 truncate block">{selected.checkout_link}</a>
                      </div>
                    </div>
                  )}
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

function PowerSyncInboxContent() {
  const { data } = useQuery<Message>("SELECT * FROM omni_inbox_messages ORDER BY created_at DESC");
  return <InboxWorkspace messages={data || []} sourceLabel="Local database sync is active." />;
}

function InboxLoadingState() {
  return (
    <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
      <div className="app-panel">
        <div className="app-empty">Loading inbox messages...</div>
      </div>
    </AppShell>
  );
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
  }, []);

  if (error) {
    return (
      <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
        <div className="app-panel" data-testid="inbox-settled">
          <div className="app-empty">{error}</div>
        </div>
      </AppShell>
    );
  }

  if (loading) {
    return <InboxLoadingState />;
  }

  return <InboxWorkspace messages={messages} sourceLabel="Live inbox messages for the current tenant." />;
}

export default function InboxPage() {
  // Note: The native Omnichannel chat widget has been removed in favor of the new Web/Tauri implementation.
  // The backend was completely rewritten in Rust to remove the Chatwoot dependency.
  return (
    <PowerSyncProvider
      fallback={<InboxLoadingState />}
      unsupportedFallback={<ApiInboxFallback />}
    >
      <PowerSyncInboxContent />
    </PowerSyncProvider>
  );
}
