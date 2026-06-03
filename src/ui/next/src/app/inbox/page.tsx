"use client";

import React, { useState } from "react";
import Head from "next/head";

// --- Mock Data ---

type Channel = "instagram" | "sms" | "email" | "whatsapp" | "facebook";
type MessageStatus = "ai_handled" | "human_required" | "sent" | "received";

interface Message {
  id: string;
  sender: string;
  channel: Channel;
  content: string;
  timestamp: string;
  status: MessageStatus;
  aiDraft?: string;
  isEscalated?: boolean;
}

const mockConversations: Message[] = [
  {
    id: "msg_1",
    sender: "Instagram User",
    channel: "instagram",
    content: "When will my order be shipped?",
    timestamp: "Yesterday",
    status: "received",
    aiDraft:
      '"Your order is currently being prepared and will be shipped within 24 hours. You will receive a tracking link shortly."',
    isEscalated: false,
  },
  {
    id: "msg_2",
    sender: "SMS Customer",
    channel: "sms",
    content: "I need a quote for a plumbing job at 123 Main St.",
    timestamp: "2 days ago",
    status: "received",
    aiDraft: "", // Escalated
    isEscalated: true,
  },
  {
    id: "msg_3",
    sender: "Facebook User",
    channel: "facebook",
    content: "Do you have vegan birthday cake options?",
    timestamp: "10:00 AM",
    status: "received",
    aiDraft:
      '"Yes, we have several vegan birthday cake options available! You can order them directly from our website or let me know what flavors you are interested in."',
    isEscalated: false,
  },
];

const channelIcons: Record<Channel, string> = {
  instagram: "📸",
  sms: "💬",
  email: "✉️",
  whatsapp: "🟢",
  facebook: "📘",
};

