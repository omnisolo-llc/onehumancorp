import React, { useState } from 'react';

export function OmnichannelChatWidget() {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<{sender: string, text: string}[]>([]);
  const [inputValue, setInputValue] = useState('');

  const handleSend = async () => {
    if (!inputValue.trim()) return;
    const userText = inputValue;
    setMessages((prev) => [...prev, { sender: 'You', text: userText }]);
    setInputValue('');

    try {
      const response = await fetch('/api/v1/agent/rpc', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          jsonrpc: '2.0',
          method: 'omnichannel_chat_webhook',
          params: {
            id: Date.now(),
            account: { id: 1, name: "Web User" },
            content_type: "text",
            content: userText,
            conversation: {
                id: 101,
                display_id: 101,
                account: { id: 1, name: "Web User" },
                channel: "web_widget",
                inbox_id: 1,
                status: "open"
            },
            created_at: new Date().toISOString(),
            inbox: { id: 1, name: "Web Inbox" },
            message_type: "incoming",
            private: false
          },
        }),
      });

      const json = await response.json();

      if (json.result?.Reply) {
        setMessages((prev) => [...prev, { sender: 'Copilot', text: json.result.Reply }]);
      } else if (json.result?.Handoff) {
        setMessages((prev) => [...prev, { sender: 'System', text: `Transferring to human: ${json.result.Handoff}` }]);
      } else {
         setMessages((prev) => [...prev, { sender: 'System', text: 'Message sent.' }]);
      }
    } catch (e) {
      console.error(e);
      setMessages((prev) => [...prev, { sender: 'System', text: 'Failed to send message.' }]);
    }
  };

  return (
    <div className="fixed bottom-4 right-4 z-50">
      {!isOpen && (
        <button
          data-testid="native-chat-trigger"
          onClick={() => setIsOpen(true)}
          className="bg-blue-600 text-white rounded-full p-4 shadow-lg hover:bg-blue-700 transition"
        >
          Chat
        </button>
      )}
      {isOpen && (
        <div className="bg-white w-80 h-96 rounded-lg shadow-xl flex flex-col overflow-hidden border border-gray-200">
          <div className="bg-blue-600 text-white p-3 font-semibold flex justify-between">
            <span>OHC Support</span>
            <button onClick={() => setIsOpen(false)} className="text-white hover:text-gray-200">&times;</button>
          </div>
          <div className="flex-1 p-3 overflow-y-auto bg-gray-50 flex flex-col gap-2">
             {messages.length === 0 && <div className="text-sm text-gray-400 italic">No messages yet.</div>}
             {messages.map((m, i) => (
                <div key={i} className={`text-sm p-2 rounded-lg max-w-[80%] ${m.sender === 'You' ? 'bg-blue-100 self-end text-blue-900' : 'bg-gray-200 self-start text-gray-800'}`}>
                    <span className="font-bold block text-xs mb-1">{m.sender}</span>
                    {m.text}
                </div>
             ))}
          </div>
          <div className="p-2 border-t border-gray-200 flex gap-2">
            <input
              className="flex-1 border rounded-md p-2 text-sm focus:outline-none focus:ring-1 focus:ring-blue-500"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              placeholder="Type a message..."
            />
            <button onClick={handleSend} className="bg-blue-600 text-white rounded-md px-3 text-sm hover:bg-blue-700">Send</button>
          </div>
        </div>
      )}
    </div>
  );
}
