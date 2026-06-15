"use client";

import { useEffect, useState, useMemo } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { enqueueAction, getActions, removeAction } from "../utils/offlineQueue";


type Order = {
  id: string;
  customer_name?: string;
  total_amount?: number;
  status?: string;
  created_at?: string;
};

type InboxMessage = {
  id: string;
  from_name?: string;
  from_email?: string;
  subject?: string;
  snippet?: string;
  status?: string;
  created_at?: string;
};

type Booking = {
  id: string;
  customer_name?: string;
  service_name?: string;
  start_time?: string;
  status?: string;
  created_at?: string;
};

type UnifiedItem = {
  id: string;
  sortDate: Date;
  type: "proposal" | "triage" | "order" | "booking" | "message";
  title: string;
  description: string;
  department: string;
  status: string;
  original: any;
};

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

  const [orders, setOrders] = useState<Order[]>([]);
  const [bookings, setBookings] = useState<Booking[]>([]);
  const [messages, setMessages] = useState<InboxMessage[]>([]);
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

  const unifiedFeedItems = useMemo(() => {
    const list: UnifiedItem[] = [];

    // Triage items
    triageItems.forEach(t => {
      if (t.status === "COMPLETED" || t.status === "DISMISSED") return;
      list.push({
        id: t.id,
        sortDate: t.created_at ? new Date(t.created_at) : new Date(),
        type: "triage",
        title: "Triage Alert: " + (t.action_type || t.source || "System"),
        description: t.context || t.action_payload || "Requires attention",
        department: t.source || "System",
        status: t.status || "PENDING",
        original: t
      });
    });

    // Owner Feed items
    items.forEach(i => {
      list.push({
        id: i.id,
        sortDate: i.created_at ? new Date(i.created_at) : new Date(),
        type: "proposal",
        title: i.proposed_action?.title || i.event_source?.replace(/_/g, " ") || "Agent Action",
        description: i.context_payload?.summary || i.proposed_action?.description || "A new update requires your attention",
        department: i.event_source || "Agent",
        status: i.lifecycle_state || "PENDING",
        original: i
      });
    });

    // Orders
    orders.forEach(o => {
      if (o.status === "FULFILLED" || o.status === "CANCELLED") return;
      list.push({
        id: o.id,
        sortDate: o.created_at ? new Date(o.created_at) : new Date(),
        type: "order",
        title: "New Order",
        description: `Order from ${o.customer_name || 'Customer'} (${(o.total_amount || 0).toFixed(2)})`,
        department: "Sales",
        status: o.status || "PENDING",
        original: o
      });
    });

    // Bookings
    bookings.forEach(b => {
      if (b.status === "COMPLETED" || b.status === "CANCELLED") return;
      list.push({
        id: b.id,
        sortDate: b.created_at ? new Date(b.created_at) : new Date(),
        type: "booking",
        title: "New Booking",
        description: `${b.service_name || 'Service'} with ${b.customer_name || 'Customer'} at ${b.start_time ? new Date(b.start_time).toLocaleString() : ''}`,
        department: "Operations",
        status: b.status || "PENDING",
        original: b
      });
    });

    // Messages
    messages.forEach(m => {
      if (m.status === "READ" || m.status === "REPLIED") return;
      list.push({
        id: m.id,
        sortDate: m.created_at ? new Date(m.created_at) : new Date(),
        type: "message",
        title: "New Message",
        description: `From ${m.from_name || m.from_email || 'Customer'}: ${m.subject || m.snippet || ''}`,
        department: "Customer Service",
        status: m.status || "UNREAD",
        original: m
      });
    });

    return list.sort((a, b) => b.sortDate.getTime() - a.sortDate.getTime());
  }, [triageItems, items, orders, bookings, messages]);


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
    <section className="mb-6 w-full w-full overflow-hidden" aria-label="Unified Owner Feed">
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
          Owner Feed ({unifiedFeedItems.length})
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
            {loading && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Owner Feed...
              </div>
            )}
            {!loading && unifiedFeedItems.length === 0 && (
              <div className="w-full p-6 glassmorphism rounded-[16px] text-center" data-testid="unified-owner-feed-empty">
                <div className="w-16 h-16 bg-[#e8f7ef] dark:bg-[rgba(23,166,106,0.2)] rounded-full flex items-center justify-center mx-auto mb-4">
                  <svg className="w-8 h-8 text-[#17a66a]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path>
                  </svg>
                </div>
                <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-2">You're all caught up!</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 break-words">
                  There are no pending actions for you right now.
                </p>
              </div>
            )}
            {unifiedFeedItems.map((item) => {
              const isProcessing = queuedActionIds.has(item.id);
              let icon = "⚡️";
              let colorClass = "text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30";
              if (item.type === "order") { icon = "📦"; colorClass = "text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30"; }
              if (item.type === "booking") { icon = "📅"; colorClass = "text-orange-600 dark:text-orange-400 bg-orange-50 dark:bg-orange-900/30"; }
              if (item.type === "message") { icon = "💬"; colorClass = "text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/30"; }
              if (item.type === "proposal") { icon = "🤖"; colorClass = "text-purple-600 dark:text-purple-400 bg-purple-50 dark:bg-purple-900/30"; }

              return (
                <div
                  key={item.id}
                  className={`glassmorphism p-5 rounded-[16px] transition-all duration-300 shadow-sm border border-white/40 dark:border-white/10 ${isProcessing ? "opacity-50 scale-[0.98] border-green-500" : "animate-fade-in hover:shadow-md hover:-translate-y-0.5"}`}
                  data-testid="agent-feed-card"
                >
                  <div className="flex items-center justify-between mb-3">
                    <span className={`text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-md flex items-center gap-1.5 ${colorClass}`}>
                      <span>{icon}</span> {item.type}
                    </span>
                    <span className="text-xs text-gray-400 font-inter font-medium bg-gray-50 dark:bg-gray-800 px-2 py-1 rounded-md">
                      {item.sortDate.toLocaleString(undefined, { month: 'short', day: 'numeric', hour: 'numeric', minute: '2-digit' })}
                    </span>
                  </div>

                  <div className="flex items-center gap-2 mb-2">
                    <div className="w-8 h-8 rounded-full bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-800 flex items-center justify-center shadow-inner">
                      <span className="text-sm">👤</span>
                    </div>
                    <h3 className="text-[15px] font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug">
                      {item.title}
                    </h3>
                  </div>

                  {editingId === item.id ? (
                      <div className="mb-5 pl-10">
                        <textarea
                          className="w-full p-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none shadow-inner"
                          rows={4}
                          value={editContent}
                          onChange={(e) => setEditContent(e.target.value)}
                          data-testid="edit-proposal-textarea"
                          autoFocus
                        />
                        <div className="flex gap-3 mt-3">
                          <button
                            onClick={() => {
                              handleDecision(item.id, true, editContent);
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
                    <div className="mb-5 pl-10">
                      <p className="text-[14px] text-gray-700 dark:text-gray-300 leading-relaxed bg-white/50 dark:bg-black/20 p-3 rounded-xl border border-white/60 dark:border-white/5">
                        {item.description}
                      </p>

                      {item.type === 'proposal' && (
                         <div className="mt-2 flex justify-end">
                           <button
                            onClick={() => {
                              setEditingId(item.id);
                              setEditContent(item.description);
                            }}
                            className="text-xs text-[#0066FF] hover:text-[#0052CC] dark:text-[#0071E3] dark:hover:text-[#005bb5] font-semibold transition-colors flex items-center gap-1"
                            data-testid="edit-proposal"
                           >
                            <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                            Edit Action
                           </button>
                         </div>
                      )}
                    </div>
                  )}

                  {!editingId && (
                    <div className="flex gap-3 mt-4 pl-10">
                      {item.type === 'proposal' && (
                        <>
                          <button
                            onClick={() => handleDecision(item.id, true)}
                            disabled={isProcessing}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] dark:bg-[#0071E3] dark:hover:bg-[#005bb5] transition-all shadow-md flex items-center justify-center gap-2"
                            data-testid="approve-proposal"
                          >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                            {isProcessing ? 'Processing...' : 'Approve'}
                          </button>
                          <button
                            onClick={() => handleDecision(item.id, false)}
                            disabled={isProcessing}
                            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center gap-2"
                            data-testid="reject-proposal"
                          >
                            <svg className="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
                            Dismiss
                          </button>
                        </>
                      )}
                      {(item.type === 'order' || item.type === 'booking') && (
                        <button
                          className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all flex items-center justify-center gap-2"
                        >
                           <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path></svg>
                           View Details
                        </button>
                      )}
                      {item.type === 'message' && (
                        <button
                          className="flex-1 min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all shadow-md flex items-center justify-center gap-2"
                        >
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"></path></svg>
                          Draft Reply
                        </button>
                      )}
                    </div>
                  )}
                </div>
              );
            })}
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
