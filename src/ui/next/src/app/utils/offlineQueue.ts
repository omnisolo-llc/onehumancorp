import { SyncManager, OfflineAction } from "../../lib/sync/SyncManager";

export type { OfflineAction };

export async function enqueueAction(action: OfflineAction): Promise<void> {
  const syncManager = SyncManager.getInstance();
  await syncManager.enqueue(action);
}

export async function getActions(): Promise<OfflineAction[]> {
  const syncManager = SyncManager.getInstance();
  return await syncManager.getQueue();
}

export async function removeAction(id: string): Promise<void> {
  const syncManager = SyncManager.getInstance();
  await syncManager.removeAction(id);
}
