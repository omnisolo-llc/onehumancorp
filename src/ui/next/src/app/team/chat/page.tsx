"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export type ActionCard = {
  id: string;
  department: string;
  description: string;
  status: 'pending' | 'approved';
};

export default function TeamChatPage() {
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<{id: string, role: 'user'|'system', content: string, card?: ActionCard}[]>([]);
  const router = useRouter();

  const handleApprove = (msgId: string) => {
    setMessages(prev => prev.map(msg => {
      if (msg.id === msgId && msg.card) {
        return {
          ...msg,
          card: {
            ...msg.card,
            status: 'approved'
          }
        };
      }
      return msg;
    }));
  };

  const handleSend = async () => {
    if (!message.trim()) return;
    const userMsg = message;
    setMessage('');

    const userMsgId = Date.now().toString() + '-user';
    setMessages(prev => [...prev, {id: userMsgId, role: 'user', content: userMsg}]);

    try {
      const response = await fetch('/api/agents/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          // Assuming authorization is handled by Next.js API routes / middleware in reality
          // 'Authorization': 'Bearer ' + token
        },
        body: JSON.stringify({ message: userMsg })
      });
      if (response.ok) {
        const data = await response.json();

        const msgId = Date.now().toString() + '-system';
        setMessages(prev => [...prev, {
          id: msgId,
          role: 'system',
          content: "I've drafted an action for your approval.",
          card: {
            id: Date.now().toString() + '-card',
            department: data.agent || 'The Manager',
            description: data.description || `Drafted action based on: "${userMsg}"`,
            status: 'pending'
          }
        }]);

      } else {
        setMessages(prev => [...prev, {id: Date.now().toString(), role: 'system', content: "Failed to process your request. Ensure backend auth is provided."}]);
      }
    } catch (e) {
      setMessages(prev => [...prev, {id: Date.now().toString(), role: 'system', content: "Error connecting to the team."}]);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 border-b border-gray-100 flex items-center gap-4">
          <button onClick={() => router.push('/team')} className="text-gray-500">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          </button>
          <div>
            <h1 className="text-xl font-bold font-outfit text-gray-900">Team Chat</h1>
            <p className="text-xs text-green-500 font-medium">All departments online</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          <div className="flex gap-2 mb-4">
            <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
               <span className="text-xs font-bold text-blue-600">OHC</span>
            </div>
            <div className="bg-gray-100 p-3 rounded-2xl rounded-tl-none text-sm text-gray-800">
              Hello! I'm your unified team interface. How can we help your business today?
            </div>
          </div>

          {messages.map((msg) => (
            <div key={msg.id} className={`flex gap-2 ${msg.role === 'user' ? 'flex-row-reverse' : 'flex-row'}`}>
              {msg.role === 'system' && (
                <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0 mt-1">
                  <span className="text-xs font-bold text-blue-600">OHC</span>
                </div>
              )}

              <div className="flex flex-col gap-2 max-w-[80%]">
                {/* Text Bubble */}
                <div className={`p-3 text-sm ${msg.role === 'user' ? 'bg-blue-500 text-white rounded-2xl rounded-tr-none' : 'bg-gray-100 text-gray-800 rounded-2xl rounded-tl-none'}`}>
                  {msg.content}
                </div>

                {/* Action Card if present */}
                {msg.card && (
                  <div className="bg-white/60 backdrop-blur-md border border-gray-200 rounded-xl p-4 shadow-sm relative overflow-hidden" data-testid="action-card">
                    <div className={`absolute top-0 left-0 w-full h-1 ${msg.card.status === 'approved' ? 'bg-green-500' : 'bg-gradient-to-r from-blue-400 to-indigo-500'}`}></div>
                    <div className="flex items-center gap-2 mb-2">
                      {msg.card.status === 'pending' ? (
                        <span className="text-xs font-bold px-2 py-0.5 bg-orange-100 text-orange-700 rounded-full uppercase tracking-wide">Needs Approval</span>
                      ) : (
                        <span className="text-xs font-bold px-2 py-0.5 bg-green-100 text-green-700 rounded-full uppercase tracking-wide">Approved</span>
                      )}
                    </div>
                    <p className="text-sm font-semibold text-gray-900 mb-1">{msg.card.department}</p>
                    <p className="text-xs text-gray-600 mb-4">{msg.card.description}</p>

                    {msg.card.status === 'pending' && (
                      <div className="flex gap-2">
                        <button
                          onClick={() => handleApprove(msg.id)}
                          className="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-xs font-semibold py-2 px-3 rounded-lg transition-colors"
                          data-testid="approve-action-btn"
                        >
                          Approve & Execute
                        </button>
                        <button className="bg-gray-100 hover:bg-gray-200 text-gray-700 text-xs font-medium py-2 px-3 rounded-lg transition-colors">
                          Edit
                        </button>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>

        {/* Input */}
        <div className="p-4 border-t border-gray-100 bg-white">
          <div className="flex items-center bg-gray-50 rounded-full border border-gray-200 px-4 py-2">
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-sm"
              placeholder="Message your team..."
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              data-testid="team-chat-input"
            />
            <button
              className="ml-2 text-blue-500 font-medium text-sm disabled:opacity-50"
              onClick={handleSend}
              disabled={!message.trim()}
              data-testid="team-chat-send"
            >
              Send
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
