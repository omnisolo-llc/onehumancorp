"use client";

import { useEffect, useState } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { enqueueAction, getActions, removeAction } from "../utils/offlineQueue";

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

type AgentFeedItem = {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload: any;
  proposed_action: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
};

type ApprovalsResponse = {
  pending_approvals: AgentFeedItem[];
  next_cursor?: string | null;
};

type OHCLedgerEntry = {
  id: string;
  tenant_id: string;
  event_type: string;
  department: string;
  payload: any;
  created_at: string;
};

type LedgerResponse = {
  entries: OHCLedgerEntry[];
};

type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
  payload: any;
};

export function UnifiedAgentFeed({ initialData }: { initialData?: any }) {
  const [triageItems, setTriageItems] = useState<TriageItem[]>([]);
  const [triageLoading, setTriageLoading] = useState(true);
  const [triageError, setTriageError] = useState("");

  const [items, setItems] = useState<AgentFeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">("proposals");
  const [activities, setActivities] = useState<OHCLedgerEntry[]>([]);
  const [activityLoading, setActivityLoading] = useState(false);
  const [isOffline, setIsOffline] = useState(false);
  const [offlineActionsCount, setOfflineActionsCount] = useState(0);
  const [queuedActionIds, setQueuedActionIds] = useState<Set<string>>(new Set());

  const tenantId = () => {
    if (typeof window === "undefined") return "default";
    return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
  };

  useEffect(() => {
    const handleVoiceCommandProcessed = (event: CustomEvent) => {
      // Wait a moment for backend DB write consistency, then reload the feed completely.
      setTimeout(() => {
        window.location.reload();
      }, 500);
    };
    window.addEventListener('voice-command-processed', handleVoiceCommandProcessed as EventListener);
    return () => window.removeEventListener('voice-command-processed', handleVoiceCommandProcessed as EventListener);
  }, []);

  useEffect(() => {
    const updateOfflineCount = async () => {
      try {
        const actions = await getActions();
        setOfflineActionsCount(actions.length);
        const ids = new Set<string>();
        actions.forEach(a => { if (a.payload && a.payload.id) ids.add(a.payload.id) });
        setQueuedActionIds(ids);
      } catch (err) {}
    };
    updateOfflineCount();

    setIsOffline(!navigator.onLine);

    const handleOnline = async () => {
      setIsOffline(false);
      // Sync queued offline actions
      try {
        const actions = await getActions();
        for (const action of actions) {
          if (action.type === 'approve_agent_feed') {
            await submitDecision(action.payload.id, action.payload.approved);
            await removeAction(action.id);
            setOfflineActionsCount(prev => Math.max(0, prev - 1));
            setQueuedActionIds(prev => {
              const newSet = new Set(prev); newSet.delete(action.payload.id); return newSet;
            });
          }
        }
      } catch (err) {
        console.error("Failed to sync offline actions", err);
      }
    };

    const handleOffline = () => {
      setIsOffline(true);
    };

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  useEffect(() => {
    let mounted = true;

    async function fetchTriage() {
      setTriageLoading(true);
      setTriageError("");
      try {
        const tenant = tenantId();
        const res = await fetch(`/api/ui/triage?tenant_id=${encodeURIComponent(tenant)}`);
        if (!res.ok) throw new Error("Failed to load triage items from the database");
        const data = await res.json();
        const rows = Array.isArray(data) ? data : (Array.isArray(data?.items) ? data.items : []);
        if (mounted) setTriageItems(rows);
      } catch (e: any) {
        if (mounted) setTriageError(e?.message || "Failed to load triage items");
      } finally {
        if (mounted) setTriageLoading(false);
      }
    }

    async function fetchAll() {
      fetchTriage();
      try {
        setLoading(true);
        setActivityLoading(true);
        const tenant = tenantId();

        let unifiedData = initialData;

        if (!unifiedData) {
          const unifiedRes = await fetch(`/api/agent-feed?tenant_id=${tenant}`, {
            headers: {
              "x-tenant-id": tenant,
              "x-user-id": "default",
            },
          });

          if (!unifiedRes.ok) {
            throw new Error("Failed to load agent feed");
          }

          unifiedData = await unifiedRes.json();
        }

        if (mounted) {
          if (unifiedData?.items) {
            setItems(unifiedData.items.filter((i: any) => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED"));

            // Map items for activity feed as well
            const mappedActivities = unifiedData.items.filter((i: any) => i.lifecycle_state === "APPROVED" || i.lifecycle_state === "DISMISSED").map((a: any) => ({
              id: a.id,
              tenant_id: a.tenant_id,
              event_type: a.lifecycle_state,
              department: a.event_source,
              payload: JSON.stringify({ original_payload: { description: a.proposed_action?.message || a.proposed_action?.action_type || a.event_source } }),
              created_at: a.updated_at || a.created_at || new Date().toISOString()
            }));
            setActivities(mappedActivities);
          }
        }

        if (mounted) {
          // Listen to SSE updates
          if (typeof EventSource === "undefined") return;
          const eventSource = new EventSource(`/api/agents/approvals/stream?tenant_id=${tenant}`);

          eventSource.onmessage = (event) => {
            try {
              const payload = JSON.parse(event.data);

              if (payload.event_type === "approval_request") {
                setItems((prev) => {
                  if (prev.find((a) => a.id === payload.data.id)) return prev;
                  return [payload.data, ...prev];
                });
              } else if (payload.event_type === "approval_decision") {
                setItems((prev) => prev.filter((a) => a.id !== payload.data.request_id));
                setActivities((prev) => {
                  const newActivity = {
                    id: crypto.randomUUID(),
                    tenant_id: tenant,
                    event_type: payload.data.status || 'APPROVED',
                    department: payload.data.department || 'general',
                    payload: payload.data,
                    created_at: new Date().toISOString(),
                  };
                  return [newActivity, ...prev];
                });
              }
            } catch (e) {
              console.error("Error parsing SSE event", e);
            }
          };

          eventSource.onerror = (error) => {
            console.error("SSE connection error", error);
            eventSource.close();
          };

          return () => {
            eventSource.close();
            mounted = false;
          };
        }
      } catch (err: any) {
        if (mounted) {
          setError(err.message || "Failed to load feed");
        }
        console.error("Failed to load activity", err);
      } finally {
        if (mounted) {
          setLoading(false);
          setActivityLoading(false);
        }
      }
    }

    const cleanup = fetchAll();
    return () => {
      mounted = false;
      cleanup.then((fn: any) => fn && typeof fn === 'function' && fn());
    };
  }, [initialData]);

  useEffect(() => {
    if (typeof EventSource === 'undefined') return;
    const events = new EventSource('/api/agents/events');
    events.onmessage = (event) => {
      try {
        const item = JSON.parse(event.data);
        if (!item?.id || !item?.description) return;

        // If it's a DRAFT or PENDING, add to proposals
        if (String(item.status || '').toUpperCase() === 'DRAFT' || String(item.status || '').toUpperCase() === 'PENDING') {
          setItems((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);
        } else {
          // It's an activity event (Approved, Rejected, etc.)
          setActivities((current) => {
            const mappedActivity = {
              id: item.id,
              tenant_id: item.tenant_id || "default",
              event_type: item.status,
              department: item.department,
              payload: typeof item.payload === 'object' ? JSON.stringify({ original_payload: item.payload }) : item.payload,
              created_at: new Date().toISOString()
            };
            return [mappedActivity, ...current.filter((existing) => existing.id !== item.id)];
          });
          // Also remove from approvals if it was there
          setItems((current) => current.filter((existing) => existing.id !== item.id));
        }
      } catch (err) {
        console.error('Failed to parse agent feed event:', err);
      }
    };
    events.onerror = () => events.close();
    return () => events.close();
  }, []);

  useEffect(() => {
    if (typeof EventSource === 'undefined') return;
    const events = new EventSource('/api/agents/events');
    events.onmessage = (event) => {
      try {
        const item = JSON.parse(event.data);
        if (!item?.id || !item?.description) return;

        // If it's a DRAFT or PENDING, add to proposals
        if (String(item.status || '').toUpperCase() === 'DRAFT' || String(item.status || '').toUpperCase() === 'PENDING') {
          setItems((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);
        } else {
          // It's an activity event (Approved, Rejected, etc.)
          setActivities((current) => {
            const mappedActivity = {
              id: item.id,
              tenant_id: item.tenant_id || "default",
              event_type: item.status,
              department: item.department,
              payload: typeof item.payload === 'object' ? JSON.stringify({ original_payload: item.payload }) : item.payload,
              created_at: new Date().toISOString()
            };
            return [mappedActivity, ...current.filter((existing) => existing.id !== item.id)];
          });
          // Also remove from approvals if it was there
          setItems((current) => current.filter((existing) => existing.id !== item.id));
        }
      } catch (err) {
        console.error('Failed to parse agent feed event:', err);
      }
    };
    events.onerror = () => events.close();
    return () => events.close();
  }, []);

  const handleTriageDecision = async (id: string, approved: boolean) => {
    try {
      const res = await fetch(`/api/ui/triage/action?tenant_id=${encodeURIComponent(tenantId())}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ triage_item_id: id, approved })
      });
      if (!res.ok) throw new Error("Failed to update action");

      // Optimistic UI update
      setTriageItems(prev => prev.filter(i => i.id !== id));
    } catch (e) {
      console.error(e);
    }
  };

  const badgeTone = (priority?: string) => {
    const p = (priority || "").toLowerCase();
    if (p === "high" || p === "urgent") return "bad";
    if (p === "low") return "neutral";
    return "warning";
  };

  const submitDecision = async (id: string, approved: boolean) => {
    const tenant = tenantId();
    const res = await fetch(`/api/agent-feed/${id}/state`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        "x-tenant-id": tenant,
        "x-user-id": "default",
      },
      body: JSON.stringify({ state: approved ? "APPROVED" : "DISMISSED" }),
    });

    if (!res.ok) {
      throw new Error("Failed to submit decision");
    }
  };

  const handleDecision = async (id: string, approved: boolean) => {
    if (isOffline) {
      // Enqueue offline action
      await enqueueAction({
        id: crypto.randomUUID(),
        type: 'approve_agent_feed',
        payload: { id, approved },
        timestamp: Date.now()
      });
      setOfflineActionsCount(prev => prev + 1);
      setQueuedActionIds(prev => new Set(prev).add(id));
      return;
    }

    // Optimistic UI update
    setItems(prev => prev.filter(app => app.id !== id));

    try {
      await submitDecision(id, approved);
    } catch (err: any) {
      // Revert optimistic update gracefully by refetching
      const tenant = tenantId();
      try {
        const refreshRes = await fetch(`/api/agent-feed?tenant_id=${tenant}`, {
            headers: { "x-tenant-id": tenant, "x-user-id": "default" }
        });
        if (refreshRes.ok) {
            const data: any = await refreshRes.json();
            if (data.items) {
               setItems(data.items.filter((i: any) => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED"));
            }
        }
      } catch (e) {
        console.error("Failed to restore state", e);
      }
      setError(err.message || "Action failed");
    }
  };



  if (error) {
    return (
      <div className="w-full mb-6 p-4 glassmorphism rounded-[16px] border border-[#FF3B30]/50 bg-[#FF3B30]/10 text-[#FF3B30] text-center">
        {error}
      </div>
    );
  }

  return (
    <section className="mb-6 w-full max-w-full overflow-hidden sm:max-w-none" aria-label="Unified Agent Feed">
      <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 hidden md:block">Action Center</h2>
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
      <div className="mb-4 flex items-center border-b border-gray-200 dark:border-gray-700">
        <button
          onClick={() => setActiveTab("proposals")}
          className={`flex-1 min-h-[44px] min-w-[44px] py-3 text-center text-sm font-semibold transition-all duration-200 ${
            activeTab === "proposals"
              ? "border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Proposals ({items.length})
        </button>
        <button
          onClick={() => setActiveTab("activity")}
          className={`flex-1 min-h-[44px] min-w-[44px] py-3 text-center text-sm font-semibold transition-all duration-200 ${
            activeTab === "activity"
              ? "border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Activity Feed
        </button>
      </div>

      <div className="flex flex-col gap-4">
        {activeTab === "proposals" && (
          <>
