import { PowerSyncDatabase } from '@powersync/web';
import { AppSchema } from './AppSchema';
import { SyncManager } from '../sync/SyncManager';

export class BackendConnector {
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
    // Check for any built-in crud transactions from standard PowerSync tables
    // We complete them instantly to avoid infinite loops, but we use custom sync logic.
    const tx = await database.getNextCrudTransaction();
    if (tx) {
        try {
            await tx.complete();
        } catch(e) {
            console.error("Failed to complete transaction", e);
        }
    }

    // Trigger the actual queue worker logic which uses exponential backoff
    if (typeof window !== 'undefined') {
       // Get the singleton instance dynamically to avoid circular dependency
       const syncManagerModule = await import('../sync/SyncManager');
       await syncManagerModule.SyncManager.getInstance().sync();
    }
  }
}

let powerSyncInstance: PowerSyncDatabase | null = null;
let initPromise: Promise<PowerSyncDatabase> | null = null;

export function isPowerSyncSupportedForLocation(isSecureContext: boolean, hostname: string) {
  return isSecureContext || hostname === 'localhost' || hostname === '127.0.0.1' || hostname === '[::1]';
}

export function browserSupportsPowerSync() {
  if (typeof window === 'undefined') return false;
  return isPowerSyncSupportedForLocation(window.isSecureContext, window.location.hostname);
}

export const getPowerSyncDB = async (): Promise<PowerSyncDatabase> => {
  if (powerSyncInstance) {
    return powerSyncInstance;
  }

  if (initPromise) {
    return initPromise;
  }

  initPromise = (async () => {
    if (!browserSupportsPowerSync()) {
      throw new Error("PowerSync not supported in this environment");
    }

    const db = new PowerSyncDatabase({
      database: {
        dbFilename: 'ohc-offline.db'
      },
      schema: AppSchema,
    });

    await db.init();

    const connector = new BackendConnector();
    db.connect(connector);

    powerSyncInstance = db;
    return db;
  })();

  return initPromise;
};
