'use client';

import { useOfflineSync } from '@/hooks/useOfflineSync';
import { useState } from 'react';

export default function TestOfflineSyncPage() {
  const { queue, isOnline, dispatchAction } = useOfflineSync();
  const [localActionId, setLocalActionId] = useState<string | null>(null);

  const handleAction = () => {
    dispatchAction('test_action', { foo: 'bar' });
    setLocalActionId('pending'); // simple local state to drive the class
  };

  // Find the action in the queue to determine the UI state
  const queuedAction = queue.find(a => a.action_type === 'test_action');

  const statusClass = queuedAction?.status === 'pending' || queuedAction?.status === 'syncing'
    ? 'pending-sync opacity-50 backdrop-blur-md'
    : queuedAction?.status === 'completed' ? 'completed bg-green-100' : '';

  return (
    <div className="p-4 max-w-[375px] mx-auto border min-h-screen">
      <h1 className="text-xl font-bold mb-4">Offline Sync Test</h1>

      <div id="network-status" className={`mb-4 px-2 py-1 rounded inline-block ${isOnline ? 'bg-green-200' : 'bg-red-200'}`}>
        {isOnline ? 'Online' : 'Offline'}
      </div>

      <div className={`action-card p-4 border rounded shadow-sm ${statusClass}`}>
        <h3 className="font-semibold">Approve quote for Maya's Cake</h3>
        <p className="text-sm text-gray-600 mb-4">Action required</p>

        <button
          id="trigger-action-btn"
          onClick={handleAction}
          className="w-full min-h-[44px] bg-blue-600 text-white rounded font-medium"
        >
          Approve
        </button>

        {(queuedAction?.status === 'pending' || queuedAction?.status === 'syncing') && (
            <div className="sync-icon mt-2 text-sm text-gray-500 flex items-center justify-center">
               <svg className="w-4 h-4 mr-1 animate-spin" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                 <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
               </svg>
               Pending Sync...
            </div>
        )}
      </div>
    </div>
  );
}
