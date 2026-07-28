"use client";

import { useState, useEffect, useRef, Fragment } from "react";
import { AppShell } from "../components/AppShell";

type Message = {
  id: string;
  tenant_id: string;
  conversation_id: string;
  sender_type: string;
  sender_id?: string;
  content: string;
  created_at: string;
};

type Conversation = {
  id: string;
  tenant_id: string;
  inbox_id: string;
  contact_id: string;
  assignee_id?: string;
  status: string;
  created_at: string;
};

type Inbox = {
  id: string;
  name: string;
};

export default function ChatPage() {
  const [inboxes, setInboxes] = useState<Inbox[]>([]);
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [messages, setMessages] = useState<Message[]>([]);
  const [activeConversation, setActiveConversation] = useState<string | null>(null);
  const [newMessage, setNewMessage] = useState("");
  const [loading, setLoading] = useState(true);
  const wsRef = useRef<WebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    async function loadData() {
      try {
        const inboxRes = await fetch("/api/v1/chat/inboxes");
        const inboxesData = await inboxRes.json();
        setInboxes(inboxesData);

        const convRes = await fetch("/api/v1/chat/conversations");
        const convData = await convRes.json();
        setConversations(convData);

        if (convData.length > 0) {
          setActiveConversation(convData[0].id);
        }
      } catch (err) {
        console.error("Failed to load chat data", err);
      } finally {
        setLoading(false);
      }
    }
    loadData();
  }, []);

  useEffect(() => {
    if (!activeConversation) return;

    async function loadMessages() {
      try {
        const res = await fetch(`/api/v1/chat/conversations/${activeConversation}/messages`);
        const data = await res.json();
        setMessages(data);
      } catch (err) {
        console.error("Failed to load messages", err);
      }
    }
    loadMessages();
  }, [activeConversation]);

  useEffect(() => {
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const host = window.location.host;
    const ws = new WebSocket(`${protocol}//${host}/ws?channels=chat`);

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.channel === "chat" && data.data) {
           const msg = data.data as Message;
           if (msg.conversation_id === activeConversation) {
              setMessages((prev) => {
                  if (prev.find(m => m.id === msg.id)) return prev;
                  return [...prev, msg];
              });
           }
        }
      } catch (err) {
        console.error("WS error", err);
      }
    };

    wsRef.current = ws;

    return () => {
      ws.close();
    };
  }, [activeConversation]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSendMessage = async () => {
    if (!activeConversation || !newMessage.trim()) return;

    try {
      const res = await fetch(`/api/v1/chat/conversations/${activeConversation}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          sender_type: "agent",
          content: newMessage,
        }),
      });
      if (res.ok) {
        const msg = await res.json();
        setMessages((prev) => {
            if (prev.find(m => m.id === msg.id)) return prev;
            return [...prev, msg];
        });
        setNewMessage("");
      }
    } catch (err) {
      console.error("Failed to send message", err);
    }
  };

  if (loading) {
    return <AppShell title="Chat"><div className="p-4">Loading...</div></AppShell>;
  }

  // Derive if there's a draft
  const aiDraft = messages.length > 0 && messages[messages.length - 1].sender_type === "bot" && messages[messages.length - 1].content.startsWith("DRAFT: ") ? messages[messages.length - 1] : null;

  return (
    <AppShell title="Chat" subtitle="Omnichannel Support">
      <div className="flex h-[calc(100vh-120px)] border rounded overflow-hidden">
        {/* Sidebar */}
        <div className="w-1/3 border-r bg-gray-50 overflow-y-auto">
           {conversations.map(conv => (
             <div
               key={conv.id}
               onClick={() => setActiveConversation(conv.id)}
               className={`p-4 border-b cursor-pointer ${activeConversation === conv.id ? "bg-blue-50" : "hover:bg-gray-100"}`}
             >
               <div className="font-semibold text-sm">Conversation {conv.id.slice(0, 8)}</div>
               <div className="text-xs text-gray-500">{new Date(conv.created_at).toLocaleString()}</div>
             </div>
           ))}
        </div>

        {/* Main Chat Area */}
        <div className="flex-1 flex flex-col bg-white">
          <div className="flex-1 overflow-y-auto p-4 space-y-4">
            {messages.map(msg => {
               if (msg.sender_type === "bot" && msg.content.startsWith("DRAFT: ")) return null;

               const isAgent = msg.sender_type === "agent";
               return (
                 <div key={msg.id} className={`flex ${isAgent ? "justify-end" : "justify-start"}`}>
                    <div className={`max-w-[75%] rounded-lg p-3 ${isAgent ? "bg-blue-600 text-white" : "bg-gray-200 text-gray-800"}`}>
                       <div className="text-sm whitespace-pre-wrap">{msg.content}</div>
                       <div className={`text-[10px] mt-1 ${isAgent ? "text-blue-200" : "text-gray-500"}`}>
                         {new Date(msg.created_at).toLocaleTimeString()}
                       </div>
                    </div>
                 </div>
               );
            })}
            <div ref={messagesEndRef} />
          </div>

          <div className="p-4 border-t bg-gray-50">
            {aiDraft && (
              <div className="mb-2 p-3 bg-indigo-50 border border-indigo-100 rounded-md shadow-sm">
                <div className="text-xs font-semibold text-indigo-700 mb-1">AI Suggestion</div>
                <div className="text-sm text-gray-700 mb-2">{aiDraft.content.replace("DRAFT: ", "")}</div>
                <button
                  onClick={() => {
                     setNewMessage(aiDraft.content.replace("DRAFT: ", ""));
                  }}
                  className="px-3 py-1 bg-indigo-600 text-white text-xs rounded hover:bg-indigo-700"
                >
                  Approve AI Draft
                </button>
              </div>
            )}
            <div className="flex items-center space-x-2">
              <input
                 type="text"
                 className="flex-1 p-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                 placeholder="Type a message..."
                 value={newMessage}
                 onChange={e => setNewMessage(e.target.value)}
                 onKeyDown={e => { if (e.key === "Enter") handleSendMessage(); }}
              />
              <button
                className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 font-medium"
                onClick={handleSendMessage}
              >
                Send
              </button>
            </div>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
