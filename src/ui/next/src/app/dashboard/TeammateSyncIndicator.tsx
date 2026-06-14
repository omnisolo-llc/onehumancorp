"use client";

import { useEffect, useState, useMemo } from "react";

export function TeammateSyncIndicator() {
  const [messages, setMessages] = useState<string[]>([]);

  useEffect(() => {
    let ws: WebSocket;
    let isSubscribed = true;

    const connect = () => {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${window.location.host}/v1/orchestration/tasks/stream?channel=mesh:tasks`;
      ws = new WebSocket(wsUrl);

      ws.onmessage = (event) => {
        if (!isSubscribed) return;
        try {
          const data = JSON.parse(event.data);

          if (data.action && data.agent_id) {
            let readableMessage = `Agent ${data.agent_id} completed an action: ${data.action}`;

            if (data.action === "task_completed" && data.agent_id === "Marketing Agent") {
                readableMessage = "The Promoter is briefing The Manager";
            }
            if (data.action === "task_completed" && data.agent_id === "Operations Agent") {
                readableMessage = "Operations has updated the system";
            }
            if (data.action.startsWith("state_transition")) {
                readableMessage = `Teammates synchronized: ${data.action}`;
            }

            setMessages((prev) => [readableMessage, ...prev].slice(0, 3));
          }
        } catch (e) {
          // ignore parsing errors
        }
      };

      ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

      ws.onclose = () => {
        if (isSubscribed) {
          setTimeout(connect, 3000);
        }
      };
    };

    connect();

    return () => {
      isSubscribed = false;
      if (ws) {
        ws.close();
      }
    };
  }, []);

  if (messages.length === 0) {
    return null;
  }

  return (
    <div className="w-full mb-6">
      <div className="p-4 rounded-[16px] glassmorphism border border-[#0066FF]/20 bg-white/60 dark:bg-[#16161A]/70 shadow-sm flex flex-col space-y-2">
        <div className="flex items-center space-x-2 text-[#0066FF] dark:text-[#34C759] font-outfit text-sm font-semibold tracking-wide">
          <span className="relative flex h-3 w-3">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#0066FF] opacity-75"></span>
            <span className="relative inline-flex rounded-full h-3 w-3 bg-[#0066FF]"></span>
          </span>
          <span>TEAMMATE SYNC</span>
        </div>

        <div className="flex flex-col space-y-1">
            {messages.map((msg, idx) => (
                <div key={idx} className="text-sm font-inter text-gray-700 dark:text-gray-300">
                {msg}
                </div>
            ))}
        </div>
      </div>
    </div>
  );
}
