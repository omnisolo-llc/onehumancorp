import { getPowerSyncInstance } from '../../lib/powersync/PowerSyncProvider';

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const powerSync = getPowerSyncInstance();
    if (!powerSync) {
      if (process.env.NODE_ENV !== 'test') {
        console.warn("PowerSync instance not available yet");
      }
      return;
    }

    // Direct SQLite usage over powersync DB
    // Since pending_actions is local-only, write local
    await powerSync.writeTransaction(async (tx) => {
       await tx.execute(
          `INSERT INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)`,
          [action.id, action.type, JSON.stringify(action.payload), action.timestamp]
       );
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to enqueue action", err);
    }
  }
}

export async function getActions(): Promise<OfflineAction[]> {
  if (typeof window === "undefined") return [];
  try {
    const powerSync = getPowerSyncInstance();
    if (!powerSync) {
      return [];
    }

    const result = await powerSync.getAll(`SELECT * FROM pending_actions ORDER BY timestamp ASC`);
    return result.map((row: any) => ({
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
    const powerSync = getPowerSyncInstance();
    if (!powerSync) {
      return;
    }

    await powerSync.writeTransaction(async (tx) => {
       await tx.execute(`DELETE FROM pending_actions WHERE id = ?`, [id]);
    });
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}
