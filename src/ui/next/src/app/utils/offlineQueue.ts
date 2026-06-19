/// <reference types="node" />
import { sqliteEnqueueAction, sqliteGetActions, sqliteRemoveAction } from '../../lib/sync/sqliteQueue';

export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

export async function enqueueAction(action: OfflineAction): Promise<void> {
  return sqliteEnqueueAction(action);
}

export async function getActions(): Promise<OfflineAction[]> {
  return sqliteGetActions();
}

export async function removeAction(id: string): Promise<void> {
  return sqliteRemoveAction(id);
}

// Bind to window for E2E tests
if (typeof window !== 'undefined') {
  (window as any).enqueueOfflineMutation = enqueueAction;
  (window as any).getQueue = getActions;
  (window as any).clearQueue = async () => {
    // Basic clear wrapper if needed by tests
    const actions = await getActions();
    for (const a of actions) {
      await removeAction(a.id);
    }
  };
}
