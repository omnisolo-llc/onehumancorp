"use client";

import { Fragment, useEffect, useMemo, useState, useRef, type ReactNode } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { useQuery } from "@powersync/react";
import { PowerSyncProvider } from "../../lib/powersync/PowerSyncProvider";

type Message = {
  id: string;
  source?: string;
  content?: string;
  original_content?: string;
  translated_from_language?: string;
  draft_reply?: string;
  status?: string;
  sender_id?: string;
  customer_id?: string;
  created_at?: string;
};

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved", "auto_replied"].includes(normalized)) return "good";
  if (["open", "pending", ""].includes(normalized)) return "warn";
  if (["failed", "blocked"].includes(normalized)) return "bad";
  return "";
}

function normalizeExternalHttpUrl(value: string) {
  try {
    const url = new URL(value);
    return url.protocol === "http:" || url.protocol === "https:" ? url.toString() : null;
  } catch {
    return null;
  }
}

function textWithLineBreaks(value: string, keyPrefix: string): ReactNode[] {
  return value.split("\n").flatMap((line, index) => [
    index > 0 ? <br key={`${keyPrefix}-break-${index}`} /> : null,
    <Fragment key={`${keyPrefix}-line-${index}`}>{line}</Fragment>,
  ]);
}

function renderMessageContent(content: string): ReactNode {
  if (!content) return "Empty message";

  const tokenPattern = /\[Media:\s*(.+?)\s+-\s+(https?:\/\/[^\]\s]+)\]|!\[([^\]]*)\]\((https?:\/\/[^)\s]+)\)/g;

  const elements: ReactNode[] = [];
  let lastIndex = 0;
  let match;

  while ((match = tokenPattern.exec(content)) !== null) {
    if (match.index > lastIndex) {
      const text = content.slice(lastIndex, match.index);
      elements.push(...textWithLineBreaks(text, `text-${lastIndex}`));
    }

    if (match[1] && match[2]) {
      const type = match[1].toLowerCase();
      const rawUrl = match[2];
      const validUrl = normalizeExternalHttpUrl(rawUrl);

      if (validUrl) {
        if (type === "image") {
          elements.push(
            <div key={`media-${match.index}`} className="my-2 max-w-full overflow-hidden rounded-lg border border-white/20">
              <img src={validUrl} alt="Attached Media" className="max-w-full max-h-64 object-cover block" loading="lazy" />
            </div>
          );
        } else if (type === "video") {
          elements.push(
            <div key={`media-${match.index}`} className="my-2 max-w-full overflow-hidden rounded-lg border border-white/20">
              <video src={validUrl} controls className="max-w-full max-h-64 object-cover block" preload="metadata" />
            </div>
          );
        } else if (type === "audio") {
          elements.push(
            <div key={`media-${match.index}`} className="my-2 max-w-full overflow-hidden rounded-lg border border-white/20">
              <audio src={validUrl} controls className="max-w-full block" preload="metadata" />
            </div>
          );
        } else if (type === "document" || type === "file") {
          elements.push(
            <a key={`media-${match.index}`} href={validUrl} target="_blank" rel="noopener noreferrer" className="inline-flex items-center gap-2 px-3 py-2 my-2 rounded-lg bg-white/10 hover:bg-white/20 border border-white/20 text-sm font-medium transition-colors">
              <svg className="w-4 h-4 opacity-70" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" /></svg>
              View Document
            </a>
          );
        } else {
           elements.push(
            <a key={`media-${match.index}`} href={validUrl} target="_blank" rel="noopener noreferrer" className="inline-block text-blue-400 hover:text-blue-300 underline underline-offset-4 decoration-blue-400/30">
              {rawUrl}
            </a>
          );
        }
      } else {
         elements.push(
            <span key={`media-${match.index}`} className="text-red-400 italic">
              [Invalid Media Link]
            </span>
          );
      }
    } else if (match[3] !== undefined && match[4]) {
      const altText = match[3];
      const rawUrl = match[4];
      const validUrl = normalizeExternalHttpUrl(rawUrl);

      if (validUrl) {
         elements.push(
            <div key={`md-img-${match.index}`} className="my-2 max-w-full overflow-hidden rounded-lg border border-white/20">
              <img src={validUrl} alt={altText || "Image"} className="max-w-full max-h-64 object-cover block" loading="lazy" />
            </div>
          );
      } else {
         elements.push(
            <span key={`md-img-${match.index}`} className="text-red-400 italic">
              [Invalid Image Link]
            </span>
          );
      }
    }

    lastIndex = match.index + match[0].length;
  }

  if (lastIndex < content.length) {
    const text = content.slice(lastIndex);
    elements.push(...textWithLineBreaks(text, `text-${lastIndex}`));
  }

  return <>{elements}</>;
}

