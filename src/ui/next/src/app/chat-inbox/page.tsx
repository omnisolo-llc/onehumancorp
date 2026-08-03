"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type ChatInbox = {
  id: string;
  tenant_id: string;
  name: string;
  created_at: string;
  updated_at: string;
};

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

export default function NativeChatInboxPage() {
  const [inboxes, setInboxes] = useState<ChatInbox[]>([]);
  const [activeInboxId, setActiveInboxId] = useState<string | null>(null);
  const [conversations, setConversations] = useState<ChatConversation[]>([]);
  const [activeConversationId, setActiveConversationId] = useState<string | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [newMessage, setNewMessage] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchInboxes() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch("/api/v1/chat_engine/inboxes");
        if (!res.ok) throw new Error("Failed to fetch inboxes");
        const data = await res.json();
        setInboxes(data);
        if (data.length > 0) {
          setActiveInboxId(data[0].id);
        }
      } catch (err: any) {
        setError(err.message || "Failed to load inboxes");
      } finally {
        setLoading(false);
      }
    }
    fetchInboxes();
  }, []);

  useEffect(() => {
    if (!activeInboxId) return;
    async function fetchConversations() {
      try {
        const res = await fetch(`/api/v1/chat_engine/inboxes/${activeInboxId}/conversations`);
        if (!res.ok) throw new Error("Failed to fetch conversations");
        const data = await res.json();
        setConversations(data);
        if (data.length > 0) {
          setActiveConversationId(data[0].id);
        }
      } catch (err: any) {
        setError(err.message || "Failed to load conversations");
      }
    }
    fetchConversations();
  }, [activeInboxId]);

  useEffect(() => {
    if (!activeConversationId) return;
    async function fetchMessages() {
      try {
        const res = await fetch(`/api/v1/chat_engine/conversations/${activeConversationId}/messages`);
        if (!res.ok) throw new Error("Failed to fetch messages");
        const data = await res.json();
        setMessages(data);
      } catch (err: any) {
        setError(err.message || "Failed to load messages");
      }
    }
    fetchMessages();
  }, [activeConversationId]);

  const handleSendMessage = async () => {
    if (!newMessage.trim() || !activeConversationId) return;
    try {
      const res = await fetch(`/api/v1/chat_engine/conversations/${activeConversationId}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          sender_type: "agent",
          sender_id: null,
          content: newMessage,
        }),
      });
      if (!res.ok) throw new Error("Failed to send message");
      const msg = await res.json();
      setMessages((prev) => [...prev, msg]);
      setNewMessage("");
    } catch (err: any) {
      setError(err.message || "Failed to send message");
    }
  };

  return (
    <AppShell title="Native Omnichannel Inbox" subtitle="Native high-performance Rust chat engine.">
      <div className="flex h-full w-full bg-white dark:bg-zinc-900 overflow-hidden text-sm">
        {/* Inboxes Sidebar */}
        <div className="w-1/4 border-r border-zinc-200 dark:border-zinc-800 p-4">
          <h2 className="font-bold mb-4">Inboxes</h2>
          {loading && <p>Loading inboxes...</p>}
          {error && <p className="text-red-500">{error}</p>}
          <ul>
            {inboxes.map((inbox) => (
              <li
                key={inbox.id}
                className={`p-2 cursor-pointer rounded ${activeInboxId === inbox.id ? "bg-zinc-100 dark:bg-zinc-800" : ""}`}
                onClick={() => setActiveInboxId(inbox.id)}
              >
                {inbox.name}
              </li>
            ))}
          </ul>

          <h2 className="font-bold mt-8 mb-4">Conversations</h2>
          <ul>
            {conversations.map((conv) => (
              <li
                key={conv.id}
                className={`p-2 cursor-pointer rounded ${activeConversationId === conv.id ? "bg-zinc-100 dark:bg-zinc-800" : ""}`}
                onClick={() => setActiveConversationId(conv.id)}
              >
                {conv.id.substring(0, 8)}... ({conv.status})
              </li>
            ))}
          </ul>
        </div>

        {/* Chat Area */}
        <div className="w-3/4 p-4 flex flex-col">
          <h2 className="font-bold mb-4">Messages</h2>
          <div className="flex-1 overflow-y-auto mb-4 border border-zinc-200 dark:border-zinc-800 p-4 rounded">
            {messages.length === 0 && <p className="text-zinc-500">No messages in this conversation.</p>}
            {messages.map((msg) => (
              <div key={msg.id} className={`mb-2 p-2 rounded max-w-[80%] ${msg.sender_type === "agent" ? "bg-blue-100 dark:bg-blue-900 ml-auto" : "bg-gray-100 dark:bg-zinc-800"}`}>
                <p className="text-xs text-zinc-500 mb-1">{msg.sender_type}</p>
                <p>{msg.content}</p>
              </div>
            ))}
          </div>

          <div className="flex gap-2">
            <input
              type="text"
              className="flex-1 border border-zinc-200 dark:border-zinc-800 p-2 rounded bg-transparent"
              placeholder="Type a message..."
              value={newMessage}
              onChange={(e) => setNewMessage(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") handleSendMessage();
              }}
            />
            <button
              className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
              onClick={handleSendMessage}
            >
              Send
            </button>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
