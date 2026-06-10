"use client";

import { useEffect, useState } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { ActionCard, AgentFeedItem } from "./ActionCard";

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

export function UnifiedAgentFeed() {
  const [items, setItems] = useState<AgentFeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">("proposals");
  const [activities, setActivities] = useState<OHCLedgerEntry[]>([]);
  const [activityLoading, setActivityLoading] = useState(false);

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
    let mounted = true;

    async function fetchAll() {
      try {
        setLoading(true);
        setActivityLoading(true);
        const tenant = tenantId();

        const unifiedRes = await fetch(`/api/agent-feed?tenant_id=${tenant}`, {
          headers: {
            "x-tenant-id": tenant,
            "x-user-id": "default",
          },
        });

        if (!unifiedRes.ok) {
          throw new Error("Failed to load agent feed");
        }

        const unifiedData = await unifiedRes.json();

        if (mounted) {
          if (unifiedData.items) {
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

  const handleDecision = async (id: string, approved: boolean) => {
    // Optimistic UI update
    setItems(prev => prev.filter(app => app.id !== id));

    try {
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
        // If it fails, we might want to fetch again to restore state
        const refreshRes = await fetch(`/api/agents/approvals?tenant_id=${tenant}`, {
            headers: { "x-tenant-id": tenant, "x-user-id": "default" }
        });
        if (refreshRes.ok) {
            const data: ApprovalsResponse = await refreshRes.json();
            setItems(data.pending_approvals);
        }
        throw new Error("Failed to submit decision");
      }
    } catch (err: any) {
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
    <section className="mb-6 w-full" aria-label="Unified Agent Feed">
      <div className="mb-4 flex items-center border-b border-gray-200 dark:border-gray-700">
        <button
          onClick={() => setActiveTab("proposals")}
          className={`flex-1 py-3 text-center text-sm font-semibold transition-colors ${
            activeTab === "proposals"
              ? "border-b-2 border-[#0066FF] text-[#0066FF] dark:text-[#3388FF]"
              : "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          }`}
        >
          Proposals ({items.length})
        </button>
        <button
          onClick={() => setActiveTab("activity")}
          className={`flex-1 py-3 text-center text-sm font-semibold transition-colors ${
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
              <ActionCard
                key={approval.id}
                item={approval}
                onApprove={(id) => handleDecision(id, true)}
                onEdit={(id) => {}}
                onDiscard={(id) => handleDecision(id, false)}
              />
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
