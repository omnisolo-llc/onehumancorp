import { PowerSyncDatabase, Schema, Table, column } from '@powersync/web';

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

const queueSchema = new Schema({
  pending_actions: new Table({
    type: column.text,
    payload: column.text,
    timestamp: column.integer
  })
});

let dbInstance: PowerSyncDatabase | null = null;

async function getDB(): Promise<PowerSyncDatabase> {
  if (typeof window === 'undefined') {
     throw new Error("Cannot initialize SQLite on server side");
  }
  if (!dbInstance) {
    dbInstance = new PowerSyncDatabase({
      database: {
        dbFilename: 'ohc-action-queue.db'
      },
      schema: queueSchema
    });
    await dbInstance.init();
  }
  return dbInstance;
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const db = await getDB();
    await db.execute(
      'INSERT INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)',
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
    const db = await getDB();
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
    const db = await getDB();
    await db.execute('DELETE FROM pending_actions WHERE id = ?', [id]);
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}
