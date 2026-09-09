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
    return normalizePowerSyncCredentials(body);
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

type PowerSyncCredentials = {
  endpoint: string;
  token: string;
  expiresAt?: Date;
};

export function normalizePowerSyncCredentials(body: unknown): PowerSyncCredentials | null {
  if (body === null || typeof body !== 'object') return null;

  const response = body as Record<string, unknown>;
  const endpoint = typeof response.powersync_url === 'string'
    ? response.powersync_url.trim()
    : '';
  const token = typeof response.token === 'string' ? response.token.trim() : '';
  if (!endpoint || !token) return null;

  const expiresAtValue = response.expires_at;
  let expiresAt: Date | undefined;
  if (typeof expiresAtValue === 'number' && Number.isFinite(expiresAtValue)) {
    const candidate = new Date(expiresAtValue * 1000);
    if (!Number.isNaN(candidate.getTime())) expiresAt = candidate;
  } else if (typeof expiresAtValue === 'string' && expiresAtValue.trim()) {
    const candidate = new Date(expiresAtValue);
    if (!Number.isNaN(candidate.getTime())) expiresAt = candidate;
  }

  return expiresAt ? { endpoint, token, expiresAt } : { endpoint, token };
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
  const [supported, setSupported] = useState<boolean | null>(null);

  useEffect(() => {
    setSupported(browserSupportsPowerSync());
  }, []);

  useEffect(() => {
    if (supported !== true) return;

    let cancelled = false;

    const handleInitializationError = (err: unknown) => {
      if (cancelled) return;
      console.error(err);
      setError(err instanceof Error ? err : new Error('Failed to initialize PowerSync'));
    };

    const handleConnectionError = () => {
      if (cancelled) return;
      console.warn('PowerSync background sync connection failed; local data remains available.');
    };

    const init = async () => {
      const powerSyncDatabase = await getPowerSyncDB();
      if (cancelled) return;

      await powerSyncDatabase.init();
      if (cancelled) return;

      const connector = new BackendConnector();
      setPowerSync(powerSyncDatabase);
      setReady(true);
      void powerSyncDatabase.connect(connector).catch(handleConnectionError);
    };

    void init().catch(handleInitializationError);

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
