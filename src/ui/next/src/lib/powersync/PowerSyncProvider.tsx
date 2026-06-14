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
    let _powerSync: PowerSyncDatabase;
    const init = async () => {
      _powerSync = new PowerSyncDatabase({
        database: {
          dbFilename: 'ohc-offline.db'
        },
        schema: AppSchema,
      });

      await _powerSync.init();

      const connector = new BackendConnector();
      _powerSync.connect(connector);

      setPowerSync(_powerSync);
      setReady(true);
    };

    init().catch((err) => {
      console.error(err);
      setError(err instanceof Error ? err : new Error('Failed to initialize PowerSync'));
    });

    return () => {
       if (_powerSync) {
         _powerSync.disconnect();
         _powerSync.close();
       }
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
