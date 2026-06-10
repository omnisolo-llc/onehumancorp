export class SyncManager {
  private static instance: SyncManager;
  private queueKey = 'ohc_offline_queue';
  private syncInProgress = false;
  private retryDelayMs = 1000;
  private maxRetries = 5;

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

  public enqueue(mutation: any) {
    if (typeof window === 'undefined') return;

    const queue = this.getQueue();
    queue.push(mutation);
    localStorage.setItem(this.queueKey, JSON.stringify(queue));
    this.notifyListeners();

    if (navigator.onLine) {
      this.sync();
    }
  }

  public getQueueLength(): number {
    return this.getQueue().length;
  }

  private getQueue(): any[] {
    if (typeof window === 'undefined') return [];
    try {
      return JSON.parse(localStorage.getItem(this.queueKey) || '[]');
    } catch {
      return [];
    }
  }

  private notifyListeners() {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new Event('ohc_queue_updated'));
      window.dispatchEvent(new Event('storage')); // trigger fallback storage listeners
    }
  }

  public async sync(retryCount = 0) {
    if (typeof window === 'undefined' || this.syncInProgress || !navigator.onLine) return;

    let queue = this.getQueue();
    if (queue.length === 0) return;

    this.syncInProgress = true;

    try {
      // Separate POS transactions from general offline sync
      const posTransactions = queue.filter(m => m.type === 'tap_to_pay').map(m => {
        return {
          id: m.id,
          client_id: 'terminal_client', // Default fallback
          amount_cents: Math.round(m.amount),
          currency: m.currency || 'usd',
          payload: JSON.stringify([{ product_id: m.product_id, quantity: m.quantity || 1 }]),
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
              currency: null
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
             payload: m.notes
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
        }
      }

      if (allOk) {
        localStorage.setItem(this.queueKey, '[]');
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
