"use client";

import { useEffect, useRef, useState } from "react";
import { AppShell } from "../components/AppShell";
import { SyncManager } from "../../lib/sync/SyncManager";
import { getActions } from "../utils/offlineQueue";
import { subscribeOnboardingInvalidation } from "../onboarding/draftSession";
import { ActionCard, type TriageItem } from "./components/ActionCard";

function tenantId() {
  if (typeof window === "undefined") return "default";
  const urlParams = new URLSearchParams(window.location.search);
  const urlTenant = urlParams.get("tenant_id");
  if (urlTenant) return urlTenant;
  return (
    localStorage.getItem("business_display_name") ||
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
  if (s.includes("whatsapp")) return "💬";
  if (s.includes("email")) return "📧";
  if (s.includes("booking") || s.includes("calendar")) return "📅";
  if (s.includes("payment") || s.includes("stripe")) return "💳";
  if (s.includes("alert") || s.includes("inventory")) return "⚠️";
  return "✉️";
};

export default function TriagePage() {
  const decisionInFlight = useRef(false);
  const sessionEpoch = useRef(0);
  const sessionRetired = useRef(false);
  const [items, setItems] = useState<TriageItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");
  const [processingId, setProcessingId] = useState<string | null>(null);
  const [isOffline, setIsOffline] = useState(false);
  const [offlineActionsCount, setOfflineActionsCount] = useState(0);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editValue, setEditValue] = useState<string>("");
  const [selectedItemId, setSelectedItemId] = useState<string | null>(null);

  useEffect(() => {
    const unsubscribe = subscribeOnboardingInvalidation(() => {
      ++sessionEpoch.current;
      sessionRetired.current = true;
      decisionInFlight.current = false;
      setItems([]); setEditingId(null); setEditValue(''); setSelectedItemId(null);
      setProcessingId(null); setActionStatus(''); setOfflineActionsCount(0); setLoading(false);
      setError('Your session changed. Reload to review work for the current account.');
    });
    loadItems();

    const updateOfflineCount = async () => {
      if (sessionRetired.current) return;
      const generation = sessionEpoch.current;
      try {
        const actions = await getActions();
        if (generation !== sessionEpoch.current) return;
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
    window.addEventListener('omnisolo_queue_updated', handleQueueUpdated);

    return () => {
      ++sessionEpoch.current;
      unsubscribe();
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
      window.removeEventListener('omnisolo_queue_updated', handleQueueUpdated);
    };
  }, []);

  async function loadItems() {
    const generation = sessionEpoch.current;
    setLoading(true);
    setError("");
    try {
      const res = await fetch(
        `/api/v1/triage/pending?tenant_id=${encodeURIComponent(tenantId())}`,
      );
      if (generation !== sessionEpoch.current) return;
      if (res.status === 401) {
        setItems([]);
        return;
      }
      if (!res.ok)
        throw new Error("Triage items temporarily unavailable");
      const data = await res.json();
      if (generation !== sessionEpoch.current) return;
      const rows = Array.isArray(data)
        ? data
        : Array.isArray(data?.items)
          ? data.items
          : [];
      setItems(rows);
    } catch (e: unknown) {
      if (generation !== sessionEpoch.current) return;
      const msg = e instanceof Error ? e.message : "";
      setError(msg && !/failed to load/i.test(msg) ? msg : "Triage items temporarily unavailable");
    } finally {
      if (generation === sessionEpoch.current) setLoading(false);
    }
  }

  const activeCount = items.length;
  const urgentCount = items.filter((item) =>
    ["urgent", "high"].includes((item.priority || "").toLowerCase()),
  ).length;

  async function handleDecision(id: string, approved: boolean, edited_payload?: string) {
    if (decisionInFlight.current || sessionRetired.current) return;
    const generation = sessionEpoch.current;
    const item = items.find(item => item.id === id);
    if (!item) return;
    decisionInFlight.current = true;
    setProcessingId(id);
    try {
      if (isOffline) {
        await SyncManager.getInstance().enqueue({
          id: crypto.randomUUID ? crypto.randomUUID() : Date.now().toString(),
          type: 'triage_action',
          payload: { triage_item_id: id, approved, edited_payload },
          timestamp: Date.now(),
        });
        if (generation !== sessionEpoch.current) return;
        setOfflineActionsCount(count => count + 1);
        setActionStatus("Decision queued offline. Approval or dismissal is not yet recorded.");
      } else {
        setActionStatus("Waiting for the recorded decision. No execution or delivery is confirmed.");
        const res = await fetch(
          `/api/v1/triage/action?tenant_id=${encodeURIComponent(tenantId())}`,
          {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ triage_item_id: id, approved, edited_payload }),
          },
        );
        const receipt: unknown = await res.json().catch(() => null);
        if (generation !== sessionEpoch.current) return;
        const record = (value: unknown): Record<string, unknown> | null => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : null;
        const result = record(receipt), stored = record(result?.item);
        if (res.status !== 200 || result?.success !== true || result.decision_recorded !== true || result.error != null
          || !item.tenant_id || stored?.id !== item.id || stored.tenant_id !== item.tenant_id
          || stored.lifecycle_state !== (approved ? 'APPROVED' : 'DISMISSED')
          || (edited_payload !== undefined && stored.edited_payload !== edited_payload)) {
          throw new Error("Outcome unconfirmed. Your card and draft are retained. Check recorded decisions before retrying.");
        }
        setActionStatus(approved
          ? "Approval recorded. Execution or delivery is not verified by this decision."
          : "Dismissal recorded.");
      }
      setItems(previous => previous.filter(item => item.id !== id));
      setEditingId(null);
    } catch {
      if (generation !== sessionEpoch.current) return;
      setActionStatus(isOffline
        ? "Decision was not queued. Your card and draft are retained."
        : "Outcome unconfirmed. Your card and draft are retained. Check recorded decisions before retrying.");
    } finally {
      if (generation === sessionEpoch.current) {
        setProcessingId(null);
        decisionInFlight.current = false;
      }
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
        <div id="action-status" className="mb-4 app-badge neutral" role="status">
          {actionStatus}
        </div>
      )}

      <div className="flex flex-col gap-4 w-full max-w-[375px] mx-auto pb-20">
        {error && <div className="app-empty">{error}</div>}
        {loading ? (
          <div className="p-6 space-y-4">
            <div className="h-20 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-full"></div>
            <div className="h-20 bg-gray-200 dark:bg-gray-700 rounded animate-pulse w-full"></div>
          </div>
        ) : !error && items.length === 0 ? (
          <div className="app-empty flex flex-col items-center justify-center py-16 px-4 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[16px] border border-white/40 dark:border-white/10" data-testid="triage-feed-empty">
            <div className="text-5xl mb-6">✨</div>
            <div className="text-xl font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
              No pending triage items are recorded.
            </div>
            <div className="text-[15px] text-gray-500 dark:text-gray-400 text-center max-w-[280px]">
              This view shows recorded pending items. Queued decisions still need to sync.
            </div>
          </div>
        ) : (
          items.map((item) => (
            <ActionCard
              key={item.id}
              item={item}
              isSelected={selectedItemId === item.id}
              isProcessing={processingId === item.id}
              editingId={editingId}
              editValue={editValue}
              getSourceIcon={getSourceIcon}
              badgeTone={badgeTone}
              onSelect={(id) => {
                setSelectedItemId(id);
                setEditingId(null);
              }}
              onDecision={handleDecision}
              onEditChange={setEditValue}
              onReview={() => {
                setEditingId(item.id);
                setEditValue(item.action_payload || "");
              }}
              onCancelEdit={() => {
                setEditingId(null);
                setEditValue("");
              }}
            />
          ))
        )}
      </div>
    </AppShell>
  );
}
