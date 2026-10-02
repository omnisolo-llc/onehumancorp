import { SyncManager } from './SyncManager';
import { enqueueActions, OfflineAction as OperationIntent } from '../../app/utils/offlineQueue';
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
    payload: OperationIntent['payload'],
    optimisticUpdate: () => void,
    rollback: () => void
  ): Promise<void> {
    return this.executeMutationBatch(actionType, [payload], optimisticUpdate, rollback);
  }

  /** All items in one offline sale are persisted or rolled back together. */
  public async executeMutationBatch(
    actionType: string,
    payloads: OperationIntent['payload'][],
    optimisticUpdate: () => void,
    rollback: () => void,
  ): Promise<void> {
    const intents = payloads.map(payload => ({
      id: uuidv4(), type: actionType, payload, timestamp: Date.now(),
    }));
    try {
      optimisticUpdate();
      await enqueueActions(intents);
    } catch (error) {
      rollback();
      throw error;
    }

    // Once committed, notification or connectivity errors must never turn a
    // persisted sale into a failed write that the caller might duplicate.
    try {
      const syncManager = SyncManager.getInstance();
      if (typeof window !== 'undefined') window.dispatchEvent(new Event('omnisolo_queue_updated'));
      if (typeof navigator !== 'undefined' && navigator.onLine) {
        void syncManager.sync().catch(error => console.error('Queued sale sync deferred:', error));
      }
    } catch (error) {
      console.error('Queued sale notification deferred:', error);
    }
  }
}
