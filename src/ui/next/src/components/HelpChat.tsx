"use client";

import React, { useState, useRef, useEffect, useCallback } from "react";
import DOMPurify from "dompurify";
import { marked } from "marked";
import { WalkthroughTarget } from "./Walkthrough";

type Message = {
  id: string;
  sender: "user" | "agent";
  text: string;
  link?: { url: string; title: string };
};

function SpinnerIcon() {
  return (
    <svg
      className="w-5 h-5 animate-spin"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
      />
    </svg>
  );
}

function SendIcon() {
  return (
    <svg
      className="w-5 h-5"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
      />
    </svg>
  );
}

function createMarkup(msgText: string) {
  return {
    __html: DOMPurify.sanitize(
      msgText.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
             .replace(/\*(.*?)\*/g, '<em>$1</em>')
             .replace(/\n\n/g, '</p><p>')
             .replace(/^(.+)$/gm, '<p>$1</p>')
             .replace(/<p><p>/g, '<p>')
             .replace(/<\/p><\/p>/g, '</p>')
             .replace(/\- (.*?)<\/p>/g, '<li>$1</li>')
             .replace(/(<li>[\s\S]*?<\/li>)/, '<ul>$1</ul>')
    )
  };
}

function isSafeLink(url: unknown): url is string {
  return (
    typeof url === "string" &&
    (url.startsWith("/") ||
      url.startsWith("https://") ||
      url.startsWith("http://"))
  );
}

function normalizeAgentReply(data: unknown): Pick<Message, "text" | "link"> {
  if (!data || typeof data !== "object") {
    throw new Error("Invalid chat response");
  }

  const reply =
    "reply" in data ? (data as { reply?: unknown }).reply : undefined;
  if (typeof reply !== "string" || !reply.trim()) {
    throw new Error("Invalid chat reply");
  }

  const link = "link" in data ? (data as { link?: unknown }).link : undefined;
  if (link && typeof link === "object") {
    const candidate = link as { url?: unknown; title?: unknown };
    if (
      isSafeLink(candidate.url) &&
      typeof candidate.title === "string" &&
      candidate.title.trim()
    ) {
      return {
        text: reply,
        link: { url: candidate.url, title: candidate.title },
      };
    }
  }

  return { text: reply };
}

