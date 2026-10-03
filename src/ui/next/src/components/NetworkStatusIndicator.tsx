"use client";
import { WithTooltip } from './TooltipRegistry';
import React, { useEffect, useState } from 'react';
import { SyncManager } from '../lib/sync/SyncManager';
import { QUEUE_IDENTITY_EPOCH_KEY, hasVerifiedOfflineQueueOwner, subscribeQueueIdentityReadiness } from '../lib/sync/queueIdentity';
import type { QueueSummary } from '../app/utils/offlineQueue';
const empty: QueueSummary = { pending: 0, needsAttention: 0, reconciliation: 0, legacyHeld: 0, storageUnavailable: false };
export function NetworkStatusIndicator() {
  const [isOffline, setIsOffline] = useState(false);
  const [summary, setSummary] = useState<QueueSummary>(empty);
  const [identityReady, setIdentityReady] = useState(false);
  const [storageReady, setStorageReady] = useState(false);
  useEffect(() => subscribeQueueIdentityReadiness(() => setIdentityReady(hasVerifiedOfflineQueueOwner())), []);
  useEffect(() => {
    let active = true; let sequence = 0;
    const refresh = async () => {
      const current = ++sequence;
      setIsOffline(!navigator.onLine); setStorageReady(false);
      try {
        const next = await SyncManager.getInstance().getQueueSummary();
        if (active && current === sequence) { setSummary(next); setStorageReady(true); }
      } catch { if (active && current === sequence) setSummary({ ...empty, storageUnavailable: true }); }
    };
    void refresh();
    const identityChanged = () => {
      setSummary({ ...empty, storageUnavailable: true });
      void refresh();
    };
    const storageChanged = (event: StorageEvent) => {
      if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) identityChanged();
      else void refresh();
    };
    const events = ['online', 'offline', 'omnisolo_queue_updated'];
    for (const event of events) window.addEventListener(event, refresh);
    window.addEventListener('omnisolo_auth_changed', identityChanged);
    window.addEventListener('storage', storageChanged);
    return () => {
      active = false;
      for (const event of events) window.removeEventListener(event, refresh);
      window.removeEventListener('omnisolo_auth_changed', identityChanged);
      window.removeEventListener('storage', storageChanged);
    };
  }, []);
  const labels = [isOffline ? 'Offline' : '', summary.pending ? `Pending: ${summary.pending}` : '', summary.needsAttention ? `Needs attention: ${summary.needsAttention}` : '', summary.reconciliation ? `Reconciliation: ${summary.reconciliation}` : '', summary.legacyHeld ? `Unassigned: ${summary.legacyHeld}` : '', summary.storageUnavailable ? 'Queue status unavailable' : ''].filter(Boolean);
  const readiness = identityReady && storageReady ? 'ready' : 'held';
  if (!labels.length) return <span data-testid="offline-queue-readiness" data-state={readiness} hidden />;
  return <div data-testid="offline-queue-readiness" data-state={readiness} className="fixed top-2 left-1/2 transform -translate-x-1/2 z-50 flex items-center justify-center pointer-events-none">
    <WithTooltip id="network-status-tooltip" defaultText="Pending actions wait for a verified result. Actions needing attention or reconciliation are retained and are not automatically retried.">
      <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] px-4 py-1.5 rounded-full shadow border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex items-center gap-2 pointer-events-auto">
        <span role="status" className="text-sm font-semibold text-gray-800">{labels.join(' · ')}</span>
      </div>
    </WithTooltip>
  </div>;
}
