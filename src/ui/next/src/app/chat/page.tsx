"use client";

import { useEffect, useState, useRef } from "react";
import { AppShell } from "../components/AppShell";
import { format } from "date-fns";

type ChatMessage = {
  id: string;
  sender_type: string;
  content: string;
  created_at: string;
};

type ChatConversation = {
  id: string;
  contact_id?: string;
  status: string;
  created_at: string;
  updated_at: string;
  contact_name?: string; // Optional metadata for UI
};

export default function NativeChatInboxPage() {
  const [conversations, setConversations] = useState<ChatConversation[]>([]);
  const [activeConversation, setActiveConversation] = useState<ChatConversation | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [replyText, setReplyText] = useState("");
  const [loading, setLoading] = useState(true);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    async function loadConversations() {
      setLoading(true);
      try {
        const res = await fetch("/api/v1/chat-inbox/conversations");
        if (res.ok) {
          const data = await res.json();
          setConversations(data);
        }
      } catch (err) {
        console.error("Failed to load conversations:", err);
      } finally {
        setLoading(false);
      }
    }
    loadConversations();
  }, []);

  useEffect(() => {
    if (activeConversation) {
      loadMessages(activeConversation.id);
    }
  }, [activeConversation]);

  useEffect(() => {
    // Scroll to bottom when messages update
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  async function loadMessages(conversationId: string) {
    try {
      const res = await fetch(`/api/v1/chat-inbox/conversations/${conversationId}/messages`);
      if (res.ok) {
        const data = await res.json();
        setMessages(data);
      }
    } catch (err) {
      console.error("Failed to load messages:", err);
    }
  }

  async function handleSendReply() {
    if (!replyText.trim() || !activeConversation) return;

    try {
      const res = await fetch(`/api/v1/chat-inbox/conversations/${activeConversation.id}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          content: replyText,
          sender_type: "agent",
        }),
      });

      if (res.ok) {
        const newMsg = await res.json();
        setMessages((prev) => [...prev, newMsg]);
        setReplyText("");
      }
    } catch (err) {
      console.error("Failed to send reply:", err);
    }
  }

  return (
    <AppShell title="Conversations" subtitle="Your unified omnichannel customer inbox.">
      <div className="flex flex-col md:flex-row h-[calc(100vh-140px)] w-full overflow-hidden bg-white/60 dark:bg-black/20 glassmorphism rounded-xl border border-white/40 dark:border-white/10">

        {/* Sidebar / List View */}
        <div className={`w-full md:w-80 flex flex-col border-r border-black/5 dark:border-white/5 ${activeConversation ? "hidden md:flex" : "flex"}`}>
          <div className="p-4 border-b border-black/5 dark:border-white/5">
            <h2 className="font-outfit font-bold text-lg text-gray-900 dark:text-white">Active Chats</h2>
          </div>
          <div className="flex-1 overflow-y-auto">
            {loading ? (
              <div className="p-4 text-sm text-gray-500">Loading conversations...</div>
            ) : conversations.length === 0 ? (
              <div className="p-4 text-sm text-gray-500">No active conversations.</div>
            ) : (
              conversations.map((conv) => (
                <button
                  key={conv.id}
                  onClick={() => setActiveConversation(conv)}
                  className={`w-full text-left p-4 border-b border-black/5 dark:border-white/5 hover:bg-black/5 dark:hover:bg-white/5 transition-colors ${
                    activeConversation?.id === conv.id ? "bg-blue-50/50 dark:bg-blue-900/20" : ""
                  }`}
                  data-testid={`conversation-${conv.id}`}
                >
                  <div className="flex justify-between items-start mb-1">
                    <span className="font-semibold text-sm text-gray-900 dark:text-white truncate">
                      {conv.contact_name || "Unknown Customer"}
                    </span>
                    <span className="text-xs text-gray-500 whitespace-nowrap ml-2">
                      {conv.updated_at ? format(new Date(conv.updated_at), "MMM d") : ""}
                    </span>
                  </div>
                  <div className="text-xs text-gray-500 truncate">
                    Status: {conv.status}
                  </div>
                </button>
              ))
            )}
          </div>
        </div>

        {/* Detail / Thread View */}
        <div className={`flex-1 flex-col h-full bg-gray-50/30 dark:bg-gray-900/30 ${!activeConversation ? "hidden md:flex" : "flex"}`}>
          {activeConversation ? (
            <>
              {/* Thread Header */}
              <div className="p-4 border-b border-black/5 dark:border-white/5 flex items-center gap-3 bg-white/80 dark:bg-black/40 backdrop-blur-md z-10 sticky top-0">
                <button
                  className="md:hidden text-gray-500 hover:text-gray-900 dark:hover:text-white p-2 -ml-2"
                  onClick={() => setActiveConversation(null)}
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                </button>
                <div>
                  <h3 className="font-bold text-gray-900 dark:text-white">{activeConversation.contact_name || "Conversation Details"}</h3>
                  <p className="text-xs text-gray-500">{activeConversation.id}</p>
                </div>
              </div>

              {/* Messages Area */}
              <div className="flex-1 overflow-y-auto p-4 space-y-4" ref={scrollRef}>
                {messages.length === 0 ? (
                  <div className="text-center text-gray-500 text-sm py-10">No messages in this conversation yet.</div>
                ) : (
                  messages.map((msg) => {
                    const isOwner = msg.sender_type === "agent";
                    return (
                      <div key={msg.id} className={`flex w-full ${isOwner ? "justify-end" : "justify-start"}`}>
                        <div className={`max-w-[85%] rounded-2xl p-3 text-sm shadow-sm ${
                          isOwner
                            ? "bg-blue-600 text-white rounded-tr-none"
                            : "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-900 dark:text-gray-100 rounded-tl-none"
                        }`}>
                          {msg.content}
                          <div className={`text-[10px] mt-1 ${isOwner ? "text-blue-100/80" : "text-gray-400"}`}>
                            {msg.created_at ? format(new Date(msg.created_at), "h:mm a") : ""}
                          </div>
                        </div>
                      </div>
                    );
                  })
                )}
              </div>

              {/* Compose Area */}
              <div className="p-3 border-t border-black/5 dark:border-white/5 bg-white/80 dark:bg-black/40 backdrop-blur-md">
                <div className="flex gap-2 relative">
                  <textarea
                    className="flex-1 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 px-4 py-3 text-sm focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 resize-none"
                    placeholder="Type your reply..."
                    rows={1}
                    value={replyText}
                    onChange={(e) => setReplyText(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter" && !e.shiftKey) {
                        e.preventDefault();
                        handleSendReply();
                      }
                    }}
                    data-testid="chat-input"
                  />
                  <button
                    onClick={handleSendReply}
                    disabled={!replyText.trim()}
                    className="app-button primary rounded-xl px-4 flex items-center justify-center disabled:opacity-50"
                    data-testid="chat-send"
                  >
                    Send
                  </button>
                </div>
              </div>
            </>
          ) : (
            <div className="flex-1 flex items-center justify-center text-gray-500 text-sm">
              Select a conversation to view messages.
            </div>
          )}
        </div>
      </div>
    </AppShell>
  );
}
