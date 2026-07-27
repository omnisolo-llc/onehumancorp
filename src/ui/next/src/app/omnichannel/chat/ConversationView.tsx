"use client";

import React, { useState, useEffect, useRef } from "react";

export default function ConversationView({ conversationId, tenantId, onBack }: any) {
  const [messages, setMessages] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [replyText, setReplyText] = useState("");
  const wsRef = useRef<WebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    fetchMessages();
    setupWebSocket();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [conversationId]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const fetchMessages = async () => {
    try {
      setLoading(true);
      const res = await fetch(`/api/v1/omnichannel/conversations/${conversationId}/messages`);
      if (res.ok) {
        const data = await res.json();
        setMessages(data);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const setupWebSocket = () => {
    const wsUrl = window.location.protocol === "https:"
        ? `wss://${window.location.host}/ws`
        : `ws://${window.location.host}/ws`;

    const ws = new WebSocket(wsUrl);
    wsRef.current = ws;

    ws.onopen = () => {
      ws.send(JSON.stringify({
        action: "subscribe",
        channel: "omnichannel",
        topic: `chat:${tenantId}`
      }));
    };

    ws.onmessage = (event) => {
      try {
        const envelope = JSON.parse(event.data);
        if (envelope.channel === "omnichannel" && envelope.data?.action === "new_message") {
          const newMsg = envelope.data.message;
          if (newMsg.conversation_id === conversationId) {
            setMessages((prev) => [...prev, newMsg]);
          }
        }
      } catch (e) {
        console.error(e);
      }
    };
  };

  const handleSend = async (text: string, type: string) => {
    if (!text.trim()) return;

    try {
      await fetch(`/api/v1/omnichannel/conversations/${conversationId}/messages`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          sender_type: type,
          content: text
        }),
      });
      if (type === "agent") setReplyText("");
    } catch (e) {
      console.error("Failed to send message", e);
    }
  };

  const handleApproveDraft = (msg: any) => {
    // Treat "Approve" as sending a new agent message based on the draft
    // In a full implementation, we might actually update the draft status.
    handleSend(msg.content, "agent");
  };

  return (
    <div className="flex flex-col h-full bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] relative border-l border-white/40">
      <div className="pt-12 pb-4 px-6 border-b border-white/40 flex items-center gap-4">
        <button onClick={onBack} className="text-gray-500 font-bold" data-testid="omni-back-btn">
          {"< Back"}
        </button>
        <h2 className="text-xl font-bold font-outfit text-gray-900">Conversation</h2>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {loading && <p>Loading messages...</p>}
        {!loading && messages.length === 0 && <p className="text-gray-500 text-sm">No messages yet.</p>}

        {messages.map((msg, idx) => {
          const isDraft = msg.sender_type === "bot" || msg.status === "draft";
          const isAgent = msg.sender_type === "agent";
          const isContact = msg.sender_type === "contact";

          return (
            <div key={msg.id || idx} className={`flex ${isContact ? "justify-start" : "justify-end"}`}>
              <div
                className={`max-w-[80%] p-3 rounded-2xl ${
                  isContact
                    ? "bg-gray-100 text-gray-800 rounded-tl-none"
                    : isDraft
                    ? "bg-yellow-50 border border-yellow-200 text-yellow-900 rounded-tr-none shadow-sm"
                    : "bg-[#0066FF] text-white rounded-tr-none"
                }`}
              >
                <div className="text-sm">{msg.content}</div>
                {isDraft && (
                  <div className="mt-2 pt-2 border-t border-yellow-200/50 flex gap-2">
                    <span className="text-xs font-bold text-yellow-700 bg-yellow-100 px-2 py-0.5 rounded-full uppercase">AI Draft</span>
                    <button
                      onClick={() => handleApproveDraft(msg)}
                      className="text-xs bg-yellow-600 hover:bg-yellow-700 text-white px-2 py-0.5 rounded"
                      data-testid="approve-draft-btn"
                    >
                      Approve & Send
                    </button>
                  </div>
                )}
              </div>
            </div>
          );
        })}
        <div ref={messagesEndRef} />
      </div>

      <div className="p-4 border-t border-white/40">
        <div className="flex items-center bg-gray-50 rounded-full border border-gray-200 px-4 py-2">
          <input
            type="text"
            className="flex-1 bg-transparent border-none outline-none text-sm"
            placeholder="Type your reply..."
            value={replyText}
            onChange={(e) => setReplyText(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSend(replyText, "agent")}
            data-testid="omni-chat-input"
          />
          <button
            className="ml-2 text-[#0066FF] font-medium text-sm disabled:opacity-50"
            onClick={() => handleSend(replyText, "agent")}
            disabled={!replyText.trim()}
            data-testid="omni-chat-send"
          >
            Send
          </button>
          <button
            className="ml-2 text-gray-600 font-medium text-sm"
            onClick={() => handleSend(replyText, "contact")}
            title="Simulate Contact Message"
            data-testid="omni-chat-simulate"
          >
            Simulate
          </button>
        </div>
      </div>
    </div>
  );
}
