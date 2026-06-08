'use client';
import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import styles from './action-feed.module.css';

interface ActionCard {
  id: string;
  tenant_id: string;
  agent_id: string;
  trigger_event: string;
  context_summary: string;
  proposed_action: any;
  state: string;
  created_at: string;
  updated_at: string;
}

export default function ActionFeedPage() {
  const router = useRouter();
  const [cards, setCards] = useState<ActionCard[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // In a real app we'd get this from auth context. Using e2e default for testing
  const tenantId = 'e2e-tenant';

  useEffect(() => {
    fetchCards();
  }, []);

  const fetchCards = async () => {
    try {
      setLoading(true);
      const res = await fetch(`/api/v1/action-feed/${tenantId}/cards`);
      if (!res.ok) {
        throw new Error('Failed to fetch action cards');
      }
      const data = await res.json();
      setCards(data);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleApprove = async (cardId: string) => {
    try {
      const res = await fetch(`/api/v1/action-feed/${tenantId}/cards/${cardId}/state`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ state: 'APPROVED' }),
      });
      if (!res.ok) {
        throw new Error('Failed to approve card');
      }
      // Refresh the feed
      await fetchCards();
    } catch (err: any) {
      alert(err.message);
    }
  };

  const handleDismiss = async (cardId: string) => {
    try {
      const res = await fetch(`/api/v1/action-feed/${tenantId}/cards/${cardId}/state`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ state: 'REJECTED' }),
      });
      if (!res.ok) {
        throw new Error('Failed to dismiss card');
      }
      // Refresh the feed
      await fetchCards();
    } catch (err: any) {
      alert(err.message);
    }
  };

  if (loading) return <div className={styles.loading}>Loading Action Feed...</div>;
  if (error) return <div className={styles.error}>Error: {error}</div>;

  const pendingCards = cards.filter(c => c.state === 'PENDING_APPROVAL');

  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <h1>Agent Action Feed</h1>
        <p>Prioritized items needing your attention</p>
      </header>

      <main className={styles.feed}>
        {pendingCards.length === 0 ? (
          <div className={styles.emptyState}>
            <p>You're all caught up!</p>
          </div>
        ) : (
          pendingCards.map(card => (
            <div key={card.id} className={styles.actionCard}>
              <div className={styles.cardHeader}>
                <span className={styles.agentId}>{card.agent_id} Agent</span>
                <span className={styles.time}>{new Date(card.created_at).toLocaleTimeString()}</span>
              </div>
              <div className={styles.cardContext}>
                <h3>{card.trigger_event}</h3>
                <p>{card.context_summary}</p>
              </div>
              <div className={styles.cardProposedAction}>
                <pre>{JSON.stringify(card.proposed_action, null, 2)}</pre>
              </div>
              <div className={styles.actionBar}>
                <button
                  className={styles.primaryButton}
                  onClick={() => handleApprove(card.id)}
                >
                  Approve & Execute
                </button>
                <button
                  className={styles.secondaryButton}
                  onClick={() => handleDismiss(card.id)}
                >
                  Dismiss
                </button>
              </div>
            </div>
          ))
        )}
      </main>
    </div>
  );
}
