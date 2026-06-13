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

  useEffect(() => {
    async function fetchFeed() {
      try {
        const res = await fetch('/api/agent-feed');
        if (!res.ok) {
          throw new Error('Failed to fetch feed');
        }
        const data = await res.json();
        setItems(data.items || []);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }

    fetchFeed();
  }, []);

  const handleAction = async (id: string, state: string) => {
    try {
      const res = await fetch(`/api/agent-feed/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ state }),
      });
      if (!res.ok) throw new Error('Action failed');

      // Update UI optimistically or refetch
      setItems((prev) => prev.filter((item) => item.id !== id));
    } catch (err: any) {
      alert(err.message);
    }
  };

  return (
    <AppShell title="Agent Feed" subtitle="Your daily priorities, prepared by your team.">
      <div className="max-w-md mx-auto p-4 space-y-4" data-testid="agent-feed">
        {loading && <p>Loading feed...</p>}
        {error && <p className="text-red-500">Error: {error}</p>}
        {!loading && !error && items.length === 0 && (
          <p>You have no pending actions in your feed.</p>
        )}

        {items.map((item) => (
          <div key={item.id} className="bg-white dark:bg-gray-800 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-gray-700" data-testid="agent-feed-card">
            <div className="flex justify-between items-start mb-2">
              <span className="text-xs font-semibold uppercase tracking-wider text-indigo-500">
                {item.event_source.replace(/_/g, ' ')}
              </span>
              <span className="text-xs text-gray-500">
                {new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
              </span>
            </div>

            <h3 className="font-medium text-gray-900 dark:text-white mb-1">
              {item.proposed_action?.title || 'Review Required'}
            </h3>

            <p className="text-sm text-gray-600 dark:text-gray-300 mb-4">
              {item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.'}
            </p>

            <div className="flex gap-2 mt-4">
              <button
                onClick={() => handleAction(item.id, 'APPROVED')}
                className="flex-1 bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-3 px-4 rounded-lg min-h-[44px] min-w-[44px] transition-colors"
                style={{ minHeight: '44px' }}
              >
                Approve
              </button>
              <button
                onClick={() => handleAction(item.id, 'DISMISSED')}
                className="flex-1 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-900 dark:text-white font-medium py-3 px-4 rounded-lg min-h-[44px] min-w-[44px] transition-colors"
                style={{ minHeight: '44px' }}
              >
                Dismiss
              </button>
            </div>
          </div>
        ))}
      </div>
    </AppShell>
  );
}