export default function InboxPage() {
  const [messages, setMessages] = useState<Message[]>(mockConversations);
  const [aiActive, setAiActive] = useState(true);

  const simulateIncomingMessage = () => {
    const newMessage: Message = {
      id: `msg_${Date.now()}`,
      sender: "New Customer",
      channel: "instagram",
      content: "Do you have any availability this weekend?",
      timestamp: "Just now",
      status: "received",
      aiDraft: '"We have a few slots open on Saturday afternoon. Would you like me to hold one for you?"',
      isEscalated: false,
    };
    setMessages((prev) => [newMessage, ...prev]);
  };

  const handleApproveDraft = (id: string) => {
    setMessages((prev) =>
      prev.map((msg) =>
        msg.id === id
          ? {
              ...msg,
              content: msg.aiDraft || msg.content,
              status: "sent",
              sender: "✨ Ambassador",
              aiDraft: undefined, // Clear draft after sending
            }
          : msg
      )
    );
  };

  return (
    <div className="min-h-screen bg-[#f8f9fa] flex justify-center font-sans">
      <Head>
        <title>Omnichannel Inbox</title>
        <meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1" />
      </Head>

      {/* Main Mobile Container (375px max-width constraint) */}
      <div className="w-full max-w-[375px] bg-white h-screen shadow-2xl relative flex flex-col overflow-hidden">
        {/* Blurred Glass App Bar */}
        <header className="absolute top-0 left-0 right-0 z-10 bg-white/70 backdrop-blur-md border-b border-gray-200/50 flex items-center justify-between px-4 py-4">
          <div className="flex items-center space-x-2">
            <button className="text-blue-500 font-medium p-1 -ml-1">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 inline-block -mt-0.5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
              </svg>
              Back
            </button>
            <h1 className="text-xl font-bold text-gray-900 tracking-tight">Inbox</h1>
          </div>
          <div className="flex items-center space-x-3">
            {/* AI Toggle */}
            <button
              onClick={() => setAiActive(!aiActive)}
              className={`flex items-center px-2.5 py-1 rounded-full text-xs font-semibold transition-colors duration-200 ${
                aiActive ? "bg-green-100 text-green-700 border border-green-200" : "bg-gray-100 text-gray-500 border border-gray-200"
              }`}
            >
              <div className={`w-1.5 h-1.5 rounded-full mr-1.5 ${aiActive ? "bg-green-500" : "bg-gray-400"}`} />
              {aiActive ? "AI: Active" : "AI: Paused"}
            </button>
            {/* Settings Icon */}
            <button className="text-gray-600 p-1">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
            </button>
          </div>
        </header>

        {/* Scrollable Feed Area */}
        <main className="flex-1 overflow-y-auto bg-gray-50 pt-20 pb-20 px-4 space-y-4">

          {/* Main call to action block - UniFi Style */}
           <div className="w-full bg-[#0055FF] text-white rounded-xl p-4 shadow-sm mb-6 flex justify-center items-center font-medium cursor-pointer hover:bg-blue-600 transition">
             Schedule Outbound Post
           </div>

          {messages.map((msg) => (
            <div key={msg.id} className="relative group">
              {/* Message Thread Card */}
              <div className="bg-white rounded-2xl p-4 shadow-[0_2px_8px_rgba(0,0,0,0.04)] border border-gray-100 flex flex-col space-y-3 relative z-0">
                {/* Header: Channel & Sender */}
                <div className="flex justify-between items-center">
                  <div className="flex items-center space-x-2">
                    <span className="text-sm" role="img" aria-label={msg.channel}>
                      {channelIcons[msg.channel]}
                    </span>
                    <span className="font-semibold text-gray-900 text-sm">{msg.sender}</span>
                  </div>
                  <span className="text-xs text-gray-400">{msg.timestamp}</span>
                </div>

                {/* Customer Message Content */}
                <div className="text-gray-800 text-[15px] leading-relaxed">
                  {msg.content}
                </div>
              </div>

              {/* AI Draft / Escalation Block (Frosted Glass attached to bottom of card) */}
              {(msg.aiDraft || msg.isEscalated) && msg.status === "received" && (
                <div className="bg-yellow-50/80 backdrop-blur-sm border border-yellow-200/50 rounded-b-2xl rounded-t-none -mt-4 pt-6 pb-4 px-4 shadow-sm flex flex-col space-y-3 relative z-10">
                  <div className="flex justify-between items-center -mt-2 mb-1">
                     {msg.aiDraft && !msg.isEscalated && (
                        <span className="bg-yellow-400 text-yellow-900 text-[10px] uppercase font-bold px-2 py-0.5 rounded-full flex items-center shadow-sm">
                           <span className="mr-1">✨</span> AI DRAFT
                        </span>
                     )}
                     {msg.isEscalated && (
                        <span className="bg-red-100 text-red-700 text-[10px] uppercase font-bold px-2 py-0.5 rounded-full flex items-center border border-red-200">
                           <span className="mr-1">⚠️</span> HUMAN REQUIRED
                        </span>
                     )}
                  </div>

                  {/* AI Draft Text */}
                  {msg.aiDraft && !msg.isEscalated && (
                    <div className="text-gray-700 text-[15px] leading-relaxed italic">
                      {msg.aiDraft}
                    </div>
                  )}

                  {/* Escalation Text */}
                  {msg.isEscalated && (
                     <div className="text-gray-600 text-sm flex flex-col space-y-3">
                         <p>AI was unsure how to quote this custom job. Please review and respond manually.</p>
                         <button className="w-full bg-white border border-gray-300 text-gray-700 py-2.5 rounded-xl font-medium shadow-sm hover:bg-gray-50 transition active:scale-95">
                           Open Thread
                         </button>
                     </div>
                  )}

                  {/* Actions for Draft */}
                  {msg.aiDraft && !msg.isEscalated && (
                    <div className="flex space-x-2 pt-1">
                      <button className="flex-1 bg-white border border-gray-200 text-gray-700 py-2.5 rounded-xl font-medium shadow-sm hover:bg-gray-50 transition active:scale-95">
                        Edit
                      </button>
                      <button
                        onClick={() => handleApproveDraft(msg.id)}
                        className="flex-[2] bg-[#FACC15] text-yellow-900 py-2.5 rounded-xl font-semibold shadow-sm hover:bg-[#FDE047] transition active:scale-95">
                        Approve & Send
                      </button>
                    </div>
                  )}
                </div>
              )}
            </div>
          ))}

          {/* Hidden element for Playwright to click and trigger the E2E test without showing it to the user */}
          <button
            onClick={simulateIncomingMessage}
            className="opacity-0 h-0 w-0 overflow-hidden absolute"
            aria-hidden="true"
            data-testid="simulate-message-btn"
          >
            Simulate Incoming Message
          </button>
        </main>
      </div>
    </div>
  );
}
