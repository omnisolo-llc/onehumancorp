"use client";

import { useEffect, useState, useMemo } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { enqueueAction, getActions, removeAction } from "../utils/offlineQueue";
import { AmbassadorReplyCard } from "./AmbassadorReplyCard";
import { InstagramDMCard } from "./InstagramDMCard";
import { AgentActionCard } from "../../components/feed/AgentActionCard";
import { GroupedAgentActionCard } from "../../components/feed/GroupedAgentActionCard";

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
  const [items, setItems] = useState<AgentFeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">(
    "proposals",
  );
  const [activities, setActivities] = useState<any[]>(initialData?.activity || []);

  const groupedProposals = useMemo(() => {
    const groups: Record<string, { groupKey: string; title: string; items: AgentFeedItem[] }> = {};
    items.forEach(item => {
      const featureType = item.proposed_action?.feature_type || item.context_payload?.feature_type || item.event_source || "unknown";
      const actionType = item.proposed_action?.action_type || "default";
      const key = `${featureType}-${actionType}`;
      if (!groups[key]) {
        groups[key] = {
          groupKey: key,
          title: featureType.replace(/_/g, " "),
          items: []
        };
      }
      groups[key].items.push(item);
    });
    return Object.values(groups);
  }, [items]);
  const [activityLoading, setActivityLoading] = useState(false);
  const [isOffline, setIsOffline] = useState(false);
  const [offlineActionsCount, setOfflineActionsCount] = useState(0);
  const [queuedActionIds, setQueuedActionIds] = useState<Set<string>>(
    new Set(),
  );
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editContent, setEditContent] = useState<string>("");
  const [editQuotePrice, setEditQuotePrice] = useState<string>("");
  const [editQuoteScope, setEditQuoteScope] = useState<string>("");

  const tenantId = () => {
    if (typeof window === "undefined") return "default";
    return (
      localStorage.getItem("tenant_id") ||
      localStorage.getItem("tenant") ||
      "default"
    );
  };

  useEffect(() => {
    const handleVoiceCommandProcessed = (event: CustomEvent) => {
      // Wait a moment for backend DB write consistency, then reload the feed completely.
      setTimeout(() => {
        window.location.reload();
      }, 500);
    };
    window.addEventListener(
      "voice-command-processed",
      handleVoiceCommandProcessed as EventListener,
    );
    return () =>
      window.removeEventListener(
        "voice-command-processed",
        handleVoiceCommandProcessed as EventListener,
      );
  }, []);

  useEffect(() => {
    const updateOfflineCount = async () => {
      try {
        const actions = await getActions();
        setOfflineActionsCount(actions.length);
        const ids = new Set<string>();
        actions.forEach((a) => {
          if (a.payload && a.payload.id) ids.add(a.payload.id);
        });
        setQueuedActionIds(ids);
      } catch (err) {}
    };
    updateOfflineCount();

    window.addEventListener("ohc_queue_updated", updateOfflineCount);

    setIsOffline(!navigator.onLine);

    const handleOnline = async () => {
      setIsOffline(false);
      // Sync queued offline actions
      try {
        const actions = await getActions();
        for (const action of actions) {
          if (action.type === "approve_agent_feed") {
            await submitDecision(
              action.payload.id,
              action.payload.approved,
              action.payload.modified_content,
              action.payload.event_source,
            );
            await removeAction(action.id);
            setOfflineActionsCount((prev) => Math.max(0, prev - 1));
            setQueuedActionIds((prev) => {
              const newSet = new Set(prev);
              newSet.delete(action.payload.id);
              return newSet;
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
      window.removeEventListener("ohc_queue_updated", updateOfflineCount);
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  useEffect(() => {
    let mounted = true;

    async function fetchAll() {
      try {
        setLoading(true);
        setActivityLoading(true);
        const tenant = tenantId();

        let unifiedData = initialData;

        if (!unifiedData) {
          const unifiedRes = await fetch(
            `/api/agent-feed?tenant_id=${tenant}`,
            {
              headers: {
                "x-tenant-id": tenant,
                "x-user-id": "default",
              },
            },
          );
          if (!unifiedRes.ok) {
            throw new Error("Failed to load agent feed");
          }
          unifiedData = await unifiedRes.json();
        }

        if (mounted) {
          if (unifiedData?.items) {
            let combinedItems = [...unifiedData.items];
            // Integrate Priority Tasks
            if (
              unifiedData.priority_tasks &&
              Array.isArray(unifiedData.priority_tasks)
            ) {
              combinedItems = [
                ...combinedItems,
                ...unifiedData.priority_tasks.map((pt: any) => ({
                  id: pt.id,
                  tenant_id: pt.tenant_id || "default",
                  event_source: "task",
                  context_payload: {
                    description: pt.description || pt.title,
                    feature_type: "task"
                  },
                  proposed_action: {
                    message: "Task Pending",
                    action_type: "complete_task",
                    feature_type: "task"
                  },
                  lifecycle_state:
                    pt.status === "PENDING" ? "PENDING_APPROVAL" : "DISMISSED",
                  created_at: pt.created_at || new Date().toISOString(),
                  updated_at: pt.updated_at || new Date().toISOString(),
                })),
              ];
            }

            // Integrate Triage Items (Messages)
            if (unifiedData.triage && Array.isArray(unifiedData.triage)) {
              combinedItems = [
                ...combinedItems,
                ...unifiedData.triage.slice(0, 3).map((ti: any) => {
                  let featureType = "triage";

                  if (ti.source?.toLowerCase() === "instagram dm") {
                    featureType = "instagram_dm";
                  }

                  let draftReply = ti.action_payload || "Triage item";
                  try {
                      if (ti.action_payload && typeof ti.action_payload === "string" && ti.action_payload.startsWith("{")) {
                          const parsed = JSON.parse(ti.action_payload);
                          if (parsed.feature_type) {
                              featureType = parsed.feature_type;
                          }
                          if (parsed.draft_reply) {
                              draftReply = parsed.draft_reply;
                          } else if (parsed.action_payload) {
                              draftReply = parsed.action_payload;
                          }
                      }
                  } catch (e) {
                      // ignore parse errors
                  }

                  let customerMessage = ti.context || ti.customer_message || "Message requires attention";

                  return {
                    id: ti.id,
                    tenant_id: ti.tenant_id || "default",
                    event_source: "triage",
                    context_payload: {
                      description: customerMessage,
                      customer_message: customerMessage,
                      feature_type: featureType
                    },
                    proposed_action: {
                      message: ti.action_payload || draftReply,
                      draft_reply: draftReply,
                      action_type: ti.action_type || "resolve",
                      feature_type: featureType
                    },
                    lifecycle_state:
                      ti.status === "RESOLVED" || ti.status === "resolved" ? "DISMISSED" : "PENDING_APPROVAL",
                    created_at: ti.created_at || new Date().toISOString(),
                    updated_at: ti.created_at || new Date().toISOString(),
                  };
                }),
              ];
            }

            // Integrate Orders
            if (unifiedData.orders && Array.isArray(unifiedData.orders)) {
              combinedItems = [
                ...combinedItems,
                ...unifiedData.orders.map((or: any) => ({
                  id: or.id,
                  tenant_id: or.tenant_id || "default",
                  event_source: "order",
                  context_payload: {
                    description: `Order ${or.id} needs fulfillment`,
                    feature_type: "order",
                    order: or
                  },
                  proposed_action: {
                    message: "Fulfill Order",
                    action_type: "fulfill_order",
                    feature_type: "order"
                  },
                  lifecycle_state:
                    or.status === "pending" || or.status === "unfulfilled"
                      ? "PENDING_APPROVAL"
                      : "DISMISSED",
                  created_at: or.created_at || new Date().toISOString(),
                  updated_at: or.created_at || new Date().toISOString(),
                })),
              ];
            }

            // Integrate Pending Reviews
            if (unifiedData.pendingReviews && Array.isArray(unifiedData.pendingReviews)) {
              combinedItems = [
                ...combinedItems,
                ...unifiedData.pendingReviews.map((pr: any) => ({
                  id: pr.response?.id || crypto.randomUUID(),
                  tenant_id: tenant,
                  event_source: "review",
                  context_payload: {
                    description: pr.review?.content || "Pending Review",
                    source: pr.review?.source,
                    rating: pr.review?.rating,
                    original_message: pr.review?.content,
                    feature_type: "review",
                    review: pr.review
                  },
                  proposed_action: {
                    message: pr.response?.draftedContent || "",
                    action_type: "review_reply",
                    generated_response: pr.response?.draftedContent,
                    feature_type: "review",
                    response: pr.response
                  },
                  lifecycle_state: pr.response?.status === "draft" ? "PENDING_APPROVAL" : "DISMISSED",
                  created_at: new Date((pr.review?.createdAtUnix || Date.now() / 1000) * 1000).toISOString(),
                  updated_at: new Date((pr.review?.createdAtUnix || Date.now() / 1000) * 1000).toISOString(),
                })),
              ];
            }

            // Sort by created_at desc
            combinedItems.sort(
              (a, b) =>
                new Date(b.created_at).getTime() -
                new Date(a.created_at).getTime(),
            );

            setItems(
              combinedItems.filter(
                (i: any) =>
                  i.lifecycle_state !== "APPROVED" &&
                  i.lifecycle_state !== "DISMISSED" &&
                  i.lifecycle_state !== "PAUSED",
              ),
            );

            // Map items for activity feed as well
            const mappedActivities = combinedItems
              .filter(
                (i: any) =>
                  i.lifecycle_state === "APPROVED" ||
                  i.lifecycle_state === "DISMISSED" ||
                  i.lifecycle_state === "PAUSED",
              )
              .map((a: any) => ({
                id: a.id,
                tenant_id: a.tenant_id,
                event_type: a.lifecycle_state,
                department: a.event_source,
                payload: JSON.stringify({
                  original_payload: {
                    description:
                      a.proposed_action?.message ||
                      a.proposed_action?.action_type ||
                      a.event_source,
                  },
                }),
                created_at:
                  a.updated_at || a.created_at || new Date().toISOString(),
              }));
            setActivities(mappedActivities);
          }
        }

        if (mounted) {

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
      cleanup.then((fn: any) => fn && typeof fn === "function" && fn());
    };
  }, [initialData]);

  useEffect(() => {
    let ws: WebSocket;
    let reconnectTimeout: NodeJS.Timeout;

    const connect = () => {
      const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
      const isLocalhost =
        window.location.hostname === "localhost" ||
        window.location.hostname === "127.0.0.1";
      // In production, Next.js proxy doesn't support WS well so we route directly to backend. Local dev also hits backend directly.
      const wsUrl = isLocalhost
        ? `ws://127.0.0.1:18789/api/v1/feed/ws`
        : `${protocol}//${window.location.host}/api/v1/feed/ws`;
      if (
        typeof process.env.VITEST !== "undefined" ||
        process.env.NODE_ENV === "test"
      )
        return;
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
          if (
            String(item.lifecycle_state || "").toUpperCase() ===
            "PENDING_APPROVAL"
          ) {
            setItems((current) => [
              item,
              ...current.filter((existing) => existing.id !== item.id),
            ]);

            // Also map and remove from activities if it somehow got back to pending (unlikely)
            setActivities((current) =>
              current.filter((existing) => existing.id !== item.id),
            );
          } else if (
            String(item.lifecycle_state || "").toUpperCase() === "APPROVED" ||
            String(item.lifecycle_state || "").toUpperCase() === "DISMISSED"
          ) {
            // It's an activity event (Approved, Rejected, etc.)
            setActivities((current) => {
              const mappedActivity = {
                id: item.id,
                tenant_id: item.tenant_id || "default",
                event_type: item.lifecycle_state,
                department: item.event_source || "system",
                payload:
                  typeof item.proposed_action === "object"
                    ? JSON.stringify({ original_payload: item.proposed_action })
                    : item.proposed_action,
                created_at: new Date().toISOString(),
              };
              return [
                mappedActivity,
                ...current.filter((existing) => existing.id !== item.id),
              ];
            });
            // Also remove from approvals
            setItems((current) =>
              current.filter((existing) => existing.id !== item.id),
            );
          } else {
            // Fallback for legacy SSE structure matching
            if (
              String(item.status || "").toUpperCase() === "DRAFT" ||
              String(item.status || "").toUpperCase() === "PENDING"
            ) {
              setItems((current) => [
                item,
                ...current.filter((existing) => existing.id !== item.id),
              ]);
            } else if (item.status) {
              setActivities((current) => {
                const mappedActivity = {
                  id: item.id,
                  tenant_id: item.tenant_id || "default",
                  event_type: item.status,
                  department: item.department,
                  payload:
                    typeof item.payload === "object"
                      ? JSON.stringify({ original_payload: item.payload })
                      : item.payload,
                  created_at: new Date().toISOString(),
                };
                return [
                  mappedActivity,
                  ...current.filter((existing) => existing.id !== item.id),
                ];
              });
              setItems((current) =>
                current.filter((existing) => existing.id !== item.id),
              );
            }
          }
        } catch (err) {
          console.error("Failed to parse websocket feed event:", err);
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

  const badgeTone = (priority?: string) => {
    const p = (priority || "").toLowerCase();
    if (p === "high" || p === "urgent") return "bad";
    if (p === "low") return "neutral";
    return "warning";
  };

  const submitDecision = async (
    id: string,
    approved: boolean,
    modified_content?: string,
    event_source?: string,
  ) => {
    if (event_source === "review") {
        const res = await fetch('/api/reviews/action', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action: approved ? 'approve' : 'dismiss', responseId: id, content: modified_content })
        });
        if (!res.ok) {
            throw new Error("Failed to submit review decision");
        }
        return;
    }

    if (
      event_source === "triage" ||
      event_source === "task" ||
      event_source === "order"
    ) {
      const tenant = tenantId();
      const res = await fetch(
        `/api/triage/action?tenant_id=${encodeURIComponent(tenant)}`,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ triage_item_id: id, approved }),
        },
      );
      if (!res.ok) {
        throw new Error("Failed to submit decision");
      }
      return;
    }

    const tenant = tenantId();
    const res = await fetch(`/api/agent-feed/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        "x-tenant-id": tenant,
        "x-user-id": "default",
      },
      body: JSON.stringify({
        state: approved ? "APPROVED" : "DISMISSED",
        modified_content,
      }),
    });

    if (!res.ok) {
      throw new Error("Failed to submit decision");
    }
  };

  const handleDecision = async (
    id: string,
    approved: boolean,
    modified_content?: string,
    event_source?: string,
  ): Promise<void> => {
    if (isOffline) {
      // Enqueue offline action
      await enqueueAction({
        id: crypto.randomUUID(),
        type: "approve_agent_feed",
        payload: { id, approved, modified_content, event_source },
        timestamp: Date.now(),
      });
      setOfflineActionsCount((prev) => prev + 1);
      setQueuedActionIds((prev) => new Set(prev).add(id));
      setItems((prev) => prev.filter((app) => app.id !== id));
      return;
    }

    try {
      await submitDecision(id, approved, modified_content, event_source);
      // Remove item only after successful submission
      setItems((prev) => prev.filter((app) => app.id !== id));
    } catch (err: any) {
      setError(err.message || "Action failed");
      throw err;
    }
  };

  if (error) {
    return (
      <div className="w-full mb-6 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[#FF3B30] text-[#FF3B30] text-center">
        {error}
      </div>
    );
  }

  return (
    <section
      id="unified-agent-feed-section"
      className="app-panel mb-6 w-full max-w-full md:max-w-2xl mx-auto overflow-hidden bg-white dark:bg-slate-950 p-4 rounded-xl shadow-lg border border-gray-100 dark:border-gray-800"
      aria-label="Unified Agent Feed"
    >
      <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 ">
        Action Required
      </h2>
      {isOffline && (
        <div className="mb-4 w-full p-2 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm rounded-[8px] bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
          <span>📡</span> You are offline. Actions will sync when online.
        </div>
      )}
      {offlineActionsCount > 0 && (
        <div className="mb-4 w-full p-2 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm rounded-[8px] bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
          <span>🔄</span> Pending Sync ({offlineActionsCount})
        </div>
      )}
      <div className="mb-4 flex items-center border-b border-gray-200 dark:border-gray-700">
        <button
          onClick={() => setActiveTab("proposals")}
          className={`flex-1 min-h-[44px] min-w-[44px] px-2 py-3 text-center text-sm font-semibold transition-all duration-200 ${
            activeTab === "proposals"
              ? "border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Proposals ({items.length})
        </button>
        <button
          onClick={() => setActiveTab("activity")}
          className={`flex-1 min-h-[44px] min-w-[44px] px-2 py-3 text-center text-sm font-semibold transition-all duration-200 ${
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
              <div className="w-full p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-center text-[#1D1D1F] dark:text-[#F5F5F7]">
                Loading Agent Proposals...
              </div>
            )}
            {!loading && items.length === 0 && (
              <div
                className="w-full flex flex-col items-center gap-6 p-6 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm  shadow-sm opacity-90 text-center"
                data-testid="triage-feed-empty"
              >
                <div className="text-3xl mb-2">✨</div>
                <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                  All caught up! Your business is running smoothly.
                </h3>
                <div className="w-full max-w-md text-left">
                  <GrowthReferralWidget />
                </div>
              </div>
            )}
            {groupedProposals.map((group) => {
              if (group.items.length === 1) {
                const approval = group.items[0];
                return (
                  <AgentActionCard
                    key={approval.id}
                    approval={approval}
                    queuedActionIds={queuedActionIds}
                    editingId={editingId}
                    editContent={editContent}
                    editQuotePrice={editQuotePrice}
                    editQuoteScope={editQuoteScope}
                    setEditingId={setEditingId}
                    setEditContent={setEditContent}
                    setEditQuotePrice={setEditQuotePrice}
                    setEditQuoteScope={setEditQuoteScope}
                    handleDecision={handleDecision}
                  />
                );
              }
              return (
                <GroupedAgentActionCard
                  key={group.groupKey}
                  groupKey={group.groupKey}
                  title={group.title}
                  items={group.items}
                  queuedActionIds={queuedActionIds}
                  editingId={editingId}
                  editContent={editContent}
                  editQuotePrice={editQuotePrice}
                  editQuoteScope={editQuoteScope}
                  setEditingId={setEditingId}
                  setEditContent={setEditContent}
                  setEditQuotePrice={setEditQuotePrice}
                  setEditQuoteScope={setEditQuoteScope}
                  handleDecision={handleDecision}
                />
              );
            })}
          </>
        )}

        {activeTab === "activity" && (
          <>
            {activityLoading && (
              <div className="w-full p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-center text-[#1D1D1F] dark:text-[#F5F5F7]">
                Loading Activity Feed...
              </div>
            )}
            {!activityLoading && activities.length === 0 && (
              <div className="w-full p-6 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-center">
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 break-words">
                  No recent activity found.
                </p>
              </div>
            )}
            <div className="flex flex-col gap-3 ">
              {activities.map((activity) => (
                <div
                  key={activity.id}
                  className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 shadow-sm flex flex-col gap-3 opacity-90 min-h-[44px]"
                >
                  <div className="flex items-center justify-between">
                    <span className="text-xs font-bold font-outfit uppercase tracking-wider text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30 px-2 py-1 rounded-[8px]">
                      {activity.department.replace("_", " ")}
                    </span>
                    {activity.event_type === "Paused" ||
                    activity.event_type === "PAUSED" ? (
                      <span className="text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-[8px] text-yellow-600 bg-yellow-50 dark:text-yellow-400 dark:bg-yellow-900/30">
                        PAUSED
                      </span>
                    ) : (
                      <span className="text-xs font-bold font-outfit uppercase tracking-wider px-2 py-1 rounded-[8px] text-green-600 bg-green-50 dark:text-green-400 dark:bg-green-900/30">
                        {activity.event_type === "Approved" ||
                        activity.event_type === "APPROVED"
                          ? "APPROVED"
                          : activity.event_type}
                      </span>
                    )}
                  </div>
                  <h3 className="text-md font-semibold font-sans text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug break-words">
                    {(() => {
                      try {
                        const p =
                          typeof activity.payload === "string"
                            ? JSON.parse(activity.payload)
                            : activity.payload;
                        // Fallback logic specific to Paused state that gets stored inside proposed_content
                        if (
                          p?.action?.proposed_content?.includes(
                            "System is paused",
                          ) || p?.original_payload?.proposed_content?.includes(
                            "System is paused",
                          )
                        ) {
                          return p?.action?.proposed_content || p?.original_payload?.proposed_content;
                        }
                        return (
                          p?.context?.description || p?.original_payload?.description || "Action completed"
                        );
                      } catch (e) {
                        return "Action completed";
                      }
                    })()}
                  </h3>
                  <span className="text-xs text-gray-500 font-sans">
                    {new Date(activity.created_at).toLocaleString()}
                  </span>
                </div>
              ))}
            </div>
          </>
        )}
      </div>
    </section>
  );
}
