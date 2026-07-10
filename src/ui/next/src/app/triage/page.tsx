"use client";

import React, { useState, useEffect } from "react";
import { useRouter } from "next/navigation";

export default function TriagePage() {
  const [items, setItems] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  const fetchTriageItems = async () => {
    try {
      const res = await fetch("/api/ui/triage");
      if (res.ok) {
        const data = await res.json();
        setItems(data.items || []);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTriageItems();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      await fetch(`/api/ui/triage/action/${id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ action_status: "APPROVED" }),
      });
      fetchTriageItems();
    } catch (e) {
      console.error(e);
    }
  };

  const handleSimulate = async () => {
    try {
      await fetch("/api/webhook/simulate_intake", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          customer_name: "Maya",
          channel: "Instagram DM",
          message: "Need a cake for next Tuesday",
        }),
      });
      // Wait for AI Job to process
      setTimeout(fetchTriageItems, 3000);
    } catch (e) {
      console.error(e);
    }
  };

  if (loading) return <div>Loading...</div>;

  return (
    <div style={{ maxWidth: 375, margin: "0 auto", padding: 16 }}>
      <h1>Work Triage</h1>
      <button onClick={handleSimulate} style={{ marginBottom: 16, padding: "8px 16px" }}>
        Simulate Inquiry
      </button>

      <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
        {items.map((item) => (
          <div
            key={item.id}
            style={{
              border: "1px solid #ccc",
              borderRadius: 8,
              padding: 16,
              background: "#fff",
            }}
          >
            <h3 style={{ margin: "0 0 8px 0" }}>New Lead: {item.customer_name}</h3>
            <p style={{ margin: "0 0 16px 0", color: "#666" }}>{item.context}</p>
            <div style={{ background: "#f5f5f5", padding: 8, borderRadius: 4, marginBottom: 16 }}>
              <p style={{ margin: "0 0 8px 0" }}>
                <strong>Agent Proposal:</strong>
              </p>
              <p style={{ margin: 0 }}>Drafted reply: "{item.draft_reply}"</p>
              {item.deposit_amount && (
                <p style={{ margin: "4px 0 0 0" }}>& ${item.deposit_amount} deposit link</p>
              )}
            </div>
            <button
              onClick={() => handleApprove(item.id)}
              style={{
                width: "100%",
                minHeight: 44,
                background: "#007AFF",
                color: "white",
                border: "none",
                borderRadius: 8,
                fontSize: 16,
                fontWeight: "bold",
                cursor: "pointer",
              }}
            >
              Approve & Send
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
