"use client";


import { currentVerifiedQueueOwner, currentVerifiedQueueLease, hasVerifiedOfflineQueueOwner, readQueueOwner, sameOwner, subscribeQueueIdentityReadiness, QUEUE_IDENTITY_EPOCH_KEY, type QueueOwner } from '@/lib/sync/queueIdentity';
import { subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { errorMessage } from '@/lib/errors';
import { useEffect, useState, useMemo, useRef } from "react";
import GrowthReferralWidget from "../components/GrowthReferralWidget";
import { enqueueAction, getActions } from "../utils/offlineQueue";
import { SyncManager } from "../../lib/sync/SyncManager";
import { AmbassadorReplyCard } from "./AmbassadorReplyCard";
import { InstagramDMCard } from "./InstagramDMCard";
import { AgentActionCard } from "../../components/feed/AgentActionCard";
import { GroupedAgentActionCard } from "../../components/feed/GroupedAgentActionCard";



import type { AgentFeedItem, AgentFeedData, ActivityItem } from '@/lib/agent-feed-types';









export function UnifiedAgentFeed({ initialData }: { initialData?: AgentFeedData }) {
  const hasFetchedCanonicalFeedRef = useRef(false);
  const initialDataRetiredRef = useRef(false);
  const decidedIdsRef = useRef<Set<string>>(new Set());
  const pendingDecisionIdsRef = useRef<Set<string>>(new Set());
  const unconfirmedDecisionIdsRef = useRef<Set<string>>(new Set());
  const decisionTimers = useRef(new Set<ReturnType<typeof setTimeout>>());
  const decisionEpoch = useRef(0);
  const [decisionStatus, setDecisionStatus] = useState('');
  const [items, setItems] = useState<AgentFeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [activeTab, setActiveTab] = useState<"proposals" | "activity">(
    "proposals",
  );
  const [activities, setActivities] = useState<ActivityItem[]>(initialData?.activity || []);
  const [chatInput, setChatInput] = useState("");
  const [chatNotice, setChatNotice] = useState('');
  const [chatReady, setChatReady] = useState(() => currentVerifiedQueueOwner() !== null);
  const chatScope = useRef<{ owner: QueueOwner | null; storageEpoch: string | null; expiresAt: number } | null>(null);
  const chatExpiry = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  const retireChat = () => {
    clearTimeout(chatExpiry.current); chatScope.current = null; setChatInput(''); setChatNotice(''); setChatReady(currentVerifiedQueueOwner() !== null);
  };
  useEffect(() => {
    const retire = () => {
      clearTimeout(chatExpiry.current); chatScope.current = null; setChatInput(''); setChatNotice(''); setChatReady(currentVerifiedQueueOwner() !== null);
    };
    const unsubscribe = subscribeOnboardingInvalidation(retire);
    const unsubscribeReadiness = subscribeQueueIdentityReadiness(() => {
      const binding = chatScope.current;
      if (!binding) { setChatReady(currentVerifiedQueueOwner() !== null); return; }
      if (binding.expiresAt <= Date.now()) { retire(); setChatNotice('Your unsent draft expired. Enter a new draft after verifying your session.'); return; }
      try { if (binding.storageEpoch !== localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY)) { retire(); return; } }
      catch { retire(); return; }
      const current = currentVerifiedQueueOwner();
      if (!current) { setChatReady(false); return; }
      // Text entered before identity was established is never assigned to the
      // first verified login. Only a previously bound same owner can recover it.
      if (!binding.owner || !sameOwner(binding.owner, current)) retire();
      else setChatReady(true);
    });
    // This editor must establish its own lease before accepting private text;
    // it cannot depend on another dashboard widget finishing verification first.
    if (!currentVerifiedQueueOwner()) void readQueueOwner().catch(() => {});
    return () => { clearTimeout(chatExpiry.current); unsubscribe(); unsubscribeReadiness(); };
  }, []);

  const editChat = (value: string) => {
    const lease = currentVerifiedQueueLease();
    const current = lease?.owner;
    if (!current) { setChatReady(false); return; }
    const previous = chatScope.current;
    if (previous?.owner && (!current || !sameOwner(previous.owner, current))) { retireChat(); return; }
    try {
      const storageEpoch = localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY);
      if (previous && (previous.expiresAt <= Date.now() || previous.storageEpoch !== storageEpoch)) { retireChat(); return; }
      const binding = previous ?? { owner: current, storageEpoch, expiresAt: lease!.expiresAt };
      chatScope.current = binding;
      clearTimeout(chatExpiry.current);
      chatExpiry.current = setTimeout(() => {
        if (chatScope.current === binding) { retireChat(); setChatNotice('Your unsent draft expired. Enter a new draft after verifying your session.'); }
      }, Math.min(binding.expiresAt - Date.now(), 2_147_483_647));
      setChatInput(value); setChatNotice(''); setChatReady(true);
    } catch { retireChat(); }
  };
  const handleSendChatMessage = (event: React.FormEvent) => {
    event.preventDefault();
    if (!chatInput.trim()) return;
    const binding = chatScope.current;
    try {
      if (!binding?.owner || !chatReady || binding.expiresAt <= Date.now() || binding.storageEpoch !== localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY)
        || !hasVerifiedOfflineQueueOwner(binding.owner)) {
        retireChat(); return;
      }
    } catch { retireChat(); return; }
    setChatNotice('Memory chat is unavailable. Your draft has not been sent or saved.');
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

  useEffect(() => {
    const clearTimers = () => {
      for (const timer of decisionTimers.current) clearTimeout(timer);
      decisionTimers.current.clear();
    };
    const retireDecisions = () => {
      ++decisionEpoch.current;
      clearTimers();
      pendingDecisionIdsRef.current.clear();
      unconfirmedDecisionIdsRef.current.clear();
      decidedIdsRef.current.clear();
      hasFetchedCanonicalFeedRef.current = true;
      initialDataRetiredRef.current = true;
      setQueuedActionIds(new Set()); setOfflineActionsCount(0);
      setItems([]); setActivities([]); setEditingId(null); setEditContent('');
      setEditQuotePrice(''); setEditQuoteScope(''); setDecisionStatus('');
    };
    const unsubscribe = subscribeOnboardingInvalidation(retireDecisions);
    return () => { ++decisionEpoch.current; clearTimers(); unsubscribe(); };
  }, []);

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
      const generation = decisionEpoch.current;
      try {
        const actions = await getActions();
        if (generation !== decisionEpoch.current) return;
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
      await SyncManager.getInstance().sync();
      await updateOfflineCount();
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
      const generation = decisionEpoch.current;
      try {
        if (!refresh) {
          setError("");
          setLoading(true);
          setActivityLoading(true);
        }
        // Props from the retired account remain stale even if a parent renders
        // them again. After invalidation, only fresh authenticated reads apply.
        const aggregate = initialDataRetiredRef.current ? undefined : initialData;
        let unifiedData = aggregate;

        // A late dashboard aggregate may be older than our own completed read.
        // Once we have read the canonical feed, revalidate it instead of letting
        // new aggregate props replace its rows with stale or partial projections.
        if (refresh || hasFetchedCanonicalFeedRef.current || !unifiedData || !unifiedData.items || unifiedData.items.length === 0) {
          const unifiedRes = await fetch("/api/v1/agent-feed");
          if (!unifiedRes.ok) {
            throw new Error("Feed temporarily unavailable");
          }
          const refreshedData = await unifiedRes.json();
          if (mounted) hasFetchedCanonicalFeedRef.current = true;
          unifiedData = aggregate
            ? {
                ...aggregate,
                ...refreshedData,
                items: refreshedData.items || aggregate.items || [],
                priority_tasks:
                  refreshedData.priority_tasks === undefined
                    ? aggregate.priority_tasks
                    : refreshedData.priority_tasks,
                triage:
                  refreshedData.triage === undefined
                    ? aggregate.triage
                    : refreshedData.triage,
              }
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
              // The aggregate triage endpoint also projects agent_feed_items.
              // Keep the canonical row and its lifecycle state when both reads
              // include the same tenant/record; a stale projection must not
              // create a second action or reopen an approved decision.
              const canonicalRecords = new Set(unifiedData.items.map(
                (item) => JSON.stringify([item.tenant_id, item.id]),
              ));
              combinedItems = [
                ...combinedItems,
                ...unifiedData.triage.filter(
                  (item) => !canonicalRecords.has(JSON.stringify([item.tenant_id, item.id])),
                ).slice(0, 3).map((ti) => {
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

            if (!mounted || generation !== decisionEpoch.current) return;
            setItems((previous) => [
              ...previous.filter((item) => pendingDecisionIdsRef.current.has(item.id) || unconfirmedDecisionIdsRef.current.has(item.id)),
              ...parsedCombinedItems.filter(
                (i) =>
                  !decidedIdsRef.current.has(i.id) &&
                  !pendingDecisionIdsRef.current.has(i.id) &&
                  !unconfirmedDecisionIdsRef.current.has(i.id) &&
                  i.lifecycle_state !== "APPROVED" &&
                  i.lifecycle_state !== "DISMISSED" &&
                  i.lifecycle_state !== "PAUSED",
              ),
            ]);

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

    const expectedItem = items.find(item => item.id === id);
    if (!expectedItem?.tenant_id) throw new Error("Decision was not sent. The proposal's tenant could not be verified.");
    const legacy = event_source === "triage" || event_source === "task" || event_source === "order";
    const state = approved ? "APPROVED" : "DISMISSED";
    const res = await fetch(legacy ? "/api/v1/triage/action" : `/api/v1/agent-feed/${id}`, {
      method: legacy ? "POST" : "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(legacy
        ? { triage_item_id: id, approved, edited_payload: modified_content }
        : { state, modified_content }),
    });
    if (!res.ok) throw new Error("Failed to submit decision");
    const value: unknown = await res.json().catch(() => null);
    const record = (value: unknown): Record<string, unknown> | null => value !== null && typeof value === 'object' && !Array.isArray(value) ? value as Record<string, unknown> : null;
    const receipt = record(value), stored = legacy ? record(receipt?.item) : receipt;
    let confirmed = res.status === 200 && receipt?.decision_recorded === true && receipt.error == null
      && (!('success' in receipt) || receipt.success === true)
      && (!legacy || receipt.success === true)
      && stored?.id === id && stored.tenant_id === expectedItem.tenant_id && stored.lifecycle_state === state;
    if (confirmed && modified_content !== undefined) {
      if (legacy) confirmed = stored?.edited_payload === modified_content;
      else {
        const payload = record(stored?.proposed_action);
        const content = payload && ['draft_reply', 'generated_response', 'summary', 'message', 'draft_message', 'draft_action']
          .filter(key => typeof payload[key] === 'string').map(key => payload[key]);
        confirmed = !!content?.length && content.every(text => text === modified_content);
      }
    }
    if (!confirmed) throw new Error("Outcome unconfirmed. Your card and draft are retained. Check recorded decisions before retrying.");
  };

  const handleDecision = async (
    id: string,
    approved: boolean,
    modified_content?: string,
    event_source?: string,
  ): Promise<boolean> => {
    if (pendingDecisionIdsRef.current.has(id) || decidedIdsRef.current.has(id)) return false;
    const generation = decisionEpoch.current;
    const expectedOwner = currentVerifiedQueueOwner();
    const expectedTenant = items.find(item => item.id === id)?.tenant_id;
    pendingDecisionIdsRef.current.add(id);
    setError("");
    setDecisionStatus(isOffline ? "Saving decision to the offline queue..." : "Waiting for the recorded decision. No execution or delivery is confirmed.");
    try {
      if (isOffline) {
        await enqueueAction({
          id: crypto.randomUUID(),
          type: "approve_agent_feed",
          payload: { id, approved, modified_content, event_source },
          timestamp: Date.now(),
        });
        if (generation !== decisionEpoch.current) return false;
        setOfflineActionsCount((prev) => prev + 1);
        setQueuedActionIds((prev) => new Set(prev).add(id));
        unconfirmedDecisionIdsRef.current.delete(id);
        setDecisionStatus("Decision queued offline. Approval or dismissal is not yet recorded.");
        decidedIdsRef.current.add(id);
        setItems((prev) => prev.filter((item) => item.id !== id));
        return false;
      }
      await submitDecision(id, approved, modified_content, event_source);
      if (generation !== decisionEpoch.current) return false;
      const currentOwner = currentVerifiedQueueOwner();
      if (expectedOwner && (!currentOwner || !sameOwner(expectedOwner, currentOwner))) {
        throw new Error("Your session changed. The decision outcome must be checked in the original account.");
      }
      unconfirmedDecisionIdsRef.current.delete(id);
      setDecisionStatus(event_source === "review" ? "Review request accepted. Delivery is not verified." : approved
        ? "Approval recorded. Execution or delivery is not verified by this decision."
        : "Dismissal recorded.");
      decidedIdsRef.current.add(id);
      // Return the verified acknowledgment to the card before its exit animation.
      const timer = setTimeout(() => {
        decisionTimers.current.delete(timer);
        pendingDecisionIdsRef.current.delete(id);
        if (generation !== decisionEpoch.current) return;
        const currentOwner = currentVerifiedQueueOwner();
        // The matching receipt already confirmed this decision. Other widgets
        // temporarily clear cached identity while revalidating the same owner;
        // that does not cancel cleanup of this exact tenant/record. Account
        // invalidation is fenced above, and a verified different owner below.
        if (expectedOwner && currentOwner && !sameOwner(expectedOwner, currentOwner)) return;
        setItems(previous => previous.filter(item => item.id !== id || item.tenant_id !== expectedTenant));
      }, 500);
      decisionTimers.current.add(timer);
      return true;
    } catch (err) {
      if (generation !== decisionEpoch.current) return false;
      unconfirmedDecisionIdsRef.current.add(id);
      setDecisionStatus("");
      setError(errorMessage(err, "Outcome unconfirmed. Your card and draft are retained."));
      return false;
    } finally {
      if (!decidedIdsRef.current.has(id)) pendingDecisionIdsRef.current.delete(id);
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
        <p className="mb-2 text-sm text-gray-600 dark:text-gray-300">Memory chat is not configured.</p>
        {chatNotice && <p role="alert" className="mb-2 text-sm text-gray-600 dark:text-gray-300">{chatNotice}</p>}
        {!chatReady && <div><p>Verify your current session to view this unsent draft.</p><button type="button" onClick={() => { void readQueueOwner().catch(() => { setChatReady(false); }); }}>Reverify draft access</button></div>}
        <form onSubmit={handleSendChatMessage} className="flex gap-2">
          <input
            type="text"
            placeholder="Message..."
            value={chatReady ? chatInput : ''}
            disabled={!chatReady}
            onChange={(e) => editChat(e.target.value)}
            className="flex-1 px-3 py-2 text-sm rounded-lg border border-gray-200 dark:border-gray-700 bg-transparent text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            type="submit"
            disabled={!chatReady || !chatInput.trim()}
            className="px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition"
          >
            Send
          </button>
        </form>
      </div>
      {decisionStatus && <p role="status" aria-label="Decision status" className="mb-4 text-sm">{decisionStatus}</p>}
      {error && (
        <div role="alert" className="w-full mb-6 p-4 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[#FF3B30] text-[#FF3B30] text-center">
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
          aria-pressed={activeTab === "proposals"}
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
          aria-pressed={activeTab === "activity"}
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
            {!loading && !error && items.length === 0 && (
              <div
                className="w-full flex flex-col items-center gap-6 p-6 rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm  shadow-sm opacity-90 text-center"
                data-testid="triage-feed-empty"
              >
                <div className="text-3xl mb-2">✨</div>
                <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                  No pending proposals are recorded.
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

                const isInstagramDM = approval.event_source === "instagram_dm" || propAction?.feature_type === "instagram_dm" || ctxPayload?.feature_type === "instagram_dm";

                if (isInstagramDM) {

                  return (

                    <InstagramDMCard

                      key={approval.id}

                      approval={approval}

                      onApprove={() => handleDecision(approval.id, true)}

                      onDismiss={() => handleDecision(approval.id, false)}

                    />

                  );

                }

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
                {items.length} recorded proposal{items.length === 1 ? "" : "s"} in this feed.
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
