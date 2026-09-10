'use client';

import React, { useState, useEffect } from 'react';
import Head from 'next/head';

interface Conversation {
  id: string;
  channel: string;
  customerName: string;
  snippet: string;
  status: string;
  nextAction?: string;
}

interface Message {
  id: string;
  content: string;
  direction: 'INBOUND' | 'OUTBOUND';
  status: string;
  draftReply?: string;
}

export default function OmniInbox() {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [selectedConversation, setSelectedConversation] = useState<Conversation | null>(null);
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(true);
  const [wsConnection, setWsConnection] = useState<WebSocket | null>(null);

  useEffect(() => {
    // Initial fetch of conversations (mocked for now, until backend is wired to nextjs routes)
    const mockConversations: Conversation[] = [
      { id: '1', channel: 'Instagram', customerName: 'Maya Baker', snippet: 'Do you do vegan cakes?', status: 'PENDING', nextAction: 'Draft ready' },
      { id: '2', channel: 'WhatsApp', customerName: 'Carlos Fix', snippet: 'Need a quote for sink repair', status: 'OPEN', nextAction: 'Needs quote' }
    ];
    setConversations(mockConversations);
    setLoading(false);

    // Mock WS connection for real-time updates
    const ws = new WebSocket('ws://localhost:8080/ws/omnichannel');
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      if (data.type === 'NEW_MESSAGE' && selectedConversation?.id === data.conversationId) {
        setMessages(prev => [...prev, data.message]);
      } else if (data.type === 'DRAFT_READY' && selectedConversation?.id === data.conversationId) {
          // Update draft logic
      }
    };
    setWsConnection(ws);

    return () => ws.close();
  }, [selectedConversation]);

  const handleSelectConversation = (conv: Conversation) => {
    setSelectedConversation(conv);
    // Mock fetch messages
    setMessages([
      { id: 'm1', content: conv.snippet, direction: 'INBOUND', status: 'DELIVERED', draftReply: conv.id === '1' ? 'Hi Maya! Yes, we absolutely do vegan cakes. Would you like a quote?' : undefined }
    ]);
  };

  const handleApproveDraft = (msgId: string) => {
    // Implement API call to approve and send
    setMessages(prev => prev.map(m => m.id === msgId ? { ...m, draftReply: undefined, status: 'SENT' } : m));
    setConversations(prev => prev.map(c => c.id === selectedConversation?.id ? { ...c, status: 'RESOLVED', nextAction: undefined } : c));
  };

  return (
    <div className="flex h-screen bg-gray-50 flex-col md:flex-row font-sans">
      <Head>
        <title>Omnichannel Inbox | OneHumanCorp</title>
      </Head>

      {/* Triage View (Sidebar on Desktop, Full on Mobile if no conversation selected) */}
      <div className={`w-full md:w-1/3 bg-white border-r border-gray-200 overflow-y-auto ${selectedConversation ? 'hidden md:block' : 'block'}`}>
        <div className="p-4 border-b border-gray-200 bg-gray-100 flex items-center justify-between">
            <h1 className="text-xl font-bold text-gray-800">Inbox</h1>
            <span className="bg-red-500 text-white text-xs px-2 py-1 rounded-full">{conversations.filter(c => c.status === 'PENDING').length} Urgent</span>
        </div>

        {loading ? (
          <div className="p-4 text-center text-gray-500">Loading conversations...</div>
        ) : conversations.length === 0 ? (
          <div className="p-4 text-center text-gray-500">No active conversations.</div>
        ) : (
          <ul className="divide-y divide-gray-200">
            {conversations.map(conv => (
              <li
                key={conv.id}
                className={`p-4 cursor-pointer hover:bg-gray-50 transition-colors ${selectedConversation?.id === conv.id ? 'bg-blue-50' : ''}`}
                onClick={() => handleSelectConversation(conv)}
                data-testid={`conv-${conv.id}`}
              >
                <div className="flex justify-between items-center mb-1">
                  <div className="flex items-center space-x-2">
                    <span className="text-sm font-semibold text-gray-700">{conv.channel}</span>
                    <span className="text-gray-900 font-bold">{conv.customerName}</span>
                  </div>
                  {conv.status === 'PENDING' && <span className="w-2 h-2 bg-blue-500 rounded-full"></span>}
                </div>
                <p className="text-sm text-gray-600 truncate">{conv.snippet}</p>
                {conv.nextAction && (
                    <div className="mt-2 text-xs font-medium text-blue-600 bg-blue-100 inline-block px-2 py-1 rounded">
                        AI: {conv.nextAction}
                    </div>
                )}
              </li>
            ))}
          </ul>
        )}
      </div>

      {/* Conversation View */}
      <div className={`w-full md:w-2/3 flex flex-col bg-white ${!selectedConversation ? 'hidden md:flex items-center justify-center' : 'flex'}`}>
        {!selectedConversation ? (
          <div className="text-gray-400 text-lg">Select a conversation to begin.</div>
        ) : (
          <>
            {/* Header */}
            <div className="p-4 border-b border-gray-200 flex items-center bg-white shadow-sm z-10">
              <button
                className="md:hidden mr-4 text-blue-500"
                onClick={() => setSelectedConversation(null)}
              >
                &larr; Back
              </button>
              <div>
                  <h2 className="text-lg font-bold text-gray-800">{selectedConversation.customerName}</h2>
                  <p className="text-xs text-gray-500">via {selectedConversation.channel}</p>
              </div>
            </div>

            {/* Messages Area */}
            <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-gray-50">
              {messages.map(msg => (
                <div key={msg.id} className={`flex flex-col ${msg.direction === 'OUTBOUND' ? 'items-end' : 'items-start'}`}>
                  <div className={`max-w-[80%] rounded-2xl px-4 py-2 ${msg.direction === 'OUTBOUND' ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-900'}`}>
                    {msg.content}
                  </div>

                  {msg.draftReply && (
                      <div className="mt-2 max-w-[80%] border border-blue-200 bg-blue-50 rounded-2xl p-3 shadow-sm relative">
                          <div className="absolute -top-3 left-4 bg-white px-2 text-xs font-bold text-blue-600 rounded shadow-sm border border-blue-100">AI Draft</div>
                          <p className="text-gray-800 text-sm mt-1">{msg.draftReply}</p>
                          <div className="mt-3 flex space-x-2">
                              <button
                                onClick={() => handleApproveDraft(msg.id)}
                                className="bg-blue-600 text-white text-sm px-4 py-2 rounded-lg font-medium hover:bg-blue-700 transition-colors w-full"
                                data-testid={`approve-draft-${msg.id}`}
                              >
                                  Approve & Send
                              </button>
                          </div>
                      </div>
                  )}
                </div>
              ))}
            </div>

            {/* Input Area */}
            <div className="p-4 border-t border-gray-200 bg-white flex space-x-2 items-center">
              <input
                type="text"
                placeholder="Type a message..."
                className="flex-1 border border-gray-300 rounded-full px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 text-black"
                disabled={!!messages.find(m => m.draftReply)}
              />
              <button
                className={`bg-blue-500 text-white p-2 rounded-full w-10 h-10 flex items-center justify-center ${messages.find(m => m.draftReply) ? 'opacity-50 cursor-not-allowed' : 'hover:bg-blue-600'}`}
                disabled={!!messages.find(m => m.draftReply)}
              >
                &rarr;
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
