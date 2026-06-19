"use client";


import { WithTooltip } from "./TooltipRegistry";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../lib/sync/SyncManager';

export function NetworkStatusIndicator() {
  const [isOffline, setIsOffline] = useState(false);
  const [syncQueueLength, setSyncQueueLength] = useState(0);
  const [isSyncing, setIsSyncing] = useState(false);

  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    const handleQueueUpdate = async () => {
      const length = await SyncManager.getInstance().getQueueLength();
      setSyncQueueLength(length);
    };
    const handleSyncStarted = () => setIsSyncing(true);
    const handleSyncCompleted = () => setIsSyncing(false);

    if (typeof window !== 'undefined') {
      setIsOffline(!navigator.onLine);
      handleQueueUpdate();
      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);
      window.addEventListener('ohc_queue_updated', handleQueueUpdate);
      window.addEventListener('ohc_sync_started', handleSyncStarted);
      window.addEventListener('ohc_sync_completed', handleSyncCompleted);

      return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
        window.removeEventListener('ohc_queue_updated', handleQueueUpdate);
        window.removeEventListener('ohc_sync_started', handleSyncStarted);
        window.removeEventListener('ohc_sync_completed', handleSyncCompleted);
      };
    }
  }, []);

  if (!isOffline && syncQueueLength === 0 && !isSyncing) return null;

  return (
    <div
      className="fixed top-2 left-1/2 transform -translate-x-1/2 z-50 flex items-center justify-center pointer-events-none"
    >
      <WithTooltip id="network-status-tooltip" defaultText={isOffline ? "You are currently disconnected. Changes will be saved locally." : "Your changes are syncing to the cloud."}>
        <div className={`backdrop-blur-[30px] saturate-[210%] px-4 py-1.5 rounded-[8px] shadow flex items-center gap-2 pointer-events-auto transition-colors duration-300 ${
          isOffline
            ? 'bg-[rgba(255,149,0,0.65)] dark:bg-[rgba(255,159,26,0.7)] border-[1px] border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]'
            : 'bg-[rgba(255,255,255,0.65)] dark:bg-[#16161a]/70 border-[1px] border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)]'
        }`}>
          <div className={`w-2 h-2 rounded-full ${isOffline ? 'bg-[#FF9500] dark:bg-[#FF9F1A]' : 'bg-[#0066FF] animate-pulse'}`}></div>
          <span className={`text-sm font-semibold tracking-wide ${isOffline ? 'text-[#1D1D1F] dark:text-[#1D1D1F]' : 'text-[#1D1D1F] dark:text-[#F5F5F7]'}`}>
            {isOffline ? 'Working offline. Changes saved.' : 'Syncing...'}
          </span>
        </div>
      </WithTooltip>
    </div>
  );
}
