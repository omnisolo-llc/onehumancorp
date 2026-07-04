import { SyncManager } from './SyncManager';
import { enqueueAction, OfflineAction } from '../../app/utils/offlineQueue';
import { v4 as uuidv4 } from 'uuid';

export class MutationService {
  private static instance: MutationService;

  private constructor() {}

  public static getInstance(): MutationService {
    if (!MutationService.instance) {
      MutationService.instance = new MutationService();
    }
    return MutationService.instance;
  }

  /**
   * Encapsulate a mutation intent, apply it optimistically to UI (via callback), and queue it for sync.
   * @param actionType Description of the action (e.g. 'mark_sold_out', 'process_payment')
   * @param payload Payload specific to the action
   * @param optimisticUpdate Callback to update the local UI optimistically
   * @param rollback Callback to revert the optimistic update if queuing fails
   */
  public async executeMutation(
    actionType: string,
    payload: any,
    optimisticUpdate: () => void,
    rollback: () => void
  ): Promise<void> {
    const intent: OfflineAction = {
      id: uuidv4(),
      type: actionType,
      payload,
      timestamp: Date.now()
    };

    try {
      // 1. Optimistically apply the update
      optimisticUpdate();

      // 2. Queue the intent
      await enqueueAction(intent);

      // 3. Trigger sync via SyncManager
      const syncManager = SyncManager.getInstance();
      if (typeof window !== 'undefined') {
        window.dispatchEvent(new Event('ohc_queue_updated'));
      }
      if (typeof navigator !== 'undefined' && navigator.onLine) {
        syncManager.sync();
      }
    } catch (e) {
      console.error('Failed to execute mutation, rolling back:', e);
      rollback();
      throw e;
    }
  }
}
