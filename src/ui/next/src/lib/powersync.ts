import { PowerSyncDatabase, Schema, Table, column } from "@powersync/web";
import { WASQLitePowerSyncDatabaseOpenFactory } from '@powersync/web';

const omni_inbox_messages = new Table({
  tenant_id: column.text,
  source: column.text,
  original_content: column.text,
  translated_content: column.text,
  source_language: column.text,
  target_language: column.text,
  draft_reply: column.text,
  status: column.text,
  sender_id: column.text,
  customer_id: column.text,
  created_at: column.text,
  updated_at: column.text
});

const agent_feed_items = new Table({
  tenant_id: column.text,
  event_source: column.text,
  context_payload: column.text,
  proposed_action: column.text,
  lifecycle_state: column.text,
  created_at: column.text,
  updated_at: column.text
});

export const AppSchema = new Schema({
  omni_inbox_messages,
  agent_feed_items
});

class BackendConnector {
  async fetchCredentials() {
    // In a real app this would call our backend to get a valid PowerSync JWT token
    // For this prototype/local-first flow, we'll fetch from a dedicated endpoint if available
    // or stub a token. We must connect to the backend power sync service.
    const res = await fetch('/api/auth/powersync/token');
    if (!res.ok) {
      throw new Error(`Failed to fetch PowerSync token: ${res.status}`);
    }
    const { token, endpoint } = await res.json();

    return {
      endpoint: endpoint || "http://localhost:8080", // fallback
      token: token,
    };
  }

  async uploadData(database: PowerSyncDatabase) {
    const batch = await database.getCrudBatch();
    if (!batch) return;

    for (const op of batch.crud) {
      try {
        const url = `/api/data/${op.table}`;
        const headers = { 'Content-Type': 'application/json' };
        let method = 'POST';
        let body: any = { id: op.id, ...op.opData };

        if (op.op === 'PUT') {
            method = 'PUT';
        } else if (op.op === 'PATCH') {
            method = 'PATCH';
        } else if (op.op === 'DELETE') {
            method = 'DELETE';
            body = undefined;
        }

        const res = await fetch(url, {
          method,
          headers,
          body: body ? JSON.stringify(body) : undefined,
        });

        if (!res.ok) {
           throw new Error(`Upload failed for ${op.table}`);
        }
      } catch(ex) {
        // Keep batch to retry later
        return;
      }
    }

    await batch.complete();
  }
}

export const db = new PowerSyncDatabase({
  schema: AppSchema,
  database: new WASQLitePowerSyncDatabaseOpenFactory({
    dbFilename: 'ohc_local.db'
  })
});

export const setupPowerSync = async () => {
    try {
        await db.init();
        const connector = new BackendConnector();
        db.connect(connector);
    } catch (e) {
        console.error("PowerSync setup error:", e);
    }
};

export { PowerSyncContext } from "@powersync/react";