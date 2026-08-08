"use client";

import React, { createContext, useContext, useEffect, useState, useRef } from "react";

type WebSocketContextType = {
  messages: any[];
  sendMessage: (msg: any) => void;
};

const WebSocketContext = createContext<WebSocketContextType>({
  messages: [],
  sendMessage: () => {},
});

export const useChatWebSocket = () => useContext(WebSocketContext);

export function ChatWebSocketProvider({ children, tenantId }: { children: React.ReactNode; tenantId?: string }) {
  const [messages, setMessages] = useState<any[]>([]);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    // Only connect if we are in browser
    if (typeof window === "undefined") return;

    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const host = window.location.host;
    const wsUrl = `${protocol}//${host}/api/v1/ui/omni_chat/ws`;

    const ws = new WebSocket(wsUrl);
    wsRef.current = ws;

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        setMessages((prev) => [...prev, data]);
      } catch (err) {
        console.error("Failed to parse websocket message", err);
      }
    };

    ws.onerror = (err) => {
      console.error("Omnichannel WebSocket error:", err);
    };

    return () => {
      ws.close();
    };
  }, [tenantId]);

  const sendMessage = (msg: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
    } else {
      console.error("WebSocket is not connected.");
    }
  };

  return (
    <WebSocketContext.Provider value={{ messages, sendMessage }}>
      {children}
    </WebSocketContext.Provider>
  );
}
