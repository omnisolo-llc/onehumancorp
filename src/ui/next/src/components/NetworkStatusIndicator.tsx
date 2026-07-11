"use client";


import { WithTooltip } from "./TooltipRegistry";

import React, { useState, useEffect } from 'react';
import { SyncManager } from '../lib/sync/SyncManager';

export function NetworkStatusIndicator() {
  const [isOffline, setIsOffline] = useState(false);
  const [syncQueueLength, setSyncQueueLength] = useState(0);

  useEffect(() => {
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    const handleQueueUpdate = async () => {
      const length = await SyncManager.getInstance().getQueueLength();
      setSyncQueueLength(length);
    };

    if (typeof window !== 'undefined') {
      setIsOffline(!navigator.onLine);
      handleQueueUpdate();
      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);
      window.addEventListener('ohc_queue_updated', handleQueueUpdate);
      return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
        window.removeEventListener('ohc_queue_updated', handleQueueUpdate);
      };
    }
  }, []);

  if (!isOffline && syncQueueLength === 0) return null;

  return (
    <div
      className="fixed top-2 left-1/2 transform -translate-x-1/2 z-50 flex items-center justify-center pointer-events-none"
    >
      <WithTooltip id="network-status-tooltip" defaultText={isOffline ? "You are currently disconnected. Changes will be saved locally." : "Your changes are syncing to the cloud."}>
        <div className={`bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] px-4 py-1.5 rounded-full shadow border border-white/40 dark:border-white/10 flex items-center gap-2 pointer-events-auto animate-in slide-in-from-top duration-300`}>
          <div className={`w-2 h-2 rounded-full ${isOffline ? 'bg-amber-500' : 'bg-blue-500 animate-pulse'}`}></div>
          <span className="text-sm font-semibold text-gray-800">
            {isOffline ? 'Offline - Changes saved locally' : `Syncing ${syncQueueLength} action${syncQueueLength !== 1 ? 's' : ''}...`}
          </span>
        </div>
      </WithTooltip>
    </div>
  );
}
