"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";
import { ActionCard } from "../components/ActionCard";

type TriageItem = {
  id: string;
  tenant_id: string;
  customer_id?: string;
  source?: string;
  priority?: string;
  context?: string;
  action_type?: string;
  action_payload?: string;
  status?: string;
  created_at: string;
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
      const res = await fetch(`/api/ui/triage?tenant_id=${encodeURIComponent(tenantId())}`);
      if (!res.ok) throw new Error("Failed to load triage items from the database");
      const data = await res.json();
      const rows = Array.isArray(data) ? data : (Array.isArray(data?.items) ? data.items : []);
      setItems(rows);
    } catch (e: any) {
      setError(e?.message || "Failed to load triage items");
    } finally {
      setLoading(false);
    }
  }

  const activeCount = items.length;
  const urgentCount = items.filter(item => ["urgent", "high"].includes((item.priority || "").toLowerCase())).length;

  async function handleDecision(id: string, approved: boolean) {
    try {
      setActionStatus(approved ? "Approving..." : "Dismissing...");
      const res = await fetch(`/api/ui/triage/action?tenant_id=${encodeURIComponent(tenantId())}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved })
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
            eventSource={item.source || ""}
            priority={item.priority || "Normal"}
            context={item.context || ""}
            proposedAction={{ action_type: item.action_type || "", payload: item.action_payload }}
            createdAt={item.created_at || Date.now().toString()}
            onDecision={handleDecision}
          />
        ))}
      </div>
    </AppShell>
  );
}
