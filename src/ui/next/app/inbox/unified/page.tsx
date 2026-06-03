"use client";

import { useState, useEffect } from "react";

export default function UnifiedInbox() {
  const [messages, setMessages] = useState([]);

  useEffect(() => {
    const fetchMessages = async () => {
      try {
        // Need to add API routes to support the backend
        const res = await fetch("/api/support/messages");
        if (res.ok) {
          const data = await res.json();
          setMessages(data);
        } else {
          console.error("Failed to fetch messages");
        }
      } catch (err) {
        console.error(err);
      }
    };
    fetchMessages();
  }, []);

  return (
    <div className="flex flex-col p-4 w-full max-w-[375px] mx-auto bg-white/50 backdrop-blur-xl min-h-screen">
      <h1 className="text-2xl font-bold mb-4">Unified Inbox</h1>

      <div className="flex flex-col gap-4">
        {messages.map((msg: any) => (
          <div key={msg.id} className="p-4 rounded-xl shadow-sm bg-white/80 border border-gray-100 flex flex-col gap-2">
            <div className="flex justify-between items-center text-sm text-gray-500">
              <span className="uppercase font-semibold text-xs tracking-wider">{msg.channel}</span>
              <span>{msg.sender_id}</span>
            </div>

            <p className="text-gray-900 mt-2">"{msg.content}"</p>

            {msg.status === "needs-review" && (
              <div className="mt-4 p-3 rounded-lg bg-blue-50/50 border border-blue-100">
                <span className="text-xs text-blue-500 font-semibold mb-1 block">AI Drafted Reply</span>
                <p className="text-sm text-gray-700">{msg.draft_reply}</p>
                <div className="flex gap-2 mt-3">
                  <button className="flex-1 bg-blue-500 text-white rounded-lg py-2 text-sm font-medium hover:bg-blue-600 transition-colors">
                    Approve
                  </button>
                  <button className="flex-1 bg-white text-gray-700 border border-gray-200 rounded-lg py-2 text-sm font-medium hover:bg-gray-50 transition-colors">
                    Edit
                  </button>
                </div>
              </div>
            )}

            {msg.status === "escalated" && (
              <div className="mt-4 p-3 rounded-lg bg-red-50/50 border border-red-100">
                <span className="text-xs text-red-500 font-semibold mb-1 block">Requires Manual Reply</span>
              </div>
            )}
            {msg.status === "auto-replied" && (
              <div className="mt-4 p-3 rounded-lg bg-green-50/50 border border-green-100">
                <span className="text-xs text-green-500 font-semibold mb-1 block">Auto-Replied</span>
                <p className="text-sm text-gray-700">{msg.draft_reply}</p>
              </div>
            )}
          </div>
        ))}

        {messages.length === 0 && (
          <div className="text-center text-gray-500 mt-10">
            No messages in inbox
          </div>
        )}
      </div>
    </div>
  );
}
