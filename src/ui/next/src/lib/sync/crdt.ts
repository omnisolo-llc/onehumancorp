import Dexie, { Table } from 'dexie';

export interface CRDTDelta {
  id: string;
  tenantId: string;
  entityId: string;
  data: string;
  updatedAt: string;
  syncedToCloud: boolean;
}

export class OfflineSyncDatabase extends Dexie {
  deltas!: Table<CRDTDelta, string>;

  constructor() {
    super('OfflineSyncDatabase');
    this.version(1).stores({
      deltas: 'id, tenantId, entityId, updatedAt, syncedToCloud'
    });
  }
}

export const syncDb = new OfflineSyncDatabase();

export class SyncEngine {
  private isSyncing = false;

  async pushDeltas() {
    if (this.isSyncing || typeof navigator === 'undefined' || !navigator.onLine) return;
    this.isSyncing = true;

    try {
      const pendingDeltas = await syncDb.deltas.filter(d => !d.syncedToCloud).toArray();
      if (pendingDeltas.length === 0) {
        this.isSyncing = false;
        return;
      }

      const response = await fetch('/api/v1/sync/mcp-deltas', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ deltas: pendingDeltas })
      });

      if (response.ok) {
        const ids = pendingDeltas.map(d => d.id);
        await syncDb.deltas.where('id').anyOf(ids).modify({ syncedToCloud: true });
      }
    } catch (e) {
      console.error('Failed to sync CRDT deltas:', e);
    } finally {
      this.isSyncing = false;
    }
  }

  async applyDelta(delta: Omit<CRDTDelta, 'syncedToCloud'>) {
    await syncDb.deltas.put({
      ...delta,
      syncedToCloud: false
    });
    this.pushDeltas();
  }
}

export const syncEngine = new SyncEngine();

if (typeof window !== 'undefined') {
  window.addEventListener('online', () => syncEngine.pushDeltas());
  setInterval(() => syncEngine.pushDeltas(), 30000);
}
