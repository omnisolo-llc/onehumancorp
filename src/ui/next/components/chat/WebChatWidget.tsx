import React, { useState } from 'react';

export const WebChatWidget: React.FC = () => {
    const [messages, setMessages] = useState<string[]>([]);
    const [input, setInput] = useState('');

    const handleSend = () => {
        if (!input.trim()) return;
        setMessages([...messages, input]);
        setInput('');
    };

    return (
        <div className="fixed bottom-4 right-4 w-80 bg-white/80 backdrop-blur-md border border-gray-200 rounded-2xl shadow-xl flex flex-col overflow-hidden" style={{ minHeight: '400px' }}>
            <div className="bg-blue-600 p-4 text-white font-semibold">
                Chat Support
            </div>
            <div className="flex-1 p-4 overflow-y-auto space-y-2">
                {messages.map((msg, i) => (
                    <div key={i} className="bg-blue-100 p-2 rounded-lg self-end w-max max-w-[80%]">
                        {msg}
                    </div>
                ))}
            </div>
            <div className="p-4 bg-gray-50 flex items-center gap-2">
                <input
                    type="text"
                    value={input}
                    onChange={(e) => setInput(e.target.value)}
                    className="flex-1 px-3 py-2 rounded-full border border-gray-300 focus:outline-none focus:border-blue-500"
                    placeholder="Type a message..."
                    data-testid="chat-input"
                />
                <button
                    onClick={handleSend}
                    className="w-11 h-11 bg-blue-600 text-white rounded-full flex items-center justify-center font-bold"
                    data-testid="chat-send"
                >
                    &gt;
                </button>
            </div>
        </div>
    );
};
