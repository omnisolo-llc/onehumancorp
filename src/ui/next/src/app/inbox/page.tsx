"use client";

import React, { useState, useEffect } from "react";

// Mock data structures
type Message = {
  id: string;
  sender: "business" | "customer";
  text: string;
  timestamp: string;
};

type Conversation = {
  id: string;
  customerName: string;
  platform: "Facebook" | "Instagram";
  avatar: string;
  lastMessage: string;
  unread: boolean;
  messages: Message[];
};

const INITIAL_MOCK_CONVERSATIONS: Conversation[] = [
  {
    id: "conv_1",
    customerName: "Maya's Bakery Fan",
    platform: "Instagram",
    avatar: "MB",
    lastMessage: "Do you make custom vegan cakes?",
    unread: true,
    messages: [
      { id: "m1", sender: "customer", text: "Hi there!", timestamp: "10:00 AM" },
      { id: "m2", sender: "customer", text: "Do you make custom vegan cakes?", timestamp: "10:02 AM" },
    ],
  },
  {
    id: "conv_2",
    customerName: "John Doe",
    platform: "Facebook",
    avatar: "JD",
    lastMessage: "I need a quote for a leaky pipe.",
    unread: false,
    messages: [
      { id: "m1", sender: "customer", text: "Hello", timestamp: "Yesterday" },
      { id: "m2", sender: "business", text: "Hi John, how can I help you?", timestamp: "Yesterday" },
      { id: "m3", sender: "customer", text: "I need a quote for a leaky pipe.", timestamp: "Yesterday" },
    ],
  },
  {
    id: "conv_3",
    customerName: "Sarah Smith",
    platform: "Instagram",
    avatar: "SS",
    lastMessage: "Are you open on Sundays?",
    unread: true,
    messages: [
      { id: "m1", sender: "customer", text: "Are you open on Sundays?", timestamp: "9:15 AM" },
    ],
  }
];

