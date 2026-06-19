import { getPowerSyncDB } from "../../lib/powersync/db";

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

let tableInitialized = false;

async function ensureTable() {
    if (tableInitialized) return;
    const db = await getPowerSyncDB();
    await db.execute(
        'CREATE TABLE IF NOT EXISTS pending_actions (id TEXT PRIMARY KEY, type TEXT, payload TEXT, timestamp INTEGER)'
    );
    tableInitialized = true;
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    await ensureTable();
    const db = await getPowerSyncDB();
    const payloadStr = typeof action.payload === 'string' ? action.payload : JSON.stringify(action.payload);
    await db.execute(
      'INSERT INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)',
      [action.id, action.type, payloadStr, action.timestamp]
    );
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to enqueue action", err);
    }
  }
}

export async function getActions(): Promise<OfflineAction[]> {
  if (typeof window === "undefined") return [];
  try {
    await ensureTable();
    const db = await getPowerSyncDB();
    const result = await db.getAll('SELECT * FROM pending_actions ORDER BY timestamp ASC');
    return result.map((row: any) => ({
      id: row.id,
      type: row.type,
      payload: (function(p){ try { return JSON.parse(p); } catch { return p; } })(row.payload),
      timestamp: row.timestamp
    }));
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to get actions", err);
    }
    return [];
  }
}

export async function removeAction(id: string): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    await ensureTable();
    const db = await getPowerSyncDB();
    await db.execute('DELETE FROM pending_actions WHERE id = ?', [id]);
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}
