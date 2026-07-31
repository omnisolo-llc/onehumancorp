"use client";

import { useEffect, useState, useRef } from "react";
import { AppShell } from "../components/AppShell";
import { useRouter } from "next/navigation";

export default function NativeChatPage() {
  const [messages, setMessages] = useState<any[]>([]);
  const [ws, setWs] = useState<WebSocket | null>(null);
  const router = useRouter();

  useEffect(() => {
    // Generate a pseudo-random tenant ID for the example
    const tenantId = "00000000-0000-0000-0000-000000000000";

    // Connect to WebSocket
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const wsUrl = `${protocol}//${window.location.host}/api/v1/native-chat/ws/${tenantId}`;

    const socket = new WebSocket(wsUrl);

    socket.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === "new_message") {
          setMessages(prev => [...prev, data]);
        }
      } catch (e) {
        console.error("Failed to parse websocket message", e);
      }
    };

    setWs(socket);

    return () => {
      socket.close();
    };
  }, []);

  return (
    <AppShell title="Native Chat" subtitle="Real-time native Rust omnichannel chat system">
      <div className="w-full max-w-[375px] mx-auto bg-white dark:bg-zinc-900 border border-gray-200 dark:border-gray-800 rounded-xl overflow-hidden shadow-sm flex flex-col h-[600px]">
        <div className="p-4 border-b border-gray-200 dark:border-gray-800 font-bold">Inbox</div>
        <div className="flex-1 p-4 overflow-y-auto flex flex-col gap-3">
          {messages.length === 0 ? (
            <div className="text-gray-400 text-sm text-center mt-10">No messages yet.</div>
          ) : (
            messages.map((msg, idx) => {
              const isDraft = msg.message.startsWith("AI Draft:");
              return (
                <div key={idx} className={`p-3 rounded-lg text-sm max-w-[85%] ${isDraft ? 'bg-purple-100 dark:bg-purple-900/30 text-purple-900 dark:text-purple-100 self-end rounded-tr-sm' : 'bg-blue-50 dark:bg-blue-900/20 text-gray-800 dark:text-gray-200 self-start rounded-tl-sm'}`}>
                  {msg.message}
                </div>
              );
            })
          )}
        </div>
        <div className="p-4 border-t border-gray-200 dark:border-gray-800 flex gap-2">
           <button className="bg-black dark:bg-white text-white dark:text-black font-semibold rounded-lg px-4 py-3 flex-1 text-sm" onClick={() => {
              const tenantId = "00000000-0000-0000-0000-000000000000";
              fetch(`/api/v1/native-chat/webhook/${tenantId}`, {
                 method: 'POST',
                 headers: { 'Content-Type': 'application/json' },
                 body: JSON.stringify({
                    inbox_id: "11111111-1111-1111-1111-111111111111",
                    sender_name: "Maya (Customer)",
                    content: "Hello, I want to book a service."
                 })
              });
           }}>Simulate Incoming Webhook DM</button>
        </div>
      </div>
    </AppShell>
  );
}
