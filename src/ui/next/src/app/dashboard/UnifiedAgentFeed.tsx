"use client";

import { useEffect, useState } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { enqueueAction, getActions, removeAction } from "../utils/offlineQueue";

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
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">("proposals");
  const [activities, setActivities] = useState<OHCLedgerEntry[]>([]);
  const [activityLoading, setActivityLoading] = useState(false);
  const [isOffline, setIsOffline] = useState(false);

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
    setIsOffline(!navigator.onLine);

    const handleOnline = async () => {
      setIsOffline(false);
      // Sync queued offline actions
      try {
        const actions = await getActions();
        for (const action of actions) {
          if (action.type === 'approve_agent_feed') {
            await submitDecision(action.id, action.payload.approved);
            await removeAction(action.id);
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

    async function fetchAll() {
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

  const submitDecision = async (id: string, approved: boolean) => {
    const tenant = tenantId();
    const res = await fetch(`/api/agents/approvals/${id}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-tenant-id": tenant,
        "x-user-id": "default",
      },
      body: JSON.stringify({ approved }),
    });

    if (!res.ok) {
      throw new Error("Failed to submit decision");
    }
  };

  const handleDecision = async (id: string, approved: boolean) => {
    // Optimistic UI update
    setItems(prev => prev.filter(app => app.id !== id));

    if (isOffline) {
      // Enqueue offline action
      await enqueueAction({
        id: crypto.randomUUID(),
        type: 'approve_agent_feed',
        payload: { id, approved },
        timestamp: Date.now()
      });
      return;
    }

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
      <div className="w-full mb-6 p-4 glassmorphism rounded-[16px] border border-red-500/50 bg-red-500/10 text-red-500 text-center">
        {error}
      </div>
    );
  }

  return (
    <section className="mb-6 max-w-[375px] w-full mx-auto sm:max-w-none" aria-label="Unified Agent Feed">
      {isOffline && (
        <div className="mb-4 w-full p-2 glassmorphism rounded-[8px] bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200 text-center text-sm font-semibold flex items-center justify-center gap-2">
          <span>📡</span> You are offline. Actions will sync when online.
        </div>
      )}
      <div className="mb-4 flex items-center border-b border-gray-200 dark:border-gray-700">
        <button
          onClick={() => setActiveTab("proposals")}
          className={`flex-1 py-3 text-center text-sm font-semibold transition-all duration-200 ${
            activeTab === "proposals"
              ? "border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Proposals ({items.length})
        </button>
        <button
          onClick={() => setActiveTab("activity")}
          className={`flex-1 py-3 text-center text-sm font-semibold transition-all duration-200 ${
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
            <div className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4">
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

            <div className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4">
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
                  className="flex-1 min-h-[44px] rounded-lg font-bold text-sm bg-green-500 hover:bg-green-600 text-white shadow-sm transition-transform active:scale-[0.98]"
                >
                  Approve
                </button>
                <button
                  className="flex-1 min-h-[44px] rounded-lg font-bold text-sm bg-gray-200 dark:bg-gray-800 hover:bg-gray-300 dark:hover:bg-gray-700 text-[#1D1D1F] dark:text-[#F5F5F7] transition-transform active:scale-[0.98]"
                >
                  Edit
                </button>
                <button
                  className="flex-1 min-h-[44px] rounded-lg font-bold text-sm bg-red-100 hover:bg-red-200 text-red-600 dark:bg-red-900/30 dark:hover:bg-red-900/50 dark:text-red-400 transition-transform active:scale-[0.98]"
                >
                  Deny
                </button>
              </div>
            </div>

            {loading && (
              <div className="w-full p-4 glassmorphism rounded-[16px] text-center text-gray-500">
                Loading Agent Proposals...
              </div>
            )}
            {!loading && items.length === 0 && (
              <div className="w-full flex flex-col items-center gap-6 p-6 glassmorphism rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm opacity-90 text-center">
                <div className="text-3xl mb-2">✨</div>
                <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up!</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
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
                className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4"
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
                  </div>
                  <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1">
                    {(approval.context_payload?.description || approval.proposed_action?.message || approval.proposed_action?.action_type || approval.event_source)}
                  </h3>
                  {((approval.proposed_action || approval.context_payload)?.context || (approval.proposed_action || approval.context_payload)?.remaining_stock !== undefined || (approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" || (approval.proposed_action || approval.context_payload)?.feature_type === "social_post_draft") && (
                    <div className="mt-2 flex flex-col gap-1 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
                      {(approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" && (
                        <div className="mb-4 p-4 rounded-xl glassmorphism border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="quote-draft-card">
                          <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
                            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                            </svg>
                            Draft Quote: {(approval.proposed_action || approval.context_payload).service || 'Plumbing Fix'} for Customer
                          </div>
                          <div className="text-xs text-[#0066FF] dark:text-blue-400 font-medium">
                            {(approval.proposed_action || approval.context_payload).customer_inquiry}
                          </div>
                          <div className="glassmorphism dark:bg-gray-800 p-3 rounded-lg border border-white/40 dark:border-white/10 relative mt-2">
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
                  {(approval.proposed_action || approval.context_payload)?.feature_type === 'social_post_draft' ? (
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
                  ) : (approval.proposed_action || approval.context_payload)?.feature_type === "quote_draft" ? (
                    <div className="flex flex-col sm:flex-row gap-3 w-full">
                      <button
                        onClick={() => handleDecision(approval.id, true)}
                        className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                        aria-label="Approve & Send Proposal"
                        data-testid="approve-quote-draft"
                      >
                        Approve & Send Proposal
                      </button>
                      <a
                        href={`/quoting?id=${approval.id}`}
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
                        aria-label="Approve & Send Proposal"
                        data-testid="approve-send-proposal"
                      >
                        Approve & Send Proposal
                      </button>
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <a
                          href={`/quoting?id=${approval.id}`}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit Draft"
                          data-testid="edit-proposal"
                        >
                          Edit Draft
                        </a>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Ask Agent to Adjust"
                          data-testid="reject-proposal"
                        >
                          Ask Agent to Adjust
                        </button>
                      </div>
                    </>
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
                          onClick={() => {}}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit proposal"
                          data-testid="edit-proposal"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleDecision(approval.id, false)}
                          className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Reject proposal"
                          data-testid="reject-proposal"
                        >
                          Deny
                        </button>
                      </div>
                    </>
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
                <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  No recent activity found.
                </p>
              </div>
            )}
            <div className="flex flex-col gap-3 min-w-[320px] max-w-full">
            {activities.map((activity) => (
              <div
                key={activity.id}
                className="glassmorphism p-5 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-3 opacity-90 min-h-[44px]"
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
