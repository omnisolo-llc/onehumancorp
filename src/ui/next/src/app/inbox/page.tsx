'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function InboxPage() {
  const [messages, setMessages] = useState<any[]>([]);
  const [replyInput, setReplyInput] = useState('');
  const [activeMessageId, setActiveMessageId] = useState<string | null>(null);
  const [activeChannel, setActiveChannel] = useState<string>('all');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    fetchMessages();
  }, []);

  const fetchMessages = async () => {
    try {
      setIsLoading(true);
      const token = localStorage.getItem('token') || 'test-token';
      const res = await fetch('/api/inbox/messages', {
        headers: { 'Authorization': `Bearer ${token}` }
      });
      if (res.ok) {
        const data = await res.json();
        setMessages(data);
      }
    } catch (e) {
      console.error("Failed to load messages", e);
    } finally {
      setIsLoading(false);
    }
  };

  const generateDraft = async () => {
    try {
      const activeMessage = messages.find(m => m.id === activeMessageId);
      if (!activeMessage) return;

      const token = localStorage.getItem('token') || 'test-token';
      const res = await fetch('/api/v1/ai/draft-reply', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ customer_message: activeMessage.content })
      });

      if (res.ok) {
        const data = await res.json();
        setReplyInput(data.output || '');
      } else {
        setReplyInput('AI draft is unavailable. Please try again.');
      }
    } catch (e) {
      setReplyInput('AI draft is unavailable. Please try again.');
    }
  };

  const sendReply = async () => {
    if (!replyInput || !activeMessageId) return;

    try {
      const token = localStorage.getItem('token') || 'test-token';
      const res = await fetch('/api/inbox/reply', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ message_id: activeMessageId, content: replyInput })
      });

      if (res.ok) {
        // Update local state to show message was replied to
        setMessages(messages.map(m =>
          m.id === activeMessageId ? { ...m, status: 'replied' } : m
        ));
        setReplyInput('');
        setActiveMessageId(null);
        alert('Reply sent successfully!');
      } else {
        alert('Failed to send reply');
      }
    } catch (e) {
      alert('Failed to send reply');
    }
  };

  const filteredMessages = activeChannel === 'all'
    ? messages
    : messages.filter(m => {
        if (activeChannel === 'whatsapp') return m.source?.toLowerCase().includes('whatsapp');
        if (activeChannel === 'instagram') return m.source?.toLowerCase().includes('instagram');
        if (activeChannel === 'facebook') return m.source?.toLowerCase().includes('facebook');
        return m.source?.toLowerCase().includes(activeChannel);
      });

  const getSourceIcon = (source: string) => {
    const s = source?.toLowerCase() || '';
    if (s.includes('whatsapp')) return '💬 WhatsApp';
    if (s.includes('instagram') || s.includes('ig')) return '📸 Instagram';
    if (s.includes('facebook') || s.includes('fb')) return '📘 Facebook';
    if (s.includes('twilio') || s.includes('sms')) return '📱 SMS';
    return `📨 ${source || 'Unknown'}`;
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <div className="bg-gradient-to-r from-gray-900 to-black text-white px-6 py-8 shadow-md">
        <div className="max-w-5xl mx-auto flex items-center justify-between">
          <div>
            <Link href="/dashboard" className="text-gray-400 hover:text-white mb-4 inline-block text-sm">
              &larr; Back to Dashboard
            </Link>
            <h1 className="text-3xl font-bold font-outfit mb-1">Omnichannel Inbox</h1>
            <p className="text-gray-400 text-sm">Manage all your customer conversations in one place.</p>
          </div>
          <div className="hidden md:block w-16 h-16 bg-white/10 rounded-2xl border border-white/20 flex items-center justify-center text-3xl">
            📥
          </div>
        </div>
      </div>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full">
        {/* Channel Filters */}
        <div className="flex gap-4 mb-8 border-b border-gray-200 pb-4 overflow-x-auto hide-scrollbar">
          {[
            { id: 'all', label: 'All Messages' },
            { id: 'whatsapp', label: 'WhatsApp' },
            { id: 'instagram', label: 'Instagram' },
            { id: 'facebook', label: 'Facebook' }
          ].map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveChannel(tab.id)}
              className={`px-4 py-2 rounded-full text-sm font-semibold whitespace-nowrap transition-colors ${
                activeChannel === tab.id
                  ? "bg-gray-900 text-white"
                  : "bg-white text-gray-600 border border-gray-200 hover:bg-gray-50"
              }`}
            >
              {tab.label}
            </button>
          ))}
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 h-[600px]">
          {/* Message List */}
          <div className="md:col-span-1 bg-white rounded-[16px] shadow-sm border border-gray-200 overflow-hidden flex flex-col">
            <div className="p-4 border-b border-gray-100 bg-gray-50">
              <h2 className="font-bold text-gray-800">Conversations</h2>
            </div>
            <div className="flex-1 overflow-y-auto p-2">
              {isLoading ? (
                <div className="p-4 text-center text-gray-500">Loading messages...</div>
              ) : filteredMessages.length === 0 ? (
                <div className="p-4 text-center text-gray-500">No messages found.</div>
              ) : (
                filteredMessages.map(msg => (
                  <div
                    key={msg.id}
                    onClick={() => setActiveMessageId(msg.id)}
                    className={`p-3 mb-2 rounded-xl cursor-pointer transition-colors border ${
                      activeMessageId === msg.id
                        ? 'bg-blue-50 border-blue-200'
                        : 'bg-white border-transparent hover:bg-gray-50'
                    }`}
                  >
                    <div className="flex justify-between items-start mb-1">
                      <span className="text-xs font-bold text-gray-500 bg-gray-100 px-2 py-0.5 rounded">
                        {getSourceIcon(msg.source)}
                      </span>
                      <span className="text-xs text-gray-400">{msg.created_at?.split(' ')[0] || 'Recently'}</span>
                    </div>
                    <p className="text-sm text-gray-800 line-clamp-2 mt-2">{msg.content}</p>
                    {msg.status === 'replied' && (
                      <span className="text-xs text-green-600 mt-2 inline-block font-medium">✓ Replied</span>
                    )}
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Message Detail & Reply */}
          <div className="md:col-span-2 bg-white rounded-[16px] shadow-sm border border-gray-200 flex flex-col">
            {activeMessageId ? (
              <>
                <div className="p-6 border-b border-gray-100 flex-1 overflow-y-auto bg-gray-50/50">
                  {(() => {
                    const msg = messages.find(m => m.id === activeMessageId);
                    return msg ? (
                      <div className="flex flex-col gap-4">
                        <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 self-start max-w-[80%]">
                          <div className="text-xs text-gray-400 mb-1 font-medium">{getSourceIcon(msg.source)} • Customer</div>
                          <p className="text-gray-800">{msg.content}</p>
                        </div>
                        {msg.status === 'replied' && (
                          <div className="bg-blue-50 p-4 rounded-xl shadow-sm border border-blue-100 self-end max-w-[80%]">
                            <div className="text-xs text-blue-400 mb-1 font-medium">You (via {msg.source})</div>
                            <p className="text-gray-800 font-medium italic">Message already replied.</p>
                          </div>
                        )}
                      </div>
                    ) : null;
                  })()}
                </div>

                {messages.find(m => m.id === activeMessageId)?.status !== 'replied' && (
                  <div className="p-4 bg-white border-t border-gray-200">
                    <div className="flex gap-2 mb-3">
                      <button
                        onClick={generateDraft}
                        className="bg-purple-50 text-purple-700 px-4 py-2 rounded-lg text-sm font-medium hover:bg-purple-100 transition-colors border border-purple-100 flex items-center gap-2"
                      >
                        ✨ AI Draft
                      </button>
                    </div>
                    <textarea
                      value={replyInput}
                      onChange={e => setReplyInput(e.target.value)}
                      className="w-full border border-gray-300 rounded-xl p-3 text-sm text-gray-800 bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none resize-none"
                      rows={4}
                      placeholder="Type your reply here. It will be sent directly to the customer's native app..."
                    />
                    <div className="flex justify-end mt-3">
                      <button
                        onClick={sendReply}
                        disabled={!replyInput.trim()}
                        className="bg-blue-600 text-white px-6 py-2.5 rounded-lg text-sm font-semibold hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed shadow-sm"
                      >
                        Send Reply
                      </button>
                    </div>
                  </div>
                )}
              </>
            ) : (
              <div className="flex-1 flex flex-col items-center justify-center text-gray-400 p-8">
                <div className="text-4xl mb-4">💬</div>
                <p className="text-lg font-medium">Select a conversation</p>
                <p className="text-sm">Choose a message from the list to view and reply.</p>
              </div>
            )}
          </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
      `}} />
    </div>
  );
}
