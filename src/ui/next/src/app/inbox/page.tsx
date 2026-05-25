'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function InboxPage() {
  const router = useRouter();
  const [messages, setMessages] = useState<any[]>([]);
  const [replyInput, setReplyInput] = useState('');

  useEffect(() => {
    // Ideally we fetch messages from our backend here, which will call Buffer integration
    setMessages([
      { id: '1', sender: 'Alice', content: 'Do you have vegan birthday cake options?', date: '10:00 AM', platform: 'twitter' },
      { id: '2', sender: 'Bob', content: 'When will my order be shipped?', date: 'Yesterday', platform: 'facebook' },
    ]);
  }, []);

  const generateDraft = () => {
    setReplyInput('Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.');
  };

  const sendReply = async () => {
    if (!replyInput) return;

    // Ideally we call our backend here, which will call Buffer integration

    setMessages([...messages, { id: Date.now().toString(), sender: 'Me', content: replyInput, date: 'Just now', platform: 'buffer' }]);
    setReplyInput('');
  };

  return (
    <div className="p-4 max-w-md mx-auto">
      <div className="flex items-center mb-4">
        <button onClick={() => router.push('/dashboard')} className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </button>
        <h1 className="text-2xl font-bold">Customer Inbox</h1>
        <div className="ml-auto">
          <Link href="/settings/social-media" aria-label="Social Media Settings" title="Social Media Settings" className="p-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-semibold text-black inline-block">
            Connect Buffer
          </Link>
        </div>
      </div>

      <div id="messages-list" className="bg-white rounded shadow p-4 mb-4 h-64 overflow-y-auto text-black">
        {messages.map(msg => (
          <div key={msg.id} className={`mb-3 ${msg.sender === 'Me' ? 'text-right' : ''}`}>
            <span className="font-semibold text-sm">{msg.sender}</span>
            <span className="text-xs text-gray-500 ml-2">{msg.date}</span>
            {msg.platform && <span className="text-xs text-blue-500 ml-2">[{msg.platform}]</span>}
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
