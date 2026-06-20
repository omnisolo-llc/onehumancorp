"use client";

import React, { useState, useEffect } from 'react';
import { getConflicts, removeConflict, ConflictRecord } from '../lib/sync/db';
import { SyncManager } from '../lib/sync/SyncManager';

export function ConflictResolutionCard() {
  const [conflicts, setConflicts] = useState<ConflictRecord[]>([]);

  useEffect(() => {
    const fetchConflicts = async () => {
      const records = await getConflicts();
      setConflicts(records);
    };

    fetchConflicts();

    const handleConflictDetected = () => {
      fetchConflicts();
    };

    if (typeof window !== 'undefined') {
      window.addEventListener('ohc_sync_conflict_detected', handleConflictDetected);
      return () => window.removeEventListener('ohc_sync_conflict_detected', handleConflictDetected);
    }
  }, []);

  if (conflicts.length === 0) return null;

  const handleKeepMine = async (conflict: ConflictRecord) => {
    await removeConflict(conflict.id);
    setConflicts(prev => prev.filter(c => c.id !== conflict.id));

    // Re-queue the mutation as a fresh action
    await SyncManager.getInstance().enqueue({
        id: `resolved-${Date.now()}-${conflict.id}`,
        type: conflict.type,
        payload: conflict.payload,
        timestamp: new Date().toISOString()
    });
  };

  const handleUseLatest = async (conflict: ConflictRecord) => {
    // Just remove the conflict and drop the local change
    await removeConflict(conflict.id);
    setConflicts(prev => prev.filter(c => c.id !== conflict.id));

    // Trigger a refetch or state update so the app reflects the cloud state
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new Event('ohc_queue_updated'));
      window.dispatchEvent(new Event('ohc_sync_resolved_latest'));
    }
  };

  return (
    <div className="fixed bottom-4 left-1/2 transform -translate-x-1/2 z-50 w-[345px] flex flex-col gap-3">
      {conflicts.map(conflict => (
        <div key={conflict.id} className="app-card bg-white/80 backdrop-blur-xl rounded-2xl p-4 shadow-xl border border-red-200/50 flex flex-col animate-in slide-in-from-bottom-5">
          <div className="flex items-center gap-2 mb-2">
            <span className="bg-red-100 text-red-700 px-2 py-1 rounded text-xs font-bold uppercase">Conflict</span>
            <span className="text-gray-800 font-semibold text-sm truncate flex-1">{conflict.errorMsg}</span>
          </div>
          <p className="text-gray-600 text-sm mb-4 leading-snug">
            {conflict.type === 'UPDATE_ORDER_STATUS' ? "This order was updated elsewhere." : "This item was updated elsewhere."} Keep your changes or use the latest?
          </p>
          <div className="flex gap-2">
             <button
                onClick={() => handleKeepMine(conflict)}
                className="flex-1 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-xl text-sm font-bold shadow-sm transition active:scale-95"
                data-testid={`btn-conflict-keep-mine-${conflict.id}`}
             >
                Keep Mine
             </button>
             <button
                onClick={() => handleUseLatest(conflict)}
                className="flex-1 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-xl text-sm font-bold shadow-sm transition active:scale-95"
                data-testid={`btn-conflict-use-latest-${conflict.id}`}
             >
                Use Latest
             </button>
          </div>
        </div>
      ))}
    </div>
  );
}
