'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

type Conversation = {
  id: string;
  channel: string;
  customer_name: string;
  last_message: string;
  last_message_at_unix: number;
  ai_enabled: boolean;
  status: string;
};

type ChatMessage = {
  id: string;
  conversation_id: string;
  sender_role: 'customer' | 'agent' | 'ai';
  content: string;
  timestamp_unix: number;
  is_draft: boolean;
};

export default function InboxPage() {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [selectedConversation, setSelectedConversation] = useState<Conversation | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [loading, setLoading] = useState(true);
  const [replyInput, setReplyInput] = useState('');
  const [editingId, setEditingId] = useState<string | null>(null);
  const [aiActive, setAiActive] = useState(true);

  // Fetch conversations
  useEffect(() => {
    fetch('/api/inbox/conversations')
      .then(res => res.json())
      .then(data => {
        setConversations(data.conversations);
        setLoading(false);
      });
  }, []);

  // Fetch messages when conversation selected
  useEffect(() => {
    if (selectedConversation) {
      setMessages([]);
      fetch(`/api/inbox/messages?conversation_id=${selectedConversation.id}`)
        .then(res => res.json())
        .then(data => setMessages(data.messages));
    }
  }, [selectedConversation]);

  const handleSend = async () => {
    if (!replyInput.trim() || !selectedConversation) return;

    const res = await fetch('/api/inbox/messages', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        conversation_id: selectedConversation.id,
        content: replyInput,
        is_draft: false
      })
    });

    const newMessage = await res.json();
    setMessages([...messages, newMessage]);
    setReplyInput('');
  };

  const handleApprove = async (msgId: string, content: string) => {
    const res = await fetch('/api/inbox/approve', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message_id: msgId, content })
    });

    const data = await res.json();
    if (data.success) {
      setMessages(prev => prev.map(m => m.id === msgId ? data.message : m));
      setEditingId(null);
    }
  };

  const formatTime = (unix: number) => {
    const date = new Date(unix * 1000);
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const getChannelIcon = (channel: string) => {
    switch (channel.toLowerCase()) {
      case 'instagram': return '📸';
      case 'whatsapp': return '💬';
      case 'sms': return '📱';
      default: return '✉️';
    }
  };

  return (
    <div className="flex flex-col h-screen bg-[#F5F5F7] font-inter overflow-hidden">
      {/* Premium Glass App Bar */}
      <header className="h-16 flex items-center px-6 border-b shrink-0 z-10" style={{ background: "rgba(255, 255, 255, 0.7)", backdropFilter: "blur(20px) saturate(180%)", borderColor: "rgba(0,0,0,0.05)" }}>
        <Link href="/dashboard" className="mr-4 text-blue-600 hover:text-blue-800 transition-colors flex items-center gap-1 font-semibold">
           <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
           Dashboard
        </Link>
        <h1 className="text-xl font-bold font-outfit text-gray-900">Inbox</h1>

        <div className="ml-auto flex items-center gap-4">
           <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-white/50 border border-black/5 shadow-sm">
              <div className={`w-2 h-2 rounded-full ${aiActive ? 'bg-[#34C759] animate-pulse' : 'bg-gray-400'}`}></div>
              <span className="text-xs font-bold text-gray-700 uppercase tracking-wider">AI Ambassador: {aiActive ? 'Active' : 'Paused'}</span>
              <button onClick={() => setAiActive(!aiActive)} className="ml-1 text-[10px] bg-black/5 hover:bg-black/10 px-2 py-0.5 rounded-md transition-colors font-bold uppercase">Toggle</button>
           </div>
        </div>
      </header>

      <div className="flex flex-1 overflow-hidden relative">
        {/* Feed View (The Queue) */}
        <div className={`w-full md:w-96 flex flex-col border-r bg-white/30 backdrop-blur-md ${selectedConversation ? 'hidden md:flex' : 'flex'}`}>
          <div className="p-4 border-b bg-white/20">
             <input type="text" placeholder="Search conversations..." className="w-full px-4 py-2 bg-black/5 border-none rounded-xl text-sm focus:ring-2 focus:ring-blue-500 outline-none" />
          </div>
          <div className="flex-1 overflow-y-auto p-3 space-y-3">
             {loading ? (
                <div className="flex flex-col items-center justify-center h-64 gap-3">
                   <div className="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                   <span className="text-sm text-gray-500 font-medium">Loading messages...</span>
                </div>
             ) : (
                conversations.map(conv => (
                   <button
                    key={conv.id}
                    onClick={() => setSelectedConversation(conv)}
                    className={`w-full text-left p-4 rounded-2xl transition-all duration-200 border relative group ${selectedConversation?.id === conv.id ? 'bg-white shadow-lg border-blue-200 ring-1 ring-blue-100' : 'bg-white/50 hover:bg-white/80 border-transparent shadow-sm'}`}
                   >
                      <div className="flex justify-between items-start mb-1">
                        <div className="flex items-center gap-2">
                           <span className="text-lg">{getChannelIcon(conv.channel)}</span>
                           <span className="font-bold text-gray-900 font-outfit truncate max-w-[140px]">{conv.customer_name}</span>
                        </div>
                        <span className="text-[10px] font-semibold text-gray-400 uppercase">{formatTime(conv.last_message_at_unix)}</span>
                      </div>
                      <p className="text-sm text-gray-500 line-clamp-1 leading-relaxed">{conv.last_message}</p>

                      {conv.ai_enabled && (
                        <div className="absolute bottom-4 right-4 opacity-0 group-hover:opacity-100 md:opacity-100 transition-opacity">
                           <div className="w-2 h-2 rounded-full bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.5)]"></div>
                        </div>
                      )}
                   </button>
                ))
             )}
          </div>
        </div>

        {/* Thread View */}
        <div className={`flex-1 flex flex-col bg-white relative ${selectedConversation ? 'flex' : 'hidden md:flex items-center justify-center'}`}>
          {selectedConversation ? (
            <>
              {/* Thread Header */}
              <div className="h-16 px-6 flex items-center border-b bg-white/80 backdrop-blur-md z-10 shrink-0">
                <button onClick={() => setSelectedConversation(null)} className="md:hidden mr-3 p-2 hover:bg-gray-100 rounded-full transition-colors">
                  <svg className="w-6 h-6 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
                </button>
                <div className="flex items-center gap-3">
                   <div className="w-10 h-10 rounded-full bg-[#F5F5F7] flex items-center justify-center text-xl border">
                      {getChannelIcon(selectedConversation.channel)}
                   </div>
                   <div>
                      <h2 className="font-bold text-gray-900 font-outfit">{selectedConversation.customer_name}</h2>
                      <p className="text-[10px] font-bold text-blue-500 uppercase tracking-widest">{selectedConversation.channel} • Active</p>
                   </div>
                </div>
              </div>

              {/* Messages List */}
              <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] [background-size:16px_16px]">
                {messages.map(msg => {
                  const isCustomer = msg.sender_role === 'customer';
                  const isAI = msg.sender_role === 'ai';

                  return (
                    <div key={msg.id} className={`flex flex-col ${isCustomer ? 'items-start' : 'items-end'}`}>
                       <div className="flex items-center gap-2 mb-1 px-1">
                          <span className="text-[10px] font-bold text-gray-400 uppercase tracking-tighter">
                            {isAI ? '✨ Ambassador' : isCustomer ? selectedConversation.customer_name : 'Me'} • {formatTime(msg.timestamp_unix)}
                          </span>
                       </div>

                       <div className={`relative max-w-[85%] md:max-w-[70%] p-4 rounded-2xl shadow-sm text-sm leading-relaxed ${
                         isAI ? 'bg-indigo-50 text-indigo-900 border border-indigo-100' :
                         isCustomer ? 'bg-white text-gray-800 border border-gray-100' :
                         'bg-blue-600 text-white border border-blue-500'
                       }`}>
                          {msg.is_draft && editingId === msg.id ? (
                            <div className="space-y-3">
                               <textarea
                                value={replyInput}
                                onChange={(e) => setReplyInput(e.target.value)}
                                className="w-full bg-white/50 border border-indigo-200 rounded-xl p-3 text-indigo-900 focus:ring-2 focus:ring-indigo-500 outline-none min-h-[100px]"
                               />
                               <div className="flex justify-end gap-2">
                                  <button onClick={() => { setEditingId(null); setReplyInput(''); }} className="px-4 py-2 text-xs font-bold text-indigo-600 hover:bg-indigo-100 rounded-lg transition-colors">Cancel</button>
                                  <button onClick={() => handleApprove(msg.id, replyInput)} className="px-5 py-2 text-xs font-bold bg-indigo-600 text-white rounded-lg shadow-md hover:bg-indigo-700 transition-all">Save & Send</button>
                               </div>
                            </div>
                          ) : (
                            <>
                              <p>{msg.content}</p>
                              {msg.is_draft && (
                                <div className="mt-4 pt-4 border-t border-indigo-200/50 flex gap-2">
                                   <button
                                    onClick={() => handleApprove(msg.id, msg.content)}
                                    className="flex-1 bg-indigo-600 text-white font-bold py-2.5 rounded-xl shadow-md hover:bg-indigo-700 transition-all flex items-center justify-center gap-2"
                                   >
                                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                                      Approve
                                   </button>
                                   <button
                                    onClick={() => { setEditingId(msg.id); setReplyInput(msg.content); }}
                                    className="flex-1 bg-white text-indigo-600 border border-indigo-200 font-bold py-2.5 rounded-xl hover:bg-indigo-50 transition-all flex items-center justify-center gap-2"
                                   >
                                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" /></svg>
                                      Edit
                                   </button>
                                </div>
                              )}
                            </>
                          )}
                       </div>
                    </div>
                  );
                })}
              </div>

              {/* Input Area */}
              <div className="p-4 border-t bg-white/80 backdrop-blur-md shrink-0">
                 <div className="flex gap-2 items-end max-w-4xl mx-auto">
                    <div className="flex-1 bg-gray-100 rounded-2xl p-1 focus-within:ring-2 focus-within:ring-blue-500 transition-all shadow-inner">
                       <textarea
                        value={replyInput}
                        onChange={(e) => setReplyInput(e.target.value)}
                        placeholder="Type a message..."
                        className="w-full bg-transparent border-none focus:ring-0 text-sm p-3 min-h-[44px] max-h-32 text-gray-800 resize-none"
                        rows={1}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter' && !e.shiftKey) {
                            e.preventDefault();
                            handleSend();
                          }
                        }}
                       />
                    </div>
                    <button
                      onClick={handleSend}
                      disabled={!replyInput.trim()}
                      className="w-12 h-12 flex items-center justify-center bg-blue-600 text-white rounded-2xl shadow-lg hover:bg-blue-700 transition-all disabled:opacity-50 disabled:grayscale"
                    >
                      <svg className="w-6 h-6 rotate-90" fill="currentColor" viewBox="0 0 24 24"><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z" /></svg>
                    </button>
                 </div>
                 <p className="text-[10px] text-center text-gray-400 font-medium mt-2 uppercase tracking-widest">Shift + Enter for new line</p>
              </div>
            </>
          ) : (
            <div className="text-center p-8 max-w-sm">
               <div className="w-24 h-24 bg-gray-50 rounded-full flex items-center justify-center text-5xl mx-auto mb-6 border shadow-inner">📬</div>
               <h2 className="text-2xl font-bold text-gray-900 font-outfit mb-2">No conversation selected</h2>
               <p className="text-sm text-gray-500 font-medium leading-relaxed">Select a message from the queue to start chatting with your customers.</p>
            </div>
          )}
        </div>
      </div>

      <style jsx global>{`
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}</style>
    </div>
  );
}
