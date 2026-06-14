import { enqueueAction, getActions, removeAction } from '../../app/utils/offlineQueue';

export class SyncManager {
  private static instance: SyncManager;
  private syncInProgress = false;
  private retryDelayMs = 1000;
  private maxRetries = 5;
  private listeners: (() => void)[] = [];

  private constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.sync());
    }
  }

  public static getInstance(): SyncManager {
    if (!SyncManager.instance) {
      SyncManager.instance = new SyncManager();
    }
    return SyncManager.instance;
  }

  public subscribe(listener: () => void) {
    this.listeners.push(listener);
    return () => {
      this.listeners = this.listeners.filter(l => l !== listener);
    };
  }

  private notifyListeners() {
    this.listeners.forEach(l => l());
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new Event('ohc_queue_updated'));
    }
  }

  public async enqueue(mutation: any) {
    if (typeof window === 'undefined') return;

    // ensure mutation has an id and timestamp
    if (!mutation.id) {
        mutation.id = `mutation-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    }
    if (!mutation.timestamp) {
        mutation.timestamp = Date.now();
    }

    await enqueueAction(mutation);
    this.notifyListeners();

    if (navigator.onLine) {
      this.sync();
    }
  }

  public async getQueue(): Promise<any[]> {
    if (typeof window === 'undefined') return [];
    return await getActions();
  }

  public async getQueueLength(): Promise<number> {
    if (typeof window === 'undefined') return 0;
    const actions = await getActions();
    return actions.length;
  }

  public async sync(retryCount = 0) {
    if (typeof window === 'undefined' || this.syncInProgress || !navigator.onLine) return;

    let queue = await getActions();
    if (queue.length === 0) return;

    this.syncInProgress = true;

    try {
      // Separate POS transactions from general offline sync
      const posTransactions = queue.filter(m => m.type === 'tap_to_pay').map(m => {
        return {
          id: m.id,
          client_id: 'terminal_client', // Default fallback
          amount_cents: Math.round(m.payload?.amount || m.amount),
          currency: m.payload?.currency || m.currency || 'usd',
          payload: JSON.stringify([{ product_id: m.payload?.product_id || m.product_id, quantity: m.payload?.quantity || m.quantity || 1 }]),
          timestamp: new Date().toISOString()
        };
      });

      const generalMutations = queue.filter(m => m.type !== 'tap_to_pay').map(m => {
        if (m.type === 'inventory_toggle') {
           return {
              transaction_id: m.id,
              product_id: m.id.replace('e2e-product-', ''),
              quantity_deducted: 1, // Assume 1 for E2E logic
              amount: null,
              payment_method: null,
              payment_intent_id: null,
              currency: null,
              mutation_type: m.type
           };
        } else if (m.type === 'draft_quote') {
          return {
             transaction_id: m.id,
             product_id: 'draft_quote',
             quantity_deducted: 0,
             amount: null,
             payment_method: null,
             payment_intent_id: null,
             currency: 'usd',
             mutation_type: 'draft_quote',
             payload: m.notes || m.payload?.notes
          };
        } else if (m.type === 'approve_agent_feed') {
          return {
             transaction_id: m.id,
             product_id: 'approve_agent_feed',
             quantity_deducted: 0,
             amount: null,
             payment_method: null,
             payment_intent_id: null,
             currency: 'usd',
             mutation_type: 'approve_agent_feed',
             payload: JSON.stringify({
                id: m.payload?.id,
                approved: m.payload?.approved
             })
          };
        }
        return m;
      });

      const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
      const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

      let allOk = true;

      // Sync POS transactions
      if (posTransactions.length > 0) {
        const sessionId = localStorage.getItem('ohc_active_terminal_session_id');
        const resPos = await fetch('/api/v1/payments/terminal/sync_offline', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({
            session_id: sessionId || undefined,
            transactions: posTransactions
          })
        });
        if (!resPos.ok) {
          allOk = false;
          throw new Error(`POS Sync failed with status ${resPos.status}`);
        } else {
            // Remove successful POS transactions
            const posIds = new Set(queue.filter(m => m.type === 'tap_to_pay').map(m => m.id));
            for (const m of queue) {
                if (posIds.has(m.id)) await removeAction(m.id);
            }
        }
      }

      // Sync general mutations
      if (generalMutations.length > 0) {
        const resGen = await fetch('/api/v1/sync/offline', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({ mutations: generalMutations })
        });
        if (!resGen.ok) {
          allOk = false;
          throw new Error(`General Sync failed with status ${resGen.status}`);
        } else {
             const data = await resGen.json();
             if (data && data.failed_count && data.failed_count > 0) {
                 allOk = false;
                 // Note: Ideally we should only remove the ones that succeeded, but the API doesn't tell us which ones failed.
                 // So we keep them in the queue.
             } else {
                 // Remove successful general mutations
                 const genIds = new Set(queue.filter(m => m.type !== 'tap_to_pay').map(m => m.id));
                 for (const m of queue) {
                     if (genIds.has(m.id)) await removeAction(m.id);
                 }
             }
             if (data && data.failed_count && data.failed_count > 0) {
                 // Throw so we trigger the retry logic
                 throw new Error(`${data.failed_count} general mutations failed to sync`);
             }
        }
      }

      if (allOk) {
        this.notifyListeners();
        this.retryDelayMs = 1000; // Reset delay on success
      }
    } catch (e) {
      console.error('Failed to sync offline queue:', e);
      if (retryCount < this.maxRetries) {
        const delay = this.retryDelayMs * Math.pow(2, retryCount);
        setTimeout(() => {
          this.syncInProgress = false;
          this.sync(retryCount + 1);
        }, delay);
        return; // Don't unset syncInProgress yet
      }
    } finally {
      this.syncInProgress = false;
    }
  }
}
