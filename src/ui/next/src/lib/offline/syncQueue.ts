import { enqueueAction } from '../../app/utils/offlineQueue';
import { SyncManager } from '../sync/SyncManager';

export async function queueOrderAction(orderId: string, actionType: string, payload: any) {
    if (typeof window !== 'undefined') {
        const action = {
            id: crypto.randomUUID(),
            type: actionType,
            payload: { orderId, ...payload },
            timestamp: Date.now()
        };
        await enqueueAction(action);
        if (navigator.onLine) {
            SyncManager.getInstance().sync();
        } else {
             window.dispatchEvent(new Event('ohc_queue_updated'));
        }
    }
}
