'use client';

import React, { useEffect, useState } from 'react';
import { AppShell } from '../components/AppShell';

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
  const [items, setItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [processingId, setProcessingId] = useState<string | null>(null);
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [editedPayloads, setEditedPayloads] = useState<Record<string, string>>({});

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
      const wsUrl = `${protocol}//${window.location.host}/api/agent-feed/ws`;
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

  const handleAction = async (id: string, state: string, modifiedPayload?: string) => {
    try {
      setProcessingId(id);
      const res = await fetch(`/api/agent-feed/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ state, modified_payload: modifiedPayload }),
      });
      if (!res.ok) throw new Error('Action failed');

      // Update UI optimistically or refetch
      setItems((prev) => prev.filter((item) => item.id !== id));
      if (expandedId === id) setExpandedId(null);
    } catch (err: any) {
      alert(err.message);
    } finally {
      setProcessingId(null);
    }
  };

  const toggleExpand = (id: string, currentDraft: string) => {
    if (expandedId === id) {
      setExpandedId(null);
    } else {
      setExpandedId(id);
      if (!editedPayloads[id]) {
        setEditedPayloads(prev => ({ ...prev, [id]: currentDraft }));
      }
    }
  };

  return (
    <AppShell title="Daily Work" subtitle="Your daily priorities, coordinated by your team.">
      <div className="w-full max-w-md mx-auto p-4 space-y-4" data-testid="agent-feed">
        {loading && (
          <div className="flex justify-center items-center py-12">
            <p className="text-gray-500 font-medium">Checking your feed...</p>
          </div>
        )}

        {error && (
          <div className="glassmorphism p-4 text-center">
            <p className="text-[#FF3B30] dark:text-[#DE1B1B] font-medium mb-2">We couldn't load your feed.</p>
            <p className="text-sm text-gray-500">{error}</p>
          </div>
        )}

        {!loading && !error && items.length === 0 && (
          <div className="glassmorphism flex flex-col items-center justify-center p-12 text-center" data-testid="agent-feed-empty">
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
            const isExpanded = expandedId === item.id;
            const contextText = item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.';
            const draftResponse = item.proposed_action?.payload || item.proposed_action?.message || contextText;

            return (
              <div
                key={item.id}
                className={`glassmorphism p-5 relative overflow-hidden transition-all duration-300 ${isProcessing ? 'opacity-50 scale-[0.98]' : 'animate-fade-in'}`}
                data-testid="agent-feed-card"
                onClick={() => { if (!isExpanded) toggleExpand(item.id, draftResponse); }}
              >
                <div className="flex justify-between items-start mb-3 cursor-pointer">
                  <span className="text-[11px] font-bold uppercase tracking-wider text-[#0066FF] dark:text-[#0071E3] flex items-center gap-1.5">
                    <span className="w-2 h-2 rounded-full bg-[#0066FF] dark:bg-[#0071E3] opacity-80"></span>
                    {item.event_source.replace(/_/g, ' ')}
                  </span>
                  <span className="text-[11px] text-gray-400 font-medium">
                    {new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                  </span>
                </div>

                <h3 className="font-bold text-gray-900 dark:text-white text-[15px] mb-2 leading-snug cursor-pointer">
                  {item.proposed_action?.title || 'Review Required'}
                </h3>

                {isExpanded ? (
                  <div className="mb-5 animate-fade-in cursor-default" onClick={(e) => e.stopPropagation()}>
                    <div className="mb-4">
                      <div className="text-xs uppercase tracking-wider font-semibold text-gray-500 mb-1">Context</div>
                      <pre className="text-[13px] text-gray-800 dark:text-gray-200 whitespace-pre-wrap font-sans bg-gray-50 dark:bg-black/20 p-3 rounded-lg border border-gray-100 dark:border-white/5">
                        {contextText}
                      </pre>
                    </div>
                    <div className="mb-4">
                      <div className="text-xs uppercase tracking-wider font-semibold text-gray-500 mb-1">AI Drafted Action</div>
                      <textarea
                        className="w-full text-[13px] text-gray-900 dark:text-white bg-white dark:bg-black/40 border border-[#0066FF]/30 focus:border-[#0066FF] focus:ring-1 focus:ring-[#0066FF] rounded-lg p-3 min-h-[100px] resize-y"
                        value={editedPayloads[item.id] !== undefined ? editedPayloads[item.id] : draftResponse}
                        onChange={(e) => setEditedPayloads(prev => ({ ...prev, [item.id]: e.target.value }))}
                        data-testid="feed-edit-textarea"
                      />
                    </div>
                  </div>
                ) : (
                  <p className="text-[13px] text-gray-600 dark:text-gray-300 mb-5 leading-relaxed line-clamp-2 cursor-pointer">
                    {contextText}
                  </p>
                )}

                <div className="flex gap-3" onClick={(e) => e.stopPropagation()}>
                  <button
                    onClick={() => handleAction(item.id, 'APPROVED', editedPayloads[item.id])}
                    disabled={isProcessing}
                    className="flex-1 bg-[#0066FF] hover:bg-[#0052CC] dark:bg-[#0071E3] dark:hover:bg-[#005bb5] text-white font-bold py-3 px-4 rounded-lg min-h-[44px] transition-colors flex items-center justify-center gap-2 border-0 cursor-pointer"
                    data-testid="feed-approve-btn"
                  >
                    {isProcessing ? 'Processing...' : (isExpanded ? 'Send' : 'Review')}
                  </button>
                  <button
                    onClick={(e) => {
                      if (isExpanded) {
                        toggleExpand(item.id, draftResponse);
                      } else {
                        handleAction(item.id, 'DISMISSED');
                      }
                    }}
                    disabled={isProcessing}
                    className="flex-1 bg-[rgba(0,0,0,0.05)] hover:bg-[rgba(0,0,0,0.1)] dark:bg-[rgba(255,255,255,0.1)] dark:hover:bg-[rgba(255,255,255,0.15)] text-gray-700 dark:text-white font-bold py-3 px-4 rounded-lg min-h-[44px] transition-colors border-0 cursor-pointer"
                    data-testid="feed-dismiss-btn"
                  >
                    {isExpanded ? 'Cancel' : 'Dismiss'}
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </AppShell>
  );
}