function InnerInboxPage() {
  const router = useRouter();
  const [selectedMessageId, setSelectedMessageId] = useState<string | null>(null);

  // Real-time Chat state
  const [wsConnected, setWsConnected] = useState(false);
  const [realtimeMessages, setRealtimeMessages] = useState<Message[]>([]);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    // We would connect to our native Rust backend websocket instead of Chatwoot
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/api/chat/ws`;

    console.log("Connecting to Native OHC Chat WebSocket...");

    try {
      const ws = new WebSocket(wsUrl);

      ws.onopen = () => {
        setWsConnected(true);
      };

      ws.onmessage = (event) => {
        // Native chat real-time events
        console.log("WS message:", event.data);
      };

      ws.onclose = () => {
        setWsConnected(false);
      };

      wsRef.current = ws;
    } catch (e) {
      console.error("Failed to connect to WS:", e);
    }

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, []);

  const { data: messages = [], isLoading, error } = useQuery(`
    SELECT
      m.id,
      m.sender_type as source,
      m.content,
      '' as original_content,
      '' as translated_from_language,
      '' as draft_reply,
      m.status,
      m.sender_id,
      c.contact_id as customer_id,
      m.created_at
    FROM chat_messages m
    LEFT JOIN chat_conversations c ON m.conversation_id = c.id
    ORDER BY m.created_at DESC
    LIMIT 100
  `);

  const displayMessages = useMemo(() => {
    // Combine local messages and real-time messages for display
    return messages as Message[];
  }, [messages]);

  const selectedMessage = useMemo(() => {
    return displayMessages.find(m => m.id === selectedMessageId) || null;
  }, [displayMessages, selectedMessageId]);

  return (
    <AppShell>
      <div className="flex h-[calc(100vh-64px)] w-full">
        {/* Inbox Sidebar (Translucent Glass) */}
        <div className={`
          flex flex-col border-r border-white/10
          bg-white/60 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%]
          w-full md:w-[380px] flex-shrink-0 transition-all duration-300
          ${selectedMessageId ? 'hidden md:flex' : 'flex'}
        `}>
          <div className="flex items-center justify-between p-4 border-b border-white/10 h-16 flex-shrink-0">
            <h1 className="text-[17px] font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]">Unified Inbox</h1>
            <div className="flex items-center gap-2">
              <span className={`flex h-2 w-2 rounded-full ${wsConnected ? 'bg-[#34C759]' : 'bg-[#FF9500]'}`} title={wsConnected ? "Connected to OHC Chat Engine" : "Connecting..."} />
            </div>
          </div>

          <div className="flex-1 overflow-y-auto">
            {isLoading && (
              <div className="p-6 text-center text-sm font-medium opacity-50">Loading messages...</div>
            )}

            {error && (
              <div className="p-6 text-center text-sm font-medium text-[#FF3B30]">Error: {error.message}</div>
            )}

            {!isLoading && displayMessages.length === 0 && (
              <div className="p-8 text-center flex flex-col items-center justify-center h-full">
                <div className="w-12 h-12 rounded-full bg-white/5 border border-white/10 flex items-center justify-center mb-4">
                  <svg className="w-5 h-5 opacity-40" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
                  </svg>
                </div>
                <h3 className="text-base font-semibold mb-1">All Caught Up</h3>
                <p className="text-[13px] opacity-60">No pending messages in your inbox.</p>
              </div>
            )}

            <div className="divide-y divide-white/5">
              {displayMessages.map((msg) => (
                <button
                  key={msg.id}
                  onClick={() => setSelectedMessageId(msg.id)}
                  className={`
                    w-full text-left p-4 transition-all hover:bg-white/5
                    min-h-[44px] /* Touch target minimum */
                    ${selectedMessageId === msg.id ? 'bg-[#0066FF]/10 dark:bg-[#0071E3]/20 border-l-2 border-l-[#0066FF] dark:border-l-[#0071E3]' : 'border-l-2 border-l-transparent'}
                  `}
                >
                  <div className="flex justify-between items-baseline mb-1 gap-2">
                    <span className="font-semibold text-[15px] truncate text-[#1D1D1F] dark:text-[#F5F5F7]">
                      {msg.source || "Direct Message"}
                    </span>
                    <span className="text-[11px] font-medium opacity-50 whitespace-nowrap">
                      {msg.created_at ? new Date(msg.created_at).toLocaleDateString() : ""}
                    </span>
                  </div>
                  <div className="text-[13px] opacity-70 line-clamp-2 leading-relaxed mb-2">
                    {msg.content || "No content"}
                  </div>
                  <div className="flex gap-1.5 mt-2">
                    <span className={`inline-flex items-center px-1.5 py-0.5 rounded-[4px] text-[10px] font-semibold tracking-wide uppercase ${
                      badgeTone(msg.status) === 'good' ? 'bg-[#34C759]/20 text-[#34C759]' :
                      badgeTone(msg.status) === 'warn' ? 'bg-[#FF9500]/20 text-[#FF9500]' :
                      badgeTone(msg.status) === 'bad' ? 'bg-[#FF3B30]/20 text-[#FF3B30]' :
                      'bg-white/10 opacity-70'
                    }`}>
                      {msg.status || "NEW"}
                    </span>
                    {msg.draft_reply && (
                      <span className="inline-flex items-center px-1.5 py-0.5 rounded-[4px] text-[10px] font-semibold tracking-wide uppercase bg-purple-500/20 text-purple-400">
                        AI Draft
                      </span>
                    )}
                  </div>
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Conversation Stream (Translucent Glass) */}
        <div className={`
          flex-1 flex flex-col bg-white/40 dark:bg-[#16161A]/40 backdrop-blur-[30px]
          transition-all duration-300
          ${!selectedMessageId ? 'hidden md:flex' : 'flex'}
        `}>
          {selectedMessage ? (
            <>
              {/* Header */}
              <div className="h-16 flex items-center justify-between px-6 border-b border-white/10 bg-white/60 dark:bg-[#16161A]/80 backdrop-blur-md flex-shrink-0">
                <div className="flex items-center gap-3">
                  <button
                    onClick={() => setSelectedMessageId(null)}
                    className="md:hidden flex items-center justify-center w-10 h-10 -ml-3 rounded-full hover:bg-white/10 transition-colors"
                  >
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
                    </svg>
                  </button>
                  <h2 className="text-[17px] font-semibold tracking-tight text-[#1D1D1F] dark:text-[#F5F5F7]">
                    Conversation
                  </h2>
                  <span className="px-2 py-0.5 rounded-full bg-white/10 text-[11px] font-medium opacity-60">
                    ID: {selectedMessage.id.split('-')[0]}
                  </span>
                </div>

                <div className="flex gap-2">
                  <button className="h-9 px-4 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 text-[13px] font-medium transition-colors">
                    Close
                  </button>
                </div>
              </div>

              {/* Message Stream */}
              <div className="flex-1 overflow-y-auto p-6 space-y-6">

                <div className="flex flex-col gap-1 items-start max-w-[85%]">
                  <span className="text-[11px] font-medium opacity-50 px-3">
                    {selectedMessage.created_at ? new Date(selectedMessage.created_at).toLocaleString() : ""}
                  </span>
                  <div className="bg-white/80 dark:bg-white/10 border border-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] px-5 py-3.5 rounded-2xl rounded-tl-sm text-[15px] leading-relaxed shadow-sm">
                    {renderMessageContent(selectedMessage.content || "Empty message")}
                  </div>
                </div>

                {selectedMessage.original_content && (
                  <div className="flex flex-col gap-1 items-start max-w-[85%] mt-2">
                    <span className="text-[11px] font-medium text-[#0066FF] dark:text-[#0071E3] px-3">
                      Translated from {selectedMessage.translated_from_language || 'unknown'}
                    </span>
                    <div className="bg-white/40 dark:bg-white/5 border border-white/10 text-[#1D1D1F] dark:text-[#F5F5F7] px-4 py-3 rounded-2xl rounded-tl-sm text-[13px] opacity-70 italic leading-relaxed">
                      {selectedMessage.original_content}
                    </div>
                  </div>
                )}

                {selectedMessage.draft_reply && (
                  <div className="flex flex-col gap-1 items-end max-w-[85%] self-end ml-auto mt-6 relative">
                    <span className="text-[11px] font-medium text-purple-500 px-3 flex items-center gap-1.5">
                      <svg className="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                      AI Drafted Response
                    </span>
                    <div className="bg-purple-500/10 border border-purple-500/30 text-[#1D1D1F] dark:text-[#F5F5F7] px-5 py-3.5 rounded-2xl rounded-tr-sm text-[15px] leading-relaxed shadow-sm">
                       {renderMessageContent(selectedMessage.draft_reply)}
                    </div>

                    <div className="flex gap-2 mt-2 w-full justify-end">
                      <button className="h-9 px-4 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 text-[13px] font-medium transition-colors">
                        Edit
                      </button>
                      <button className="h-9 px-4 rounded-lg bg-purple-500 hover:bg-purple-600 text-white shadow-sm text-[13px] font-semibold transition-colors flex items-center gap-2">
                        <span>Send Draft</span>
                        <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M14 5l7 7m0 0l-7 7m7-7H3" /></svg>
                      </button>
                    </div>
                  </div>
                )}
              </div>

              {/* Composer */}
              <div className="p-4 border-t border-white/10 bg-white/60 dark:bg-[#16161A]/80 backdrop-blur-md flex-shrink-0">
                <div className="flex items-end gap-3 bg-white/80 dark:bg-black/20 border border-white/20 rounded-xl p-2 shadow-inner">
                  <button className="flex-shrink-0 w-9 h-9 rounded-lg flex items-center justify-center hover:bg-white/10 text-opacity-60 transition-colors">
                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" /></svg>
                  </button>
                  <textarea
                    className="flex-1 bg-transparent border-none focus:ring-0 resize-none min-h-[44px] max-h-32 text-[15px] placeholder-opacity-50 py-2.5 outline-none"
                    placeholder="Type a message..."
                    rows={1}
                  />
                  <button className="flex-shrink-0 h-9 px-4 rounded-lg bg-[#0066FF] dark:bg-[#0071E3] hover:brightness-110 text-white font-semibold text-[13px] shadow-sm transition-all flex items-center gap-2">
                    Send
                  </button>
                </div>
              </div>
            </>
          ) : (
            <div className="flex-1 flex flex-col items-center justify-center opacity-50 p-8 text-center">
              <div className="w-16 h-16 rounded-full bg-white/5 border border-white/10 flex items-center justify-center mb-6">
                <svg className="w-6 h-6 opacity-40" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                </svg>
              </div>
              <p className="text-[15px] font-medium">Select a conversation</p>
              <p className="text-[13px] mt-1 opacity-70">Choose a message from the list to view details and reply.</p>
            </div>
          )}
        </div>
      </div>
    </AppShell>
  );
}

export default function InboxPage() {
  return (
    <PowerSyncProvider>
      <InnerInboxPage />
    </PowerSyncProvider>
  );
}
