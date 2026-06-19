import SQLiteESMFactory from '@journeyapps/wa-sqlite/dist/wa-sqlite-async.mjs';
import * as SQLite from '@journeyapps/wa-sqlite';
import { IDBBatchAtomicVFS } from '@journeyapps/wa-sqlite/src/examples/IDBBatchAtomicVFS.js';

export interface OfflineAction {
  id: string;
  type: string;
  payload: any;
  timestamp: number;
}

const DB_NAME = "OHC_Offline_Queue";

let dbPromise: Promise<number> | null = null;
let sqlite3: any | null = null;

async function getDB(): Promise<{ db: number, sqlite3: any }> {
  if (typeof window === "undefined") {
      throw new Error("IndexedDB/SQLite only available in browser");
  }

  if (dbPromise && sqlite3) {
      return { db: await dbPromise, sqlite3 };
  }

  let module;
  if (process.env.NODE_ENV === 'test') {
     const memoryFactory = require('@journeyapps/wa-sqlite/dist/wa-sqlite.cjs');
     module = await memoryFactory();
  } else {
     module = await SQLiteESMFactory();
  }

  sqlite3 = SQLite.Factory(module);

  if (process.env.NODE_ENV !== 'test') {
      const vfs = new IDBBatchAtomicVFS('idb-batch-atomic');
      sqlite3.vfs_register(vfs, true);
  }

  dbPromise = new Promise(async (resolve, reject) => {
      try {
          const vfsName = process.env.NODE_ENV === 'test' ? undefined : 'idb-batch-atomic';
          const db = await sqlite3!.open_v2(DB_NAME, 0x00000004 | 0x00000002, vfsName);
          await sqlite3!.exec(db, `
              CREATE TABLE IF NOT EXISTS pending_actions (
                  id TEXT PRIMARY KEY,
                  type TEXT NOT NULL,
                  payload TEXT NOT NULL,
                  timestamp INTEGER NOT NULL
              );
          `);
          resolve(db);
      } catch (e) {
          reject(e);
      }
  });

  return { db: await dbPromise, sqlite3 };
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === "undefined") return;
  try {
    const { db, sqlite3 } = await getDB();
    const sql = `INSERT OR REPLACE INTO pending_actions (id, type, payload, timestamp) VALUES ('${action.id}', '${action.type}', '${JSON.stringify(action.payload).replace(/'/g, "''")}', ${action.timestamp})`;
    await sqlite3.exec(db, sql);
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to enqueue action", err);
    }
  }
}

export async function getActions(): Promise<OfflineAction[]> {
  if (typeof window === "undefined") return [];
  try {
    const { db, sqlite3 } = await getDB();
    const actions: OfflineAction[] = [];
    await sqlite3.exec(db, 'SELECT id, type, payload, timestamp FROM pending_actions ORDER BY timestamp ASC', (row: any[]) => {
        actions.push({
            id: row[0] as string,
            type: row[1] as string,
            payload: JSON.parse(row[2] as string),
            timestamp: parseInt(row[3] as string, 10)
        });
    });
    return actions;
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
    const { db, sqlite3 } = await getDB();
    const sql = `DELETE FROM pending_actions WHERE id = '${id}'`;
    await sqlite3.exec(db, sql);
  } catch (err) {
    if (process.env.NODE_ENV !== 'test') {
      console.error("Failed to remove action", err);
    }
  }
}
