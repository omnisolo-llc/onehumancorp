'use client';

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';
import { ProposalDraftCard } from '../../components/feed/ProposalDraftCard';
import { AgentActionCard } from '../../components/feed/AgentActionCard';


interface FeedItem {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload?: any;
  proposed_action?: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
}

export default function FeedPage() {
  const router = useRouter();
  const [items, setItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [processingId, setProcessingId] = useState<string | null>(null);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editValue, setEditValue] = useState<string>('');

  useEffect(() => {
    async function fetchFeed() {
      try {
        const res = await fetch('/api/agent-feed');
        if (!res.ok) {
          throw new Error('Failed to fetch feed');
        }
        const data = await res.json();
        // Only show pending items on this feed view
        setItems((data.items || []).filter((i: any) => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED"));
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }

    fetchFeed();

    let ws: WebSocket;
    let reconnectTimeout: NodeJS.Timeout;

    const connect = () => {
      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const isLocalhost = window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1';
      // In production, Next.js proxy doesn't support WS well so we route directly to backend. Local dev also hits backend directly.
      const wsUrl = isLocalhost ? `ws://127.0.0.1:18789/api/v1/feed/ws` : `${protocol}//${window.location.host}/api/v1/feed/ws`;
      ws = new WebSocket(wsUrl);

      ws.onmessage = (event) => {
        try {
          const item = JSON.parse(event.data);
          if (item.error) {
             console.error("Agent feed WS error:", item.error);
             return;
          }

          if (!item?.id) return;

          if (String(item.lifecycle_state || '').toUpperCase() === 'PENDING_APPROVAL') {
            setItems((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);
          } else if (String(item.lifecycle_state || '').toUpperCase() === 'APPROVED' || String(item.lifecycle_state || '').toUpperCase() === 'DISMISSED') {
            setItems((current) => current.filter((existing) => existing.id !== item.id));
          } else if (String(item.status || '').toUpperCase() === 'DRAFT' || String(item.status || '').toUpperCase() === 'PENDING') {
            setItems((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);
          } else if (item.status) {
             setItems((current) => current.filter((existing) => existing.id !== item.id));
          }
        } catch (err) {
          console.error('Failed to parse websocket feed event:', err);
        }
      };

      ws.onclose = () => {
        reconnectTimeout = setTimeout(connect, 3000);
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

  const startEditing = (item: FeedItem) => {
    setEditingId(item.id);
    const isAmbassador = item.proposed_action?.feature_type === 'ambassador_reply' || item.context_payload?.feature_type === 'ambassador_reply';
    const textToEdit = isAmbassador ?
        (item.proposed_action || item.context_payload)?.generated_response || (item.proposed_action || item.context_payload)?.draft_reply :
        (item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.');
    setEditValue(textToEdit || "");
  };

  const saveEdit = async (id: string) => {
    const item = items.find(i => i.id === id);
    if (!item) return;

    const isAmbassador = item.proposed_action?.feature_type === 'ambassador_reply' || item.context_payload?.feature_type === 'ambassador_reply';

    const updatedProposed = {
        ...item.proposed_action,
        description: isAmbassador ? item.proposed_action?.description : editValue,
        generated_response: isAmbassador ? editValue : item.proposed_action?.generated_response,
    };

    const updatedContext = {
        ...item.context_payload,
        summary: isAmbassador ? item.context_payload?.summary : editValue,
        generated_response: isAmbassador ? editValue : item.context_payload?.generated_response,
    };

    setItems((prev) => prev.map((i) => {
      if (i.id === id) {
        return { ...i, proposed_action: updatedProposed, context_payload: updatedContext };
      }
      return i;
    }));

    if (isAmbassador) {
       await handleAction(id, 'APPROVED', updatedProposed, updatedContext);
    } else {
       await handleAction(id, 'PENDING_APPROVAL', updatedProposed, updatedContext);
    }
    setEditingId(null);
  };

  const cancelEdit = () => {
    setEditingId(null);
  };

  const handleAction = async (id: string, state: string, updatedProposed?: any, updatedContext?: any) => {
    const item = items.find(i => i.id === id);
    if (state === 'APPROVED') {
      if (item?.proposed_action?.action_type === 'Draft Quote') {
        router.push(`/quotes/${item.proposed_action.quote_id}`);
        return;
      }
      if (item?.proposed_action?.action_type === 'Draft Booking') {
        // Optimistic UI or fetch the status change
        // For Draft Booking, it confirms it in the backend and maybe we navigate to booking detail or just resolve here.
        // We'll proceed with normal backend request to approve it so `action_router` handles it.
      }
    }

    try {
      setProcessingId(id);

      const bodyPayload: any = { state };
      const proposed = updatedProposed || item?.proposed_action;
      const context = updatedContext || item?.context_payload;

      if (proposed) bodyPayload.proposed_action = proposed;
      if (context) bodyPayload.context_payload = context;

      const res = await fetch(`/api/agent-feed/${id}/state`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(bodyPayload),
      });
      if (!res.ok) throw new Error('Action failed');

      // Update UI optimistically or refetch
      if (state === 'APPROVED' || state === 'DISMISSED') {
          setItems((prev) => prev.filter((item) => item.id !== id));
      }
    } catch (err: any) {
      alert(err.message);
    } finally {
      setProcessingId(null);
    }
  };

  const simulateDisputeDraft = async () => {
    try {
      setLoading(true);
      await fetch('/api/agents/approvals/simulate-dispute-resolution', { method: 'POST' });
      // The websocket should pick it up, but we can also refetch
      const res = await fetch('/api/agent-feed');
      const data = await res.json();
      setItems((data.items || []).filter((i: any) => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED"));
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const simulateAmbassadorDraft = async () => {
    try {
      setLoading(true);
      await fetch('/api/agents/approvals/simulate-ambassador-draft', { method: 'POST' });
      // The websocket should pick it up, but we can also refetch
      const res = await fetch('/api/agent-feed');
      const data = await res.json();
      setItems((data.items || []).filter((i: any) => i.lifecycle_state !== "APPROVED" && i.lifecycle_state !== "DISMISSED"));
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <AppShell title="Daily Work" subtitle="Your daily priorities, coordinated by your team.">
      <div className="w-full max-w-[375px] mx-auto p-4 space-y-4" data-testid="agent-feed">
        {items.find(i => i.proposed_action?.action_type === "Draft Proposal") && <ProposalDraftCard item={items.find(i => i.proposed_action?.action_type === "Draft Proposal") as any} />}
        {loading && (
          <div className="flex justify-center items-center py-12">
            <p className="text-gray-500 font-medium">Checking your feed...</p>
          </div>
        )}

        {error && (
          <div className="glassmorphism p-4 text-center rounded-[16px] backdrop-blur-[30px] backdrop-saturate-[210%]">
            <p className="text-[#FF3B30] dark:text-[#DE1B1B] font-medium mb-2">We couldn't load your feed.</p>
            <p className="text-sm text-gray-500">{error}</p>
          </div>
        )}

        {!loading && !error && items.length === 0 && (
          <div className="glassmorphism flex flex-col items-center justify-center p-12 text-center rounded-[16px] backdrop-blur-[30px] backdrop-saturate-[210%]" data-testid="agent-feed-empty">
            <div className="w-16 h-16 bg-[#e8f7ef] dark:bg-[rgba(23,166,106,0.2)] rounded-full flex items-center justify-center mb-4">
              <svg className="w-8 h-8 text-[#17a66a]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path>
              </svg>
            </div>
            <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-2">You're all caught up!</h3>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              There are no pending actions for you right now. Your team is handling things.
            </p>
          </div>
        )}

        <div className="flex flex-col gap-4">
          {items.map((item) => {
            const isProcessing = processingId === item.id;
            const isAmbassador = item.proposed_action?.feature_type === 'ambassador_reply' || item.context_payload?.feature_type === 'ambassador_reply';
            const ambassadorPayload = isAmbassador ? (item.proposed_action || item.context_payload) : null;
            const isDisputeResolution = item.proposed_action?.feature_type === 'dispute_resolution' || item.context_payload?.feature_type === 'dispute_resolution';
            const disputePayload = isDisputeResolution ? (item.proposed_action || item.context_payload) : null;


            const departmentStr = isDisputeResolution ? 'DISPUTE RESOLUTION' : isAmbassador ? 'CUSTOMER MESSAGE' : item.proposed_action?.action_type === 'Draft Quote' ? 'SMART ESTIMATE' : item.proposed_action?.action_type === 'Draft Follow-up' ? 'DEPOSIT FOLLOW-UP' : item.proposed_action?.action_type === 'Draft Booking' ? 'NEW BOOKING REQUEST' : item.event_source.replace(/_/g, ' ');
            const titleStr = isDisputeResolution
                    ? `Dispute from ${disputePayload?.sender_id || 'Customer'}`
                    : isAmbassador
                    ? `New Message from ${ambassadorPayload.sender_id || 'Customer'}`
                    : item.proposed_action?.title || item.context_payload?.title || 'Action Required';

            const renderContent = () => {
                if (editingId === item.id) {
                    return (
                      <div className="flex flex-col gap-3 w-full animate-fade-in mb-4">
                        <textarea
                          value={editValue}
                          onChange={(e) => setEditValue(e.target.value)}
                          className="w-full min-h-[120px] p-3 text-[13px] bg-white dark:bg-[#1C1C1E] border border-gray-200 dark:border-gray-700 rounded-xl focus:ring-2 focus:ring-[#0066FF] dark:text-white outline-none resize-none transition-all shadow-inner"
                          placeholder="Edit the draft before sending..."
                          autoFocus
                        />
                        <div className="flex gap-2 justify-end">
                          <button
                            onClick={() => setEditingId(null)}
                            className="px-4 min-h-[36px] min-w-[44px] text-[13px] font-medium text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-full transition-colors"
                          >
                            Cancel
                          </button>
                          <button
                            onClick={() => saveEdit(item.id)}
                            className="px-4 min-h-[36px] min-w-[44px] text-[13px] font-medium bg-[#0066FF] text-white hover:bg-[#0052CC] rounded-full shadow-sm transition-colors"
                          >
                            Save changes
                          </button>
                        </div>
                      </div>
                    );
                }

                if (isDisputeResolution) {
                      return (
                      <div className="flex flex-col gap-3">
                        <div className="bg-[#FFF5E5] dark:bg-[rgba(255,149,0,0.1)] p-3 rounded-lg border border-[#FFD699] dark:border-[rgba(255,149,0,0.2)]">
                          <p className="text-[13px] text-gray-800 dark:text-gray-200 italic mb-2">"{disputePayload?.issue_description || 'Customer reported an issue.'}"</p>
                          <div className="flex flex-col gap-2 mt-3 pt-3 border-t border-[#FFD699] dark:border-[rgba(255,149,0,0.2)]">
                            <p className="text-[11px] font-bold text-[#FF9500] uppercase">Proposed Resolution</p>
                            {disputePayload?.refund_amount > 0 && (
                              <div className="flex items-center gap-3 p-3 bg-white dark:bg-[#1C1C1E] rounded-[12px] shadow-sm">
                                <input type="checkbox" defaultChecked className="w-4 h-4 text-[#FF9500] rounded border-gray-300 focus:ring-[#FF9500]" />
                                <span className="text-[13px] text-gray-800 dark:text-gray-200 font-medium">Issue ${disputePayload?.refund_amount} Refund</span>
                              </div>
                            )}
                            {disputePayload?.operational_action && (
                              <div className="flex items-center gap-3 p-3">
                                <input type="checkbox" defaultChecked className="w-4 h-4 text-[#FF9500] rounded border-gray-300 focus:ring-[#FF9500]" />
                                <span className="text-[13px] text-gray-800 dark:text-gray-200 font-medium">{disputePayload?.operational_action}</span>
                              </div>
                            )}
                          </div>
                        </div>
                      </div>
                    );
                } else if (isAmbassador) {
                      return (
                      <div className="flex flex-col gap-3">
                        <div className="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border border-gray-100 dark:border-gray-700">
                          <p className="text-[13px] text-gray-700 dark:text-gray-300 italic mb-1">"{ambassadorPayload.original_message}"</p>
                          {ambassadorPayload.past_orders && (
                            <span className="inline-block text-[10px] font-semibold text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30 px-2 py-0.5 rounded-full mt-1">
                              {ambassadorPayload.past_orders}
                            </span>
                          )}
                        </div>
                        <div>
                          <p className="text-[11px] font-bold text-gray-500 uppercase mb-1">Agent Draft</p>
                          <p className="text-[13px] text-gray-900 dark:text-white leading-relaxed">
                            {ambassadorPayload.generated_response}
                          </p>
                        </div>
                      </div>
                    );
                } else {
                      return (
                      <p className="text-[13px] text-gray-600 dark:text-gray-300 leading-relaxed mb-2">
                        {item.proposed_action?.action_type === 'Draft Quote'
                          ? (item.context_payload?.context || 'AI has drafted a new estimate based on recent customer inquiry.')
                          : item.proposed_action?.action_type === 'Draft Booking'
                          ? (item.context_payload?.context || 'AI has locked in a tentative time slot based on recent customer inquiry.')
                          : (item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.')}
                      </p>
                    );
                }
            };

            const primaryLabel = isDisputeResolution ? 'Approve & Resolve' : isAmbassador ? 'Send Draft' : item.proposed_action?.action_type === 'Draft Quote' ? 'Review Estimate' : item.proposed_action?.action_type === 'Draft Follow-up' ? 'Send Follow-up' : item.proposed_action?.action_type === 'Draft Booking' ? 'Approve & Confirm' : 'Approve';

            return (
              <AgentActionCard
                key={item.id}
                id={item.id}
                department={departmentStr}
                title={titleStr}
                content={renderContent()}
                isProcessing={isProcessing}
                timestamp={new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                primaryAction={(!editingId || editingId !== item.id) ? {
                  label: primaryLabel,
                  onClick: () => handleAction(item.id, 'APPROVED'),
                  testId: isDisputeResolution ? 'feed-approve-resolve-btn' : 'feed-approve-btn'
                } : {
                  label: 'Save Changes',
                  onClick: () => saveEdit(item.id),
                  testId: 'feed-save-btn'
                }}
                secondaryAction={(!editingId || editingId !== item.id) ? {
                  label: 'Edit',
                  onClick: () => startEditing(item),
                  testId: 'feed-edit-btn'
                } : undefined}
                tertiaryAction={(!editingId || editingId !== item.id) ? {
                  label: 'Dismiss',
                  onClick: () => handleAction(item.id, 'DISMISSED'),
                  testId: 'feed-dismiss-btn'
                } : undefined}
              />
            );
          })}
        </div>

        {/* Hidden test button to trigger simulation easily during development/testing */}
        <div className="pt-8 opacity-20 hover:opacity-100 transition-opacity flex justify-center gap-2">
          <button
             onClick={simulateAmbassadorDraft}
             data-testid="simulate-ambassador-btn"
             className="text-xs bg-gray-200 text-gray-600 px-3 py-1 rounded min-h-[44px] min-w-[44px]"
          >
            Simulate Ambassador Draft
          </button>
          <button
             onClick={simulateDisputeDraft}
             data-testid="simulate-dispute-btn"
             className="text-xs bg-[#FFF5E5] text-[#FF9500] border border-[#FFD699] px-3 py-1 rounded min-h-[44px] min-w-[44px]"
          >
            Simulate Dispute
          </button>
        </div>
      </div>
    </AppShell>
  );
}
