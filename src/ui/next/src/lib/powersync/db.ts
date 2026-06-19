import { PowerSyncDatabase } from '@powersync/web';
import { AppSchema } from './AppSchema';

export const getPowerSyncDB = (() => {
  let db: PowerSyncDatabase | null = null;
  let initPromise: Promise<PowerSyncDatabase> | null = null;

  return async (): Promise<PowerSyncDatabase> => {
    if (typeof window === 'undefined') {
       throw new Error('Not running in browser');
    }

    if (db) return db;
    if (initPromise) return initPromise;

    initPromise = (async () => {
      const _db = new PowerSyncDatabase({
        database: { dbFilename: 'ohc-offline.db' },
        schema: AppSchema
      });
      await _db.init();
      await _db.execute(`
        CREATE TABLE IF NOT EXISTS local_pending_actions (
          id TEXT PRIMARY KEY,
          type TEXT,
          payload TEXT,
          timestamp INTEGER
        )
      `);
      db = _db;
      return db;
    })();

    return initPromise;
  };
})();
