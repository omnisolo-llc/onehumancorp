"use client"

import React, { useState } from 'react';

// Mock data
const mockConversations = [
  { id: '1', customerName: 'Alice', channel: 'instagram', status: 'needs_attention', lastMessage: 'Can you do a vegan cake?', aiHandled: false },
  { id: '2', customerName: 'Bob', channel: 'email', status: 'resolved', lastMessage: 'Yes we do! Delivery to downtown is $5.', aiHandled: true },
];

export default function UnifiedInboxPage() {
  const [conversations, setConversations] = useState(mockConversations);
  const [activeTab, setActiveTab] = useState('needs_attention');

  const filteredConversations = conversations.filter(c =>
    activeTab === 'needs_attention' ? c.status === 'needs_attention' : true
  );

  return (
    <div className="p-4 max-w-md mx-auto">
      <h1 className="text-2xl font-bold mb-4">Unified Inbox</h1>

      <div className="flex space-x-4 mb-4 border-b">
        <button
          className={`py-2 ${activeTab === 'needs_attention' ? 'border-b-2 border-blue-500 font-bold' : ''}`}
          onClick={() => setActiveTab('needs_attention')}
        >
          Needs Attention
        </button>
        <button
          className={`py-2 ${activeTab === 'all' ? 'border-b-2 border-blue-500 font-bold' : ''}`}
          onClick={() => setActiveTab('all')}
        >
          All Messages
        </button>
      </div>

      <div className="space-y-4">
        {filteredConversations.map(conv => (
          <div key={conv.id} className="p-4 border rounded-lg shadow-sm bg-white">
            <div className="flex justify-between items-center mb-2">
              <span className="font-bold">{conv.customerName}</span>
              <span className="text-xs text-gray-500 uppercase">{conv.channel}</span>
            </div>
            <p className="text-sm text-gray-700">{conv.lastMessage}</p>
            {conv.aiHandled && (
              <span className="inline-block mt-2 px-2 py-1 bg-green-100 text-green-800 text-xs rounded-full">
                AI Handled
              </span>
            )}
            {!conv.aiHandled && (
              <div className="mt-2">
                <p className="text-xs text-gray-500 mb-1">AI Draft Suggestion:</p>
                <div className="p-2 bg-gray-50 rounded border text-sm text-gray-700 mb-2">
                  We can do a vegan cake for you! Let me know what flavor.
                </div>
                <div className="flex space-x-2">
                    <button className="flex-1 bg-blue-500 text-white py-1 rounded text-sm">Send</button>
                    <button className="flex-1 bg-gray-200 text-gray-800 py-1 rounded text-sm">Edit</button>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
