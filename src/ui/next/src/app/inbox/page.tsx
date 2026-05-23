'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function InboxPage() {
  const [messages, setMessages] = useState([
    { id: 1, sender: 'Alice', content: 'Do you have vegan birthday cake options?', date: '10:00 AM' },
    { id: 2, sender: 'Bob', content: 'When will my order be shipped?', date: 'Yesterday' },
  ]);
  const [replyInput, setReplyInput] = useState('');

  const generateDraft = () => {
    setReplyInput('Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.');
  };

  const sendReply = () => {
    if (!replyInput) return;
    setMessages([...messages, { id: Date.now(), sender: 'Me', content: replyInput, date: 'Just now' }]);
    setReplyInput('');
  };

  return (
    <div className="p-4 max-w-md mx-auto">
      <div className="flex items-center mb-4">
        <Link href="/dashboard" className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold">Customer Inbox</h1>
      </div>

      <div id="messages-list" className="bg-white rounded shadow p-4 mb-4 h-64 overflow-y-auto text-black">
        {messages.map(msg => (
          <div key={msg.id} className={`mb-3 ${msg.sender === 'Me' ? 'text-right' : ''}`}>
            <span className="font-semibold text-sm">{msg.sender}</span>
            <span className="text-xs text-gray-500 ml-2">{msg.date}</span>
            <p className={`p-2 rounded mt-1 inline-block text-left ${msg.sender === 'Me' ? 'bg-blue-100' : 'bg-gray-100'}`}>
              {msg.content}
            </p>
          </div>
        ))}
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
