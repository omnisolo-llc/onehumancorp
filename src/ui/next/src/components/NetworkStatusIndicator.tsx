"use client";
import { WithTooltip } from './TooltipRegistry';
import React, { useEffect, useState } from 'react';
import { SyncManager } from '../lib/sync/SyncManager';
import type { QueueSummary } from '../app/utils/offlineQueue';
const empty: QueueSummary = { pending: 0, needsAttention: 0, reconciliation: 0, legacyHeld: 0, storageUnavailable: false };
export function NetworkStatusIndicator() {
  const [isOffline, setIsOffline] = useState(false);
  const [summary, setSummary] = useState<QueueSummary>(empty);
  useEffect(() => {
    let active = true; let sequence = 0;
    const refresh = async () => {
      const current = ++sequence;
      setIsOffline(!navigator.onLine);
      try {
        const next = await SyncManager.getInstance().getQueueSummary();
        if (active && current === sequence) setSummary(next);
      } catch { if (active && current === sequence) setSummary({ ...empty, storageUnavailable: true }); }
    };
    void refresh();
    const events = ['online', 'offline', 'omnisolo_queue_updated', 'omnisolo_auth_changed', 'storage'];
    for (const event of events) window.addEventListener(event, refresh);
    return () => { active = false; for (const event of events) window.removeEventListener(event, refresh); };
  }, []);
  const labels = [isOffline ? 'Offline' : '', summary.pending ? `Pending: ${summary.pending}` : '', summary.needsAttention ? `Needs attention: ${summary.needsAttention}` : '', summary.reconciliation ? `Reconciliation: ${summary.reconciliation}` : '', summary.legacyHeld ? `Unassigned: ${summary.legacyHeld}` : '', summary.storageUnavailable ? 'Queue status unavailable' : ''].filter(Boolean);
  if (!labels.length) return null;
  return <div className="fixed top-2 left-1/2 transform -translate-x-1/2 z-50 flex items-center justify-center pointer-events-none">
    <WithTooltip id="network-status-tooltip" defaultText="Pending actions wait for a verified result. Actions needing attention or reconciliation are retained and are not automatically retried.">
      <div className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] px-4 py-1.5 rounded-full shadow border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex items-center gap-2 pointer-events-auto">
        <span role="status" className="text-sm font-semibold text-gray-800">{labels.join(' · ')}</span>
      </div>
    </WithTooltip>
  </div>;
}
