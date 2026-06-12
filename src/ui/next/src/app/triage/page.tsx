"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";
import { ActionCard } from "../components/ActionCard";

type TriageItem = {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload?: any;
  proposed_action?: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
};

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function TriagePage() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    loadItems();
  }, []);

  async function loadItems() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/agent-feed?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load agent feed items from the database");
      const data = await res.json();
      const rows = Array.isArray(data?.items) ? data.items : [];
      setItems(rows);
    } catch (e: any) {
      setError(e?.message || "Failed to load agent feed items");
    } finally {
      setLoading(false);
    }
  }

  const activeCount = items.length;
  const urgentCount = items.filter(item => ["urgent", "high"].includes((item.context_payload?.priority || "").toLowerCase())).length;

  async function handleDecision(id: string, approved: boolean) {
    try {
      setActionStatus(approved ? "Approving..." : "Dismissing...");
      const res = await fetch(`/api/agent-feed/${id}/state`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ state: approved ? "APPROVED" : "DISMISSED" })
      });
      if (!res.ok) throw new Error("Failed to update action");

      setActionStatus(approved ? "Approved!" : "Dismissed.");

      // Optimistic UI update
      const newItems = items.filter(i => i.id !== id);
      setItems(newItems);

      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    }
  }

  return (
    <AppShell
      title="Work Triage"
      subtitle="AI-prioritized inbox and action center."
      statusItems={[
        { label: "Active", value: String(activeCount), tone: activeCount > 0 ? "warn" : "good" },
        { label: "Urgent", value: String(urgentCount), tone: urgentCount > 0 ? "bad" : "neutral" },
      ]}
    >
      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

      <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Unified Agent Feed</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
      </div>

      <div className="flex flex-col gap-4">
        {error && <div className="app-empty">{error}</div>}
        {!error && items.length === 0 ? (
          <div className="app-empty">{loading ? "Loading triage items..." : "No items need your attention right now. Great job!"}</div>
        ) : items.map((item) => (
          <ActionCard
            key={item.id}
            id={item.id}
            eventSource={item.event_source}
            priority={item.context_payload?.priority || "Normal"}
            context={item.context_payload?.context}
            proposedAction={item.proposed_action}
            createdAt={item.created_at || Date.now().toString()}
            onDecision={handleDecision}
          />
        ))}
      </div>
    </AppShell>
  );
}
