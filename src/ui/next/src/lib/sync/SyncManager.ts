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
      // Map mutations to the format expected by the backend
      const mutations = queue.map(m => {
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
        } else if (m.type === 'tap_to_pay') {
          return {
             transaction_id: m.id,
             product_id: m.product_id || 'offline_payment',
             quantity_deducted: m.quantity || 1,
             amount: Math.round(m.amount),
             payment_method: 'terminal',
             payment_intent_id: m.idempotency_key,
             currency: m.currency || 'usd'
          };
        }
        return m;
      });

      // Get Spiffe ID safely
      const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
      const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

      const response = await fetch('/api/v1/sync/offline', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': spiffeId
        },
        body: JSON.stringify({ mutations })
      });

      if (response.ok) {
        localStorage.setItem(this.queueKey, '[]');
        this.notifyListeners();
        this.retryDelayMs = 1000; // Reset delay on success
      } else {
        throw new Error(`Sync failed with status ${response.status}`);
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
