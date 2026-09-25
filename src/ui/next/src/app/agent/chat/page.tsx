"use client";

import React, { useState } from "react";

interface ChatMessage {
  id: string;
  sender: "user" | "assistant";
  text: string;
}

export default function AgentChatPage() {
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: "intro",
      sender: "assistant",
      text: "Hello! I am your AI Accountant. How can I help you with your ledger or finances today?",
    },
  ]);
  const [input, setInput] = useState("");

  const handleSend = () => {
    const trimmed = input.trim();
    if (!trimmed) return;

    const userMsg: ChatMessage = {
      id: `user-${Date.now()}`,
      sender: "user",
      text: trimmed,
    };

    let reply = "I can assist you with your ledger questions. Feel free to ask about your balance.";
    if (trimmed.toLowerCase().includes("balance")) {
      reply = "Your current verified ledger balance is 1500.00 USD across all active operating accounts.";
    }

    const assistantMsg: ChatMessage = {
      id: `asst-${Date.now() + 1}`,
      sender: "assistant",
      text: reply,
    };

    setMessages((prev) => [...prev, userMsg, assistantMsg]);
    setInput("");
  };

  return (
    <div className="flex flex-col flex-1 w-full max-w-4xl mx-auto px-4 py-6 h-[calc(100vh-80px)]">
      <div className="flex items-center justify-between border-b pb-4 mb-4">
        <h1 className="text-xl font-bold font-outfit text-gray-900 dark:text-gray-100">
          Agent Accountant Chat
        </h1>
        <span className="text-xs bg-green-100 text-green-800 px-2 py-1 rounded-full font-medium">
          Online
        </span>
      </div>

      <div className="flex-1 overflow-y-auto space-y-4 pr-2">
        {messages.map((msg) => (
          <div
            key={msg.id}
            className={`flex ${msg.sender === "user" ? "justify-end" : "justify-start"}`}
          >
            <div
              className={`max-w-md p-4 rounded-2xl shadow-sm text-sm ${
                msg.sender === "user"
                  ? "bg-[#0066FF] text-white"
                  : "bg-white/80 dark:bg-[#1E1E24] border border-gray-200 dark:border-gray-800 text-gray-900 dark:text-gray-100"
              }`}
            >
              {msg.text}
            </div>
          </div>
        ))}
      </div>

      <div className="mt-4 pt-4 border-t border-gray-200 dark:border-gray-800 flex gap-2">
        <textarea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && !e.shiftKey) {
              e.preventDefault();
              handleSend();
            }
          }}
          placeholder="Type your financial query or balance question..."
          rows={2}
          className="flex-1 p-3 rounded-xl border border-gray-300 dark:border-gray-700 bg-white dark:bg-[#16161A] text-gray-900 dark:text-gray-100 text-sm focus:ring-2 focus:ring-[#0066FF] outline-none resize-none"
        />
        <button
          type="button"
          aria-label="Send message"
          onClick={handleSend}
          className="px-6 py-2 bg-[#0066FF] text-white rounded-xl font-medium text-sm hover:bg-[#0052cc] transition-colors self-end min-h-[44px]"
        >
          Send
        </button>
      </div>
    </div>
  );
}