export function HelpChat() {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<Message[]>([
    {
      id: "1",
      sender: "agent",
      text: "Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?",
    },
  ]);
  const [inputValue, setInputValue] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const nextIdRef = useRef(2);

  const nextMessageId = (suffix: string) =>
    `${Date.now()}-${nextIdRef.current++}-${suffix}`;

  const scrollToBottom = () => {
    if (
      messagesEndRef.current &&
      typeof messagesEndRef.current.scrollIntoView === "function"
    ) {
      messagesEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isOpen, isLoading]);

  useEffect(() => {
    const handleOpenHelpChat = () => setIsOpen(true);
    window.addEventListener('open-help-chat', handleOpenHelpChat);
    return () => window.removeEventListener('open-help-chat', handleOpenHelpChat);
  }, []);

  const handleSend = useCallback(async (e?: React.FormEvent) => {
    e?.preventDefault();
    const messageText = inputValue.trim();
    if (!messageText || isLoading) return;

    const userMessage: Message = {
      id: nextMessageId("user"),
      sender: "user",
      text: messageText,
    };
    setMessages((prev) => [...prev, userMessage]);
    setInputValue("");
    setIsLoading(true);
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 15000);

    try {
      const response = await fetch("/api/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: messageText }),
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) throw new Error("Failed to fetch");

      const data = await response.json();
      const reply = normalizeAgentReply(data);

      setMessages((prev) => [
        ...prev,
        {
          id: nextMessageId("agent"),
          sender: "agent",
          ...reply,
        },
      ]);
    } catch (err: any) {
      clearTimeout(timeoutId);
      const isTimeout = err.name === "AbortError";
      setMessages((prev) => [
        ...prev,
        {
          id: nextMessageId("agent"),
          sender: "agent",
          text: isTimeout
            ? "Sorry, the connection timed out. Please try again later or check your network connection."
            : "Sorry, I'm having trouble connecting right now.",
        },
      ]);
    } finally {
      setIsLoading(false);
    }
  }, [inputValue, isLoading]);

  const clearChat = () => {
    setMessages([
      {
        id: "1",
        sender: "agent",
        text: "Hi! I'm your AI Help Agent. Need help setting up your store or understanding payments?",
      },
    ]);
  };

  useEffect(() => {
    const handleOpenHelpChat = () => setIsOpen(true);
    window.addEventListener("open-help-chat", handleOpenHelpChat);
    return () => window.removeEventListener("open-help-chat", handleOpenHelpChat);
  }, []);

  return (
    <div className="help-chat-wrapper pointer-events-none">
      {/* Floating Button */}
      <div className="fixed bottom-24 right-6 z-[10000] pointer-events-auto">
        {!isOpen && (
          <WalkthroughTarget id="ai-chat-trigger">
            <button
              id="ai-chat-trigger-btn"
              onClick={() => setIsOpen(true)}
              className="bg-blue-600/95 text-white p-4 min-h-[44px] rounded-full shadow-2xl hover:shadow-xl hover:scale-105 transition-all flex items-center justify-center gap-2 group backdrop-blur-xl saturate-[210%]"
              aria-label="Open help chat"
              aria-expanded={isOpen}
              aria-controls="ai-chat-interface"
            >
              <span className="text-xl">✨</span>
              <span className="font-outfit font-bold max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 whitespace-nowrap px-0 group-hover:px-2">
                Ask anything
              </span>
            </button>
          </WalkthroughTarget>
        )}
      </div>

      {/* Chat Interface */}
      {isOpen && (
        <div id="ai-chat-interface" role="dialog" aria-labelledby="ai-chat-header-title" aria-modal="false" className="fixed bottom-24 right-6 z-[10000] w-full max-w-[350px] pointer-events-auto bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[24px] shadow-[0_8px_32px_rgba(0,0,0,0.08)] flex flex-col overflow-hidden animate-slide-up-chat text-gray-900 dark:text-gray-100">
          {/* Header */}
          <div
            id="ai-chat-header"
            className="bg-blue-600/95 text-white p-4 flex justify-between items-center backdrop-blur-md"
          >
            <div className="flex items-center gap-2">
              <span className="text-xl drop-shadow-md">✨</span>
              <div>
                <h3 id="ai-chat-header-title" className="font-bold font-outfit text-sm tracking-wide text-white/90">
                  Ask anything
                </h3>
                <p className="text-xs text-blue-100 font-inter font-medium">
                  Always here to help
                </p>
              </div>
            </div>
            <div className="flex gap-2">
              {messages.length > 1 && (
                <button
                  onClick={() => clearChat()}
                  className="text-blue-100 hover:text-white transition-colors bg-white/10 hover:bg-white/20 rounded-full p-1.5 min-h-[44px] px-3 flex items-center text-xs font-bold font-inter"
                  aria-label="Clear chat"
                >
                  Clear
                </button>
              )}
              <button
              onClick={() => setIsOpen(false)}
              className="text-blue-100 hover:text-white transition-colors bg-white/10 hover:bg-white/20 rounded-full p-1.5 min-h-[44px]"
              aria-label="Close help chat"
              aria-expanded={isOpen}
              aria-controls="ai-chat-interface"
            >
              <span className="sr-only">✕</span>
              <svg
                className="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
            </div>
          </div>

          {/* Messages */}
          <div role="log" aria-live="polite" aria-atomic="false" className="flex-1 p-5 overflow-y-auto h-[350px] bg-gradient-to-b from-white/40 to-transparent dark:from-white/5 flex flex-col gap-5 font-inter text-sm custom-scrollbar">
            {messages.map((msg) => (
              <div
                key={msg.id}
                className={`flex flex-col ${msg.sender === "user" ? "items-end" : "items-start"}`}
              >
                <div
                  className={`px-4 py-3 rounded-2xl max-w-[85%] leading-relaxed shadow-sm saturate-[210%] ${
                    msg.sender === "user"
                      ? "bg-blue-600/90 backdrop-blur-[30px] saturate-[210%] text-white rounded-br-sm border border-white/20"
                      : "bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/50 dark:border-white/20 text-gray-900 dark:text-gray-100 rounded-bl-sm prose prose-sm prose-blue dark:prose-invert max-w-none prose-p:my-1 prose-ul:my-1 prose-li:my-0.5"
                  }`}
                  dangerouslySetInnerHTML={createMarkup(msg.text)}
                />
                {msg.link && (
                  <a
                    href={msg.link.url}
                    className="mt-2 ml-1 text-blue-600 hover:text-blue-800 text-xs font-bold hover:underline bg-blue-50/90 backdrop-blur-[30px] px-3.5 py-1.5 rounded-full border border-blue-100 flex items-center shadow-sm transition-all hover:bg-blue-100/90"
                  >
                    Read the full article →
                  </a>
                )}
              </div>
            ))}
            {isLoading && (
              <div className="flex flex-col items-start animate-pulse">
                <div className="px-4 py-3 rounded-2xl max-w-[85%] glassmorphism text-gray-800 rounded-bl-sm shadow-[0_2px_10px_rgba(0,0,0,0.02)] flex gap-1">
                  <div
                    className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"
                    style={{ animationDelay: "0ms" }}
                  ></div>
                  <div
                    className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"
                    style={{ animationDelay: "150ms" }}
                  ></div>
                  <div
                    className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"
                    style={{ animationDelay: "300ms" }}
                  ></div>
                </div>
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>

          {/* Input */}
          <form
            onSubmit={handleSend}
            className="p-3 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border-t border-white/40 dark:border-white/10 flex gap-2 items-center"
          >
            <WalkthroughTarget id="ohc-help-input-area" className="flex-1 flex">
            <input
              id="ohc-help-input-area"
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Escape') {
                  setIsOpen(false);
                }
              }}
              autoFocus={true}
              placeholder="Ask anything..."
              disabled={isLoading}
              className="flex-1 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] border border-white/40 dark:border-white/10 rounded-[24px] px-4 py-3 text-base min-h-[44px] focus:outline-none focus:ring-2 focus:ring-blue-500/50 font-inter shadow-inner disabled:opacity-70 disabled:bg-gray-100/70 text-gray-900 dark:text-gray-100"
            />
            </WalkthroughTarget>
            <button
              type="submit"
              disabled={!inputValue.trim() || isLoading}
              className="bg-blue-600/95 backdrop-blur-md text-white p-2.5 min-w-[44px] min-h-[44px] flex items-center justify-center rounded-2xl disabled:opacity-50 disabled:cursor-not-allowed hover:bg-blue-700/95 transition-all shadow-md active:scale-95"
              aria-label="Send message"
            >
              {isLoading ? <SpinnerIcon /> : <SendIcon />}
            </button>
          </form>
        </div>
      )}

      <style
        dangerouslySetInnerHTML={{
          __html: `
        @keyframes slide-up-chat {
          0% { opacity: 0; transform: translateY(20px) scale(0.95); }
          100% { opacity: 1; transform: translateY(0) scale(1); }
        }
        .animate-slide-up-chat { animation: slide-up-chat 0.2s cubic-bezier(0.16, 1, 0.3, 1) forwards; transform-origin: bottom right; }
        .custom-scrollbar::-webkit-scrollbar { width: 6px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(0,0,0,0.1); border-radius: 10px; }
        @media (prefers-color-scheme: dark) {
          .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.1); }
        }
      `,
        }}
      />
    </div>
  );
}
