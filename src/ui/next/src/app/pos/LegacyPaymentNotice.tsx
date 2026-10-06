'use client';

import { useEffect, useState } from 'react';

const LEGACY_QUEUE_KEY = 'pos_offline_queue';
type LegacyQueueState = 'empty' | 'held' | 'unavailable';

export default function LegacyPaymentNotice() {
  const [state, setState] = useState<LegacyQueueState>('empty');

  useEffect(() => {
    const check = () => {
      let raw: string | null;
      try {
        raw = localStorage.getItem(LEGACY_QUEUE_KEY);
      } catch {
        setState('unavailable');
        return;
      }
      if (raw === null) { setState('empty'); return; }
      try {
        const value: unknown = JSON.parse(raw);
        setState(Array.isArray(value) && value.length === 0 ? 'empty' : 'held');
      } catch {
        // Malformed records are evidence too. Never rewrite or discard them.
        setState('held');
      }
    };
    const storageChanged = (event: StorageEvent) => {
      if (event.key === null || event.key === LEGACY_QUEUE_KEY) check();
    };
    check();
    window.addEventListener('storage', storageChanged);
    window.addEventListener('online', check);
    window.addEventListener('pageshow', check);
    window.addEventListener('focus', check);
    return () => {
      window.removeEventListener('storage', storageChanged);
      window.removeEventListener('online', check);
      window.removeEventListener('pageshow', check);
      window.removeEventListener('focus', check);
    };
  }, []);

  if (state === 'empty') return null;

  // The old queue has no verified owner or trustworthy payment receipt. Show
  // only its presence; do not expose its contents or import it into owned sync.
  return (
    <aside role="status" className="mx-auto max-w-xl border border-amber-300 bg-amber-50 p-4 text-sm text-amber-950">
      <h2 className="font-semibold">{state === 'held' ? 'Historical POS data needs review' : 'Historical POS data could not be checked'}</h2>
      <p>{state === 'held'
        ? 'This browser contains records from the previous POS. Payment status and account ownership are unverified.'
        : 'Browser storage is unavailable, so earlier payment attempts cannot be verified.'}</p>
      <p>Nothing in this legacy queue will be sent or removed by this terminal. Before retrying an earlier payment, have the account owner compare provider transactions and receipts to avoid charging twice.</p>
      <p>Keep this browser’s stored data until the review is complete. A saved attempt is not proof of payment.</p>
    </aside>
  );
}
