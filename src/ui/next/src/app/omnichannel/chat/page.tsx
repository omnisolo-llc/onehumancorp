'use client';

import { useState, useEffect } from 'react';

type Message = {
  id: string;
  sender_type: string;
  content: string;
  status: string;
  draft_reply?: string;
  created_at: string;
};

export default function OmnichannelChat() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');

  // Example placeholder for tenant handling
  const tenant_id = 'test-tenant';
  const conversation_id = 'test-conversation';

  useEffect(() => {
    fetchMessages();

    // Setup WS placeholder
    const ws = new WebSocket(`ws://${window.location.host}/api/v1/omnichannel/chat/ws`);
    ws.onmessage = (event) => {
        console.log("WS update:", event.data);
    };
    return () => ws.close();
  }, []);

  const fetchMessages = async () => {
    // Basic REST or gRPC call placeholder
    // We would use the client connected to our rust server
    // Setting up a dummy message for now to represent the UI required
    setMessages([
        {
            id: '1',
            sender_type: 'customer',
            content: 'Hello, I have a question about my cake order.',
            status: 'sent',
            created_at: new Date().toISOString()
        },
        {
            id: '2',
            sender_type: 'agent',
            content: 'Sure! I can help you with that. What is your order number?',
            status: 'draft',
            draft_reply: 'Sure! I can help you with that. What is your order number?',
            created_at: new Date().toISOString()
        }
    ]);
  };

  const approveDraft = async (id: string) => {
    // Call our approve_draft gRPC/REST method
    setMessages(msgs => msgs.map(m => m.id === id ? { ...m, status: 'sent' } : m));
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] glassmorphism shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-6 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <div>
            <h1 className="text-xl font-bold font-outfit text-gray-900">Unified Inbox</h1>
            <p className="text-xs text-[#34C759] font-medium">Customer Chat</p>
          </div>
        </div>

        {/* Chat Area */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {messages.map((msg) => (
            <div key={msg.id} className={`flex gap-2 flex-col ${msg.sender_type === 'agent' ? 'items-end' : 'items-start'}`}>
              <div className={`p-3 text-sm max-w-[80%] ${msg.sender_type === 'agent' ? 'bg-[#0066FF] text-white rounded-2xl rounded-tr-none' : 'bg-gray-100 text-gray-800 rounded-2xl rounded-tl-none'}`}>
                {msg.content}
              </div>
              {msg.status === 'draft' && msg.draft_reply && (
                <div className="app-card bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 p-3 shadow-sm relative mt-2 w-full max-w-[80%] rounded-xl">
                    <p className="text-xs font-semibold text-gray-900 mb-2">AI Draft Approval</p>
                    <button
                        onClick={() => approveDraft(msg.id)}
                        className="w-full bg-[#34C759] hover:bg-green-600 text-white text-xs font-semibold py-2 px-3 rounded-lg transition-colors"
                        data-testid="approve-draft-btn"
                    >
                        Approve & Send
                    </button>
                </div>
              )}
            </div>
          ))}
        </div>

        {/* Input area */}
        <div className="p-4 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border-t border-white/40 sticky bottom-0 z-10">
          <div className="flex items-center bg-gray-50 rounded-full border border-gray-200 px-4 py-2">
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-sm"
              placeholder="Type a message..."
              value={input}
              onChange={(e) => setInput(e.target.value)}
              data-testid="chat-input"
            />
            <button
              className="ml-2 text-[#0066FF] font-medium text-sm disabled:opacity-50"
              disabled={!input.trim()}
              data-testid="chat-send"
            >
              Send
            </button>
          </div>
        </div>

      </div>
    </div>
  );
}
