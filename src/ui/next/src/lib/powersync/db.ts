import { PowerSyncDatabase } from '@powersync/web';
import { AppSchema } from './AppSchema';

let _powerSyncInstance: PowerSyncDatabase | null = null;

export function getPowerSyncInstance(): PowerSyncDatabase {
  if (!_powerSyncInstance) {
    _powerSyncInstance = new PowerSyncDatabase({
      database: {
        dbFilename: 'ohc-offline.db'
      },
      schema: AppSchema,
    });
  }
  return _powerSyncInstance;
}
