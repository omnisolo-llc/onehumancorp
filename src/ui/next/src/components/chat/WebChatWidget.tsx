import React, { useState } from 'react';

export const WebChatWidget = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<{id: string, text: string, sender: 'user' | 'agent'}[]>([]);
  const [inputText, setInputText] = useState('');
  const [showCanned, setShowCanned] = useState(false);

  const cannedResponses = [
    { code: '/hello', text: 'Hello! How can I help you today?' },
    { code: '/bye', text: 'Thank you for reaching out. Have a great day!' },
    { code: '/pricing', text: 'You can find our pricing details at /pricing.' }
  ];

  const handleSend = () => {
    if (!inputText.trim()) return;

    const newMsg = { id: Date.now().toString(), text: inputText, sender: 'user' as const };
    setMessages(prev => [...prev, newMsg]);
    setInputText('');

    // Simulate agent response
    setTimeout(() => {
      setMessages(prev => [...prev, { id: (Date.now() + 1).toString(), text: 'Thanks for reaching out! We will be with you shortly.', sender: 'agent' as const }]);
    }, 1000);
  };

  return (
    <div className="fixed bottom-4 right-4 z-50 font-inter">
      {!isOpen && (
        <button
          onClick={() => setIsOpen(true)}
          className="w-14 h-14 bg-[#0066FF] hover:bg-blue-700 text-white rounded-full shadow-xl flex items-center justify-center transition-transform hover:scale-105"
          aria-label="Open Chat"
          data-testid="open-chat-widget"
        >
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
          </svg>
        </button>
      )}

      {isOpen && (
        <div className="w-[375px] h-[600px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 shadow-2xl rounded-2xl flex flex-col overflow-hidden relative" data-testid="chat-widget-window">
          {/* Header */}
          <div className="p-4 bg-[#0066FF] text-white flex justify-between items-center rounded-t-2xl">
            <div>
              <h2 className="font-bold font-outfit">OHC Support</h2>
              <p className="text-xs text-blue-100">We typically reply in a few minutes.</p>
            </div>
            <button
              onClick={() => setIsOpen(false)}
              className="text-white hover:text-blue-200"
              aria-label="Close Chat"
              data-testid="close-chat-widget"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          {/* Messages Area */}
          <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-gray-50/50" data-testid="chat-messages">
            <div className="text-center text-xs text-gray-500 my-4">Today</div>
            {messages.map(msg => (
              <div key={msg.id} className={`flex ${msg.sender === 'user' ? 'justify-end' : 'justify-start'}`}>
                <div className={`max-w-[80%] p-3 text-sm ${msg.sender === 'user' ? 'bg-[#0066FF] text-white rounded-2xl rounded-tr-none' : 'bg-white border border-gray-200 text-gray-800 rounded-2xl rounded-tl-none shadow-sm'}`}>
                  {msg.text}
                </div>
              </div>
            ))}
          </div>

          {/* Input Area */}
          <div className="p-3 bg-white border-t border-gray-100 relative">
             {showCanned && (
               <div className="absolute bottom-full left-0 w-full bg-white border border-gray-200 shadow-lg rounded-t-xl overflow-hidden mb-1" data-testid="canned-responses-menu">
                 {cannedResponses.map(resp => (
                   <button
                     key={resp.code}
                     className="w-full text-left px-4 py-2 text-sm hover:bg-gray-50 flex items-center gap-2"
                     onClick={() => {
                       setInputText(resp.text);
                       setShowCanned(false);
                     }}
                   >
                     <span className="font-bold text-gray-500 w-16">{resp.code}</span>
                     <span className="text-gray-800 truncate">{resp.text}</span>
                   </button>
                 ))}
               </div>
             )}
             <div className="flex flex-wrap gap-2 mb-2">
                {/* Example UI Labels representing ChatLabels */}
                <span className="px-2 py-0.5 text-[10px] font-bold rounded-full bg-purple-100 text-purple-700 uppercase tracking-wide">VIP</span>
                <span className="px-2 py-0.5 text-[10px] font-bold rounded-full bg-orange-100 text-orange-700 uppercase tracking-wide">Urgent</span>
             </div>
             <div className="flex items-center bg-gray-50 rounded-xl border border-gray-200 px-3 py-2">
                <button
                  onClick={() => setShowCanned(!showCanned)}
                  className="mr-2 text-gray-400 hover:text-gray-600 transition-colors font-bold"
                  aria-label="Canned Responses"
                  data-testid="canned-responses-btn"
                >
                  /
                </button>
                <input
                  type="text"
                  className="flex-1 bg-transparent border-none outline-none text-sm placeholder-gray-400"
                  placeholder="Type your message... (type / for shortcuts)"
                  value={inputText}
                  onChange={(e) => {
                    setInputText(e.target.value);
                    if (e.target.value === '/') setShowCanned(true);
                    else setShowCanned(false);
                  }}
                  onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                  data-testid="chat-widget-input"
                />
                <button
                  className="ml-2 text-[#0066FF] hover:text-blue-700 font-medium text-sm disabled:opacity-50 transition-colors"
                  onClick={handleSend}
                  disabled={!inputText.trim()}
                  data-testid="chat-widget-send"
                >
                  <svg className="w-5 h-5 transform rotate-90" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" />
                  </svg>
                </button>
             </div>
          </div>
        </div>
      )}
    </div>
  );
};
