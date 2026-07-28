"use client";

import { useEffect, useState } from "react";
import { ChatConversationView } from "./ChatConversationView";

type Conversation = {
  id: string;
  name: string;
};

export function ChatSystemView() {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [selectedConv, setSelectedConv] = useState<Conversation | null>(null);

  useEffect(() => {
    async function loadConversations() {
      try {
        const res = await fetch("/api/v1/chat_system/conversations");
        if (res.ok) {
          const data = await res.json();
          setConversations(data.conversations || []);
        }
      } catch (e) {
        console.error("Failed to load conversations", e);
      }
    }
    loadConversations();
  }, []);

  return (
    <div className="flex h-full w-full">
      {/* Inbox Feed (Mobile-First 375px handled by responsive classes) */}
      <div className={`w-full md:w-1/3 border-r h-full overflow-y-auto ${selectedConv ? 'hidden md:block' : 'block'}`}>
        <div className="p-4 border-b font-bold sticky top-0 bg-white z-10">Conversations</div>
        {conversations.length === 0 && (
            <div className="p-4 text-gray-500 text-sm">No conversations found.</div>
        )}
        {conversations.map(c => (
          <div
            key={c.id}
            className="p-4 border-b cursor-pointer hover:bg-gray-100 flex items-center"
            onClick={() => setSelectedConv(c)}
          >
            <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center mr-3 font-bold text-blue-600">
                {c.name.charAt(0)}
            </div>
            <div>
                <div className="font-medium">{c.name}</div>
                <div className="text-xs text-gray-500 line-clamp-1">Tap to view conversation...</div>
            </div>
          </div>
        ))}
      </div>

      {/* Conversation View */}
      <div className={`w-full md:w-2/3 h-full ${selectedConv ? 'block' : 'hidden md:block'}`}>
        {selectedConv ? (
          <ChatConversationView
            conversationId={selectedConv.id}
            onBack={() => setSelectedConv(null)}
          />
        ) : (
          <div className="h-full w-full flex items-center justify-center text-gray-500 bg-gray-50">
            <div className="text-center">
                <div className="text-4xl mb-2">💬</div>
                <div>Select a conversation to view.</div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
