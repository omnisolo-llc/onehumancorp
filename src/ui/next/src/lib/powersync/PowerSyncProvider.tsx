import React, { useEffect, useState } from 'react';
import { PowerSyncDatabase } from '@powersync/web';
import { PowerSyncContext } from '@powersync/react';
import { AppSchema } from './AppSchema';

class BackendConnector {
  async fetchCredentials() {
    const res = await fetch('/api/v1/auth/powersync_token');
    if (!res.ok) {
      throw new Error(`Failed to get token: ${res.status}`);
    }
    const body = await res.json();
    return {
      endpoint: body.powersync_url,
      token: body.token,
      expiresAt: body.expires_at || new Date(Date.now() + 60 * 60 * 1000).toISOString()
    };
  }
  async uploadData(database: any) {
    // Offline mutations handle local changes queue directly
  }
}

export function isPowerSyncSupportedForLocation(isSecureContext: boolean, hostname: string) {
  return isSecureContext || hostname === 'localhost' || hostname === '127.0.0.1' || hostname === '[::1]';
}

function browserSupportsPowerSync() {
  if (typeof window === 'undefined') return false;
  return isPowerSyncSupportedForLocation(window.isSecureContext, window.location.hostname);
}

import { getPowerSyncDB } from './db';

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
  const [supported, setSupported] = useState<boolean | null>(null);

  useEffect(() => {
    setSupported(browserSupportsPowerSync());
  }, []);

  useEffect(() => {
    if (supported !== true) return;

    let cancelled = false;

    const handleError = (err: unknown) => {
      if (cancelled) return;
      console.error(err);
      setError(err instanceof Error ? err : new Error('Failed to initialize PowerSync'));
    };

    const init = async () => {
      const powerSyncDatabase = await getPowerSyncDB();
      if (cancelled) return;

      await powerSyncDatabase.init();
      if (cancelled) return;

      const connector = new BackendConnector();
      setPowerSync(powerSyncDatabase);
      setReady(true);
      void powerSyncDatabase.connect(connector).catch(handleError);
    };

    void init().catch(handleError);

    return () => {
      cancelled = true;
    };
  }, [supported]);

  if (supported === null) {
    return fallback || <div>Loading local database...</div>;
  }

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
