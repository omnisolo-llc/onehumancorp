"use client";

import { useEffect, useState, ReactNode } from "react";
import { AppShell } from "../components/AppShell";
import { useQuery } from "@powersync/react";
import { PowerSyncProvider } from "../../lib/powersync/PowerSyncProvider";


type ChatConversation = {
  id: string;
  tenant_id: string;
  inbox_id: string;
  contact_id: string;
  assignee_id?: string;
  status: string;
  created_at: string;
  updated_at: string;
};

type ChatMessage = {
  id: string;
  tenant_id: string;
  conversation_id: string;
  sender_type: string;
  sender_id?: string;
  content: string;
  status: string;
  created_at: string;
  updated_at: string;
};

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved"].includes(normalized)) return "good";
  if (["open", "pending", "unread", ""].includes(normalized)) return "warn";
  if (["failed", "blocked"].includes(normalized)) return "bad";
  if (["draft"].includes(normalized)) return "info";
  return "";
}

function InboxWorkspace({ conversations, sourceLabel }: { conversations: ChatConversation[]; sourceLabel: string }) {
  const [selectedConv, setSelectedConv] = useState<ChatConversation | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [draftInput, setDraftInput] = useState("");
  const [loadingMessages, setLoadingMessages] = useState(false);

  useEffect(() => {
    if (selectedConv) {
      setLoadingMessages(true);
      fetch(`/api/v1/ui/chat/conversations/${selectedConv.id}/messages`, {
        headers: {
            "X-Dummy": `Bearer ${window.localStorage.getItem('dummy_token') || 'test-token'}`
        }
      })
        .then((res) => res.json())
        .then((data) => {
          setMessages(Array.isArray(data) ? data : []);
          setLoadingMessages(false);
        })
        .catch(() => setLoadingMessages(false));
    }
  }, [selectedConv]);

  const handleSendDraft = async (msgId: string) => {
    if (!selectedConv) return;
    const msg = messages.find((m) => m.id === msgId);
    if (!msg) return;

    try {
      const res = await fetch(`/api/v1/ui/chat/conversations/${selectedConv.id}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          content: msg.content,
          sender_type: "agent",
          sender_id: null,
        }),
      });
      if (res.ok) {
        const newMsg = await res.json();
        // Remove draft, add new sent msg
        setMessages((prev) => prev.filter((m) => m.id !== msgId).concat(newMsg));
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleSendManual = async () => {
    if (!selectedConv || !draftInput.trim()) return;

    try {
      const res = await fetch(`/api/v1/ui/chat/conversations/${selectedConv.id}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          content: draftInput,
          sender_type: "agent",
          sender_id: null,
        }),
      });
      if (res.ok) {
        const newMsg = await res.json();
        setMessages((prev) => prev.concat(newMsg));
        setDraftInput("");
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <AppShell title="Unified Inbox" subtitle={sourceLabel}>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-6xl mx-auto mt-4 px-4">

        {/* Left pane: Conversations List */}
        <section className="app-panel md:col-span-1 rounded-[16px] overflow-hidden bg-white/60 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col h-[70vh]">
          <div className="border-b border-white/20 dark:border-white/10 p-4 sticky top-0 bg-white/40 dark:bg-black/40 backdrop-blur-md">
            <h2 className="font-bold text-gray-900 dark:text-white">Active Threads</h2>
          </div>
          <div className="flex-1 overflow-y-auto p-2">
            {conversations.length === 0 ? (
              <div className="p-4 text-center text-sm text-gray-500">No active conversations.</div>
            ) : (
              <ul className="space-y-2">
                {conversations.map((conv) => (
                  <li key={conv.id}>
                    <button
                      onClick={() => setSelectedConv(conv)}
                      className={`w-full text-left p-3 rounded-xl transition-all border ${
                        selectedConv?.id === conv.id
                          ? "bg-blue-50/80 border-blue-200 dark:bg-blue-900/30 dark:border-blue-500/30"
                          : "bg-white/40 border-transparent hover:bg-white/80 dark:bg-black/20 dark:hover:bg-white/5"
                      }`}
                    >
                      <div className="flex justify-between items-center mb-1">
                        <span className="font-semibold text-sm text-gray-900 dark:text-gray-100 truncate">
                          {conv.contact_id || "Unknown Contact"}
                        </span>
                        <span className={`app-badge ${badgeTone(conv.status)} text-[10px]`}>
                          {conv.status}
                        </span>
                      </div>
                      <div className="text-xs text-gray-500 truncate">
                        {new Date(conv.updated_at).toLocaleString()}
                      </div>
                    </button>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </section>

        {/* Right pane: Conversation Detail & Messages */}
        <section className="app-panel md:col-span-2 rounded-[16px] overflow-hidden bg-white/60 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col h-[70vh]">
          {selectedConv ? (
            <>
              <div className="border-b border-white/20 dark:border-white/10 p-4 sticky top-0 bg-white/40 dark:bg-black/40 backdrop-blur-md flex justify-between items-center">
                <h2 className="font-bold text-gray-900 dark:text-white">Thread Detail</h2>
              </div>
              <div className="flex-1 overflow-y-auto p-4 space-y-4">
                {loadingMessages ? (
                  <div className="text-center text-gray-500 text-sm">Loading messages...</div>
                ) : messages.length === 0 ? (
                  <div className="text-center text-gray-500 text-sm">No messages yet.</div>
                ) : (
                  messages.map((msg) => {
                    const isDraft = msg.status === "draft";
                    const isAgent = msg.sender_type === "agent" || msg.sender_type === "bot";

                    if (isDraft) {
                      return (
                        <div key={msg.id} className="p-4 rounded-xl bg-gradient-to-r from-purple-50 to-indigo-50 dark:from-purple-900/20 dark:to-indigo-900/20 border border-purple-200 dark:border-purple-500/30 shadow-sm my-4">
                          <div className="flex justify-between items-center mb-2">
                            <span className="text-xs font-bold text-purple-600 dark:text-purple-400">✨ AI Suggested Reply</span>
                          </div>
                          <p className="text-sm text-gray-800 dark:text-gray-200 mb-4">{msg.content}</p>
                          <button
                            onClick={() => handleSendDraft(msg.id)}
                            className="w-full min-h-[44px] rounded-lg bg-blue-600 hover:bg-blue-700 text-white font-bold transition-colors"
                          >
                            Send Draft
                          </button>
                        </div>
                      );
                    }

                    return (
                      <div key={msg.id} className={`flex flex-col ${isAgent ? "items-end" : "items-start"}`}>
                        <span className="text-[10px] text-gray-500 mb-1 px-1">
                           {isAgent ? "You" : "Customer"} • {new Date(msg.created_at).toLocaleTimeString()}
                        </span>
                        <div className={`p-3 rounded-2xl max-w-[85%] text-sm shadow-sm ${
                          isAgent
                           ? "bg-blue-600 text-white rounded-br-none"
                           : "bg-white/80 dark:bg-gray-800/80 border border-black/5 dark:border-white/5 rounded-bl-none text-gray-900 dark:text-gray-100"
                        }`}>
                          {msg.content}
                        </div>
                      </div>
                    );
                  })
                )}
              </div>
              <div className="p-4 border-t border-white/20 dark:border-white/10 bg-white/40 dark:bg-black/40 backdrop-blur-md">
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={draftInput}
                    onChange={(e) => setDraftInput(e.target.value)}
                    placeholder="Type your reply..."
                    className="flex-1 min-h-[44px] px-4 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#2c2c2e] focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                  />
                  <button
                    onClick={handleSendManual}
                    className="min-h-[44px] px-6 rounded-xl bg-blue-600 hover:bg-blue-700 text-white font-bold transition-colors"
                  >
                    Send
                  </button>
                </div>
              </div>
            </>
          ) : (
            <div className="flex-1 flex items-center justify-center p-8 text-center text-gray-500">
              Select a conversation to view messages.
            </div>
          )}
        </section>
      </div>
    </AppShell>
  );
}

function ApiInboxFallback() {
  const [conversations, setConversations] = useState<ChatConversation[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function loadConversations() {
      setLoading(true);
      setError("");
      try {
        const session = null;
        const token = session?.tokens?.accessToken?.toString() || 'test-token';

        const res = await fetch(`/api/v1/ui/chat/conversations`, {
          headers: {
            "X-Dummy": `Bearer ${token}`
          }
        });
        if (!res.ok) throw new Error("Failed to load active chat threads");
        const data = await res.json();
        setConversations(Array.isArray(data) ? data : []);
      } catch (err: any) {
        setError(err?.message || "Failed to load chat conversations");
      } finally {
        setLoading(false);
      }
    }
    loadConversations();
  }, []);

  if (error) {
    return (
      <AppShell title="Unified Inbox" subtitle="Native Omnichannel Chat">
        <div className="app-panel" data-testid="inbox-settled">
          <div className="app-empty p-8 text-red-500">{error}</div>
        </div>
      </AppShell>
    );
  }

  if (loading) {
    return (
      <AppShell title="Unified Inbox" subtitle="Native Omnichannel Chat">
        <div className="app-panel h-[60vh] flex items-center justify-center">
          <div className="text-gray-500">Loading inbox threads...</div>
        </div>
      </AppShell>
    );
  }

  return <InboxWorkspace conversations={conversations} sourceLabel="Native Omnichannel Threads" />;
}

export default function InboxPage() {
  // Use native Next.js fallback since we're replacing the previous OmniInbox system
  return <ApiInboxFallback />;
}
