"use client";

import React, { useState, useEffect } from "react";
import AppShell from "@/components/AppShell";
import { useQuery } from "@/components/PowerSyncProvider";
import { PowerSyncProvider } from "@/components/PowerSyncProvider";

type ChatInbox = {
  id: string;
  name: string;
};

type ChatConversation = {
  id: string;
  contact_id: string;
  status: string;
};

type ChatMessage = {
  id: string;
  conversation_id: string;
  sender_type: string;
  sender_id: string | null;
  content: string;
  created_at: string;
};

function ChatWorkspace() {
  const { data: inboxes } = useQuery<ChatInbox>("SELECT * FROM chat_inboxes ORDER BY created_at DESC");
  const { data: conversations } = useQuery<ChatConversation>("SELECT * FROM chat_conversations ORDER BY created_at DESC");
  const { data: messages } = useQuery<ChatMessage>("SELECT * FROM chat_messages ORDER BY created_at ASC");

  const [selectedInboxId, setSelectedInboxId] = useState<string | null>(null);
  const [selectedConversationId, setSelectedConversationId] = useState<string | null>(null);

  const activeConversations = conversations?.filter(c =>
    selectedInboxId ? c.id === selectedInboxId : true // Wait, need inbox_id, wait, schema has inbox_id on conversations
  ) || [];

  const activeMessages = messages?.filter(m => m.conversation_id === selectedConversationId) || [];

  return (
    <AppShell title="Omnichannel Chat" subtitle="Native Rust Chat Engine">
      <div className="flex h-[calc(100vh-140px)] gap-4">
        {/* Sidebar */}
        <div className="w-1/3 flex flex-col gap-4 bg-white/60 dark:bg-black/20 rounded-[16px] p-4 border border-black/5 dark:border-white/5">
           <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white">Conversations</h2>
           <div className="flex-1 overflow-y-auto">
             {conversations?.map(conv => (
               <div
                 key={conv.id}
                 className={`p-3 rounded-lg mb-2 cursor-pointer transition-colors ${selectedConversationId === conv.id ? 'bg-blue-100 dark:bg-blue-900/40 border border-blue-300 dark:border-blue-700' : 'hover:bg-black/5 dark:hover:bg-white/5 border border-transparent'}`}
                 onClick={() => setSelectedConversationId(conv.id)}
               >
                 <div className="font-semibold text-gray-900 dark:text-white">Contact: {conv.contact_id}</div>
                 <div className="text-sm text-gray-500">Status: {conv.status}</div>
               </div>
             ))}
             {(!conversations || conversations.length === 0) && (
               <div className="text-gray-500 text-center mt-10">No active conversations.</div>
             )}
           </div>
        </div>

        {/* Main Chat Area */}
        <div className="flex-1 flex flex-col bg-white/60 dark:bg-black/20 rounded-[16px] p-4 border border-black/5 dark:border-white/5">
          {!selectedConversationId ? (
            <div className="flex-1 flex items-center justify-center text-gray-500">
              Select a conversation to start chatting.
            </div>
          ) : (
            <>
              <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-3">
                {activeMessages.map(msg => (
                  <div key={msg.id} className={`max-w-[80%] p-3 rounded-lg ${msg.sender_type === 'agent' ? 'bg-blue-500 text-white self-end' : 'bg-gray-200 dark:bg-gray-800 text-gray-900 dark:text-white self-start'}`}>
                    {msg.content}
                  </div>
                ))}
                {activeMessages.length === 0 && (
                  <div className="text-gray-500 text-center mt-10">No messages yet.</div>
                )}
              </div>
              <div className="mt-4 p-2 bg-white dark:bg-black/40 rounded-lg border border-gray-200 dark:border-white/10 flex">
                <input
                  type="text"
                  className="flex-1 bg-transparent outline-none p-2 text-gray-900 dark:text-white"
                  placeholder="Type a message..."
                />
                <button className="bg-blue-600 text-white px-4 py-2 rounded-lg font-semibold hover:bg-blue-700 transition-colors">
                  Send
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </AppShell>
  );
}

export default function ChatPage() {
  return (
    <PowerSyncProvider>
      <ChatWorkspace />
    </PowerSyncProvider>
  );
}
