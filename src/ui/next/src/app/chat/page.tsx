'use client';

import React, { useEffect, useState } from 'react';

export default function NativeOmnichannelChat() {
  const [conversations, setConversations] = useState<any[]>([]);
  const [selectedConversation, setSelectedConversation] = useState<any | null>(null);
  const [messages, setMessages] = useState<any[]>([]);
  const [inputMessage, setInputMessage] = useState("");

  const fetchConversations = async () => {
    try {
      const res = await fetch('/api/v1/omnichannel/conversations');
      if (res.ok) {
        const data = await res.json();
        setConversations(data);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const fetchMessages = async (convId: string) => {
    try {
      const res = await fetch(`/api/v1/omnichannel/conversations/${convId}/messages`);
      if (res.ok) {
        const data = await res.json();
        setMessages(data);
      }
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    fetchConversations();
  }, []);

  useEffect(() => {
    let ws: WebSocket;
    let reconnectTimeout: NodeJS.Timeout;

    const connect = () => {
      const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
      const isLocalhost = window.location.hostname === "localhost" || window.location.hostname === "127.0.0.1";
      const wsUrl = isLocalhost ? `ws://127.0.0.1:18789/ws` : `${protocol}//${window.location.host}/ws`;

      ws = new WebSocket(wsUrl);

      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (data.action === "message.created" || data?.data?.event === "message.created") {
            const payload = data.data ? data.data : data;
            const newMsg = payload.message;
            if (newMsg && selectedConversation && newMsg.conversation_id === selectedConversation.id) {
               setMessages(prev => [...prev, newMsg]);
            }
          }
          if (data.action === "conversation.updated" || data?.data?.event === "conversation.updated") {
             fetchConversations();
          }
        } catch (e) {
          console.error(e);
        }
      };

      ws.onclose = () => {
        reconnectTimeout = setTimeout(connect, 3000);
      };
    };

    connect();

    return () => {
      clearTimeout(reconnectTimeout);
      if (ws) ws.close();
    };
  }, [selectedConversation]);

  const handleSendMessage = async () => {
    if (!inputMessage.trim() || !selectedConversation) return;
    try {
      const res = await fetch(`/api/v1/omnichannel/conversations/${selectedConversation.id}/messages`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          sender_type: 'agent',
          sender_id: null,
          content: inputMessage
        })
      });
      if (res.ok) {
        setInputMessage("");
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="min-w-[375px] max-w-[1440px] mx-auto h-screen flex flex-col md:flex-row bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-3xl saturate-[210%]">
      {/* Inbox List */}
      <div className={`w-full md:w-1/3 border-r border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col ${selectedConversation ? 'hidden md:flex' : 'flex'}`}>
        <div className="p-4 border-b border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
          <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Unified Inbox</h2>
        </div>
        <div className="flex-1 overflow-y-auto" data-testid="conversation-list">
          {conversations.map(conv => (
            <div
              key={conv.id}
              onClick={() => {
                setSelectedConversation(conv);
                fetchMessages(conv.id);
              }}
              className="p-4 border-b border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] cursor-pointer hover:bg-white/10"
            >
              <div className="flex justify-between items-center">
                <span className="font-semibold text-sm font-inter text-[#1D1D1F] dark:text-[#F5F5F7]">Contact: {conv.contact_id.substring(0,8)}...</span>
                <span className="text-xs text-[#0066FF] dark:text-[#0071E3] bg-[#0066FF]/10 px-2 py-1 rounded-[8px]">
                  {conv.status}
                </span>
              </div>
            </div>
          ))}
          {conversations.length === 0 && (
             <div className="p-8 text-center text-sm text-gray-500 font-inter">No conversations found.</div>
          )}
        </div>
      </div>

      {/* Conversation View */}
      <div className={`w-full md:w-2/3 flex flex-col ${!selectedConversation ? 'hidden md:flex' : 'flex'}`}>
        {selectedConversation ? (
          <>
            <div className="p-4 border-b border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex items-center gap-4">
              <button
                className="md:hidden text-[#0066FF] dark:text-[#0071E3]"
                onClick={() => setSelectedConversation(null)}
              >
                ← Back
              </button>
              <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                Chat
              </h2>
            </div>
            <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-4" data-testid="message-list">
              {messages.map(msg => {
                const isAgent = msg.sender_type === 'agent';
                return (
                  <div key={msg.id} className={`flex ${isAgent ? 'justify-end' : 'justify-start'}`}>
                    <div className={`max-w-[80%] rounded-[16px] p-3 text-sm font-inter ${isAgent ? 'bg-[#0066FF] text-white' : 'bg-white/80 dark:bg-black/40 text-[#1D1D1F] dark:text-[#F5F5F7]'}`}>
                      {msg.content}
                    </div>
                  </div>
                );
              })}
            </div>
            <div className="p-4 border-t border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
              <div className="flex gap-2">
                <input
                  type="text"
                  value={inputMessage}
                  onChange={e => setInputMessage(e.target.value)}
                  onKeyDown={e => e.key === 'Enter' && handleSendMessage()}
                  className="flex-1 rounded-[8px] bg-white/50 dark:bg-black/50 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] px-4 py-2 text-sm font-inter focus:outline-none focus:border-[#0066FF]"
                  placeholder="Type a message..."
                />
                <button
                  onClick={handleSendMessage}
                  data-testid="send-message-btn"
                  className="bg-[#0066FF] text-white px-6 py-2 rounded-[8px] font-semibold text-sm font-inter min-w-[44px] min-h-[44px]"
                >
                  Send
                </button>
              </div>
            </div>
          </>
        ) : (
          <div className="flex-1 flex items-center justify-center text-sm text-gray-500 font-inter">
            Select a conversation to view
          </div>
        )}
      </div>
    </div>
  );
}