export default function InboxPage() {
  const [isConnected, setIsConnected] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [activeConversationId, setActiveConversationId] = useState<string | null>(null);
  const [replyText, setReplyText] = useState("");

  const handleConnect = () => {
    setIsConnecting(true);
    // Simulate 1-click OAuth and webhook setup
    setTimeout(() => {
      setIsConnecting(false);
      setIsConnected(true);
      setConversations(INITIAL_MOCK_CONVERSATIONS);
      setActiveConversationId(INITIAL_MOCK_CONVERSATIONS[0].id);
    }, 2000);
  };

  const activeConversation = conversations.find((c) => c.id === activeConversationId);

  const handleSendReply = () => {
    if (!replyText.trim() || !activeConversationId) return;

    setConversations((prev) =>
      prev.map((conv) => {
        if (conv.id === activeConversationId) {
          return {
            ...conv,
            lastMessage: replyText,
            messages: [
              ...conv.messages,
              {
                id: `m_${Date.now()}`,
                sender: "business",
                text: replyText,
                timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
              },
            ],
          };
        }
        return conv;
      })
    );
    setReplyText("");
  };

  const handleConversationClick = (id: string) => {
    setActiveConversationId(id);
    setConversations((prev) =>
      prev.map((conv) => (conv.id === id ? { ...conv, unread: false } : conv))
    );
  };

  // --- Styles ---
  const containerStyle: React.CSSProperties = {
    flex: 1,
    display: "flex",
    flexDirection: "column",
    height: "100vh",
    backgroundColor: "#F5F5F7",
    fontFamily: "'Inter', sans-serif",
  };

  const headerStyle: React.CSSProperties = {
    padding: "1rem 1.5rem",
    borderBottom: "1px solid rgba(255, 255, 255, 0.4)",
    background: "rgba(255, 255, 255, 0.65)",
    backdropFilter: "blur(30px) saturate(210%)",
    position: "sticky",
    top: 0,
    zIndex: 50,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
  };

  const glassCardStyle: React.CSSProperties = {
    background: "rgba(255, 255, 255, 0.65)",
    backdropFilter: "blur(30px) saturate(210%)",
    border: "1px solid rgba(255, 255, 255, 0.4)",
    borderRadius: "16px",
    boxShadow: "0 1px 2px rgba(0,0,0,0.05)",
  };

  if (!isConnected) {
    return (
      <div style={containerStyle}>
        <header style={headerStyle}>
          <h1 style={{ margin: 0, fontSize: "1.5rem", fontWeight: "bold", fontFamily: "'Outfit', sans-serif", color: "#1D1D1F", letterSpacing: "-0.02em" }}>
            Unified Inbox
          </h1>
        </header>
        <main style={{ flex: 1, display: "flex", alignItems: "center", justifyContent: "center", padding: "2rem" }}>
          <div style={{ ...glassCardStyle, padding: "3rem", textAlign: "center", maxWidth: "400px" }}>
            <div style={{ fontSize: "3rem", marginBottom: "1rem" }}>💬</div>
            <h2 style={{ fontSize: "1.25rem", fontWeight: 600, color: "#1D1D1F", marginBottom: "0.5rem" }}>
              Connect Social Media
            </h2>
            <p style={{ color: "#86868B", marginBottom: "2rem", fontSize: "0.95rem", lineHeight: 1.5 }}>
              Connect your Facebook and Instagram business accounts to manage all your customer messages in one unified inbox. We handle all the technical webhook setup automatically.
            </p>
            <button
              onClick={handleConnect}
              disabled={isConnecting}
              style={{
                backgroundColor: isConnecting ? "#A0C4FF" : "#0066FF",
                color: "white",
                border: "none",
                borderRadius: "8px",
                padding: "0.75rem 1.5rem",
                fontSize: "1rem",
                fontWeight: 500,
                cursor: isConnecting ? "not-allowed" : "pointer",
                width: "100%",
                transition: "background-color 0.2s",
              }}
            >
              {isConnecting ? "Connecting & Syncing Messages..." : "Connect Facebook & Instagram"}
            </button>
          </div>
        </main>
        <style dangerouslySetInnerHTML={{ __html: `
          @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        `}} />
      </div>
    );
  }

  return (
    <div style={containerStyle}>
      <header style={headerStyle}>
        <h1 style={{ margin: 0, fontSize: "1.5rem", fontWeight: "bold", fontFamily: "'Outfit', sans-serif", color: "#1D1D1F", letterSpacing: "-0.02em" }}>
          Unified Inbox
        </h1>
        <div style={{ display: "flex", alignItems: "center", gap: "1rem" }}>
          <span style={{ fontSize: "0.875rem", color: "#34C759", fontWeight: 500, display: "flex", alignItems: "center", gap: "0.5rem" }}>
            <span style={{ width: "8px", height: "8px", borderRadius: "50%", backgroundColor: "#34C759", display: "inline-block" }}></span>
            Connected
          </span>
          <div style={{ width: "32px", height: "32px", borderRadius: "50%", backgroundColor: "#E5E5EA", display: "flex", alignItems: "center", justifyContent: "center", fontSize: "0.875rem", fontWeight: "bold", color: "#86868B" }}>
            AC
          </div>
        </div>
      </header>

      <main style={{ flex: 1, display: "flex", overflow: "hidden" }}>
        {/* Sidebar List */}
        <div style={{ width: "320px", borderRight: "1px solid rgba(0,0,0,0.05)", display: "flex", flexDirection: "column", background: "rgba(255,255,255,0.4)" }}>
          <div style={{ padding: "1rem", borderBottom: "1px solid rgba(0,0,0,0.05)" }}>
            <input
              type="text"
              placeholder="Search messages..."
              style={{
                width: "100%",
                padding: "0.5rem 1rem",
                borderRadius: "8px",
                border: "1px solid rgba(0,0,0,0.1)",
                backgroundColor: "rgba(255,255,255,0.8)",
                fontSize: "0.9rem",
                outline: "none"
              }}
            />
          </div>
          <div style={{ flex: 1, overflowY: "auto" }}>
            {conversations.map((conv) => (
              <div
                key={conv.id}
                onClick={() => handleConversationClick(conv.id)}
                style={{
                  padding: "1rem",
                  borderBottom: "1px solid rgba(0,0,0,0.05)",
                  cursor: "pointer",
                  backgroundColor: activeConversationId === conv.id ? "rgba(0, 102, 255, 0.05)" : "transparent",
                  display: "flex",
                  gap: "0.75rem",
                  transition: "background-color 0.2s"
                }}
              >
                <div style={{ width: "40px", height: "40px", borderRadius: "50%", backgroundColor: "#E5E5EA", flexShrink: 0, display: "flex", alignItems: "center", justifyContent: "center", fontWeight: "bold", color: "#1D1D1F" }}>
                  {conv.avatar}
                </div>
                <div style={{ flex: 1, minWidth: 0 }}>
                  <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "0.25rem" }}>
                    <span style={{ fontWeight: conv.unread ? 600 : 500, color: "#1D1D1F", whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>
                      {conv.customerName}
                    </span>
                    <span style={{ fontSize: "0.75rem", color: "#86868B", fontWeight: 500 }}>
                      {conv.platform === 'Facebook' ? 'FB' : 'IG'}
                    </span>
                  </div>
                  <p style={{ margin: 0, fontSize: "0.875rem", color: conv.unread ? "#1D1D1F" : "#86868B", fontWeight: conv.unread ? 500 : 400, whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>
                    {conv.lastMessage}
                  </p>
                </div>
                {conv.unread && (
                  <div style={{ width: "8px", height: "8px", borderRadius: "50%", backgroundColor: "#0066FF", alignSelf: "center", flexShrink: 0 }}></div>
                )}
              </div>
            ))}
          </div>
        </div>

        {/* Main Chat Area */}
        <div style={{ flex: 1, display: "flex", flexDirection: "column", background: "white" }}>
          {activeConversation ? (
            <>
              {/* Chat Header */}
              <div style={{ padding: "1rem 1.5rem", borderBottom: "1px solid rgba(0,0,0,0.05)", display: "flex", alignItems: "center", gap: "1rem" }}>
                <div style={{ width: "40px", height: "40px", borderRadius: "50%", backgroundColor: "#E5E5EA", display: "flex", alignItems: "center", justifyContent: "center", fontWeight: "bold", color: "#1D1D1F" }}>
                  {activeConversation.avatar}
                </div>
                <div>
                  <h3 style={{ margin: 0, fontSize: "1.1rem", fontWeight: 600, color: "#1D1D1F" }}>{activeConversation.customerName}</h3>
                  <span style={{ fontSize: "0.8rem", color: "#86868B" }}>via {activeConversation.platform}</span>
                </div>
              </div>

              {/* Message List */}
              <div style={{ flex: 1, overflowY: "auto", padding: "1.5rem", display: "flex", flexDirection: "column", gap: "1rem" }}>
                {activeConversation.messages.map((msg) => {
                  const isBusiness = msg.sender === "business";
                  return (
                    <div key={msg.id} style={{ alignSelf: isBusiness ? "flex-end" : "flex-start", maxWidth: "70%" }}>
                      <div
                        style={{
                          backgroundColor: isBusiness ? "#0066FF" : "#F2F2F7",
                          color: isBusiness ? "white" : "#1D1D1F",
                          padding: "0.75rem 1rem",
                          borderRadius: isBusiness ? "16px 16px 0 16px" : "16px 16px 16px 0",
                          fontSize: "0.95rem",
                          lineHeight: 1.4,
                        }}
                      >
                        {msg.text}
                      </div>
                      <div style={{ fontSize: "0.75rem", color: "#86868B", marginTop: "0.25rem", textAlign: isBusiness ? "right" : "left" }}>
                        {msg.timestamp}
                      </div>
                    </div>
                  );
                })}
              </div>

              {/* Input Area */}
              <div style={{ padding: "1rem 1.5rem", borderTop: "1px solid rgba(0,0,0,0.05)", display: "flex", gap: "0.5rem", alignItems: "center" }}>
                <input
                  type="text"
                  value={replyText}
                  onChange={(e) => setReplyText(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === "Enter") handleSendReply();
                  }}
                  placeholder="Type a reply..."
                  style={{
                    flex: 1,
                    padding: "0.75rem 1rem",
                    borderRadius: "24px",
                    border: "1px solid rgba(0,0,0,0.1)",
                    fontSize: "0.95rem",
                    outline: "none",
                  }}
                />
                <button
                  onClick={handleSendReply}
                  style={{
                    backgroundColor: "#0066FF",
                    color: "white",
                    border: "none",
                    borderRadius: "50%",
                    width: "40px",
                    height: "40px",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    cursor: "pointer",
                    flexShrink: 0,
                  }}
                >
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                    <line x1="22" y1="2" x2="11" y2="13"></line>
                    <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
                  </svg>
                </button>
              </div>
            </>
          ) : (
            <div style={{ flex: 1, display: "flex", alignItems: "center", justifyContent: "center", color: "#86868B" }}>
              Select a conversation to start chatting.
            </div>
          )}
        </div>
      </main>

      <style dangerouslySetInnerHTML={{ __html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
      `}} />
    </div>
  );
}
