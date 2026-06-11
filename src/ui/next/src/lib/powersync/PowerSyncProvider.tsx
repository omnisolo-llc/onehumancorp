import React, { useEffect, useState, useMemo } from 'react';
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

export const PowerSyncProvider = ({ children }: { children: React.ReactNode }) => {
  const [powerSync, setPowerSync] = useState<PowerSyncDatabase | null>(null);
  const [ready, setReady] = useState(false);

  useEffect(() => {
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

    init().catch(console.error);

    return () => {
       if (_powerSync) {
         _powerSync.disconnect();
         _powerSync.close();
       }
    };
  }, []);

  if (!ready || !powerSync) {
    return <div>Loading local database...</div>;
  }

  return (
    <PowerSyncContext.Provider value={powerSync}>
      {children}
    </PowerSyncContext.Provider>
  );
};
