import React from 'react';
import { useOfflineSyncEngine } from '@/hooks/useOfflineSyncEngine';
import { FiCloudOff, FiRefreshCw, FiCheckCircle } from 'react-icons/fi';

export const OfflineSyncIndicator: React.FC = () => {
  const { isOnline, pendingCount } = useOfflineSyncEngine();

  if (isOnline && pendingCount === 0) {
    return null; // Silent when everything is good and online
  }

  return (
    <div className="fixed bottom-4 right-4 z-50 flex items-center gap-2 bg-white/80 backdrop-blur-md px-3 py-2 rounded-full shadow-lg border border-gray-200 text-sm font-medium transition-all">
      {!isOnline ? (
        <>
          <FiCloudOff className="text-gray-500" />
          <span className="text-gray-600">Offline</span>
          {pendingCount > 0 && (
            <span className="bg-gray-200 text-gray-800 text-xs px-2 py-0.5 rounded-full">
              {pendingCount} pending
            </span>
          )}
        </>
      ) : (
        <>
          <FiRefreshCw className="text-blue-500 animate-spin" />
          <span className="text-blue-600">Syncing...</span>
          <span className="bg-blue-100 text-blue-800 text-xs px-2 py-0.5 rounded-full">
            {pendingCount} left
          </span>
        </>
      )}
    </div>
  );
};
