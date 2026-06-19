/// <reference types="node" />
import { getSystemPowerSync } from '../../lib/powersync/db';

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const db = await getSystemPowerSync();
    await db.execute(
      'INSERT OR REPLACE INTO pending_actions (id, type, payload, timestamp, status, retry_count) VALUES (?, ?, ?, ?, ?, ?)',
      [
        action.id,
        action.type,
        JSON.stringify(action.payload),
        action.timestamp,
        'pending',
        0
      ]
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
    const db = await getSystemPowerSync();
    const result = await db.getAll('SELECT * FROM pending_actions ORDER BY timestamp ASC');
    return result.map((row: any) => ({
      id: row.id,
      type: row.type,
      payload: typeof row.payload === 'string' ? JSON.parse(row.payload) : row.payload,
      timestamp: row.timestamp,
      status: row.status,
      retry_count: row.retry_count
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
    const db = await getSystemPowerSync();
    await db.execute('DELETE FROM pending_actions WHERE id = ?', [id]);
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}
