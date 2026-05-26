'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function InboxPage() {
  const [showPaywallModal, setShowPaywallModal] = useState(false);
  const [isAutoReplyEnabled, setIsAutoReplyEnabled] = useState(false);

  const handleToggle = () => {
    setShowPaywallModal(true);
    setIsAutoReplyEnabled(false); // Simulate that they can't turn it on yet
  };

  const [messages, setMessages] = useState([
    { id: 1, sender: 'Facebook User', source: 'Facebook', icon: '📘', content: 'Do you have vegan birthday cake options?', date: '10:00 AM' },
    { id: 2, sender: 'Instagram User', source: 'Instagram', icon: '📸', content: 'When will my order be shipped?', date: 'Yesterday' },
    { id: 3, sender: 'WhatsApp User', source: 'WhatsApp', icon: '💬', content: 'Can I change my delivery address?', date: 'Yesterday' },
  ]);
  const [replyInput, setReplyInput] = useState('');

  const generateDraft = () => {
    setReplyInput('Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.');
  };

  const sendReply = () => {
    if (!replyInput) return;
    setMessages([...messages, { id: Date.now(), sender: 'Me', source: 'Me', icon: '👤', content: replyInput, date: 'Just now' }]);
    setReplyInput('');
  };

  return (
    <div className="p-4 max-w-[375px] mx-auto bg-white min-h-screen shadow-xl relative overflow-x-hidden flex flex-col font-inter">
      <div className="flex items-center mb-4 border-b pb-2">
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

      <div id="messages-list" className="bg-white rounded shadow p-4 mb-4 h-64 overflow-y-auto text-black">
        {messages.map(msg => (
          <div key={msg.id} className={`mb-3 ${msg.sender === 'Me' ? 'text-right' : ''}`}>
            <div className={`flex items-center gap-2 ${msg.sender === 'Me' ? 'justify-end' : ''}`}>
              {msg.sender !== 'Me' && <span className="text-sm">{msg.icon}</span>}
              <span className="font-semibold text-sm">{msg.sender}</span>
              <span className="text-xs text-gray-500">{msg.date}</span>
            </div>
            <p className={`p-2 rounded mt-1 inline-block text-left ${msg.sender === 'Me' ? 'bg-blue-100' : 'bg-gray-100'}`}>
              {msg.content}
            </p>
          </div>
        ))}
      </div>

      <div className="bg-gray-50 p-4 rounded border text-black">
        <div className="flex items-center justify-between mb-2">
          <div className="flex gap-2">
            <button
              onClick={generateDraft}
              className="bg-purple-100 text-purple-700 px-3 py-1 rounded text-sm font-semibold hover:bg-purple-200"
            >
              ✨ AI Draft
            </button>
          </div>
          <div className="flex items-center gap-2">
            <label className="text-sm font-semibold text-gray-700 cursor-pointer flex items-center gap-2">
              <span className="bg-indigo-100 text-indigo-700 px-2 py-0.5 rounded-full text-xs font-bold">PRO</span>
              AI Auto-Reply
              <div className="relative inline-block w-10 h-5">
                <input type="checkbox" className="sr-only peer" checked={isAutoReplyEnabled} onChange={handleToggle} />
                <div className="w-10 h-5 bg-gray-200 rounded-full peer peer-checked:bg-indigo-600 transition-colors"></div>
                <div className="absolute left-1 top-1 bg-white w-3 h-3 rounded-full transition-transform peer-checked:translate-x-5"></div>
              </div>
            </label>
          </div>
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
            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 font-semibold shadow-sm transition-colors"
          >
            Send
          </button>
        </div>
      </div>

      {showPaywallModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-sm rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-indigo-100">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-indigo-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-indigo-600">
                🤖
              </div>
            </div>

            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Unlock AI Auto-Reply</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Your free plan includes 3 manual AI drafts per day. Upgrade to Pro to let AI automatically respond to customer inquiries 24/7, keeping your response time under 5 minutes.
            </p>

            <div className="space-y-3">
              <button
                onClick={() => {
                  alert('Redirecting to upgrade checkout...');
                  setShowPaywallModal(false);
                }}
                className="w-full px-4 py-3 bg-indigo-600 text-white rounded-xl font-bold hover:bg-indigo-700 transition-colors shadow-md"
              >
                Upgrade to Pro - $19/mo
              </button>
              <button
                onClick={() => setShowPaywallModal(false)}
                className="w-full px-4 py-3 bg-gray-100 text-gray-700 rounded-xl font-bold hover:bg-gray-200 transition-colors"
              >
                Not right now
              </button>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
