import { PowerSyncDatabase } from '@powersync/web';
import { AppSchema } from './AppSchema';

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
    // Offline mutations handle local changes queue directly
  }
}

let systemPowerSync: PowerSyncDatabase | null = null;
let initPromise: Promise<PowerSyncDatabase> | null = null;

export function getSystemPowerSync(): Promise<PowerSyncDatabase> {
  if (systemPowerSync) return Promise.resolve(systemPowerSync);
  if (initPromise) return initPromise;

  initPromise = new Promise(async (resolve, reject) => {
    try {
      const ps = new PowerSyncDatabase({
        database: {
          dbFilename: 'ohc-offline.db'
        },
        schema: AppSchema,
      });
      await ps.init();
      systemPowerSync = ps;
      resolve(ps);
    } catch (e) {
      reject(e);
    }
  });

  return initPromise;
}
