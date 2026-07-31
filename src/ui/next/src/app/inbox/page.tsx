'use client';

import React, { useState } from 'react';

export default function InboxPage() {
  const [selectedConversation, setSelectedConversation] = useState<string | null>(null);
  const [draftReply, setDraftReply] = useState<string>('Yes, I can make a vegan cake for Saturday. It will be $50. [Payment Link]');
  const [sentMessage, setSentMessage] = useState<string | null>(null);

  const handleConversationClick = (id: string) => {
    setSelectedConversation(id);
    setSentMessage(null); // Reset when changing conversation
  };

  const handleSend = () => {
    setSentMessage(draftReply);
    setDraftReply('');
  };

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Inbox List */}
      <div className="w-1/3 border-r border-gray-200 bg-white flex flex-col" data-testid="inbox-list">
        <div className="p-4 border-b border-gray-200">
          <h1 className="text-xl font-semibold">Work Feed / Inbox</h1>
        </div>
        <div className="flex-1 overflow-y-auto">
          {/* Mock Conversation Item - Instagram */}
          <div
            className={`p-4 border-b border-gray-200 cursor-pointer hover:bg-gray-50 ${selectedConversation === 'ig-1' ? 'bg-blue-50' : ''}`}
            onClick={() => handleConversationClick('ig-1')}
            data-testid="conversation-item"
          >
            <div className="flex items-center justify-between">
              <span className="font-medium text-gray-900">Maya's Customer</span>
              <span className="text-pink-500" data-testid="icon-instagram">IG</span>
            </div>
            <p className="text-sm text-gray-500 mt-1 truncate">Can you make a vegan cake for Saturday?</p>
          </div>
          {/* More mock items could go here */}
        </div>
      </div>

      {/* Conversation View */}
      <div className="flex-1 flex flex-col bg-white">
        {selectedConversation ? (
          <div className="flex flex-col h-full" data-testid="conversation-view">
            {/* Header */}
            <div className="p-4 border-b border-gray-200 flex items-center shadow-sm">
              <span className="text-pink-500 mr-3" data-testid="icon-instagram">IG</span>
              <h2 className="text-lg font-medium">Maya's Customer</h2>
            </div>

            {/* Messages Area */}
            <div className="flex-1 p-4 overflow-y-auto bg-gray-50 flex flex-col gap-4">
              {/* Customer Message */}
              <div className="self-start max-w-[80%] bg-white p-3 rounded-lg shadow-sm border border-gray-100" data-testid="customer-message">
                <p className="text-gray-800">Hi Maya! Can you make a vegan cake for Saturday?</p>
                <span className="text-xs text-gray-400 mt-1 block">10:00 AM</span>
              </div>

              {/* Sent Message (Appears after sending) */}
              {sentMessage && (
                <div className="self-end max-w-[80%] bg-blue-500 text-white p-3 rounded-lg shadow-sm" data-testid="sent-message">
                  <p>{sentMessage}</p>
                  <span className="text-xs text-blue-100 mt-1 block">Just now</span>
                </div>
              )}
            </div>

            {/* AI Draft & Input Area */}
            <div className="p-4 border-t border-gray-200 bg-white">
              <div className="mb-2 text-xs font-semibold text-purple-600 flex items-center gap-1">
                <span>✨ AI Drafted Response based on inventory</span>
              </div>
              <div className="flex gap-2">
                <input
                  type="text"
                  className="flex-1 border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  value={draftReply}
                  onChange={(e) => setDraftReply(e.target.value)}
                  data-testid="ai-draft-input"
                />
                <button
                  className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md transition-colors"
                  onClick={handleSend}
                  data-testid="send-reply-button"
                  disabled={!draftReply.trim()}
                >
                  Send
                </button>
              </div>
            </div>
          </div>
        ) : (
          <div className="flex-1 flex items-center justify-center text-gray-500">
            Select a conversation to view
          </div>
        )}
      </div>
    </div>
  );
}
