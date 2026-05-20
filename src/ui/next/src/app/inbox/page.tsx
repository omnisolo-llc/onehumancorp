"use client";

import React, { useState } from 'react';

type Message = {
  id: string;
  sender: string;
  text: string;
  channel: 'instagram' | 'whatsapp' | 'website';
  handledByAI: boolean;
  needsAttention: boolean;
  time: string;
};

const MOCK_MESSAGES: Message[] = [
  {
    id: '1',
    sender: 'Sarah Jenkins',
    text: 'Do you offer vegan birthday cake options? My son is allergic to dairy.',
    channel: 'instagram',
    handledByAI: true,
    needsAttention: false,
    time: '2m ago'
  },
  {
    id: '2',
    sender: 'Mike T.',
    text: 'I ordered the custom wedding cake but I need to change the delivery address to the venue instead of my house. Is that possible this close to the date?',
    channel: 'website',
    handledByAI: true,
    needsAttention: true,
    time: '15m ago'
  },
  {
    id: '3',
    sender: 'Elena Rodriguez',
    text: 'What are your hours on Sunday?',
    channel: 'whatsapp',
    handledByAI: true,
    needsAttention: false,
    time: '1h ago'
  },
  {
    id: '4',
    sender: 'David Kim',
    text: 'Hi, checking on order #4092',
    channel: 'instagram',
    handledByAI: false,
    needsAttention: false,
    time: '2h ago'
  }
];

export default function InboxPage() {
  const [aiEnabled, setAiEnabled] = useState(true);
  const [selectedMessage, setSelectedMessage] = useState<Message | null>(null);

  const renderChannelIcon = (channel: string) => {
    switch (channel) {
      case 'instagram': return <span className="text-pink-500 text-xs">📸</span>;
      case 'whatsapp': return <span className="text-green-500 text-xs">💬</span>;
      case 'website': return <span className="text-blue-500 text-xs">🌐</span>;
      default: return null;
    }
  };

  if (selectedMessage) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
        <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
          {/* Header */}
          <div className="pt-12 pb-6 px-6 bg-white/60 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
            <button
              onClick={() => setSelectedMessage(null)}
              className="w-10 h-10 flex items-center justify-center rounded-full bg-white shadow-sm border border-gray-100 text-gray-500 hover:text-gray-900 transition-colors"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <div>
              <h1 className="text-xl font-bold font-outfit text-gray-900 tracking-tight">{selectedMessage.sender}</h1>
              <p className="text-gray-500 text-xs font-medium capitalize flex items-center gap-1">
                {renderChannelIcon(selectedMessage.channel)} {selectedMessage.channel}
              </p>
            </div>
          </div>

          <div className="flex-1 p-6 overflow-y-auto">
            <div className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 inline-block max-w-[85%] text-sm text-gray-800">
              {selectedMessage.text}
            </div>

            {/* Draft UI for testing */}
            <div className="mt-8">
               <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">Reply Draft</label>
               <textarea id="reply-input" className="w-full h-32 p-3 border border-gray-200 rounded-xl text-sm" placeholder="Type your reply..."></textarea>
               <div className="flex gap-2 mt-3">
                 <button className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 shadow-md transition-all">
                   AI Draft
                 </button>
                 <button className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-gray-900 text-white hover:bg-black shadow-md transition-all">
                   Send
                 </button>
               </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/80 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex flex-col gap-4">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Customer Inbox</h1>
            <button
              className="w-10 h-10 rounded-full bg-gray-100 flex items-center justify-center text-gray-600"
              onClick={() => window.history.back()}
            >
               <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
            </button>
          </div>

          <div className="flex items-center justify-between p-3 rounded-xl bg-white/50 border border-white/60 shadow-sm">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 text-lg">
                ✨
              </div>
              <div>
                <p className="text-sm font-semibold text-gray-900">AI Assistant</p>
                <p className="text-xs text-gray-500">Auto-reply active</p>
              </div>
            </div>

            <button
              role="switch"
              aria-checked={aiEnabled}
              onClick={() => setAiEnabled(!aiEnabled)}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${aiEnabled ? 'bg-blue-600' : 'bg-gray-300'}`}
            >
              <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${aiEnabled ? 'translate-x-6' : 'translate-x-1'}`} />
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-4 space-y-3 pb-24" id="messages-list">
          {MOCK_MESSAGES.map(msg => (
            <div
              key={msg.id}
              onClick={() => setSelectedMessage(msg)}
              className="cursor-pointer bg-white rounded-2xl p-4 shadow-[0_2px_10px_-4px_rgba(0,0,0,0.05)] border border-gray-100 hover:shadow-md transition-shadow relative overflow-hidden group"
            >
              {msg.handledByAI && (
                <div className="absolute top-0 right-0 w-16 h-16 bg-gradient-to-bl from-blue-50/50 to-transparent -z-10 group-hover:scale-110 transition-transform"></div>
              )}

              <div className="flex justify-between items-start mb-2">
                <div className="flex items-center gap-2">
                  <h3 className="font-semibold text-gray-900 text-sm">{msg.sender}</h3>
                  {renderChannelIcon(msg.channel)}
                </div>
                <span className="text-[10px] text-gray-400 font-medium">{msg.time}</span>
              </div>

              <p className="text-sm text-gray-600 line-clamp-2 leading-relaxed mb-3">
                {msg.text}
              </p>

              <div className="flex items-center gap-2 mt-2">
                {msg.handledByAI && (
                  <span className="inline-flex items-center gap-1 px-2 py-1 rounded-md bg-blue-50 text-blue-700 text-[10px] font-bold uppercase tracking-wider border border-blue-100">
                    <span>✨</span> Handled by AI
                  </span>
                )}

                {msg.needsAttention && (
                  <span className="inline-flex items-center gap-1 px-2 py-1 rounded-md bg-orange-50 text-orange-700 text-[10px] font-bold uppercase tracking-wider border border-orange-100">
                    <span style={{ color: '#FF9500' }}>⚠️</span> Needs your attention
                  </span>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
