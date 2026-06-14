'use client';

import React, { useEffect, useState, useRef } from 'react';
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
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editContent, setEditContent] = useState<string>('');
  const editInputRef = useRef<HTMLTextAreaElement>(null);

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


  const handleEditClick = (item: FeedItem) => {
    setEditingId(item.id);
    setEditContent(item.proposed_action?.generated_response || item.context_payload?.summary || item.proposed_action?.description || '');
    setTimeout(() => {
      editInputRef.current?.focus();
    }, 50);
  };

  const handleSaveEdit = (item: FeedItem) => {
    setItems((prev) =>
      prev.map((i) => {
        if (i.id === item.id) {
          if (i.proposed_action?.feature_type === 'ambassador_reply') {
            return {
              ...i,
              proposed_action: {
                ...i.proposed_action,
                generated_response: editContent,
              }
            };
          }
          return {
            ...i,
            proposed_action: {
              ...i.proposed_action,
              description: editContent,
            },
            context_payload: {
              ...i.context_payload,
              summary: editContent,
            }
          };
        }
        return i;
      })
    );
    setEditingId(null);
  };

  const handleAction = async (item: FeedItem, state: string) => {
    try {
      setProcessingId(item.id);
      const res = await fetch(`/api/agent-feed/${item.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          state,
          payload: item.proposed_action || item.context_payload || {}
        }),
      });
      if (!res.ok) throw new Error('Action failed');

      // Update UI optimistically or refetch
      setItems((prev) => prev.filter((i) => i.id !== item.id));
    } catch (err: any) {
      alert(err.message);
    } finally {
      setProcessingId(null);
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
            const isEditing = editingId === item.id;

            return (
              <div
                key={item.id}
                className={`glassmorphism p-5 relative overflow-hidden transition-all duration-300 ${isProcessing ? 'opacity-50 scale-[0.98]' : 'animate-fade-in'}`}
                data-testid="agent-feed-card"
              >
                <div className="flex justify-between items-start mb-3">
                  <span className="text-[11px] font-bold uppercase tracking-wider text-[#0066FF] dark:text-[#0071E3] flex items-center gap-1.5">
                    <span className="w-2 h-2 rounded-full bg-[#0066FF] dark:bg-[#0071E3] opacity-80"></span>
                    {item.proposed_action?.feature_type === 'ambassador_reply' ? `Draft Reply: ${item.proposed_action.source} from ${item.proposed_action.sender_id}` : item.event_source.replace(/_/g, ' ')}
                  </span>
                  <span className="text-[11px] text-gray-400 font-medium">
                    {new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                  </span>
                </div>

                <h3 className="font-bold text-gray-900 dark:text-white text-[15px] mb-2 leading-snug">
                  {item.proposed_action?.feature_type === 'ambassador_reply' ? 'Review Proposed Reply' : (item.proposed_action?.title || 'Review Required')}
                </h3>

                {item.proposed_action?.feature_type === 'ambassador_reply' && (
                  <div className="mb-4 p-3 bg-[#f2f2f7] dark:bg-[rgba(255,255,255,0.05)] rounded-xl border border-gray-100 dark:border-gray-800">
                    <p className="text-[11px] font-bold text-gray-400 uppercase mb-1">Customer Message</p>
                    <p className="text-[13px] text-gray-700 dark:text-gray-300 italic">"{item.proposed_action.original_message}"</p>
                  </div>
                )}

                {isEditing ? (
                  <div className="mb-5">
                    <textarea
                      ref={editInputRef}
                      value={editContent}
                      onChange={(e) => setEditContent(e.target.value)}
                      className="w-full min-h-[100px] p-3 text-[13px] text-gray-800 dark:text-gray-100 bg-white/50 dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#0066FF] dark:focus:ring-[#0071E3] transition-all resize-none"
                      placeholder="Edit action details..."
                      data-testid="feed-edit-input"
                    />
                  </div>
                ) : (
                  <div className="mb-5">
                    {item.proposed_action?.feature_type === 'ambassador_reply' ? (
                      <>
                        <div className="mb-3">
                          <p className="text-[11px] font-bold text-gray-400 uppercase mb-1">Context Summary</p>
                          <p className="text-[12px] text-gray-600 dark:text-gray-400 leading-snug">
                            {item.proposed_action.context_used.split('\n\nUnified Customer History:\n')[1] || item.proposed_action.context_used || 'New customer.'}
                          </p>
                        </div>
                        <div>
                          <p className="text-[11px] font-bold text-[#0066FF] dark:text-[#0071E3] uppercase mb-1">AI Drafted Response</p>
                          <p className="text-[13px] text-gray-900 dark:text-white leading-relaxed font-medium">
                            {item.proposed_action.generated_response}
                          </p>
                        </div>
                      </>
                    ) : (
                      <p className="text-[13px] text-gray-600 dark:text-gray-300 leading-relaxed">
                        {item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.'}
                      </p>
                    )}
                  </div>
                )}

                <div className="flex flex-col gap-3">
                  {isEditing ? (
                    <div className="flex gap-3">
                      <button
                        onClick={() => handleSaveEdit(item)}
                        className="flex-1 bg-[#0066FF] hover:bg-[#0052CC] dark:bg-[#0071E3] dark:hover:bg-[#005bb5] text-white font-bold py-3 px-4 rounded-lg min-h-[44px] transition-colors flex items-center justify-center gap-2 border-0 cursor-pointer"
                        data-testid="feed-save-edit-btn"
                      >
                        Save
                      </button>
                      <button
                        onClick={() => setEditingId(null)}
                        className="flex-1 bg-[rgba(0,0,0,0.05)] hover:bg-[rgba(0,0,0,0.1)] dark:bg-[rgba(255,255,255,0.1)] dark:hover:bg-[rgba(255,255,255,0.15)] text-gray-700 dark:text-white font-bold py-3 px-4 rounded-lg min-h-[44px] transition-colors border-0 cursor-pointer"
                        data-testid="feed-cancel-edit-btn"
                      >
                        Cancel
                      </button>
                    </div>
                  ) : (
                    <div className="flex gap-3">
                      <button
                        onClick={() => handleAction(item, 'APPROVED')}
                        disabled={isProcessing}
                        className="flex-1 bg-[#0066FF] hover:bg-[#0052CC] dark:bg-[#0071E3] dark:hover:bg-[#005bb5] text-white font-bold py-3 px-4 rounded-lg min-h-[44px] transition-colors flex items-center justify-center gap-2 border-0 cursor-pointer"
                        data-testid="feed-approve-btn"
                      >
                        {isProcessing ? 'Processing...' : 'Approve'}
                      </button>
                      <button
                        onClick={() => handleEditClick(item)}
                        disabled={isProcessing}
                        className="flex-1 bg-[rgba(0,0,0,0.05)] hover:bg-[rgba(0,0,0,0.1)] dark:bg-[rgba(255,255,255,0.1)] dark:hover:bg-[rgba(255,255,255,0.15)] text-gray-700 dark:text-white font-bold py-3 px-4 rounded-lg min-h-[44px] transition-colors border-0 cursor-pointer"
                        data-testid="feed-edit-btn"
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => handleAction(item, 'DISMISSED')}
                        disabled={isProcessing}
                        className="flex-1 bg-[rgba(0,0,0,0.05)] hover:bg-[rgba(0,0,0,0.1)] dark:bg-[rgba(255,255,255,0.1)] dark:hover:bg-[rgba(255,255,255,0.15)] text-gray-700 dark:text-white font-bold py-3 px-4 rounded-lg min-h-[44px] transition-colors border-0 cursor-pointer"
                        data-testid="feed-dismiss-btn"
                      >
                        Dismiss
                      </button>
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </AppShell>
  );
}
