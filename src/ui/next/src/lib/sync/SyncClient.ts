const uuidv4 = () => {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
};
import { getPowerSyncDB } from '../powersync/db';

export interface MutationPayload {
  tableName: string;
  operation: 'INSERT' | 'UPDATE' | 'DELETE';
  payloadJson: string;
  idempotencyKey?: string;
}

export class SyncClient {
  static async queueMutation(mutation: MutationPayload) {
    const db = await getPowerSyncDB();
    const id = uuidv4();
    const timestamp = Date.now();
    const idempotencyKey = mutation.idempotencyKey || uuidv4();

    await db.execute(
      `INSERT INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)`,
      [
        id,
        'LOCAL_MUTATION',
        JSON.stringify({
          ...mutation,
          id,
          timestamp,
          idempotencyKey
        }),
        timestamp
      ]
    );

    // Optimistically trigger sync if online
    if (typeof window !== 'undefined' && navigator.onLine) {
      this.syncNow().catch(console.error);
    }

    return { id, idempotencyKey };
  }

  static async syncNow() {
    const db = await getPowerSyncDB();
    const pending = await db.getAll('SELECT * FROM pending_actions ORDER BY timestamp ASC');
    if (pending.length === 0) return;

    const tokenRes = await fetch('/api/v1/auth/powersync_token');
    if (!tokenRes.ok) return; // Wait for next sync cycle

    const mutationsToSync = pending.map(p => {
        const payload = JSON.parse(p.payload);
        return {
            id: payload.id,
            table_name: payload.tableName,
            operation: payload.operation,
            payload_json: payload.payloadJson,
            timestamp: payload.timestamp,
            idempotency_key: payload.idempotencyKey
        };
    });

    try {
        const response = await fetch('/api/v1/sync/mutations', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ mutations: mutationsToSync })
        });

        if (response.ok) {
            const result = await response.json();
            for (const ackedId of result.acked_ids) {
                await db.execute('DELETE FROM pending_actions WHERE id = ?', [ackedId]);
            }
        }
    } catch (e) {
        console.error('Failed to sync mutations', e);
    }
  }
}

if (typeof window !== 'undefined') {
  window.addEventListener('online', () => {
      SyncClient.syncNow().catch(console.error);
  });
}
