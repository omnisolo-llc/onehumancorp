"use client";

import React, { useState, useEffect } from "react";

interface Conversation {
  id: string;
  tenant_id: string;
  inbox_id: string;
  contact_id: string;
  status: string;
}

interface ChatMessage {
  id: string;
  tenant_id: string;
  conversation_id: string;
  sender_type: string;
  sender_id: string | null;
  content: string;
  is_ai_draft: boolean;
  created_at: string;
}

export default function ChatDashboard() {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [replyContent, setReplyContent] = useState("");

  const fetchConversations = async () => {
    try {
      // First, get inboxes
      const inboxesRes = await fetch("/api/v1/chat/inboxes");
      if (!inboxesRes.ok) return;
      const inboxes = await inboxesRes.json();

      if (inboxes.length > 0) {
        const inboxId = inboxes[0].id;
        const convsRes = await fetch(`/api/v1/chat/inboxes/${inboxId}/conversations`);
        if (convsRes.ok) {
          const convs = await convsRes.json();
          setConversations(convs);
          if (convs.length > 0 && !selectedConversationId) {
            setSelectedConversationId(convs[0].id);
          }
        }
      }
    } catch (e) {
      console.error(e);
    }
  };

  const fetchMessages = async (convId: string) => {
    try {
      const res = await fetch(`/api/v1/chat/conversations/${convId}/messages`);
      if (res.ok) {
        setMessages(await res.json());
      }
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    fetchConversations();
  }, []);

  useEffect(() => {
    if (selectedConversationId) {
      fetchMessages(selectedConversationId);
    }
  }, [selectedConversationId]);

  const handleSend = async () => {
    if (!selectedConversationId || !replyContent) return;
    try {
      const res = await fetch(`/api/v1/chat/conversations/${selectedConversationId}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ content: replyContent }),
      });
      if (res.ok) {
        setReplyContent("");
        fetchMessages(selectedConversationId);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleApproveDraft = async (messageId: string) => {
    try {
      const res = await fetch(`/api/v1/chat/messages/${messageId}/approve`, {
        method: "PUT",
      });
      if (res.ok) {
        if (selectedConversationId) {
          fetchMessages(selectedConversationId);
        }
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="flex h-screen bg-gray-50 text-gray-900 font-sans">
      {/* Sidebar: Conversations List */}
      <div className="w-1/3 max-w-[375px] bg-white border-r border-gray-200 flex flex-col">
        <div className="p-4 border-b border-gray-200 flex items-center justify-between">
          <h1 className="text-xl font-semibold">Unified Inbox</h1>
        </div>
        <div className="flex-1 overflow-y-auto">
          {conversations.map((conv) => (
            <button
              key={conv.id}
              onClick={() => setSelectedConversationId(conv.id)}
              className={`w-full text-left p-4 border-b border-gray-100 flex flex-col hover:bg-gray-50 transition-colors ${
                selectedConversationId === conv.id ? "bg-blue-50" : ""
              }`}
            >
              <div className="flex items-center justify-between">
                <span className="font-medium truncate">Customer {conv.contact_id.substring(0,6)}</span>
                <span className="text-xs text-gray-500">{conv.status}</span>
              </div>
              <span className="text-sm text-gray-500 truncate mt-1">Conversation ID: {conv.id}</span>
            </button>
          ))}
          {conversations.length === 0 && (
            <div className="p-4 text-center text-gray-500">No conversations found.</div>
          )}
        </div>
      </div>

      {/* Main Chat Area */}
      <div className="flex-1 flex flex-col bg-gray-50">
        {selectedConversationId ? (
          <>
            <div className="p-4 bg-white border-b border-gray-200 flex items-center shadow-sm">
              <h2 className="text-lg font-medium">Conversation</h2>
            </div>

            <div className="flex-1 overflow-y-auto p-4 space-y-4">
              {messages.map((msg) => {
                const isContact = msg.sender_type === "contact";
                const isAgentDraft = msg.is_ai_draft;

                return (
                  <div key={msg.id} className={`flex flex-col ${isContact ? "items-start" : "items-end"}`}>
                    <div
                      className={`max-w-[80%] rounded-2xl px-4 py-2 ${
                        isContact
                          ? "bg-white text-gray-800 border border-gray-200 rounded-bl-none shadow-sm"
                          : isAgentDraft
                            ? "bg-blue-50 text-blue-900 border border-blue-200 rounded-br-none shadow-sm backdrop-blur-md bg-opacity-70"
                            : "bg-blue-600 text-white rounded-br-none shadow-md"
                      }`}
                    >
                      <p className="whitespace-pre-wrap">{msg.content}</p>
                      {isAgentDraft && (
                        <div className="mt-2 pt-2 border-t border-blue-200 flex justify-end">
                          <button
                            onClick={() => handleApproveDraft(msg.id)}
                            className="bg-blue-600 text-white text-xs px-3 py-1.5 rounded hover:bg-blue-700 transition-colors shadow-sm"
                          >
                            Approve & Send
                          </button>
                        </div>
                      )}
                    </div>
                    <span className="text-xs text-gray-400 mt-1 px-1">
                      {isContact ? "Customer" : isAgentDraft ? "AI Draft" : "You"}
                    </span>
                  </div>
                );
              })}
              {messages.length === 0 && (
                <div className="text-center text-gray-500 my-8">No messages yet.</div>
              )}
            </div>

            <div className="p-4 bg-white border-t border-gray-200">
              <div className="flex gap-2">
                <input
                  type="text"
                  value={replyContent}
                  onChange={(e) => setReplyContent(e.target.value)}
                  onKeyDown={(e) => e.key === "Enter" && handleSend()}
                  placeholder="Type a reply..."
                  className="flex-1 border border-gray-300 rounded-full px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
                <button
                  onClick={handleSend}
                  disabled={!replyContent.trim()}
                  className="bg-blue-600 text-white rounded-full px-6 py-2 font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                  Send
                </button>
              </div>
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-gray-500">
            Select a conversation to start chatting.
          </div>
        )}
      </div>
    </div>
  );
}
