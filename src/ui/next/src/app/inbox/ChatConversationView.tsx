"use client";

import { useEffect, useState, useRef } from "react";

type Message = {
  id: string;
  content: string;
  sender_type: string;
  is_draft: boolean;
};

export function ChatConversationView({
  conversationId,
  onBack
}: {
  conversationId: string;
  onBack: () => void;
}) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState("");
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    async function loadMessages() {
      try {
        const res = await fetch(`/api/v1/chat_system/conversations/${conversationId}/messages`);
        if (res.ok) {
          const data = await res.json();
          setMessages(data.messages || []);
        }
      } catch (e) {
        console.error("Failed to load messages", e);
      }
    }
    loadMessages();

    // WebSocket Connection Logic
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsUrl = `${protocol}//${window.location.host}/api/v1/chat_system/ws`;
    const ws = new WebSocket(wsUrl);

    ws.onmessage = (event) => {
      try {
        const newMsg = JSON.parse(event.data);
        if (newMsg.conversation_id === conversationId) {
          setMessages(prev => [...prev, newMsg]);
        }
      } catch (e) {
        console.error("Error parsing message", e);
      }
    };

    return () => {
      ws.close();
    };
  }, [conversationId]);

  useEffect(() => {
      endRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim()) return;

    try {
      const res = await fetch("/api/v1/chat_system/messages", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          conversation_id: conversationId,
          content: input,
          sender_type: "agent"
        })
      });

      if (res.ok) {
        const newMsg = await res.json();
        setMessages(prev => [...prev, newMsg]);
        setInput("");
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleApproveDraft = (msgId: string) => {
    setMessages(prev => prev.map(m =>
      m.id === msgId ? { ...m, is_draft: false, sender_type: "agent" } : m
    ));
  };

  return (
    <div className="flex flex-col h-full bg-white relative">
      <div className="p-4 border-b flex items-center bg-white sticky top-0 z-10 shadow-sm">
        <button onClick={onBack} className="md:hidden mr-4 p-2 bg-gray-100 hover:bg-gray-200 rounded text-sm font-medium">← Back</button>
        <span className="font-bold">Conversation: {conversationId}</span>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.length === 0 && (
            <div className="text-center text-gray-500 mt-10">No messages yet.</div>
        )}
        {messages.map(m => (
          <div key={m.id} className={`flex ${m.sender_type === 'customer' ? 'justify-start' : 'justify-end'}`}>
            <div className={`p-3 rounded-2xl max-w-[85%] sm:max-w-[70%] shadow-sm ${m.sender_type === 'customer' ? 'bg-gray-100 text-gray-800' : 'bg-blue-600 text-white'}`}>
              <div className="text-sm whitespace-pre-wrap">{m.content}</div>
              {m.is_draft && (
                <div className="mt-3 pt-3 border-t border-blue-400 border-opacity-30">
                   <div className="text-xs opacity-80 mb-2 font-medium">AI Draft Response</div>
                   <button
                     onClick={() => handleApproveDraft(m.id)}
                     className="bg-white text-blue-600 font-semibold text-xs px-3 py-1.5 rounded-full shadow-sm hover:bg-blue-50 w-full transition-colors"
                   >
                     Approve & Send
                   </button>
                </div>
              )}
            </div>
          </div>
        ))}
        <div ref={endRef} />
      </div>

      <div className="p-4 border-t bg-gray-50">
        <div className="flex bg-white border rounded-full overflow-hidden focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-transparent transition-shadow">
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              className="flex-1 px-4 py-3 outline-none text-sm"
              placeholder="Type your reply..."
            />
            <button onClick={handleSend} className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 font-medium transition-colors">
              Send
            </button>
        </div>
      </div>
    </div>
  );
}
