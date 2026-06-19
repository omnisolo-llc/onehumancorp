import { PowerSyncDatabase } from '@powersync/web';
import { AppSchema } from './AppSchema';

export const powerSyncDb = new PowerSyncDatabase({
  database: {
    dbFilename: 'ohc-offline.db'
  },
  schema: AppSchema,
});
