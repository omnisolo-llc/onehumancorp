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

  const handleApprove = async (msgId: string) => {
    // Find the message to get the card ID
    const msgToApprove = messages.find(m => m.id === msgId);
    if (!msgToApprove || !msgToApprove.card) return;

    try {
      const response = await fetch(`/api/agents/approvals/${msgToApprove.card.id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ approved: true })
      });

      if (response.ok) {
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
      } else {
        console.error("Failed to approve action");
      }
    } catch (e) {
      console.error("Error approving action", e);
    }
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
        },
        body: JSON.stringify({ message: userMsg, enableToolsGating: true, enableTaoOrchestrationLoop: true })
      });
      if (response.ok) {
        const data = await response.json();

        // Use a generic placeholder card ID since /api/agents/chat response
        // doesn't return an approval ID directly in this simplified version,
        // but we want to simulate the data flow as realistically as possible
        const approvalId = data.approval_id || Date.now().toString() + '-approval';

        const msgId = Date.now().toString() + '-system';
        setMessages(prev => [...prev, {
          id: msgId,
          role: 'system',
          content: "I've drafted an action for your approval.",
          card: {
            id: approvalId,
            department: data.department_assigned || 'The Manager',
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
      <div
        className="w-[375px] min-h-[812px] rounded-[16px] shadow-2xl overflow-hidden flex flex-col relative border border-white/40"
        style={{
          background: 'rgba(255, 255, 255, 0.65)',
          backdropFilter: 'blur(30px) saturate(210%)',
          WebkitBackdropFilter: 'blur(30px) saturate(210%)'
        }}
      >

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button onClick={() => router.push('/team')} className="text-gray-500 hover:text-gray-700 min-w-[44px] min-h-[44px] flex items-center justify-center">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          </button>
          <div>
            <h1 className="text-xl font-bold font-outfit text-[#1D1D1F]">Team Chat</h1>
            <p className="text-xs text-[#34C759] font-medium tracking-wide">All departments online</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          <div className="flex gap-2 mb-4">
            <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
               <span className="text-xs font-bold text-blue-600">OHC</span>
            </div>
            <div className="bg-gray-100 p-3 rounded-2xl rounded-tl-none text-sm text-gray-800">
              Hello! I'm your central team interface. How can we help your business today?
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
                <div className={`p-3 text-sm ${msg.role === 'user' ? 'bg-[#0066FF] text-[#F5F5F7] rounded-2xl rounded-tr-none' : 'bg-[rgba(255,255,255,0.8)] border border-white/40 text-[#1D1D1F] rounded-2xl rounded-tl-none'}`}>
                  {msg.content}
                </div>

                {/* Action Card if present */}
                {msg.card && (
                  <div className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-200 border border-white/40 rounded-[16px] p-4 shadow-sm relative overflow-hidden" data-testid="action-card">
                    <div className={`absolute top-0 left-0 w-full h-1 ${msg.card.status === 'approved' ? 'bg-[#34C759]' : 'bg-gradient-to-r from-[#0066FF] to-indigo-500'}`}></div>
                    <div className="flex items-center gap-2 mb-2">
                      {msg.card.status === 'pending' ? (
                        <span className="text-xs font-bold px-2 py-0.5 bg-[#FF9500]/10 text-[#FF9500] rounded-full uppercase tracking-wide">Needs Approval</span>
                      ) : (
                        <span className="text-xs font-bold px-2 py-0.5 bg-[#34C759]/10 text-[#34C759] rounded-full uppercase tracking-wide">Approved</span>
                      )}
                    </div>
                    <p className="text-sm font-semibold text-[#1D1D1F] mb-1">{msg.card.department}</p>
                    <p className="text-xs text-[#1D1D1F]/70 mb-4">{msg.card.description}</p>

                    {msg.card.status === 'pending' && (
                      <div className="flex gap-2">
                        <button
                          onClick={() => handleApprove(msg.id)}
                          className="flex-1 bg-[#0066FF] hover:bg-[#0066FF]/90 text-white text-xs font-semibold py-2 px-3 rounded-[8px] transition-colors min-h-[44px]"
                          data-testid="approve-action-btn"
                        >
                          Approve & Execute
                        </button>
                        <button className="bg-gray-100/50 hover:bg-gray-200 text-[#1D1D1F] text-xs font-medium py-2 px-3 rounded-[8px] transition-colors min-h-[44px]">
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
        <div className="p-4 border-t border-white/40 sticky bottom-0 z-10 bg-[rgba(255,255,255,0.4)] backdrop-blur-[30px] saturate-200">
          <div className="flex items-center bg-[rgba(255,255,255,0.8)] rounded-[16px] border border-white/40 px-4 py-2 min-h-[44px]">
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-sm text-[#1D1D1F] placeholder-gray-500"
              placeholder="Message your team..."
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              data-testid="team-chat-input"
            />
            <button
              className="ml-2 text-[#0066FF] font-medium text-sm disabled:opacity-50 min-w-[44px] min-h-[44px] flex items-center justify-center"
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
