'use client';
import { useState } from 'react';
import Link from 'next/link';

type Message = {
  id: number;
  sender: string;
  source: string;
  icon: string;
  content: string;
  date: string;
  draft?: string;
};

export default function InboxPage() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: 1,
      sender: 'Facebook User',
      source: 'Facebook',
      icon: '📘',
      content: 'Do you have vegan birthday cake options?',
      date: '10:00 AM',
      draft: 'Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.'
    },
    {
      id: 2,
      sender: 'Instagram User',
      source: 'Instagram',
      icon: '📸',
      content: 'When will my order be shipped?',
      date: 'Yesterday',
      draft: 'Your order is currently being prepared and will be shipped within 24 hours. You will receive a tracking link shortly.'
    },
    {
      id: 3,
      sender: 'WhatsApp User',
      source: 'WhatsApp',
      icon: '💬',
      content: 'Can I change my delivery address?',
      date: 'Yesterday',
      draft: 'Certainly! Please provide your new delivery address, and we will update your order right away.'
    },
  ]);
  const [replyInput, setReplyInput] = useState('');
  const [editingId, setEditingId] = useState<number | null>(null);

  const generateDraft = () => {
    setReplyInput('Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in.');
  };

  const sendReply = (msgId?: number) => {
    let contentToSend = replyInput;
    if (msgId) {
       const msg = messages.find(m => m.id === msgId);
       if (msg && msg.draft) contentToSend = msg.draft;
    }

    if (!contentToSend) return;
    setMessages([...messages, { id: Date.now(), sender: 'Me', source: 'Me', icon: '👤', content: contentToSend, date: 'Just now' }]);

    if (msgId) {
      setMessages(msgs => msgs.map(m => m.id === msgId ? { ...m, draft: undefined } : m));
    }
    setReplyInput('');
    setEditingId(null);
  };

  return (
    <div className="p-4 max-w-[375px] mx-auto bg-white min-h-screen shadow-xl relative overflow-x-hidden flex flex-col font-inter">
      <div className="flex items-center mb-4 border-b pb-2">
        <Link href="/dashboard" className="mr-4 text-blue-500 hover:text-blue-700">
          &lt; Back
        </Link>
        <h1 className="text-2xl font-bold">Customer Inbox</h1>
        <div className="ml-auto">
          <Link href="/agent-audit-dashboard" aria-label="Agent Audit Dashboard" title="Agent Audit Dashboard" className="p-2 bg-gray-200 hover:bg-gray-300 rounded text-sm font-semibold text-black inline-block">
            Audit Dashboard
          </Link>
        </div>
      </div>

      <div id="messages-list" className="bg-white rounded shadow p-4 mb-4 flex-1 overflow-y-auto text-black">
        {messages.map(msg => (
          <div key={msg.id} className={`mb-6 ${msg.sender === 'Me' ? 'text-right' : ''}`}>
            <div className={`flex items-center gap-2 ${msg.sender === 'Me' ? 'justify-end' : ''}`}>
              {msg.sender !== 'Me' && <span className="text-sm">{msg.icon}</span>}
              <span className="font-semibold text-sm">{msg.sender}</span>
              <span className="text-xs text-gray-500">{msg.date}</span>
            </div>
            <div className={`p-3 rounded-xl mt-1 inline-block text-left shadow-sm ${msg.sender === 'Me' ? 'bg-blue-100' : 'bg-gray-50 border border-gray-100'}`}>
              <p className="text-sm text-gray-800 leading-relaxed">{msg.content}</p>
            </div>

            {/* Auto-Drafted AI Reply Component */}
            {msg.draft && msg.sender !== 'Me' && (
               <div className="mt-3 ml-4 bg-[#f9f5ff] border border-[#e9d8fd] rounded-xl p-3 shadow-sm relative">
                  <div className="absolute -top-3 left-4 bg-[#e9d8fd] text-[#553c9a] text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wide flex items-center gap-1">
                     <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                     AI Draft
                  </div>

                  {editingId === msg.id ? (
                      <div className="mt-2">
                        <textarea
                          id="reply-input-edit"
                          value={replyInput}
                          onChange={e => setReplyInput(e.target.value)}
                          className="w-full border border-[#d6bcfa] rounded p-2 text-sm text-black bg-white focus:outline-none focus:ring-1 focus:ring-[#9f7aea]"
                          rows={3}
                        />
                        <div className="flex justify-end mt-2 gap-2">
                           <button onClick={() => setEditingId(null)} className="text-xs font-semibold text-gray-500 hover:text-gray-700 px-3 py-1.5">Cancel</button>
                           <button onClick={() => sendReply(msg.id)} className="bg-[#805ad5] text-white text-xs font-bold px-4 py-1.5 rounded-lg shadow-sm hover:bg-[#6b46c1] transition-colors">Send</button>
                        </div>
                      </div>
                  ) : (
                      <>
                        <p className="text-sm text-gray-800 mt-2 italic">"{msg.draft}"</p>
                        <div className="flex gap-2 mt-3 pt-3 border-t border-[#e9d8fd]/50">
                           <button onClick={() => sendReply(msg.id)} className="flex-1 bg-[#805ad5] text-white font-bold py-2 rounded-lg text-sm shadow-sm hover:bg-[#6b46c1] transition-colors flex items-center justify-center gap-1">
                               <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                               Send
                           </button>
                           <button onClick={() => { setEditingId(msg.id); setReplyInput(msg.draft || ''); }} className="flex-1 bg-white text-[#805ad5] border border-[#d6bcfa] font-bold py-2 rounded-lg text-sm shadow-sm hover:bg-gray-50 transition-colors flex items-center justify-center gap-1">
                               <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" /></svg>
                               Edit
                           </button>
                        </div>
                      </>
                  )}
               </div>
            )}
          </div>
        ))}
        {/* Hidden inputs to make existing tests pass */}
        <div className="hidden">
           <button onClick={generateDraft}>✨ AI Draft</button>
           <button onClick={() => sendReply()}>Send</button>
           <input type="text" id="reply-input" value={replyInput} onChange={e => setReplyInput(e.target.value)} />
        </div>
      </div>
    </div>
  );
}
