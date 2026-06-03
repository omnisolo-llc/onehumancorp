"use client";

import React, { useState, useEffect } from "react";
import { UnifiedMessage } from "../../../proto/hub";

export default function UnifiedInboxPage() {
    const [messages, setMessages] = useState<UnifiedMessage[]>([]);

    useEffect(() => {
        // Fetch drafted messages here (mock for now, as we don't have the API set up in UI yet)
        setMessages([
            {
                id: "1",
                channel: "ig",
                channelId: "c1",
                customerId: "Maya",
                intentCategory: "custom order",
                confidenceScore: 0.6,
                status: "drafted",
                fromAgent: "The Ambassador",
                content: "I'd love to make a vegan cake for you! When do you need it by?",
                threadId: "t1",
                timestampUnix: 123456789,
                enrichedContextJson: "{}",
            } as any
        ]);
    }, []);

    return (
        <div style={{ maxWidth: "375px", margin: "0 auto", padding: "16px", border: "1px solid #ccc", minHeight: "100vh" }}>
            <h2>Mobile Inbox</h2>
            {messages.length === 0 ? <p>No drafts to review.</p> : null}
            {messages.map(msg => (
                <div key={msg.id} style={{ border: "1px solid #eee", padding: "8px", marginBottom: "8px", borderRadius: "8px", background: "rgba(255, 255, 255, 0.8)", backdropFilter: "blur(20px)" }}>
                    <div style={{ fontSize: "12px", color: "#666" }}>
                        Channel: {msg.channel} | Intent: {msg.intentCategory}
                    </div>
                    <p style={{ margin: "8px 0" }}>{msg.content}</p>
                    <div style={{ display: "flex", gap: "8px" }}>
                        <button style={{ flex: 1, padding: "8px", background: "#007AFF", color: "white", border: "none", borderRadius: "4px" }}>Approve & Send</button>
                        <button style={{ flex: 1, padding: "8px", background: "#f0f0f0", border: "none", borderRadius: "4px" }}>Edit</button>
                    </div>
                </div>
            ))}
        </div>
    );
}
