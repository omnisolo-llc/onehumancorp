import { useEffect, useState } from "react";
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

export function TriageFeed({ tenantId, initialItems }: { tenantId: string, initialItems?: TriageItem[] }) {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  useEffect(() => {
    if (initialItems && initialItems.length > 0) {
      setItems(initialItems);
      setLoading(false);
    } else {
      loadItems();
    }
  }, [tenantId, initialItems]);

  async function loadItems() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(`/api/agent-feed?tenant_id=${encodeURIComponent(tenantId)}`);
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

  if (loading) {
    return <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 text-gray-500">Loading triage items...</div>;
  }

  if (items.length === 0) {
    return null;
  }

  const proactiveItems = items.filter(item => item.event_source === "Proactive Context Agent");
  const regularItems = items.filter(item => item.event_source !== "Proactive Context Agent");

  return (
    <div className="mb-6">
      {proactiveItems.map((item) => (
        <ActionCard
          key={item.id}
          id={item.id}
          eventSource={item.event_source}
          priority={item.context_payload?.priority || "High"}
          context={item.context_payload?.context}
          proposedAction={item.proposed_action}
          createdAt={item.created_at || Date.now().toString()}
          onDecision={handleDecision}
        />
      ))}

      <div className="mb-4 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Unified Agent Feed</h2>
        <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts across all channels.</p>
      </div>

      {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}

      {error && <div className="app-empty">{error}</div>}

      <div className="flex flex-col gap-4">
        {!error && regularItems.map((item) => (
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
    </div>
  );
}
