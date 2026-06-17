"use client";

import { useEffect, useState } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { WorkTriageFeed } from "../components/WorkTriageFeed";
import { enqueueAction, getActions, removeAction } from "../utils/offlineQueue";
import { AmbassadorReplyCard } from './AmbassadorReplyCard';
import { InstagramDMCard } from './InstagramDMCard';


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
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editContent, setEditContent] = useState<string>("");

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
            let combinedItems = [...unifiedData.items];

            // Integrate Priority Tasks
            if (unifiedData.priority_tasks && Array.isArray(unifiedData.priority_tasks)) {
              combinedItems = [...combinedItems, ...unifiedData.priority_tasks.map((pt: any) => ({
                id: pt.id,
                tenant_id: pt.tenant_id || "default",
                event_source: "task",
                context_payload: { description: pt.description || pt.title },
                proposed_action: { message: "Task Pending", action_type: "complete_task" },
                lifecycle_state: pt.status === "PENDING" ? "PENDING_APPROVAL" : "DISMISSED",
                created_at: pt.created_at || new Date().toISOString(),
                updated_at: pt.updated_at || new Date().toISOString()
              }))];
            }

            // Integrate Triage Items (Messages)
            if (unifiedData.triage && Array.isArray(unifiedData.triage)) {
              combinedItems = [...combinedItems, ...unifiedData.triage.map((ti: any) => ({
                id: ti.id,
                tenant_id: ti.tenant_id || "default",
                event_source: "triage",
                context_payload: { description: ti.context || "Message requires attention" },
                proposed_action: { message: ti.action_payload || "Triage item", action_type: ti.action_type || "resolve" },
                lifecycle_state: ti.status === "RESOLVED" ? "DISMISSED" : "PENDING_APPROVAL",
                created_at: ti.created_at || new Date().toISOString(),
                updated_at: ti.created_at || new Date().toISOString()
              }))];
            }

            // Integrate Orders
            if (unifiedData.orders && Array.isArray(unifiedData.orders)) {
              combinedItems = [...combinedItems, ...unifiedData.orders.map((or: any) => ({
                id: or.id,
                tenant_id: or.tenant_id || "default",
                event_source: "order",
                context_payload: { description: `Order ${or.id} needs fulfillment` },
                proposed_action: { message: "Fulfill Order", action_type: "fulfill_order" },
                lifecycle_state: or.status === "pending" || or.status === "unfulfilled" ? "PENDING_APPROVAL" : "DISMISSED",
                created_at: or.created_at || new Date().toISOString(),
                updated_at: or.created_at || new Date().toISOString()
              }))];
            }

            // Sort by created_at desc
            combinedItems.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());

            setItems(combinedItems.filter((i: any) => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED"));

            // Map items for activity feed as well
            const mappedActivities = combinedItems.filter((i: any) => i.lifecycle_state === "APPROVED" || i.lifecycle_state === "DISMISSED").map((a: any) => ({
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
    let ws: WebSocket;
    let reconnectTimeout: NodeJS.Timeout;

    const connect = () => {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${window.location.host}/api/agent-feed/ws`;
        if (typeof process.env.VITEST !== 'undefined' || process.env.NODE_ENV === 'test') return;
        ws = new WebSocket(wsUrl);

      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (data.error) {
             console.error("Agent feed WS error:", data.error);
             return;
          }

          // Depending on your message structure from agent-feed:
          // The redis pub/sub payload is currently just the AgentFeedItem JSON.
          const item = data;

          if (!item?.id) return;

          // If it's PENDING_APPROVAL add to the feed
          if (String(item.lifecycle_state || '').toUpperCase() === 'PENDING_APPROVAL') {
            setItems((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);

            // Also map and remove from activities if it somehow got back to pending (unlikely)
            setActivities((current) => current.filter((existing) => existing.id !== item.id));

          } else if (String(item.lifecycle_state || '').toUpperCase() === 'APPROVED' || String(item.lifecycle_state || '').toUpperCase() === 'DISMISSED') {
            // It's an activity event (Approved, Rejected, etc.)
            setActivities((current) => {
              const mappedActivity = {
                id: item.id,
                tenant_id: item.tenant_id || "default",
                event_type: item.lifecycle_state,
                department: item.event_source || "system",
                payload: typeof item.proposed_action === 'object' ? JSON.stringify({ original_payload: item.proposed_action }) : item.proposed_action,
                created_at: new Date().toISOString()
              };
              return [mappedActivity, ...current.filter((existing) => existing.id !== item.id)];
            });
            // Also remove from approvals
            setItems((current) => current.filter((existing) => existing.id !== item.id));
          } else {
             // Fallback for legacy SSE structure matching
             if (String(item.status || '').toUpperCase() === 'DRAFT' || String(item.status || '').toUpperCase() === 'PENDING') {
                setItems((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);
              } else if (item.status) {
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
                setItems((current) => current.filter((existing) => existing.id !== item.id));
              }
          }
        } catch (err) {
          console.error('Failed to parse websocket feed event:', err);
        }
      };

      ws.onclose = () => {
        // Attempt to reconnect
        reconnectTimeout = setTimeout(connect, 3000);
      };

      ws.onerror = (err) => {
        console.error("Websocket error:", err);
      };
    };

    connect();

    return () => {
      clearTimeout(reconnectTimeout);
      if (ws) {
        ws.onclose = null; // Prevent reconnection on unmount
        ws.close();
      }
    };
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

  const submitDecision = async (id: string, approved: boolean, modified_content?: string) => {
    const tenant = tenantId();
    const res = await fetch(`/api/agent-feed/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        "x-tenant-id": tenant,
        "x-user-id": "default",
      },
      body: JSON.stringify({ state: approved ? "APPROVED" : "DISMISSED", modified_content }),
    });

    if (!res.ok) {
      throw new Error("Failed to submit decision");
    }
  };

  const handleDecision = async (id: string, approved: boolean, modified_content?: string) => {
    if (isOffline) {
      // Enqueue offline action
      await enqueueAction({
        id: crypto.randomUUID(),
        type: 'approve_agent_feed',
        payload: { id, approved, modified_content },
        timestamp: Date.now()
      });
      setOfflineActionsCount(prev => prev + 1);
      setQueuedActionIds(prev => new Set(prev).add(id));
      return;
    }

    // Optimistic UI update
    setItems(prev => prev.filter(app => app.id !== id));

    try {
      await submitDecision(id, approved, modified_content);
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
    <section className="mb-6 w-full w-full overflow-hidden" aria-label="Unified Agent Feed">
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

      <div className="flex flex-col gap-4 w-full">
        {activeTab === "proposals" && (
          <>
            <WorkTriageFeed
              items={triageItems}
              loading={triageLoading}
              error={triageError}
              onDecision={handleTriageDecision}
            />

            <div className="glassmorphism p-5 rounded-[16px]  shadow-sm flex flex-col gap-4">
              <div className="flex flex-col gap-1">
                <div className="flex justify-between items-start">
                  <span className="text-xs font-bold uppercase tracking-wider text-green-600 bg-green-100 dark:bg-green-900 dark:text-green-300 px-2 py-1 rounded">Action Needed</span>
                  <span className="text-xs text-gray-500 font-inter">Just now</span>
                </div>
                <h3 className="text-[17px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit mt-2 leading-tight">
                  Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed.
                </h3>
              </div>
            </div>

            <div className="glassmorphism p-5 rounded-[16px]  shadow-sm flex flex-col gap-4">
              <div className="flex flex-col gap-1">
                <div className="flex justify-between items-start">
                  <span className="text-xs font-bold uppercase tracking-wider text-[#0066FF] bg-[#0066FF]/10 dark:bg-[#3388FF]/20 dark:text-[#3388FF] px-2 py-1 rounded">Approval</span>
                  <span className="text-xs text-gray-500 font-inter">5 min ago</span>
                </div>
                <h3 className="text-[17px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit mt-2 leading-tight">
                  Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?
                </h3>
              </div>
              <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
                <button
                  className="flex-1 min-h-[44px] min-w-[44px] rounded-lg font-bold text-sm bg-green-500 hover:bg-green-600 text-white shadow-sm transition-transform active:scale-[0.98]"
                >
                  Approve
                </button>
                <button
                  className="flex-1 min-h-[44px] min-w-[44px] rounded-lg font-bold text-sm bg-gray-200 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-700 text-[#1D1D1F] dark:text-[#F5F5F7] transition-transform active:scale-[0.98]"
                >
                  Edit
                </button>
                <button
                  className="flex-1 min-h-[44px] min-w-[44px] rounded-lg font-bold text-sm bg-red-100 hover:bg-red-200 text-red-600 dark:bg-red-900/30 dark:hover:bg-red-900/50 dark:text-red-400 transition-transform active:scale-[0.98]"
                >
                  Deny
                </button>
              </div>
            </div>

            {(loading || triageLoading) && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Agent Proposals...
              </div>
            )}
            {!loading && !triageLoading && items.length === 0 && triageItems.length === 0 && (
              <div className="w-full flex flex-col items-center gap-6 p-6 glassmorphism rounded-[16px]  shadow-sm opacity-90 text-center">
                <div className="text-3xl mb-2">✨</div>
                <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 break-words">
                  Your agents are currently monitoring the business. While you're here, why not help us grow?
                </p>
                <div className="w-full max-w-md text-left">
                   <GrowthReferralWidget />
                </div>
              </div>
            )}
            {items.map((approval) => (
              <div
                key={approval.id}
                className="glassmorphism p-5 rounded-[16px]  shadow-sm flex flex-col gap-4"
              >
                <div className="flex flex-col gap-1">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-bold uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                      {approval.event_source.replace('_', ' ')}
                    </span>
                    {(approval.lifecycle_state === 'PENDING_APPROVAL') && (
                      <span className="text-xs font-bold uppercase tracking-wider text-red-600 bg-red-50 px-2 py-1 rounded-md">
                        Requires Review
                      </span>
                    )}
                    {queuedActionIds.has(approval.id) && (
                      <span className="text-xs font-bold uppercase tracking-wider text-yellow-600 bg-yellow-50 px-2 py-1 rounded-md shadow-sm border border-yellow-200" data-testid="queued-badge">
                        Queued
                      </span>
                    )}
                  </div>
                  <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1">
                    {(approval.context_payload?.description || approval.proposed_action?.message || approval.proposed_action?.action_type || approval.event_source)}
                  </h3>
                  {((approval.proposed_action || approval.context_payload)?.context || (approval.proposed_action || approval.context_payload)?.remaining_stock !== undefined || (approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" || (approval.proposed_action || approval.context_payload)?.feature_type === "social_post_draft" || (approval.proposed_action || approval.context_payload)?.feature_type === "ambassador_reply" || (approval.proposed_action || approval.context_payload)?.feature_type === "incident_resolution" || (approval.proposed_action || approval.context_payload)?.feature_type === "instagram_dm") && (
                    <div className="mt-2 flex flex-col gap-1 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
                      {(approval.proposed_action || approval.context_payload)?.feature_type === "incident_resolution" && (
                        <div className="mb-4 p-4 rounded-xl bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800/50 flex flex-col gap-3" data-testid="incident-resolution-card">
                          <div className="flex items-center gap-2 text-red-600 font-semibold text-sm">
                            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                            </svg>
                            CRITICAL INCIDENT
                          </div>
                          <p className="text-gray-700 dark:text-gray-300 text-sm break-words">
                            {(approval.proposed_action || approval.context_payload)?.description || 'An operational issue requires immediate attention.'}
                          </p>
                        </div>
                      )}
                      {(approval.proposed_action || approval.context_payload)?.feature_type === "instagram_dm" && <InstagramDMCard approval={approval} />}
                      {(approval.proposed_action || approval.context_payload)?.feature_type === "ambassador_reply" && <AmbassadorReplyCard approval={approval} />}
                      {(approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" && (
                        <div className="mb-4 p-4 rounded-xl glassmorphism  flex flex-col gap-3" data-testid="quote-draft-card">
                          <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                            </svg>
                            Draft Quote: {(approval.proposed_action || approval.context_payload).service || 'Plumbing Fix'} for Customer
                          </div>
                          <div className="text-xs text-[#0066FF] dark:text-blue-400 font-medium break-words">
                            {(approval.proposed_action || approval.context_payload).customer_inquiry}
                          </div>
                          <div className="glassmorphism dark:bg-gray-800 p-3 rounded-lg  relative mt-2">
                            <div className="text-[10px] uppercase font-bold text-gray-500 mb-2">AI Proposed Quote</div>
                            <div className="space-y-2">
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500">Calculated Total:</span>
                                <span className="text-xs font-semibold text-gray-900 dark:text-gray-100">${(approval.proposed_action || approval.context_payload).suggested_price}</span>
                              </div>
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500">Scope of Work:</span>
                                <span className="text-xs font-medium text-gray-800 dark:text-gray-200">{(approval.proposed_action || approval.context_payload).scope}</span>
                              </div>
                              <div className="flex justify-between">
                                <span className="text-xs text-gray-500">Suggested Time:</span>
                                <span className="text-xs font-medium text-gray-800 dark:text-gray-200">{(approval.proposed_action || approval.context_payload).suggested_time}</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      )}
                      {(approval.proposed_action || approval.context_payload)?.feature_type === 'social_post_draft' ? (
                        <div className="flex flex-col gap-3">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400 font-semibold">New product detected!</span>
                            <span className="text-pink-500 font-bold text-xs">Schedule a post?</span>
                          </div>
                          <div className="app-card dark:bg-gray-800 p-3 rounded-lg border border-pink-100 dark:border-pink-900/50">
                            <div className="text-[10px] uppercase font-bold text-gray-400 mb-2 flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-pink-500"></span> Instagram / TikTok Draft</div>
                            <div className="text-xs text-gray-700 dark:text-gray-300 italic line-clamp-3">
                                "{(approval.proposed_action || approval.context_payload).instagram || (approval.proposed_action || approval.context_payload).tiktok || 'Check out our new product!'}"
                            </div>
                          </div>
                        </div>
                      ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'supply_order' ? (
                        <>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Current Stock:</span>
                            <span className="font-semibold text-gray-800 dark:text-gray-200" data-testid="supply-order-stock">
                              {(approval.proposed_action || approval.context_payload).remaining_stock} units
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Est. Runout:</span>
                            <span className="font-semibold text-gray-800 dark:text-gray-200">
                              {(approval.proposed_action || approval.context_payload).est_runout_days} days
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Reorder Quantity:</span>
                            <span className="font-bold text-blue-600 dark:text-blue-400 text-base" data-testid="supply-order-quantity">
                               {(approval.proposed_action || approval.context_payload).suggested_reorder_quantity} Units
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-3">
                            <span className="text-gray-500 dark:text-gray-400">Vendor:</span>
                            <span className="font-semibold text-gray-800 dark:text-gray-200">
                               {(approval.proposed_action || approval.context_payload).vendor_name} ({(approval.proposed_action || approval.context_payload).vendor_contact})
                            </span>
                          </div>
                          <div className="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border border-gray-200 dark:border-gray-700">
                            <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">Drafted Message:</div>
                            <div className="text-sm text-gray-800 dark:text-gray-200 italic font-medium">
                              "{(approval.proposed_action || approval.context_payload).draft_message}"
                            </div>
                          </div>
                        </>

                      ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'stockout_restock_and_price' ? (
                        <>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Current Price:</span>
                            <span className="font-semibold text-gray-400 dark:text-gray-500 line-through">
                               ${Number((approval.proposed_action || approval.context_payload).old_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Suggested Price:</span>
                            <span className="font-bold text-green-600 dark:text-green-400 text-base" data-testid="stockout-new-price">
                               ${Number((approval.proposed_action || approval.context_payload).new_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Reorder Quantity:</span>
                            <span className="font-bold text-blue-600 dark:text-blue-400 text-base" data-testid="stockout-reorder">
                               {(approval.proposed_action || approval.context_payload).suggested_reorder_quantity} Units
                            </span>
                          </div>
                          <div className="text-sm font-medium text-gray-800 dark:text-gray-200 mt-2">
                            {(approval.proposed_action || approval.context_payload).message}
                          </div>
                        </>
                      ) : (approval.proposed_action || approval.context_payload)?.context?.smart_pricing === true ? (
                        <>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Current Price:</span>
                            <span className="font-semibold text-gray-400 dark:text-gray-500 line-through">
                              ${Number((approval.proposed_action || approval.context_payload).context.old_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Suggested Price:</span>
                            <span className="font-bold text-green-600 dark:text-green-400 text-base" data-testid="smart-pricing-new-price">
                              ${Number((approval.proposed_action || approval.context_payload).context.new_price).toFixed(2)}
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Sales Projection:</span>
                            <span className="font-semibold text-indigo-600 dark:text-indigo-400" data-testid="smart-pricing-sales-projection">
                              {(approval.proposed_action || approval.context_payload).context.sales_projection}
                            </span>
                          </div>
                        </>
                      ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'incident_resolution' ? (
                        <div className="flex flex-col gap-2">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Proposed Actions:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">
                              {((approval.proposed_action || approval.context_payload).actions || []).length} steps
                            </span>
                          </div>
                          <div className="w-full h-px bg-gray-200 dark:bg-gray-700 my-1"></div>
                          {((approval.proposed_action || approval.context_payload).actions || []).map((action: any, idx: number) => (
                            <div key={idx} className="flex flex-col mb-2">
                              <span className="text-xs font-semibold text-gray-700 dark:text-gray-300">
                                {action.action}
                              </span>
                              <span className="text-xs text-gray-500">
                                {action.details}
                              </span>
                            </div>
                          ))}
                        </div>
                      ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'ambassador_reply' ? (
                        <div className="flex flex-col gap-2">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Context:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">{(approval.proposed_action || approval.context_payload).source || 'Message'}</span>
                          </div>
                          <div className="flex flex-col text-sm mt-1">
                            <span className="text-gray-500 dark:text-gray-400">Draft:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100 line-clamp-2 mt-1">{(approval.proposed_action || approval.context_payload).generated_response}</span>
                          </div>
                        </div>
                      ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'quote_draft' ? (
                        <div className="flex flex-col gap-2">
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Context:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">{(approval.proposed_action || approval.context_payload).customer_inquiry || 'Client Inquiry'}</span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Scope:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">{(approval.proposed_action || approval.context_payload).scope || (approval.proposed_action || approval.context_payload).service}</span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Timeline:</span>
                            <span className="font-semibold text-gray-900 dark:text-gray-100">{(approval.proposed_action || approval.context_payload).suggested_time || 'TBD'}</span>
                          </div>
                          <div className="flex justify-between items-center text-sm">
                            <span className="text-gray-500 dark:text-gray-400">Price:</span>
                            <span className="font-semibold text-green-600 dark:text-green-400">
                              ${Number((approval.proposed_action || approval.context_payload).suggested_price || (approval.proposed_action || approval.context_payload).price || 0).toFixed(2)}
                            </span>
                          </div>
                        </div>
                      ) : (
                        <>
                          {(approval.proposed_action || approval.context_payload)?.context?.weekly_health_report === true ? (                            <div className="flex flex-col gap-2">
                              <div className="text-sm text-gray-700 dark:text-gray-300">
                                <span className="font-semibold">Summary:</span> {(approval.proposed_action || approval.context_payload).context.summary}
                              </div>
                              <div className="text-sm text-indigo-600 dark:text-indigo-400 font-medium">
                                <span className="font-semibold text-gray-700 dark:text-gray-300">Suggestion:</span> {(approval.proposed_action || approval.context_payload).context.actionable_suggestion}
                              </div>
                            </div>
                          ) : (
                            <>
                              {(approval.proposed_action || approval.context_payload)?.context?.abandoned_carts_count !== undefined && (
                                <div className="flex justify-between items-center text-sm">
                                  <span className="text-gray-500 dark:text-gray-400">Abandoned Carts:</span>
                                  <span className="font-semibold text-gray-900 dark:text-gray-100">{(approval.proposed_action || approval.context_payload).context.abandoned_carts_count}</span>
                                </div>
                              )}
                              {(approval.proposed_action || approval.context_payload)?.context?.potential_revenue !== undefined && (
                                <div className="flex justify-between items-center text-sm">
                                  <span className="text-gray-500 dark:text-gray-400">Potential Revenue:</span>
                                  <span className="font-semibold text-green-600 dark:text-green-400">
                                    ${Number((approval.proposed_action || approval.context_payload).context.potential_revenue).toFixed(2)}
                                  </span>
                                </div>
                              )}
                              {(approval.proposed_action || approval.context_payload)?.remaining_stock !== undefined && (
                                <div className="flex flex-col gap-2">
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Product ID:</span>
                                    <span className="font-semibold text-gray-900 dark:text-gray-100">{(approval.proposed_action || approval.context_payload).product_id}</span>
                                  </div>
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Remaining Stock:</span>
                                    <span className="font-semibold text-red-600 dark:text-red-400">{(approval.proposed_action || approval.context_payload).remaining_stock}</span>
                                  </div>
                                  <div className="flex justify-between items-center text-sm">
                                    <span className="text-gray-500 dark:text-gray-400">Alert Message:</span>
                                    <span className="font-semibold text-gray-900 dark:text-gray-100">{(approval.proposed_action || approval.context_payload).message}</span>
                                  </div>
                                </div>
                              )}
                            </>
                          )}
                        </>
                      )}
                    </div>
                  )}
                </div>

                <div className="flex flex-col gap-3 w-full mt-2">
                  {(approval.proposed_action || approval.context_payload)?.feature_type === 'incident_resolution' ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-red-600 text-white font-medium hover:bg-red-700 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Execute Plan"
                        data-testid="approve-incident-resolution"
                      >
                        Execute Plan
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss Plan"
                        data-testid="dismiss-incident-resolution"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'instagram_dm' ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-pink-600 text-white font-medium hover:bg-pink-700 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve & Send"
                        data-testid="approve-instagram-dm"
                      >
                        Approve & Send
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss"
                        data-testid="dismiss-instagram-dm"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'supply_order' ? (
                    <>
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3"
                        aria-label="Approve & Send"
                        data-testid="approve-supply-order"
                      >
                        Approve & Send
                      </button>
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => {
                            setEditingId(approval.id);
                            setEditContent((approval.proposed_action || approval.context_payload).draft_message);
                          }}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit message"
                          data-testid="edit-supply-order"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Deny supply order"
                          data-testid="reject-supply-order"
                        >
                          Deny
                        </button>
                      </div>
                    </>

                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'social_post_draft' ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-gradient-to-r from-pink-500 to-indigo-500 text-white font-medium hover:from-pink-600 hover:to-indigo-600 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve & Schedule"
                        data-testid="approve-social-post"
                      >
                        Approve & Schedule
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss proposal"
                        data-testid="dismiss-social-post"
                      >
                        Dismiss
                      </button>
                    </div>
                      ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'supply_order' ? (
                        <>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Current Stock:</span>
                            <span className="font-semibold text-gray-800 dark:text-gray-200" data-testid="supply-order-stock">
                              {(approval.proposed_action || approval.context_payload).remaining_stock} units
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Est. Runout:</span>
                            <span className="font-semibold text-gray-800 dark:text-gray-200">
                              {(approval.proposed_action || approval.context_payload).est_runout_days} days
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-1">
                            <span className="text-gray-500 dark:text-gray-400">Reorder Quantity:</span>
                            <span className="font-bold text-blue-600 dark:text-blue-400 text-base" data-testid="supply-order-quantity">
                               {(approval.proposed_action || approval.context_payload).suggested_reorder_quantity} Units
                            </span>
                          </div>
                          <div className="flex justify-between items-center text-sm mb-3">
                            <span className="text-gray-500 dark:text-gray-400">Vendor:</span>
                            <span className="font-semibold text-gray-800 dark:text-gray-200">
                               {(approval.proposed_action || approval.context_payload).vendor_name} ({(approval.proposed_action || approval.context_payload).vendor_contact})
                            </span>
                          </div>
                          <div className="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border border-gray-200 dark:border-gray-700">
                            <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">Drafted Message:</div>
                            <div className="text-sm text-gray-800 dark:text-gray-200 italic font-medium">
                              "{(approval.proposed_action || approval.context_payload).draft_message}"
                            </div>
                          </div>
                        </>

                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'stockout_restock_and_price' ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve"
                        data-testid="approve-stockout"
                      >
                        Approve
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss"
                        data-testid="dismiss-stockout"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'ambassador_reply' ? (
                    editingId === approval.id ? (
                      <div className="flex flex-col gap-3 w-full">
                        <textarea
                          className="w-full p-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
                          rows={4}
                          value={editContent}
                          onChange={(e) => setEditContent(e.target.value)}
                          data-testid="edit-ambassador-reply-textarea"
                          autoFocus
                        />
                        <div className="flex gap-3">
                          <button
                            onClick={() => {
                              handleDecision(approval.id, true, editContent);
                              setEditingId(null);
                            }}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                            data-testid="save-send-ambassador-reply"
                          >
                            Save & Send
                          </button>
                          <button
                            onClick={() => setEditingId(null)}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                            data-testid="cancel-edit-ambassador-reply"
                          >
                            Cancel
                          </button>
                        </div>
                      </div>
                    ) : (
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => handleDecision(approval.id, true)}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                          aria-label="Approve & Send Draft"
                          data-testid="approve-ambassador-reply"
                        >
                          ✨ 1-Tap Approve
                        </button>
                        <button
                          onClick={() => {
                            setEditingId(approval.id);
                            setEditContent((approval.proposed_action || approval.context_payload)?.generated_response || "");
                          }}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit Draft"
                          data-testid="edit-ambassador-reply"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Dismiss Draft"
                          data-testid="dismiss-ambassador-reply"
                        >
                          Dismiss
                        </button>
                      </div>
                    )
                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve & Send"
                        data-testid="approve-quote-draft"
                      >
                        Approve & Send
                      </button>
                      <a
                        href={`/quoting?id=${(approval.proposed_action || approval.context_payload)?.quote_id || approval.id}`}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Edit Draft"
                        data-testid="edit-quote-draft"
                      >
                        Edit Draft
                      </a>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.context?.smart_pricing === true ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve & Run Sale"
                        data-testid="approve-run-sale"
                      >
                        Approve & Run Sale
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss proposal"
                        data-testid="dismiss-sale"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.context?.weekly_health_report === true ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-green-600 text-white font-medium hover:bg-green-700 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Draft it"
                        data-testid="approve-draft"
                      >
                        Yes, draft it!
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss proposal"
                        data-testid="dismiss-draft"
                      >
                        Dismiss
                      </button>
                    </div>                  ) : (approval.proposed_action || approval.context_payload)?.remaining_stock !== undefined ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-amber-500 text-white font-medium hover:bg-amber-600 transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve Restock"
                        data-testid="approve-restock"
                      >
                        Approve Restock
                      </button>
                      <button
                        onClick={() => handleDecision(approval.id, false)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                        aria-label="Dismiss restock"
                        data-testid="dismiss-restock"
                      >
                        Dismiss
                      </button>
                    </div>
                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === 'quote_draft' ? (
                    <>
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3"
                        aria-label="Approve & Send"
                        data-testid="approve-send-proposal"
                      >
                        Approve & Send
                      </button>
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <a
                          href={`/quoting?id=${(approval.proposed_action || approval.context_payload)?.quote_id || approval.id}`}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit Draft"
                          data-testid="edit-proposal"
                        >
                          Edit Draft
                        </a>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Ask Agent to Adjust"
                          data-testid="reject-proposal"
                        >
                          Ask Agent to Adjust
                        </button>
                      </div>
                    </>
                  ) : (
                    editingId === approval.id ? (
                      <div className="flex flex-col gap-3 w-full">
                        <textarea
                          className="w-full p-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
                          rows={4}
                          value={editContent}
                          onChange={(e) => setEditContent(e.target.value)}
                          data-testid="edit-proposal-textarea"
                          autoFocus
                        />
                        <div className="flex gap-3">
                          <button
                            onClick={() => {
                              handleDecision(approval.id, true, editContent);
                              setEditingId(null);
                            }}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center"
                            data-testid="save-proposal"
                          >
                            Save & Approve
                          </button>
                          <button
                            onClick={() => setEditingId(null)}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center"
                            data-testid="cancel-edit-proposal"
                          >
                            Cancel
                          </button>
                        </div>
                      </div>
                    ) : (
                    <>
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="w-full min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center mb-3"
                        aria-label="Approve proposal"
                        data-testid="approve-proposal"
                      >
                        Approve
                      </button>
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => {
                            setEditingId(approval.id);
                            const textToEdit =
                              (approval.proposed_action || approval.context_payload)?.generated_response ||
                              (approval.proposed_action || approval.context_payload)?.draft_reply ||
                              (approval.context_payload?.description || approval.proposed_action?.message || approval.proposed_action?.action_type || approval.event_source);
                            setEditContent(textToEdit || "");
                          }}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit proposal"
                          data-testid="edit-proposal"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Reject proposal"
                          data-testid="reject-proposal"
                        >
                          Deny
                        </button>
                      </div>
                    </>
                    )
                  )}
                </div>
              </div>
            ))}
          </>
        )}

        {activeTab === "activity" && (
          <>
            {activityLoading && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Activity Feed...
              </div>
            )}
            {!activityLoading && activities.length === 0 && (
              <div className="w-full p-6 glassmorphism rounded-[16px] text-center">
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 break-words">
                  No recent activity found.
                </p>
              </div>
            )}
            <div className="flex flex-col gap-3 min-w-[320px] max-w-full">
            {activities.map((activity) => (
              <div
                key={activity.id}
                className="glassmorphism p-5 rounded-[16px]  shadow-sm flex flex-col gap-3 opacity-90 min-h-[44px]"
              >
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold font-outfit uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-md">
                    {activity.department.replace('_', ' ')}
                  </span>
                  {activity.event_type === 'Paused' || activity.event_type === 'PAUSED' ? (
                    <span className="text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-md text-yellow-600 bg-yellow-50 dark:text-yellow-400 dark:bg-yellow-900/30">
                      PAUSED
                    </span>
                  ) : (
                    <span className="text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-md text-green-600 bg-green-50 dark:text-green-400 dark:bg-green-900/30">
                      {activity.event_type === 'Approved' || activity.event_type === 'APPROVED' ? 'APPROVED' : activity.event_type}
                    </span>
                  )}
                </div>
                <h3 className="text-md font-semibold font-inter text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                  {(() => {
                    try {
                      const p = typeof activity.payload === 'string' ? JSON.parse(activity.payload) : activity.payload;
                      // Fallback logic specific to Paused state that gets stored inside proposed_content
                      if (p?.original_payload?.proposed_content?.includes("System is paused")) {
                          return p.original_payload.proposed_content;
                      }
                      return p?.original_payload?.description || 'Action completed';
                    } catch (e) {
                      return 'Action completed';
                    }
                  })()}
                </h3>
                <span className="text-xs text-gray-500 font-inter">{new Date(activity.created_at).toLocaleString()}</span>
              </div>
            ))}
            </div>
          </>
        )}
      </div>
    </section>
  );
}
