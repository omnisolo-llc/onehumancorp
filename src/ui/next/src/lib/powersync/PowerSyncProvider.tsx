import React, { useEffect, useState } from 'react';
import { PowerSyncDatabase } from '@powersync/web';
import { PowerSyncContext } from '@powersync/react';
import { getPowerSyncDB, browserSupportsPowerSync, isPowerSyncSupportedForLocation } from './db';

// Re-export for compatibility
export { isPowerSyncSupportedForLocation };

export const PowerSyncProvider = ({
  children,
  fallback,
  unsupportedFallback,
}: {
  children: React.ReactNode;
  fallback?: React.ReactNode;
  unsupportedFallback?: React.ReactNode;
}) => {
  const [powerSync, setPowerSync] = useState<PowerSyncDatabase | null>(null);
  const [ready, setReady] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const supported = browserSupportsPowerSync();

  useEffect(() => {
    if (!supported) return;

    let isMounted = true;
    let _powerSync: PowerSyncDatabase;

    const init = async () => {
      _powerSync = await getPowerSyncDB();
      if (isMounted) {
        setPowerSync(_powerSync);
        setReady(true);
      }
    };

    init().catch((err) => {
      console.error(err);
      if (isMounted) {
        setError(err instanceof Error ? err : new Error('Failed to initialize PowerSync'));
      }
    });

    return () => {
      isMounted = false;
    };
  }, [supported]);

  if (!supported || error) {
    return unsupportedFallback || fallback || <div>Local database is unavailable in this browser context.</div>;
  }

  if (!ready || !powerSync) {
    return fallback || <div>Loading local database...</div>;
  }

  return (
    <PowerSyncContext.Provider value={powerSync}>
      {children}
    </PowerSyncContext.Provider>
  );
};
