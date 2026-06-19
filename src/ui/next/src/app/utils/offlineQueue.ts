import { getPowerSync } from '../../lib/powersync/db';
import { OfflineBackgroundWorker } from '../../lib/sync/OfflineBackgroundWorker';

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const db = await getPowerSync();
    await db.execute(
      'INSERT INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)',
      [action.id, action.type, JSON.stringify(action.payload), action.timestamp]
    );

    // Trigger background worker
    OfflineBackgroundWorker.getInstance().drainPendingActions();
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to enqueue action", err);
    }
  }
}

export async function getActions(): Promise<OfflineAction[]> {
  if (typeof window === "undefined") return [];
  try {
    const db = await getPowerSync();
    const result = await db.getAll('SELECT * FROM pending_actions ORDER BY timestamp ASC');
    return result.map((row: any) => ({
      id: row.id,
      type: row.type,
      payload: row.payload ? JSON.parse(row.payload) : null,
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
    const db = await getPowerSync();
    await db.execute('DELETE FROM pending_actions WHERE id = ?', [id]);
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}
