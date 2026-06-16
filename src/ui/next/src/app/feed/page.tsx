'use client';

import React, { useEffect, useState } from 'react';
import { AppShell } from '../components/AppShell';
import { ActionCard, ActionCardItem } from '../components/ActionCard';

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

  const saveEdit = async (id: string, newDescription: string) => {
    // Optimistic update
    setItems((prev) => prev.map((item) => {
      if (item.id === id) {
        return {
          ...item,
          proposed_action: {
            ...item.proposed_action,
            description: newDescription,
          },
          context_payload: {
            ...item.context_payload,
            summary: newDescription,
          }
        };
      }
      return item;
    }));

    // Automatically approve after saving edit
    await handleAction(id, 'APPROVED', newDescription);
  };

  const handleAction = async (id: string, state: string, editedPayload?: string) => {
    try {
      setProcessingId(id);
      const res = await fetch(`/api/agent-feed/${id}/state`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ state, edited_payload: editedPayload }),
      });
      if (!res.ok) throw new Error('Action failed');

      // Update UI optimistically or refetch
      setItems((prev) => prev.filter((item) => item.id !== id));
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
            const cardItem: ActionCardItem = {
              id: item.id,
              source: item.event_source,
              title: item.proposed_action?.title || 'Review Required',
              description: item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.',
              timestamp: new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
              payload: item
            };

            return (
              <ActionCard
                key={item.id}
                item={cardItem}
                isProcessing={isProcessing}
                onApprove={(id) => handleAction(id, 'APPROVED')}
                onDismiss={(id) => handleAction(id, 'DISMISSED')}
                onEditSave={(id, newDescription) => saveEdit(id, newDescription)}
              />
            );
          })}
        </div>
      </div>
    </AppShell>
  );
}
