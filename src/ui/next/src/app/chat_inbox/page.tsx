"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type ChatConversation = {
  id: string;
  tenant_id: string;
  inbox_id: string;
  contact_id: string;
  assignee_id: string | null;
  status: string;
  created_at: string;
  updated_at: string;
};

type ChatMessage = {
  id: string;
  tenant_id: string;
  conversation_id: string;
  sender_type: string;
  sender_id: string | null;
  content: string;
  created_at: string;
  updated_at: string;
};

export default function ChatInboxPage() {
  const [conversations, setConversations] = useState<ChatConversation[]>([]);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [selectedConversation, setSelectedConversation] = useState<ChatConversation | null>(null);
  const [newMessage, setNewMessage] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchConversations();
  }, []);

  const fetchConversations = async () => {
    setLoading(true);
    try {
      const res = await fetch("/api/v1/ui/chat_inbox/conversations");
      if (res.ok) {
        const data = await res.json();
        setConversations(data);
      }
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  };

  const selectConversation = async (conv: ChatConversation) => {
    setSelectedConversation(conv);
    try {
      const res = await fetch(`/api/v1/ui/chat_inbox/conversations/${conv.id}/messages`);
      if (res.ok) {
        const data = await res.json();
        setMessages(data);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleSendMessage = async () => {
    if (!selectedConversation || !newMessage.trim()) return;
    try {
      const res = await fetch(`/api/v1/ui/chat_inbox/conversations/${selectedConversation.id}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          sender_type: "agent",
          content: newMessage,
        }),
      });
      if (res.ok) {
        setNewMessage("");
        selectConversation(selectedConversation); // refresh
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <AppShell title="Omnichannel Chat Inbox" subtitle="Manage all customer interactions seamlessly">
      <div className="flex flex-col md:flex-row gap-4 h-[calc(100vh-140px)]">
        {/* Sidebar */}
        <section className="w-full md:w-1/3 flex flex-col gap-2 overflow-y-auto">
          {loading ? (
            <div className="text-gray-500 text-sm">Loading conversations...</div>
          ) : conversations.length === 0 ? (
            <div className="text-gray-500 text-sm">No conversations found.</div>
          ) : (
            conversations.map((conv) => (
              <div
                key={conv.id}
                onClick={() => selectConversation(conv)}
                className={`p-4 rounded-xl cursor-pointer border backdrop-filter backdrop-blur-md transition-colors ${
                  selectedConversation?.id === conv.id
                    ? "bg-blue-50/50 border-blue-200 shadow-sm"
                    : "bg-white/40 border-white/20 hover:bg-white/60"
                }`}
                data-testid={`conversation-${conv.id}`}
              >
                <div className="font-semibold text-gray-900 text-sm">Conversation {conv.id.substring(0, 8)}...</div>
                <div className="text-xs text-gray-500 mt-1">Status: {conv.status}</div>
              </div>
            ))
          )}
        </section>

        {/* Main Chat Area */}
        <section className="w-full md:w-2/3 flex flex-col bg-white/40 backdrop-filter backdrop-blur-md rounded-xl border border-white/20 overflow-hidden relative">
          {selectedConversation ? (
            <>
              {/* Header */}
              <div className="p-4 border-b border-white/20 bg-white/30 font-semibold text-gray-800">
                Conversation {selectedConversation.id.substring(0, 8)}...
              </div>

              {/* Messages */}
              <div className="flex-1 overflow-y-auto p-4 space-y-4">
                {messages.length === 0 ? (
                  <div className="text-gray-500 text-sm text-center mt-10">No messages yet.</div>
                ) : (
                  messages.map((msg) => (
                    <div
                      key={msg.id}
                      className={`flex ${msg.sender_type === 'agent' ? 'justify-end' : 'justify-start'}`}
                    >
                      <div
                        className={`max-w-[75%] p-3 rounded-2xl text-sm shadow-sm ${
                          msg.sender_type === 'agent'
                            ? 'bg-blue-500 text-white rounded-tr-none'
                            : 'bg-white/80 text-gray-800 rounded-tl-none border border-white/40'
                        }`}
                      >
                        {msg.content}
                      </div>
                    </div>
                  ))
                )}
              </div>

              {/* Input Area */}
              <div className="p-4 bg-white/30 border-t border-white/20 flex gap-2 items-center">
                <input
                  type="text"
                  value={newMessage}
                  onChange={(e) => setNewMessage(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleSendMessage()}
                  placeholder="Type a message..."
                  className="flex-1 px-4 py-2 rounded-full border border-gray-200 bg-white/50 focus:outline-none focus:ring-2 focus:ring-blue-400 text-sm"
                  data-testid="chat-input"
                />
                <button
                  onClick={handleSendMessage}
                  disabled={!newMessage.trim()}
                  className="px-4 py-2 bg-blue-500 text-white font-semibold rounded-full text-sm disabled:opacity-50 hover:bg-blue-600 transition-colors"
                  data-testid="chat-send"
                >
                  Send
                </button>
              </div>
            </>
          ) : (
            <div className="flex-1 flex items-center justify-center text-gray-500 text-sm">
              Select a conversation to view messages.
            </div>
          )}
        </section>
      </div>
    </AppShell>
  );
}
