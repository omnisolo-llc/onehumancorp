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

  private notifyListeners(status?: 'syncing' | 'synced' | 'failed') {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent('ohc_sync_status', { detail: { status, length: this.getQueueLength() } }));
      window.dispatchEvent(new Event('ohc_queue_updated'));
      window.dispatchEvent(new Event('storage'));
    }
  }

  public async sync(retryCount = 0) {
    if (typeof window === 'undefined' || this.syncInProgress || !navigator.onLine) return;

    let queue = this.getQueue();
    if (queue.length === 0) return;

    this.syncInProgress = true;
    this.notifyListeners('syncing');

    try {
      const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
      const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

      const response = await fetch('/api/v1/sync/offline', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': spiffeId
        },
        body: JSON.stringify({
          tenant_id: tenantId,
          client_id: 'browser-client',
          transactions: queue.map(m => ({
            id: m.id,
            product_id: m.product_id || 'unknown',
            quantity_deducted: m.quantity_deducted || 0,
            amount_cents: m.amount ? Math.round(m.amount * 100) : 0,
            payment_method: m.type === 'tap_to_pay' ? 'terminal' : null,
            payment_intent_id: m.idempotency_key,
            currency: m.currency || 'USD',
            timestamp: m.timestamp || new Date().toISOString()
          }))
        })
      });

      if (response.ok) {
        localStorage.setItem(this.queueKey, '[]');
        this.notifyListeners('synced');
        this.retryDelayMs = 1000;
      } else {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Sync failed with status ${response.status}`);
      }
    } catch (e) {
      console.error('Failed to sync offline queue:', e);
      this.notifyListeners('failed');
      if (retryCount < this.maxRetries) {
        const delay = this.retryDelayMs * Math.pow(2, retryCount);
        setTimeout(() => {
          this.syncInProgress = false;
          this.sync(retryCount + 1);
        }, delay);
        return;
      }
    } finally {
      this.syncInProgress = false;
    }
  }
}
