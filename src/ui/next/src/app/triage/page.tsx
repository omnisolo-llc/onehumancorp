"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";
import { ActionCard } from "../components/ActionCard";

type TriageItem = {
  id: string;
  tenant_id: string;
  source?: string;
  event_source?: string;
  priority?: string;
  context?: string;
  context_payload?: any;
  action_type?: string;
  action_payload?: string;
  proposed_action?: any;
  status?: string;
  lifecycle_state?: string;
  created_at: string;
  updated_at?: string;
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
  const [processingIds, setProcessingIds] = useState<Set<string>>(new Set());

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
  const urgentCount = items.filter(item => ["urgent", "high"].includes((item.priority || item.context_payload?.priority || "").toLowerCase())).length;

  async function handleDecision(id: string, approved: boolean) {
    if (processingIds.has(id)) return;

    try {
      setProcessingIds(prev => new Set(prev).add(id));
      setActionStatus(approved ? "Approving..." : "Dismissing...");
      const res = await fetch(`/api/ui/triage/action?tenant_id=${encodeURIComponent(tenantId())}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved })
      });
      if (!res.ok) throw new Error("Failed to update action");

      setActionStatus(approved ? "Approved!" : "Dismissed.");

      // Optimistic UI update - animate card out of feed
      setItems(prevItems => prevItems.filter(i => i.id !== id));

      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    } finally {
      setProcessingIds(prev => {
        const next = new Set(prev);
        next.delete(id);
        return next;
      });
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
      <div className="max-w-[600px] mx-auto w-full">
        {actionStatus && <div className="mb-4 app-badge good w-full justify-center" role="status">{actionStatus}</div>}

        <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 text-center">
          <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Unified Agent Feed</h2>
          <p className="text-gray-600 dark:text-gray-400">Review AI-prepared actions and reply drafts.</p>
        </div>

        <div className="flex flex-col w-full" id="triage-feed">
          {error && <div className="app-empty w-full text-center">{error}</div>}

          {!error && items.length === 0 ? (
            <div className="app-empty w-full text-center glassmorphism rounded-[16px] py-12">
              {loading ? (
                <div className="animate-pulse">Loading triage items...</div>
              ) : (
                <div className="flex flex-col items-center gap-2">
                  <span className="text-4xl">🎉</span>
                  <p className="text-[#1D1D1F] dark:text-[#F5F5F7] font-medium">Inbox Zero!</p>
                  <p className="text-sm text-gray-500">No items need your attention right now.</p>
                </div>
              )}
            </div>
          ) : (
            items.map((item) => (
              <ActionCard
                key={item.id}
                id={item.id}
                agentType={item.source || item.event_source || "Unknown Source"}
                context={item.context || item.context_payload?.context || "No context provided"}
                draftContent={item.action_payload || item.proposed_action?.payload || item.proposed_action?.message}
                priority={item.priority || item.context_payload?.priority || "Normal"}
                onApprove={(id) => handleDecision(id, true)}
                onDismiss={(id) => handleDecision(id, false)}
                isActionInProgress={processingIds.has(item.id)}
              />
            ))
          )}
        </div>
      </div>
    </AppShell>
  );
}
