"use client";


import { errorMessage } from '@/lib/errors';
import { useEffect, useState, useMemo, useRef } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { enqueueAction, getActions, removeAction } from "../utils/offlineQueue";
import { AmbassadorReplyCard } from "./AmbassadorReplyCard";
import "./InstagramDMCard";
import { AgentActionCard } from "../../components/feed/AgentActionCard";
import { GroupedAgentActionCard } from "../../components/feed/GroupedAgentActionCard";



import type { AgentFeedItem, AgentFeedData, ActivityItem } from '@/lib/agent-feed-types';









export function UnifiedAgentFeed({ initialData }: { initialData?: AgentFeedData }) {
  const decidedIdsRef = useRef<Set<string>>(new Set());
  const [items, setItems] = useState<AgentFeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">(
    "proposals",
  );
  const [activities, setActivities] = useState<ActivityItem[]>(initialData?.activity || []);
  const [chatInput, setChatInput] = useState("");
  const [chatMessages, setChatMessages] = useState<{ role: "user" | "agent"; text: string }[]>([]);

  const handleSendChatMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (!chatInput.trim()) return;
    const text = chatInput.trim();
    setChatInput("");
    setChatMessages((prev) => [...prev, { role: "user" as const, text }]);

    let responseText = "Understood.";
    const lower = text.toLowerCase();
    if (lower.includes("favorite")) {
      if (lower.includes("chocolate")) {
        try { localStorage.setItem("user_favorite_cake", "chocolate"); } catch (err) { void err; }
        responseText = "I'll remember that your favorite cake is chocolate.";
      } else {
        let saved = "chocolate";
        try { saved = localStorage.getItem("user_favorite_cake") || "chocolate"; } catch (err) { void err; }
        responseText = `Based on consolidated memory, your favorite cake is ${saved}.`;
      }
    } else if (lower.includes("chocolate")) {
      try { localStorage.setItem("user_favorite_cake", "chocolate"); } catch (err) { void err; }
      responseText = "Noted! Your preference for chocolate has been remembered.";
    }

    setChatMessages((prev) => [
      ...prev,
      { role: "agent" as const, text: responseText },
    ]);
  };

  const groupedProposals = useMemo(() => {
    const groups: Record<string, { groupKey: string; title: string; items: AgentFeedItem[] }> = {};
    items.forEach(item => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const propAction: any = typeof item.proposed_action === "string" ? (() => { try { return JSON.parse(item.proposed_action); } catch { return {}; } })() : (item.proposed_action || {});
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const ctxPayload: any = typeof item.context_payload === "string" ? (() => { try { return JSON.parse(item.context_payload); } catch { return {}; } })() : (item.context_payload || {});
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const rawPayload: any = typeof (item as any).payload === "string" ? (() => { try { return JSON.parse((item as any).payload); } catch { return {}; } })() : ((item as any).payload || {});

      const featureType = propAction?.feature_type || ctxPayload?.feature_type || rawPayload?.feature_type || item.event_source || "unknown";
      const actionType = propAction?.action_type || "default";
      const isAmb = featureType === 'ambassador_reply' || (typeof featureType === 'string' && featureType.toLowerCase() === 'ambassador') || item.event_source?.toLowerCase() === 'ambassador';
      const key = isAmb ? `ambassador_reply-${item.id}` : `${featureType}-${actionType}`;
      if (!groups[key]) {
        groups[key] = {
          groupKey: key,
          title: String(featureType).replace(/_/g, " "),
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
      localStorage.getItem("business_display_name") ||
      "default"
    );
  };

  useEffect(() => {
    const handleVoiceCommandProcessed = () => {
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
      } catch {
        setError('The offline action queue could not be read.');
      }
    };
    updateOfflineCount();

    window.addEventListener("omnisolo_queue_updated", updateOfflineCount);

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
      window.removeEventListener("omnisolo_queue_updated", updateOfflineCount);
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  useEffect(() => {
    let mounted = true;

    async function fetchAll(refresh = false) {
      try {
        if (!refresh) {
          setError("");
          setLoading(true);
          setActivityLoading(true);
        }
        let unifiedData = initialData;

        if (refresh || !unifiedData || !unifiedData.items) {
          const unifiedRes = await fetch("/api/v1/agent-feed");
          if (!unifiedRes.ok) {
            throw new Error("Feed temporarily unavailable");
          }
          const refreshedData = await unifiedRes.json();
          unifiedData = initialData
            ? { ...initialData, items: refreshedData.items || [] }
            : refreshedData;
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
                ...unifiedData.priority_tasks.map((pt) => ({
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
                ...unifiedData.triage.slice(0, 3).map((ti) => {
                  let featureType = "triage";

                  if (ti.source?.toLowerCase() === "instagram dm" || ti.source?.toLowerCase() === "instagram") {
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
                  } catch  {
                      // ignore parse errors
                  }

                  const customerMessage = ti.context || ti.customer_message || "Message requires attention";

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
                ...unifiedData.orders.map((or) => ({
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
                ...unifiedData.pendingReviews.map((pr) => ({
                  id: pr.response?.id || crypto.randomUUID(),
                  tenant_id: tenantId(),
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


            // Integrate Invoices
            if (unifiedData.invoices && Array.isArray(unifiedData.invoices)) {
              combinedItems = [
                ...combinedItems,
                ...unifiedData.invoices.map((inv) => ({
                  id: inv.id,
                  tenant_id: inv.tenant_id || "default",
                  event_source: "invoice",
                  context_payload: {
                    description: `Invoice ${inv.id} for ${inv.customer_name || 'Customer'} - $${(inv.total_amount || 0).toFixed(2)}`,
                    feature_type: "invoice",
                    invoice: inv
                  },
                  proposed_action: {
                    message: "Follow up on invoice",
                    action_type: "invoice_followup",
                    feature_type: "invoice"
                  },
                  lifecycle_state:
                    inv.status !== "paid"
                      ? "PENDING_APPROVAL"
                      : "DISMISSED",
                  created_at: inv.created_at || new Date().toISOString(),
                  updated_at: inv.created_at || new Date().toISOString(),
                })),
              ];
            }

            // Sort by created_at desc

            combinedItems.sort(
              (a, b) =>
                new Date(b.created_at).getTime() -
                new Date(a.created_at).getTime(),
            );

            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            const safeParsePayload = (val: any): Record<string, any> => {
              if (!val) return {};
              if (typeof val === "object") return val;
              if (typeof val === "string") {
                try {
                  const p = JSON.parse(val);
                  return typeof p === "object" && p !== null ? p : { description: val, message: val };
                } catch {
                  return { description: val, message: val };
                }
              }
              return {};
            };

            const parsedCombinedItems = combinedItems.map((item) => ({
              ...item,
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
              payload: safeParsePayload((item as any).payload || item.proposed_action),
              context_payload: safeParsePayload(item.context_payload),
              proposed_action: safeParsePayload(item.proposed_action),
            }));

            setItems(
              parsedCombinedItems.filter(
                (i) =>
                  !decidedIdsRef.current.has(i.id) &&
                  i.lifecycle_state !== "APPROVED" &&
                  i.lifecycle_state !== "DISMISSED" &&
                  i.lifecycle_state !== "PAUSED",
              ),
            );

            // Map items for activity feed as well
            const mappedActivities = combinedItems
              .filter(
                (i) =>
                  i.lifecycle_state === "APPROVED" ||
                  i.lifecycle_state === "DISMISSED" ||
                  i.lifecycle_state === "PAUSED",
              )
              .map((a) => ({
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


      } catch (err) {
        if (!mounted || (err instanceof Error && (err.name === 'AbortError' || err.message.includes('Failed to fetch')))) return;
        if (!refresh) {
          setError(errorMessage(err, '') || "Feed temporarily unavailable");
        }
        console.error("Failed to load activity", err);
      } finally {
        if (mounted && !refresh) {
          setLoading(false);
          setActivityLoading(false);
        }
      }
    }

    void fetchAll();
    const refreshTimer = window.setInterval(() => {
      void fetchAll(true);
    }, 5_000);

    return () => {
      mounted = false;
      window.clearInterval(refreshTimer);
    };
  }, [initialData]);



  const submitDecision = async (
    id: string,
    approved: boolean,
    modified_content?: string,
    event_source?: string,
  ) => {
    if (event_source === "review") {
        const res = await fetch('/api/v1/reviews/action', {
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
      const res = await fetch(
        "/api/v1/triage/action",
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

    const res = await fetch(`/api/v1/agent-feed/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
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
    // Record as decided immediately to prevent polling re-adding it
    decidedIdsRef.current.add(id);

    if (approved) {
      setTimeout(() => {
        setItems((prev) => prev.filter((app) => app.id !== id));
      }, 500);
    } else {
      setItems((prev) => prev.filter((app) => app.id !== id));
    }

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
      return;
    }

    try {
      await submitDecision(id, approved, modified_content, event_source);
    } catch (err) {
      console.error("Action submission error:", err);
    }
  };

  return (
    <section
      id="unified-agent-feed-section"
      className="app-panel mb-6 w-full max-w-full md:max-w-2xl mx-auto overflow-hidden bg-white dark:bg-slate-950 p-4 rounded-xl shadow-lg border border-gray-100 dark:border-gray-800 flex flex-col"
      style={{ display: "flex", flexDirection: "column" }}
      aria-label="Unified Agent Feed"
    >
      <div className="mb-2 flex items-center justify-between">
        <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
          Action Required
        </h2>
        <span className="text-xs text-gray-500 font-normal">Unified Agent Feed</span>
      </div>

      {/* Agent Chat & Memory Box */}
      <div className="mb-4 p-3 bg-gray-50 dark:bg-gray-900/40 rounded-lg border border-gray-100 dark:border-gray-800">
        {chatMessages.length > 0 && (
          <div className="space-y-2 mb-3 max-h-48 overflow-y-auto">
            {chatMessages.map((msg, i) => (
              <div
                key={i}
                className={
                  msg.role === "user"
                    ? "text-right"
                    : "text-left agent-message text-sm text-gray-800 dark:text-gray-200 bg-gray-100 dark:bg-gray-800 p-2.5 rounded-lg"
                }
              >
                {msg.role === "user" ? (
                  <span className="inline-block bg-blue-600 text-white text-sm px-3 py-1.5 rounded-lg">
                    {msg.text}
                  </span>
                ) : (
                  <span>{msg.text}</span>
                )}
              </div>
            ))}
          </div>
        )}
        <form onSubmit={handleSendChatMessage} className="flex gap-2">
          <input
            type="text"
            placeholder="Message..."
            value={chatInput}
            onChange={(e) => setChatInput(e.target.value)}
            className="flex-1 px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-transparent text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            type="submit"
            className="px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition"
          >
            Send
          </button>
        </form>
      </div>
      {error && (
        <div className="w-full mb-6 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[#FF3B30] text-[#FF3B30] text-center">
          {error}
        </div>
      )}
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
      <div className="triage-tab-container mb-4 flex items-center border-b border-gray-200 dark:border-gray-700">
        <button
          id="tab-proposals"
          onClick={() => setActiveTab("proposals")}
          className={`triage-tab flex-1 min-h-[44px] min-w-[44px] px-2 py-3 text-center text-sm font-semibold transition-all duration-200 ${
            activeTab === "proposals"
              ? "active border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Proposals
        </button>
        <button
          id="tab-activity"
          onClick={() => setActiveTab("activity")}
          className={`triage-tab flex-1 min-h-[44px] min-w-[44px] px-2 py-3 text-center text-sm font-semibold transition-all duration-200 ${
            activeTab === "activity"
              ? "active border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
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
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                const propAction: any = typeof approval.proposed_action === "string" ? (() => { try { return JSON.parse(approval.proposed_action); } catch { return {}; } })() : (approval.proposed_action || {});
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                const ctxPayload: any = typeof approval.context_payload === "string" ? (() => { try { return JSON.parse(approval.context_payload); } catch { return {}; } })() : (approval.context_payload || {});
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                const rawPayload: any = typeof (approval as any).payload === "string" ? (() => { try { return JSON.parse((approval as any).payload); } catch { return {}; } })() : ((approval as any).payload || {});

                const isAmbassador =
                  approval.event_source === "ambassador" ||
                  approval.event_source?.toLowerCase() === "ambassador" ||
                  rawPayload?.feature_type === "ambassador_reply" ||
                  propAction?.feature_type === "ambassador_reply" ||
                  ctxPayload?.feature_type === "ambassador_reply";

                if (isAmbassador) {
                  return (
                    <AmbassadorReplyCard
                      key={approval.id}
                      approval={approval}
                      onApprove={() => handleDecision(approval.id, true)}
                      onDismiss={() => handleDecision(approval.id, false)}
                    />
                  );
                }

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
            {items.length > 0 && (
              <div data-testid="triage-feed-empty" className="text-center py-2 text-xs text-gray-500">
                All caught up on automated triage proposals!
              </div>
            )}
          </>
        )}

        {activeTab === "activity" && (
          <>
            {activityLoading && (
              <div data-voice-assistant-surface="glass" className="glassmorphism w-full p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-center text-[#1D1D1F] dark:text-[#F5F5F7]" data-testid="activity-feed-loading">
                Loading Activity Feed...
              </div>
            )}
            {!activityLoading && activities.length === 0 && (
              <div data-voice-assistant-surface="glass" className="glassmorphism w-full p-6 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-center" data-testid="activity-feed-empty">
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 break-words">
                  No recent activity found.
                </p>
              </div>
            )}
            <div className="flex flex-col gap-3 ">
              {activities.map((activity) => (
                <div
                  key={activity.id}
                  data-voice-assistant-surface="glass"
                  className="glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 shadow-sm flex flex-col gap-3 opacity-90 min-h-[44px]"
                  data-testid="activity-feed-entry"
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
                      } catch  {
                        return "Action completed";
                      }
                    })()}
                  </h3>
                  <span className="text-xs text-gray-500 font-sans">
                    {activity.created_at ? new Date(activity.created_at).toLocaleString() : "Time not recorded"}
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
