'use client';

import { useState, useEffect } from 'react';
import AppShell from '@/components/layout/AppShell';

export default function UnifiedInboxPage() {
  const [conversations, setConversations] = useState([]);
  const [selectedConvoId, setSelectedConvoId] = useState<string | null>(null);
  const [messages, setMessages] = useState([]);
  const [replyText, setReplyText] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('/api/v1/omnichannel_native/conversations')
      .then((res) => res.json())
      .then((data) => {
        if (data.success) {
          setConversations(data.conversations || []);
        } else {
          setError(data.error);
        }
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, []);

  useEffect(() => {
    if (selectedConvoId) {
      fetch(`/api/v1/omnichannel_native/conversations/${selectedConvoId}/messages`)
        .then((res) => res.json())
        .then((data) => {
          if (data.success) {
            setMessages(data.messages || []);
          }
        });
    }
  }, [selectedConvoId]);

  const handleSend = async () => {
    if (!replyText.trim() || !selectedConvoId) return;

    try {
      const res = await fetch(`/api/v1/omnichannel_native/conversations/${selectedConvoId}/messages`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          content: replyText,
          sender_type: 'agent'
        })
      });

      const data = await res.json();
      if (data.success && data.message) {
        setMessages((prev) => [...prev, data.message]);
        setReplyText('');
      } else {
        alert("Failed to send: " + (data.error || "Unknown error"));
      }
    } catch (e: any) {
      alert("Error sending message: " + e.message);
    }
  };

  return (
    <AppShell>
      <div className="flex h-[calc(100vh-64px)] w-full overflow-hidden bg-white/50 backdrop-blur-md">
        {/* Sidebar */}
        <div className={`w-full md:w-80 border-r border-gray-200 bg-white/30 flex flex-col ${selectedConvoId ? 'hidden md:flex' : 'flex'}`}>
          <div className="p-4 border-b border-gray-200">
            <h1 className="text-xl font-semibold">Inbox</h1>
          </div>
          <div className="flex-1 overflow-y-auto">
            {loading ? (
              <p className="p-4 text-gray-500">Loading...</p>
            ) : error ? (
              <p className="p-4 text-red-500">{error}</p>
            ) : (
              conversations.map((c: any) => (
                <div
                  key={c.id}
                  onClick={() => setSelectedConvoId(c.id)}
                  className={`p-4 border-b border-gray-100 cursor-pointer hover:bg-gray-50/50 ${selectedConvoId === c.id ? 'bg-blue-50/50' : ''}`}
                >
                  <div className="font-medium">Contact: {c.contact_id.substring(0,8)}...</div>
                  <div className="text-sm text-gray-500">Status: {c.status}</div>
                </div>
              ))
            )}
          </div>
        </div>

        {/* Chat Area */}
        <div className={`flex-1 flex flex-col bg-white/40 ${!selectedConvoId ? 'hidden md:flex' : 'flex'}`}>
          {selectedConvoId ? (
            <>
              <div className="p-4 border-b border-gray-200 flex items-center gap-4 bg-white/50">
                <button
                  className="md:hidden text-blue-600 font-medium"
                  onClick={() => setSelectedConvoId(null)}
                >
                  &larr; Back
                </button>
                <h2 className="font-semibold text-lg">Conversation Details</h2>
              </div>
              <div className="flex-1 p-4 overflow-y-auto space-y-4">
                {messages.map((m: any) => (
                  <div key={m.id} className={`flex ${m.sender_type === 'agent' ? 'justify-end' : 'justify-start'}`}>
                    <div className={`max-w-[75%] p-3 rounded-2xl backdrop-blur-md ${m.sender_type === 'agent' ? 'bg-blue-600 text-white rounded-br-sm' : 'bg-gray-100/80 text-gray-900 rounded-bl-sm border border-gray-200/50'}`}>
                      {m.content}
                    </div>
                  </div>
                ))}
              </div>
              <div className="p-4 border-t border-gray-200 bg-white/50 backdrop-blur-md">
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={replyText}
                    onChange={(e) => setReplyText(e.target.value)}
                    onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                    placeholder="Type a reply..."
                    className="flex-1 px-4 py-2 border border-gray-300 rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500/50 bg-white/70"
                  />
                  <button onClick={handleSend} className="px-6 py-2 bg-blue-600 text-white rounded-full font-medium hover:bg-blue-700 transition-colors shadow-sm">
                    Send
                  </button>
                </div>
              </div>
            </>
          ) : (
            <div className="flex-1 flex items-center justify-center text-gray-500 bg-white/20">
              Select a conversation to start chatting
            </div>
          )}
        </div>
      </div>
    </AppShell>
  );
}
