import { OfflineAction } from '../../app/utils/offlineQueue';

// We will use Powersync or wa-sqlite to power the offline action queue
import * as SQLite from '@journeyapps/wa-sqlite';

let sqlite3: any;
let db: number;
let dbReady: Promise<void> | null = null;

async function initDB() {
  if (dbReady) return dbReady;

  dbReady = (async () => {
    if (typeof window === 'undefined') return;

    // Dynamically import to ensure it only runs in the browser
    const { default: moduleFactory } = await import('@journeyapps/wa-sqlite/dist/wa-sqlite-async.mjs');
    const { IDBBatchAtomicVFS } = await import('@journeyapps/wa-sqlite/src/examples/IDBBatchAtomicVFS.js');

    sqlite3 = SQLite.Factory(await moduleFactory());
    const vfs = new IDBBatchAtomicVFS('my-vfs');
    await vfs.isReady;
    sqlite3.vfs_register(vfs, true);

    db = await sqlite3.open_v2('ohc_offline_sync.db', 0x00000002 | 0x00000004); // OPEN_READWRITE | OPEN_CREATE

    // Create table if not exists
    await sqlite3.exec(db, `
      CREATE TABLE IF NOT EXISTS pending_actions (
        id TEXT PRIMARY KEY,
        type TEXT NOT NULL,
        payload TEXT NOT NULL,
        timestamp INTEGER NOT NULL
      )
    `);
  })();

  return dbReady;
}

export async function sqliteEnqueueAction(action: OfflineAction): Promise<void> {
  if (typeof window === 'undefined') return;
  await initDB();
  const payloadStr = JSON.stringify(action.payload || {});

  // Use bound parameters to prevent SQL injection
  const sql = `INSERT OR REPLACE INTO pending_actions (id, type, payload, timestamp) VALUES (?, ?, ?, ?)`;
  const str = sqlite3.str_new(db, sql);
  const stmt = sqlite3.prepare_v2(db, sqlite3.str_value(str));
  sqlite3.bind_text(stmt, 1, action.id);
  sqlite3.bind_text(stmt, 2, action.type);
  sqlite3.bind_text(stmt, 3, payloadStr);
  sqlite3.bind_int64(stmt, 4, BigInt(action.timestamp));

  await sqlite3.step(stmt);
  sqlite3.finalize(stmt);
  sqlite3.str_finish(str);
}

export async function sqliteGetActions(): Promise<OfflineAction[]> {
  if (typeof window === 'undefined') return [];
  await initDB();

  const actions: OfflineAction[] = [];
  const sql = `SELECT id, type, payload, timestamp FROM pending_actions ORDER BY timestamp ASC`;
  const str = sqlite3.str_new(db, sql);
  const stmt = sqlite3.prepare_v2(db, sqlite3.str_value(str));

  while (await sqlite3.step(stmt) === SQLite.SQLITE_ROW) {
    const id = sqlite3.column_text(stmt, 0);
    const type = sqlite3.column_text(stmt, 1);
    const payloadStr = sqlite3.column_text(stmt, 2);
    const timestamp = Number(sqlite3.column_int64(stmt, 3));

    actions.push({
      id,
      type,
      payload: payloadStr ? JSON.parse(payloadStr) : {},
      timestamp
    });
  }

  sqlite3.finalize(stmt);
  sqlite3.str_finish(str);
  return actions;
}

export async function sqliteRemoveAction(id: string): Promise<void> {
  if (typeof window === 'undefined') return;
  await initDB();

  const sql = `DELETE FROM pending_actions WHERE id = ?`;
  const str = sqlite3.str_new(db, sql);
  const stmt = sqlite3.prepare_v2(db, sqlite3.str_value(str));
  sqlite3.bind_text(stmt, 1, id);

  await sqlite3.step(stmt);
  sqlite3.finalize(stmt);
  sqlite3.str_finish(str);
}

// In case we want to clear everything
export async function sqliteClearActions(): Promise<void> {
  if (typeof window === 'undefined') return;
  await initDB();

  const sql = `DELETE FROM pending_actions`;
  await sqlite3.exec(db, sql);
}
