'use client';
import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function NativeOmnichannelChat() {
  const router = useRouter();
  const [messages, setMessages] = useState<any[]>([]);
  const [message, setMessage] = useState('');

  const handleApprove = async (msgId: string) => {
    const updated = messages.map(m => {
      if (m.id === msgId && m.card) {
        return { ...m, card: { ...m.card, status: 'approved' } };
      }
      return m;
    });
    setMessages(updated);

    // Call the native rust backend to send it (mocked call here)
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

    // Simulate AI Draft intervention from native rust
    setTimeout(() => {
        setMessages(prev => [...prev, {
            id: Date.now().toString() + '-ai',
            role: 'system',
            content: "I've drafted a reply for your approval.",
            card: {
                id: Date.now().toString() + '-card',
                department: 'Customer & Relationship Assistant',
                description: `Drafted reply: Yes, we can certainly help with that.`,
                status: 'pending'
            }
        }]);
    }, 1000);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] glassmorphism shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button aria-label="Back to Team" onClick={() => router.push('/team')} className="text-gray-500">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
          </button>
          <div>
            <h1 className="text-xl font-bold font-outfit text-gray-900">Inbox</h1>
            <p className="text-xs text-[#34C759] font-medium">Customer Support</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          <div className="flex gap-2 mb-4">
            <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
               <span className="text-xs font-bold text-[#0071E3]">IG</span>
            </div>
            <div className="bg-gray-100 p-3 rounded-2xl rounded-tl-none text-sm text-gray-800">
              Customer: Do you do vegan cakes?
            </div>
          </div>

          {messages.map((msg) => (
            <div key={msg.id} className={`flex gap-2 ${msg.role === 'user' ? 'flex-row-reverse' : 'flex-row'}`}>
              {msg.role === 'system' && (
                <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0 mt-1">
                  <span className="text-xs font-bold text-[#0071E3]">AI</span>
                </div>
              )}

              <div className="flex flex-col gap-2 max-w-[80%]">
                <div className={`p-3 text-sm ${msg.role === 'user' ? 'bg-[#0066FF] text-white rounded-2xl rounded-tr-none' : 'bg-gray-100 text-gray-800 rounded-2xl rounded-tl-none'}`}>
                  {msg.content}
                </div>

                {msg.card && (
                  <div className="app-card bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 p-4 shadow-sm relative overflow-hidden" data-testid="action-card">
                    <div className={`absolute top-0 left-0 w-full h-1 ${msg.card.status === 'approved' ? 'bg-[#34C759]' : 'bg-gradient-to-r from-blue-400 to-indigo-500'}`}></div>

                    <p className="text-sm font-semibold text-gray-900 mb-1">{msg.card.department}</p>
                    <p className="text-xs text-gray-600 mb-4">{msg.card.description}</p>

                    {msg.card.status === 'pending' ? (
                      <div className="flex gap-2">
                        <button
                          onClick={() => handleApprove(msg.id)}
                          className="flex-1 bg-[#0071E3] hover:bg-blue-700 text-white text-xs font-semibold py-2 px-3 rounded-lg transition-colors"
                          data-testid="approve-action-btn"
                        >
                          Approve & Send
                        </button>
                        <button
                          type="button"
                          onClick={() => handleEdit(msg.card?.description || '')}
                          className="bg-gray-100 hover:bg-gray-200 text-gray-700 text-xs font-medium py-2 px-3 rounded-lg transition-colors"
                        >
                          Edit Details
                        </button>
                      </div>
                    ) : (
                        <span className="text-xs font-bold px-2 py-0.5 bg-green-100 text-green-700 rounded-full uppercase tracking-wide">Sent</span>
                    )}
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>

        {/* Input */}
        <div className="p-4 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border-t border-white/40 sticky bottom-0 z-10">
          <div className="flex items-center bg-gray-50 rounded-full border border-gray-200 px-4 py-2">
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-sm"
              placeholder="AI Draft: Yes, we do vegan cakes..."
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
