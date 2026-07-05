"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export type ActionCard = {
  id: string;
  department: string;
  description: string;
  status: 'pending' | 'approved';
  feature_type?: string;
  suggested_price?: number;
  scope?: string;
};

type ChatMessage = {
  id: string;
  role: 'user' | 'system';
  content: string;
  detail?: string;
  card?: ActionCard;
  error?: {
    retryMessage: string;
  };
};

export default function TeamChatPage() {
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const router = useRouter();

  const handleApprove = async (msgId: string) => {
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

    const msg = messages.find(m => m.id === msgId);
    if (msg && msg.card && !msg.card.id.endsWith('-card')) {
      try {
        const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
        await fetch(`/api/agents/approvals/${msg.card.id}`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`
          },
          body: JSON.stringify({ approved: true })
        });
      } catch (e) {
        console.error("Failed to approve action", e);
      }
    }
  };

  const handleEdit = (description: string) => {
    setMessage(description);
  };

  const handleSend = async () => {
    if (!message.trim()) return;
    const userMsg = message;
    setMessage('');

    const userMsgId = Date.now().toString() + '-user';
    const pendingMsgId = Date.now().toString() + '-pending';
    setMessages(prev => [...prev, {id: userMsgId, role: 'user', content: userMsg}]);
    setMessages(prev => [...prev, {
      id: pendingMsgId,
      role: 'system',
      content: 'Working on your request...',
      detail: 'The team is still drafting the action.',
    }]);

    try {
      const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
      const response = await fetch('/api/agents/chat', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ message: userMsg, enableToolsGating: true, enableTaoOrchestrationLoop: true })
      });
      const data = await response.json().catch(() => ({}));
      if (response.ok) {

        const msgId = Date.now().toString() + '-system';
        setMessages(prev => prev.filter(msg => msg.id !== pendingMsgId).concat({
          id: msgId,
          role: 'system',
          content: "I've drafted an action for your approval.",
          card: {
            id: Date.now().toString() + '-card',
            department: data.agent || 'The Manager',
            description: data.description || `Drafted action based on: "${userMsg}"`,
              status: 'pending'
            }
        }));

      } else {
        setMessages(prev => prev.map(msg => msg.id === pendingMsgId ? {
          ...msg,
          content: 'Action needs attention',
          detail: data.error || data.message || 'Failed to process your request. Ensure backend auth is provided.',
          error: { retryMessage: userMsg },
        } : msg));
      }
    } catch (e) {
      setMessages(prev => prev.map(msg => msg.id === pendingMsgId ? {
        ...msg,
        content: 'Action needs attention',
        detail: 'Error connecting to the team.',
        error: { retryMessage: userMsg },
      } : msg));
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] glassmorphism shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button aria-label="Back to Team" onClick={() => router.push('/team')} className="text-gray-500">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          </button>
          <div>
            <h1 className="text-xl font-bold font-outfit text-gray-900">Team Chat</h1>
            <p className="text-xs text-[#34C759] font-medium">All departments online</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          <div className="flex gap-2 mb-4">
            <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
               <span className="text-xs font-bold text-[#0071E3]">OHC</span>
            </div>
            <div className="bg-gray-100 p-3 rounded-2xl rounded-tl-none text-sm text-gray-800">
              Hello! I'm your central team interface. How can we help your business today?
            </div>
          </div>

          {messages.map((msg) => (
            <div key={msg.id} className={`flex gap-2 ${msg.role === 'user' ? 'flex-row-reverse' : 'flex-row'}`}>
              {msg.role === 'system' && (
                <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0 mt-1">
                  <span className="text-xs font-bold text-[#0071E3]">OHC</span>
                </div>
              )}

              <div className="flex flex-col gap-2 max-w-[80%]">
                {/* Text Bubble */}
                <div className={`p-3 text-sm ${msg.role === 'user' ? 'bg-[#0066FF] text-white rounded-2xl rounded-tr-none' : 'bg-gray-100 text-gray-800 rounded-2xl rounded-tl-none'}`}>
                  {msg.content}
                  {msg.detail && !msg.error && (
                    <p className="mt-1 text-xs text-gray-600">
                      {msg.detail}
                    </p>
                  )}
                </div>

                {msg.error && (
                  <div className="bg-red-50 border border-red-100 rounded-xl p-3 shadow-sm" role="alert">
                    <p className="text-xs font-semibold text-red-800 mb-2">{msg.detail}</p>
                    <button
                      type="button"
                      onClick={() => setMessage(msg.error?.retryMessage || '')}
                      className="min-h-[36px] rounded-lg bg-red-600 px-3 text-xs font-semibold text-white hover:bg-red-700"
                    >
                      Try again
                    </button>
                  </div>
                )}

                {/* Action Card if present */}
                {msg.card && (
                  <div className="app-card bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 p-4 shadow-sm relative overflow-hidden" data-testid="action-card">
                    <div className={`absolute top-0 left-0 w-full h-1 ${msg.card.status === 'approved' ? 'bg-[#34C759]' : 'bg-gradient-to-r from-blue-400 to-indigo-500'}`}></div>
                    <div className="flex items-center gap-2 mb-2">
                      {msg.card.status === 'pending' ? (
                        <span className="text-xs font-bold px-2 py-0.5 bg-orange-100 text-orange-700 rounded-full uppercase tracking-wide">Needs Approval</span>
                      ) : (
                        <span className="text-xs font-bold px-2 py-0.5 bg-green-100 text-green-700 rounded-full uppercase tracking-wide">Approved</span>
                      )}
                    </div>

                    {msg.card.feature_type === 'quote_draft' ? (
                      <div data-testid="quote-draft-card">
                        <p className="text-sm font-semibold text-gray-900 mb-1">Draft Quote: {msg.card.department} for Customer</p>
                        <p className="text-xs text-gray-600 mb-2">Scope of Work: {msg.card.scope || msg.card.description}</p>
                        <p className="text-sm font-bold text-gray-900 mb-4">Calculated Total: ${msg.card.suggested_price || 0}</p>
                      </div>
                    ) : (
                      <>
                        <p className="text-sm font-semibold text-gray-900 mb-1">{msg.card.department}</p>
                        <p className="text-xs text-gray-600 mb-4">{msg.card.description}</p>
                      </>
                    )}

                    {msg.card.status === 'pending' && (
                      <div className="flex gap-2">
                        <button
                          onClick={() => handleApprove(msg.id)}
                          className="flex-1 bg-[#0071E3] hover:bg-blue-700 text-white text-xs font-semibold py-2 px-3 rounded-lg transition-colors"
                          data-testid="approve-action-btn"
                        >
                          {msg.card.feature_type === 'quote_draft' ? 'Approve & Send' : 'Approve & Execute'}
                        </button>
                        <button
                          type="button"
                          onClick={() => handleEdit(msg.card?.description || '')}
                          className="bg-gray-100 hover:bg-gray-200 text-gray-700 text-xs font-medium py-2 px-3 rounded-lg transition-colors"
                        >
                          Edit Details
                        </button>
                        {msg.card.feature_type === 'quote_draft' && (
                           <button
                             type="button"
                             onClick={() => setMessages(prev => prev.filter(m => m.id !== msg.id))}
                             className="bg-red-50 hover:bg-red-100 text-red-700 text-xs font-medium py-2 px-3 rounded-lg transition-colors"
                           >
                             Discard
                           </button>
                        )}
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>

        {/* Input */}
        <div className="p-4 bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] border-t border-white/40 sticky bottom-0 z-10">
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
              className="ml-2 text-[#0066FF] font-medium text-sm disabled:opacity-50"
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
