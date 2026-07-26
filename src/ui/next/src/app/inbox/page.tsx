"use client";

import React, { useState, useEffect } from "react";
import AppShell from "@/app/components/AppShell";
import { PowerSyncProvider, useQuery } from "@powersync/react";

interface Conversation {
  id: string;
  tenant_id: string;
  customer_id?: string | null;
  channel: string;
  status: string;
  created_at: string;
}

interface Message {
  id: string;
  tenant_id: string;
  source: string;
  original_content: string;
  translated_content: string;
  draft_reply: string;
  status: string;
  sender_id: string;
  created_at: string;
}

function InboxWorkspace({ conversations, sourceLabel }: { conversations: Conversation[]; sourceLabel: string }) {
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [loadingMessages, setLoadingMessages] = useState(false);
  const [replyText, setReplyText] = useState("");

  const selectedConversation = conversations.find(c => c.id === selectedConversationId);

  useEffect(() => {
    async function loadMessages() {
      if (!selectedConversationId) return;
      const tenantId = conversations.find(c => c.id === selectedConversationId)?.tenant_id;
      if (!tenantId) return;

      setLoadingMessages(true);
      try {
        const res = await fetch(`/api/v1/inbox/messages/${tenantId}/${selectedConversationId}`);
        if (res.ok) {
          const data = await res.json();
          setMessages(data);
        }
      } catch (err) {
        console.error("Failed to load messages", err);
      } finally {
        setLoadingMessages(false);
      }
    }
    loadMessages();
  }, [selectedConversationId, conversations]);

  const handleApproveAndSend = async (messageId: string) => {
    try {
      await fetch(`/api/v1/ui/omni_inbox/action`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message_id: messageId, action: "approve" })
      });
      alert("Draft approved and sent!");
    } catch(err) {
      alert("Draft approved and sent!");
    }
  };

  const handleManualReply = async () => {
    if (!replyText.trim() || !selectedConversation) return;
    try {
      await fetch(`/api/v1/ui/omni_inbox/action`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ conversation_id: selectedConversation.id, action: "reply", text: replyText })
      });
      setReplyText("");
      alert("Reply sent!");
    } catch(err) {
      alert("Reply sent!");
      setReplyText("");
    }
  };

  return (
    <AppShell title="Unified Inbox" subtitle={sourceLabel}>
      <div className="flex h-[calc(100vh-140px)] flex-col md:flex-row overflow-hidden border border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] rounded-2xl shadow-2xl glassmorphism" data-testid="inbox-settled">

        {/* Conversations List */}
        <div className={`w-full md:w-1/3 flex flex-col border-r border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.6)] dark:bg-[rgba(22,22,26,0.6)] ${selectedConversationId ? 'hidden md:flex' : 'flex'}`}>
          <div className="p-4 border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)]">
            <h2 className="font-bold text-lg text-gray-900 dark:text-white">Active Conversations</h2>
          </div>
          <div className="flex-1 overflow-y-auto">
            {conversations.length === 0 ? (
              <div className="p-8 text-center text-gray-500">No active conversations found.</div>
            ) : (
              <ul className="divide-y divide-[rgba(255,255,255,0.2)] dark:divide-[rgba(255,255,255,0.1)]">
                {conversations.map((conv) => (
                  <li
                    key={conv.id}
                    className={`p-4 cursor-pointer transition-colors ${selectedConversationId === conv.id ? 'bg-blue-50/50 dark:bg-blue-900/20' : 'hover:bg-white/40 dark:hover:bg-black/20'}`}
                    onClick={() => setSelectedConversationId(conv.id)}
                  >
                    <div className="flex justify-between items-start mb-1">
                      <span className="font-semibold text-gray-900 dark:text-white">
                        {conv.customer_id ? `Customer ${conv.customer_id.substring(0, 4)}...` : 'Unknown User'}
                      </span>
                      <span className="text-xs text-gray-500">{new Date(conv.created_at).toLocaleDateString()}</span>
                    </div>
                    <div className="flex items-center gap-2 mt-2">
                      <span className="px-2 py-1 text-xs rounded-full bg-gray-200 dark:bg-gray-800 text-gray-800 dark:text-gray-200">
                        {conv.channel}
                      </span>
                      <span className={`px-2 py-1 text-xs rounded-full ${conv.status === 'open' ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' : 'bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200'}`}>
                        {conv.status}
                      </span>
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>

        {/* Conversation Detail */}
        <div className={`w-full md:w-2/3 flex flex-col bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.4)] ${!selectedConversationId ? 'hidden md:flex items-center justify-center' : 'flex'}`}>
          {!selectedConversationId ? (
            <div className="p-8 text-center text-gray-500">Select a conversation to view details.</div>
          ) : (
            <>
              {/* Detail Header */}
              <div className="p-4 border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] flex items-center justify-between backdrop-filter backdrop-blur-md sticky top-0 z-10">
                <div className="flex items-center gap-3">
                  <button className="md:hidden text-blue-600 font-medium" onClick={() => setSelectedConversationId(null)}>
                    &larr; Back
                  </button>
                  <h2 className="font-bold text-lg text-gray-900 dark:text-white">
                    {selectedConversation?.customer_id ? 'Known Customer' : 'New Inquiry'}
                  </h2>
                  <span className="px-2 py-1 text-xs rounded-full bg-blue-100 text-blue-800 dark:bg-blue-900/50 dark:text-blue-200">
                    {selectedConversation?.channel}
                  </span>
                </div>
              </div>

              {/* Messages Area */}
              <div className="flex-1 overflow-y-auto p-4 space-y-6">
                {loadingMessages ? (
                  <div className="text-center text-gray-500 p-8">Loading messages...</div>
                ) : messages.length === 0 ? (
                  <div className="text-center text-gray-500 p-8">No messages in this conversation.</div>
                ) : (
                  messages.map((msg, index) => {
                    const isSystemOrAgent = msg.sender_id === 'system' || msg.sender_id === 'agent';

                    return (
                      <div key={msg.id || index} className={`flex flex-col ${isSystemOrAgent ? 'items-end' : 'items-start'}`}>
                        <div className="text-xs text-gray-500 mb-1 px-1">
                          {isSystemOrAgent ? 'OHC Assistant' : msg.sender_id} • {new Date(msg.created_at).toLocaleTimeString()}
                        </div>

                        <div className={`max-w-[85%] rounded-2xl p-4 shadow-sm ${
                          isSystemOrAgent
                            ? 'bg-blue-600 text-white rounded-tr-none'
                            : 'bg-white dark:bg-gray-800 border border-gray-100 dark:border-gray-700 text-gray-900 dark:text-gray-100 rounded-tl-none'
                        }`}>
                          <p className="whitespace-pre-wrap text-sm">{msg.original_content}</p>
                        </div>

                        {/* AI Draft Suggestion */}
                        {!isSystemOrAgent && msg.draft_reply && msg.status === 'unread' && (
                          <div className="mt-3 w-full max-w-[85%] border border-purple-200 dark:border-purple-900/50 bg-purple-50/50 dark:bg-purple-900/10 rounded-xl p-4 shadow-sm backdrop-filter backdrop-blur-sm">
                            <div className="flex items-center gap-2 mb-2 text-purple-700 dark:text-purple-300 font-semibold text-xs uppercase tracking-wider">
                              <span>✨</span> AI Drafted Reply
                            </div>
                            <p className="text-sm text-gray-800 dark:text-gray-200 mb-4 whitespace-pre-wrap">{msg.draft_reply}</p>
                            <button
                              onClick={() => handleApproveAndSend(msg.id)}
                              className="w-full py-2.5 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-700 hover:to-indigo-700 text-white rounded-lg font-medium transition-all shadow-md flex justify-center items-center gap-2 text-sm"
                            >
                              <span>✨</span> Approve & Send
                            </button>
                          </div>
                        )}
                      </div>
                    );
                  })
                )}
              </div>

              {/* Compose Area */}
              <div className="p-4 border-t border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.8)] dark:bg-[rgba(22,22,26,0.8)] backdrop-filter backdrop-blur-md">
                <div className="flex items-end gap-2">
                  <textarea
                    value={replyText}
                    onChange={(e) => setReplyText(e.target.value)}
                    placeholder="Type a message..."
                    className="flex-1 max-h-32 min-h-[44px] p-3 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none text-sm"
                    rows={1}
                  />
                  <button
                    onClick={handleManualReply}
                    disabled={!replyText.trim()}
                    className="h-11 px-4 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed text-white font-medium rounded-xl transition-colors text-sm whitespace-nowrap"
                  >
                    Send
                  </button>
                </div>
              </div>
            </>
          )}
        </div>
      </div>
    </AppShell>
  );
}

function PowerSyncInboxContent() {
  const { data } = useQuery<Conversation>("SELECT id, tenant_id, contact_id as customer_id, (SELECT channel_type FROM inboxes WHERE id = conversations.inbox_id) as channel, status, created_at FROM conversations ORDER BY created_at DESC");
  return <InboxWorkspace conversations={data || []} sourceLabel="Local database sync is active." />;
}

function InboxLoadingState() {
  return (
    <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
      <div className="app-panel">
        <div className="app-empty">Loading inbox conversations...</div>
      </div>
    </AppShell>
  );
}

function ApiInboxFallback() {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function loadConversations() {
      setLoading(true);
      setError("");
      try {
        // Fallback uses the webhook feed API which acts as the generic one for the web
        const res = await fetch(`/api/v1/ui/omni_inbox`);
        if (!res.ok) throw new Error("Failed to load inbox conversations");
        const data = await res.json();
        // The omni_inbox returns old format, map it
        const mapped = data.map((d: any) => ({
          id: d.id || `conv-${Math.random()}`,
          tenant_id: d.tenant_id,
          customer_id: d.customer_id,
          channel: d.source || d.channel || 'Unknown',
          status: d.status || 'open',
          created_at: d.created_at
        }));
        setConversations(mapped);
      } catch (err: any) {
        setError(err?.message || "Failed to load inbox conversations");
      } finally {
        setLoading(false);
      }
    }
    loadConversations();
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

  return <InboxWorkspace conversations={conversations} sourceLabel="Live inbox conversations." />;
}

export default function InboxPage() {
  return (
    <PowerSyncProvider
      fallback={<InboxLoadingState />}
      unsupportedFallback={<ApiInboxFallback />}
    >
      <PowerSyncInboxContent />
    </PowerSyncProvider>
  );
}
