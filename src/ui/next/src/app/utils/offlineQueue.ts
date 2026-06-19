/// <reference types="node" />
import { getPowerSyncInstance } from '../../lib/powersync/db';

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const db = getPowerSyncInstance();
    await db.execute(
      `INSERT OR REPLACE INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)`,
      [action.id, action.type, JSON.stringify(action.payload), action.timestamp]
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
    const db = getPowerSyncInstance();
    const result = await db.getAll('SELECT * FROM pending_actions ORDER BY timestamp ASC');
    return result.map(row => ({
      id: row.id,
      type: row.type,
      payload: JSON.parse(row.payload),
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
    const db = getPowerSyncInstance();
    await db.execute('DELETE FROM pending_actions WHERE id = ?', [id]);
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}
