import { PowerSyncDatabase } from '@powersync/web';
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

let dbInstance: PowerSyncDatabase | null = null;
let initPromise: Promise<PowerSyncDatabase> | null = null;

export async function getPowerSync(): Promise<PowerSyncDatabase> {
  if (dbInstance) {
    return dbInstance;
  }

  if (initPromise) {
    return initPromise;
  }

  initPromise = (async () => {
    const db = new PowerSyncDatabase({
      database: {
        dbFilename: 'ohc-offline.db'
      },
      schema: AppSchema,
    });

    await db.init();

    const connector = new BackendConnector();
    db.connect(connector);

    dbInstance = db;
    return db;
  })();

  return initPromise;
}

export function closePowerSync() {
  if (dbInstance) {
    dbInstance.disconnect();
    dbInstance.close();
    dbInstance = null;
    initPromise = null;
  }
}
