
"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";
import { WorkTriageFeed } from "../components/WorkTriageFeed";
import { SyncManager } from "../../lib/sync/SyncManager";
import { getActions } from "../utils/offlineQueue";


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
  return (
    localStorage.getItem("tenant_id") ||
    localStorage.getItem("tenant") ||
    "default"
  );
}

function badgeTone(priority?: string) {
  const normalized = (priority || "").toLowerCase();
  if (["urgent", "high"].includes(normalized)) return "bad";
  if (["action needed", "medium"].includes(normalized)) return "warn";
  if (["fyi", "low"].includes(normalized)) return "good";
  return "neutral";
}

const getSourceIcon = (source: string) => {
  const s = source.toLowerCase();
  if (s.includes("instagram")) return "📸";
  if (s.includes("email")) return "📧";
  if (s.includes("booking") || s.includes("calendar")) return "📅";
  if (s.includes("payment") || s.includes("stripe")) return "💳";
  if (s.includes("alert") || s.includes("inventory")) return "⚠️";
  return "✉️";
};

export default function TriagePage() {
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");
  const [processingId, setProcessingId] = useState<string | null>(null);
  const [isOffline, setIsOffline] = useState(false);
  const [offlineActionsCount, setOfflineActionsCount] = useState(0);


  useEffect(() => {
    loadItems();

    const updateOfflineCount = async () => {
      try {
        const actions = await getActions();
        setOfflineActionsCount(actions.length);
      } catch (err) {
        console.warn("Failed to fetch offline actions count:", err);
      }
    };
    updateOfflineCount();

    setIsOffline(!navigator.onLine);

    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    const handleQueueUpdated = () => updateOfflineCount();
    window.addEventListener('ohc_queue_updated', handleQueueUpdated);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
      window.removeEventListener('ohc_queue_updated', handleQueueUpdated);
    };
  }, []);


  async function loadItems() {
    setLoading(true);
    setError("");
    try {
      const res = await fetch(
        `/api/triage/pending?tenant_id=${encodeURIComponent(tenantId())}`,
      );
      if (!res.ok)
        throw new Error("Failed to load triage items from the database");
      const data = await res.json();
      const rows = Array.isArray(data)
        ? data
        : Array.isArray(data?.items)
          ? data.items
          : [];
      setItems(rows);
    } catch (e: any) {
      setError(e?.message || "Failed to load triage items");
    } finally {
      setLoading(false);
    }
  }

  const activeCount = items.length;
  const urgentCount = items.filter((item) =>
    ["urgent", "high"].includes((item.priority || "").toLowerCase()),
  ).length;

  async function handleDecision(id: string, approved: boolean) {
    if (isOffline) {
      await SyncManager.getInstance().enqueue({
        id: crypto.randomUUID ? crypto.randomUUID() : Date.now().toString(),
        type: 'triage_action',
        payload: { triage_item_id: id, approved },
        timestamp: Date.now()
      });
      const newItems = items.filter((i) => i.id !== id);
      setItems(newItems);
      setActionStatus(approved ? "Approved offline." : "Dismissed offline.");
      setTimeout(() => setActionStatus(""), 3000);
      return;
    }

    try {
      setProcessingId(id);
      setActionStatus(approved ? "Approving..." : "Dismissing...");
      const res = await fetch(
        `/api/triage/action?tenant_id=${encodeURIComponent(tenantId())}`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ triage_item_id: id, approved }),
        },
      );
      if (!res.ok) throw new Error("Failed to update action");

      setActionStatus(approved ? "Approved!" : "Dismissed.");

      // Optimistic UI update
      const newItems = items.filter((i) => i.id !== id);
      setItems(newItems);

      setTimeout(() => setActionStatus(""), 3000);
    } catch (e) {
      console.error(e);
      setActionStatus("Error updating action.");
    } finally {
      setProcessingId(null);
    }
  }

  return (
    <AppShell
      title="Work Triage"
      subtitle="AI-prioritized inbox and action center."
      statusItems={[
        {
          label: "Active",
          value: String(activeCount),
          tone: activeCount > 0 ? "warn" : "good",
        },
        {
          label: "Urgent",
          value: String(urgentCount),
          tone: urgentCount > 0 ? "bad" : "neutral",
        },
      ]}
    >
      {isOffline && (
        <div className="mb-4 w-full p-2 glassmorphism rounded-[8px] bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
          <span>📡</span> You are offline. Actions will sync when online.
        </div>
      )}
      {offlineActionsCount > 0 && (
        <div className="mb-4 w-full p-2 glassmorphism rounded-[8px] bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
          <span>🔄</span> Pending Sync ({offlineActionsCount})
        </div>
      )}
      {actionStatus && (
        <div id="action-status" className="mb-4 app-badge good" role="status">
          {actionStatus}
        </div>
      )}


      <div className="flex flex-col gap-4 w-full max-w-full pb-20">
        <WorkTriageFeed
          items={items}
          loading={loading}
          error={error}
          onDecision={handleDecision}
        />
      </div>
    </AppShell>

  );
}
