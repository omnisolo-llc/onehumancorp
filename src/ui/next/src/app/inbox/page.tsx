'use client';
import { useState, useEffect, useCallback } from 'react';
import Link from 'next/link';

export default function InboxPage() {
  const [messages, setMessages] = useState<any[]>([]);
  const [replyInput, setReplyInput] = useState('');
  const [selectedMessageId, setSelectedMessageId] = useState<string | null>(null);

  const [filters, setFilters] = useState({
    whatsapp: true,
    instagram: true,
    facebook: true,
    twilio: true,
  });

  const loadMessages = useCallback(async () => {
    try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') : '';
      const headers = token ? { 'Authorization': 'Bearer ' + token } : {};
      const res = await fetch('/api/inbox/messages', { headers });
      if (res.ok) {
        const data = await res.json();
        setMessages(data);
      }
    } catch (e) {
      console.error(e);
    }
  }, []);

  useEffect(() => {
    loadMessages();
  }, [loadMessages]);

  const generateDraft = () => {
    setReplyInput('Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.');
  };

  const sendReply = async () => {
    if (!replyInput || !selectedMessageId) return;
    try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') : '';
      const headers = {
        'Content-Type': 'application/json',
        ...(token ? { 'Authorization': 'Bearer ' + token } : {})
      };
      const res = await fetch('/api/inbox/messages/reply', {
        method: 'POST',
        headers,
        body: JSON.stringify({ message_id: selectedMessageId, reply_content: replyInput })
      });
      if (res.ok) {
        setReplyInput('');
        setSelectedMessageId(null);
        alert('Reply sent successfully!');
        loadMessages();
      } else {
        alert('Failed to send reply');
      }
    } catch (e) {
      console.error(e);
      alert('Error sending reply');
    }
  };

  const toggleFilter = (key: keyof typeof filters) => {
    setFilters(prev => ({ ...prev, [key]: !prev[key] }));
  };

  const filteredMessages = messages.filter(msg => {
    const s = (msg.source || '').toLowerCase();
    if (s.includes('whatsapp') && !filters.whatsapp) return false;
    if (s.includes('instagram') && !filters.instagram) return false;
    if (s.includes('facebook') && !filters.facebook) return false;
    if (s.includes('twilio') && !filters.twilio) return false;
    return true;
  });

  return (
    <div className="p-4 max-w-md mx-auto">
      <div className="flex items-center mb-4">
        <Link href="/dashboard" className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold">Unified Inbox</h1>
        <div className="ml-auto">
          <Link href="/agent-audit-dashboard" aria-label="Agent Audit Dashboard" title="Agent Audit Dashboard" className="p-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-semibold text-black inline-block">
            Audit Dashboard
          </Link>
        </div>
      </div>

      <div className="flex gap-4 mb-4 text-sm text-black flex-wrap">
        <label className="flex items-center gap-1">
          <input type="checkbox" checked={filters.whatsapp} onChange={() => toggleFilter('whatsapp')} /> WhatsApp
        </label>
        <label className="flex items-center gap-1">
          <input type="checkbox" checked={filters.instagram} onChange={() => toggleFilter('instagram')} /> Instagram
        </label>
        <label className="flex items-center gap-1">
          <input type="checkbox" checked={filters.facebook} onChange={() => toggleFilter('facebook')} /> Facebook
        </label>
        <label className="flex items-center gap-1">
          <input type="checkbox" checked={filters.twilio} onChange={() => toggleFilter('twilio')} /> Twilio
        </label>
      </div>

      <div id="messages-list" className="bg-white rounded shadow p-4 mb-4 h-64 overflow-y-auto text-black">
        {filteredMessages.map(msg => (
          <div
            key={msg.id}
            className={`mb-3 cursor-pointer p-2 rounded ${selectedMessageId === msg.id ? 'bg-blue-50 border-blue-200 border' : ''}`}
            onClick={() => setSelectedMessageId(msg.id)}
          >
            <div className="flex justify-between">
              <span className="font-semibold text-sm capitalize">{msg.source}</span>
              <span className="text-xs text-gray-500">{msg.created_at || 'Just now'}</span>
            </div>
            <p className="p-2 rounded mt-1 inline-block text-left bg-gray-100 w-full text-sm">
              {msg.content}
            </p>
          </div>
        ))}
        {filteredMessages.length === 0 && (
          <p className="text-sm text-gray-500">No messages found.</p>
        )}
      </div>

      <div className="bg-gray-50 p-4 rounded border text-black">
        <div className="flex gap-2 mb-2">
          <button
            onClick={generateDraft}
            className="bg-purple-100 text-purple-700 px-3 py-1 rounded text-sm hover:bg-purple-200"
          >
            ✨ AI Draft
          </button>
        </div>
        <textarea
          id="reply-input"
          value={replyInput}
          onChange={e => setReplyInput(e.target.value)}
          className="w-full border rounded p-2 text-sm text-black bg-white"
          rows={3}
          placeholder="Type a reply..."
        />
        <div className="flex justify-end mt-2">
          <button
            onClick={sendReply}
            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
