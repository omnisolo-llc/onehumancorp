"use client";

import React, { useState, useRef, useEffect } from "react";
import DOMPurify from "dompurify";

type Message = {
  id: string;
  sender: "user" | "agent";
  text: string;
  link?: { url: string; title: string };
};

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

  const handleSend = async (e?: React.FormEvent) => {
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
  };

  const isE2E = process.env.NEXT_PUBLIC_E2E === "true";
  const forceChat =
    typeof window !== "undefined" &&
    window.location.search.includes("test_chat=true");
  if (isE2E && !forceChat) {
    // We shouldn't hide the HelpChat in E2E unless we specifically want it gone, but tests rely on it.
    // Given the test failures, let's keep it mounted during E2E.
    // return null;
  }

  return (
    <div className="help-chat-wrapper">
      {/* Floating Button */}
      <div className="fixed bottom-24 left-6 z-[9999]">
        {!isOpen && (
          <button
            onClick={() => setIsOpen(true)}
            className="bg-blue-600/95 text-white p-4 rounded-full shadow-2xl hover:shadow-xl hover:scale-105 transition-all flex items-center justify-center gap-2 group backdrop-blur-[20px] saturate-200"
            aria-label="Open help chat"
          >
            <span className="text-xl">✨</span>
            <span className="font-outfit font-bold max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-300 whitespace-nowrap px-0 group-hover:px-2">
              Ask anything
            </span>
          </button>
        )}
      </div>

      {/* Chat Interface */}
      {isOpen && (
        <div className="fixed bottom-24 left-6 z-[9999] w-[350px] max-w-[calc(100vw-32px)] bg-white/70 backdrop-blur-[20px] saturate-200 rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.1)] flex flex-col overflow-hidden border border-white/60 animate-slide-up-chat">
          {/* Header */}
          <div
            id="ai-chat-header"
            className="bg-blue-600/95 text-white p-4 flex justify-between items-center backdrop-blur-[20px]"
          >
            <div className="flex items-center gap-2">
              <span className="text-xl drop-shadow-md">✨</span>
              <div>
                <h3 className="font-bold font-outfit text-sm tracking-wide text-white/90">
                  Ask AI Help
                </h3>
                <p className="text-xs text-blue-100 font-inter font-medium">
                  Always here to help
                </p>
              </div>
            </div>
            <button
              onClick={() => setIsOpen(false)}
              className="text-blue-100 hover:text-white transition-colors bg-white/10 hover:bg-white/20 rounded-full p-1.5"
              aria-label="Close help chat"
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

          {/* Messages */}
          <div className="flex-1 p-5 overflow-y-auto h-[350px] bg-gradient-to-b from-white/40 to-transparent flex flex-col gap-5 font-inter text-sm">
            {messages.map((msg) => (
              <div
                key={msg.id}
                className={`flex flex-col ${msg.sender === "user" ? "items-end" : "items-start"}`}
              >
                <div
                  className={`px-4 py-3 rounded-2xl max-w-[85%] leading-relaxed shadow-[0_2px_10px_rgba(0,0,0,0.02)] ${
                    msg.sender === "user"
                      ? "bg-blue-600/95 backdrop-blur-[20px] saturate-200 text-white rounded-br-sm border border-blue-500/50"
                      : "bg-white/90 backdrop-blur-[20px] saturate-200 border border-white/80 text-gray-800 rounded-bl-sm"
                  }`}
                  dangerouslySetInnerHTML={{
                    __html: DOMPurify.sanitize(msg.text),
                  }}
                />
                {msg.link && (
                  <a
                    href={msg.link.url}
                    className="mt-2 ml-1 text-blue-600 hover:text-blue-800 text-xs font-bold hover:underline bg-blue-50/90 backdrop-blur-[20px] px-3.5 py-1.5 rounded-full border border-blue-100 flex items-center shadow-sm transition-all hover:bg-blue-100/90"
                  >
                    {msg.link.title}
                  </a>
                )}
              </div>
            ))}
            {isLoading && (
              <div className="flex flex-col items-start animate-pulse">
                <div className="px-4 py-3 rounded-2xl max-w-[85%] bg-white/90 backdrop-blur-[20px] saturate-200 border border-white/80 text-gray-800 rounded-bl-sm shadow-[0_2px_10px_rgba(0,0,0,0.02)] flex gap-1">
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
            className="p-3 bg-white/60 backdrop-blur-[20px] saturate-200 border-t border-white/50 flex gap-2"
          >
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="Ask me anything..."
              disabled={isLoading}
              className="flex-1 bg-white/70 backdrop-blur-[20px] saturate-200 border border-white/60 rounded-xl px-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 font-inter shadow-inner disabled:opacity-70 disabled:bg-gray-50/70"
            />
            <button
              type="submit"
              disabled={!inputValue.trim() || isLoading}
              className="bg-blue-600/95 backdrop-blur-[20px] text-white p-2.5 rounded-xl disabled:opacity-50 disabled:cursor-not-allowed hover:bg-blue-700/95 transition-all shadow-[0_4px_12px_rgba(37,99,235,0.2)] active:scale-95"
              aria-label="Send message"
            >
              {isLoading ? (
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
              ) : (
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
              )}
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
      `,
        }}
      />
    </div>
  );
}
