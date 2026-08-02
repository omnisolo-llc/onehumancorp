"use client";

import React, { useState, useEffect } from 'react';

type Message = {
    id: string;
    conversation_id: string;
    sender_type: string;
    content: string;
};

type Conversation = {
    id: string;
    inbox_id: string;
    contact_id: string;
    status: string;
};

type Inbox = {
    id: string;
    name: string;
};

export default function ChatPage() {
    const [inboxes, setInboxes] = useState<Inbox[]>([]);
    const [conversations, setConversations] = useState<Conversation[]>([]);
    const [messages, setMessages] = useState<Message[]>([]);
    const [currentConversation, setCurrentConversation] = useState<string | null>(null);
    const [newMessage, setNewMessage] = useState('');

    useEffect(() => {
        fetchInboxes();
        fetchConversations();
    }, []);

    useEffect(() => {
        if (currentConversation) {
            fetchMessages(currentConversation);
        }
    }, [currentConversation]);

    const fetchInboxes = async () => {
        const res = await fetch('/api/v1/chat/inboxes');
        if (res.ok) setInboxes(await res.json());
    };

    const fetchConversations = async () => {
        const res = await fetch('/api/v1/chat/conversations');
        if (res.ok) setConversations(await res.json());
    };

    const fetchMessages = async (convId: string) => {
        const res = await fetch(`/api/v1/chat/conversations/${convId}/messages`);
        if (res.ok) setMessages(await res.json());
    };

    const sendMessage = async () => {
        if (!currentConversation || !newMessage.trim()) return;
        const res = await fetch(`/api/v1/chat/conversations/${currentConversation}/messages`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ sender_type: 'agent', content: newMessage }),
        });
        if (res.ok) {
            setNewMessage('');
            fetchMessages(currentConversation);
        }
    };

    return (
        <div className="flex h-screen bg-gray-50">
            {/* Triage Feed */}
            <div className="w-1/3 border-r bg-white p-4">
                <h2 className="text-xl font-bold mb-4">Triage Feed</h2>
                {conversations.map(conv => (
                    <div
                        key={conv.id}
                        className={`p-4 border-b cursor-pointer rounded-lg mb-2 ${currentConversation === conv.id ? 'bg-blue-50' : 'bg-white shadow-sm'}`}
                        onClick={() => setCurrentConversation(conv.id)}
                    >
                        <p className="font-semibold text-gray-800">Conversation {conv.id.substring(0,8)}</p>
                        <p className="text-sm text-gray-500">Status: {conv.status}</p>
                    </div>
                ))}
            </div>

            {/* Conversation View */}
            <div className="flex-1 flex flex-col bg-white">
                {currentConversation ? (
                    <>
                        <div className="flex-1 overflow-y-auto p-4 space-y-4">
                            {messages.map(msg => (
                                <div key={msg.id} className={`flex ${msg.sender_type === 'agent' ? 'justify-end' : 'justify-start'}`}>
                                    <div className={`p-3 rounded-xl max-w-xs ${msg.sender_type === 'agent' ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-800'}`}>
                                        {msg.content}
                                    </div>
                                </div>
                            ))}
                        </div>
                        <div className="p-4 border-t">
                            <div className="flex gap-2">
                                <input
                                    type="text"
                                    className="flex-1 border rounded-lg px-4 py-2"
                                    placeholder="Type a message..."
                                    value={newMessage}
                                    onChange={e => setNewMessage(e.target.value)}
                                    onKeyDown={e => e.key === 'Enter' && sendMessage()}
                                />
                                <button
                                    onClick={sendMessage}
                                    className="bg-blue-600 text-white px-6 py-2 rounded-lg font-medium"
                                >
                                    Send
                                </button>
                            </div>
                        </div>
                    </>
                ) : (
                    <div className="flex-1 flex items-center justify-center text-gray-500">
                        Select a conversation to start chatting
                    </div>
                )}
            </div>
        </div>
    );
}
