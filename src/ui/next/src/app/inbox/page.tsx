'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

interface Conversation {
  id: string;
  status: string;
  contactName: string;
  snippet: string;
  timestamp: string;
  channel: string;
}

export default function InboxPage() {
  const [conversations, setConversations] = useState<Conversation[]>([]);

  useEffect(() => {
    // In a real app, this would fetch from the Rust backend via REST/gRPC
    // For now, we mock the UI data as requested to build the foundation
    setConversations([
      {
        id: '1',
        status: 'open',
        contactName: 'Maya Baker',
        snippet: 'Requested vegan cake quote',
        timestamp: '10m ago',
        channel: 'IG',
      },
      {
        id: '2',
        status: 'open',
        contactName: 'Carlos Repair',
        snippet: 'Do you fix broken pipes?',
        timestamp: '1h ago',
        channel: 'Web',
      },
    ]);
  }, []);

  return (
    <div className="flex flex-col h-screen w-full max-w-[375px] mx-auto bg-gray-50 text-black">
      <header className="px-4 py-3 bg-white/80 backdrop-blur-md border-b sticky top-0 z-10">
        <h1 className="text-xl font-bold">Inbox</h1>
      </header>

      <main className="flex-1 overflow-y-auto p-4 space-y-3">
        {conversations.map((conv) => (
          <Link href={`/inbox/${conv.id}`} key={conv.id}>
            <div className="p-3 bg-white/90 backdrop-blur-sm border rounded-xl shadow-sm hover:shadow-md transition flex flex-col cursor-pointer mb-3">
              <div className="flex justify-between items-center mb-1">
                <span className="font-semibold">{conv.contactName}</span>
                <span className="text-xs text-gray-500">{conv.timestamp}</span>
              </div>
              <div className="text-sm text-gray-700 flex justify-between">
                <span className="truncate pr-2">{conv.snippet}</span>
                <span className="text-xs px-2 py-0.5 bg-gray-100 rounded-full">{conv.channel}</span>
              </div>
            </div>
          </Link>
        ))}
      </main>
    </div>
  );
}
