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
        <div className="bg-white/80 backdrop-blur-md px-4 py-1.5 rounded-full shadow border border-gray-200/50 flex items-center gap-2 pointer-events-auto">
          <div className={`w-2 h-2 rounded-full ${isOffline ? 'bg-orange-500' : 'bg-blue-500 animate-pulse'}`}></div>
          <span className="text-sm font-semibold text-gray-800">
            {isOffline ? 'Offline - Saving Locally' : `Syncing ${syncQueueLength} action${syncQueueLength !== 1 ? 's' : ''}...`}
          </span>
          {syncQueueLength > 0 && (
             <svg className="w-4 h-4 text-blue-500 animate-spin" fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
             </svg>
          )}
        </div>
      </WithTooltip>
    </div>
  );
}
