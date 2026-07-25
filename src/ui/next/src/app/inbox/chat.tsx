"use client";

import { useEffect, useState, useMemo } from "react";
import { AppShell } from "../../components/AppShell";

export type ChatInbox = {
  id: string;
  name: string;
  channel_type: string;
};

export type ChatConversation = {
  id: string;
  inbox_id: string;
  contact_id: string;
  status: string;
};

export type ChatMessage = {
  id: string;
  conversation_id: string;
  content: string;
  message_type: string;
  sender_type: string;
  created_at: string;
};

export function ChatwootInbox() {
  const [inboxes, setInboxes] = useState<ChatInbox[]>([]);
  const [conversations, setConversations] = useState<ChatConversation[]>([]);
  const [selectedConversation, setSelectedConversation] = useState<ChatConversation | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [replyText, setReplyText] = useState("");
  const [loading, setLoading] = useState(true);

  // Example load sequence: inboxes -> first inbox -> conversations -> first conversation -> messages
  useEffect(() => {
    async function loadInboxes() {
      try {
        const inboxRes = await fetch('/api/v1/chat/inboxes');
        if (inboxRes.ok) {
           const inboxesData = await inboxRes.json();
           setInboxes(inboxesData);
           if (inboxesData.length > 0) {
              const inboxId = inboxesData[0].id;
              const convRes = await fetch(`/api/v1/chat/inboxes/${inboxId}/conversations`);
              if (convRes.ok) {
                 const convData = await convRes.json();
                 setConversations(convData);
                 if (convData.length > 0) {
                    setSelectedConversation(convData[0]);
                    const msgRes = await fetch(`/api/v1/chat/conversations/${convData[0].id}/messages`);
                    if (msgRes.ok) {
                       const msgData = await msgRes.json();
                       setMessages(msgData);
                    }
                 }
              }
           }
        }
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    }
    loadInboxes();
  }, []);

  const selectConversation = async (conv: ChatConversation) => {
     setSelectedConversation(conv);
     try {
       const res = await fetch(`/api/v1/chat/conversations/${conv.id}/messages`);
       if (res.ok) {
          setMessages(await res.json());
       }
     } catch (e) {
       console.error(e);
     }
  };

  const sendMessage = async () => {
    if (!selectedConversation || !replyText.trim()) return;
    try {
      const res = await fetch(`/api/v1/chat/conversations/${selectedConversation.id}/messages`, {
         method: 'POST',
         headers: { 'Content-Type': 'application/json' },
         body: JSON.stringify({
            sender_type: 'agent',
            content: replyText,
            message_type: 'outgoing'
         })
      });
      if (res.ok) {
        const newMsg = await res.json();
        setMessages([...messages, newMsg]);
        setReplyText("");
      }
    } catch (e) {
       console.error(e);
    }
  };

  return (
    <AppShell title="Inbox" subtitle="Unified Omnichannel Messaging">
      <div className="flex h-full w-full flex-col md:flex-row overflow-hidden bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] rounded-2xl border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]">
        {/* Sidebar */}
        <div className="flex flex-col w-full md:w-[320px] border-r border-gray-200 dark:border-gray-800">
           <div className="p-4 border-b border-gray-200 dark:border-gray-800 font-semibold">
              Conversations
           </div>
           <div className="flex-1 overflow-y-auto p-2">
              {loading && <div className="text-sm text-gray-500 p-2">Loading...</div>}
              {!loading && conversations.length === 0 && (
                <div className="text-sm text-gray-500 p-2">No active conversations.</div>
              )}
              {conversations.map(conv => (
                  <div key={conv.id} onClick={() => selectConversation(conv)} className={`p-3 mb-2 rounded-xl cursor-pointer ${selectedConversation?.id === conv.id ? 'bg-blue-50 dark:bg-blue-900/20' : 'hover:bg-gray-50 dark:hover:bg-gray-800/50'}`}>
                     <div className="font-medium text-sm">Customer Thread</div>
                     <div className="text-xs text-gray-500 truncate">Status: {conv.status}</div>
                  </div>
              ))}
           </div>
        </div>

        {/* Main Conversation Area */}
        <div className="flex-1 flex flex-col min-w-0 bg-white/50 dark:bg-black/20">
            <div className="p-4 border-b border-gray-200 dark:border-gray-800 font-medium">
               Active Thread
            </div>

            <div className="flex-1 overflow-y-auto p-4 space-y-4">
                {!selectedConversation && !loading && (
                   <div className="flex items-center justify-center h-full text-sm text-gray-500">
                      Select a conversation to view messages.
                   </div>
                )}
                {messages.map((msg, i) => {
                   const isAgent = msg.sender_type === 'agent' || msg.message_type === 'outgoing';
                   return (
                    <div key={msg.id || i} className={`flex ${isAgent ? 'justify-end' : 'justify-start'}`}>
                        <div className={`${isAgent ? 'bg-[#0066FF] text-white' : 'bg-gray-100 dark:bg-gray-800'} rounded-2xl px-4 py-2 max-w-[85%] text-sm`}>
                            {msg.content}
                        </div>
                    </div>
                   );
                })}
            </div>

            <div className="p-4 border-t border-gray-200 dark:border-gray-800">
                <div className="flex gap-2">
                    <input
                      value={replyText}
                      onChange={(e) => setReplyText(e.target.value)}
                      onKeyDown={(e) => e.key === 'Enter' && sendMessage()}
                      className="flex-1 rounded-lg border border-gray-300 dark:border-gray-700 px-3 py-2 bg-transparent text-sm focus:outline-none focus:ring-1 focus:ring-[#0066FF]"
                      placeholder="Type your message..."
                      disabled={!selectedConversation}
                    />
                    <button
                      onClick={sendMessage}
                      disabled={!selectedConversation || !replyText.trim()}
                      className="bg-[#0066FF] text-white px-4 py-2 rounded-lg text-sm font-medium disabled:opacity-50"
                    >
                        Send
                    </button>
                </div>
            </div>
        </div>
      </div>
    </AppShell>
  );
}
