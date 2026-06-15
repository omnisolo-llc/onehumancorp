import { enqueueAction, getActions, removeAction } from '../../app/utils/offlineQueue';

export class SyncManager {
  private static instance: SyncManager;
  private syncInProgress = false;
  private retryDelayMs = 1000;
  private maxRetries = 5;
  private ws: WebSocket | null = null;
  private wsConnected = false;

  private constructor() {
    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.sync());
      this.initWebSocket();
    }
  }

  private initWebSocket() {
    if (typeof window === 'undefined') return;

    // Connect to the new WS gateway
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/api/sync/ws`;

    this.ws = new WebSocket(wsUrl);

    this.ws.onopen = () => {
      this.wsConnected = true;
      const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";

      // Subscribe to inventory and order updates
      this.ws?.send(JSON.stringify({ action: 'subscribe', topic: `inventory:${tenantId}` }));
      this.ws?.send(JSON.stringify({ action: 'subscribe', topic: `orders:${tenantId}` }));
    };

    this.ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.payload) {
          const payload = JSON.parse(data.payload);
          // Dispatch custom event for UI to consume and update state optimistically
          window.dispatchEvent(new CustomEvent('ohc_sync_event', { detail: { topic: data.topic, payload } }));
        }
      } catch (e) {
        console.error('Failed to parse WS message', e);
      }
    };

    this.ws.onclose = () => {
      this.wsConnected = false;
      // Reconnect with backoff
      setTimeout(() => this.initWebSocket(), 5000);
    };
  }

  public static getInstance(): SyncManager {
    if (!SyncManager.instance) {
      SyncManager.instance = new SyncManager();
    }
    return SyncManager.instance;
  }

  public async enqueue(mutation: any) {
    if (typeof window === 'undefined') return;

    if (!mutation.id) {
        mutation.id = crypto.randomUUID ? crypto.randomUUID() : Date.now().toString() + Math.random().toString();
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

  public async getQueueLength(): Promise<number> {
    const queue = await this.getQueue();
    return queue.length;
  }

  private async getQueue(): Promise<any[]> {
    if (typeof window === 'undefined') return [];
    return await getActions();
  }

  private notifyListeners() {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new Event('ohc_queue_updated'));
      window.dispatchEvent(new Event('storage')); // trigger fallback storage listeners
    }
  }

  public async sync(retryCount = 0) {
    if (typeof window === 'undefined' || this.syncInProgress || !navigator.onLine) return;

    let queue = await this.getQueue();
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
          timestamp: new Date(m.timestamp || Date.now()).toISOString()
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
        } else if (m.type === 'UPDATE_ORDER_STATUS' || m.type === 'TOGGLE_SOLD_OUT') {
            return m; // keep them for KDS
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
      const generalGenMutations = generalMutations.filter(m => m.type !== 'UPDATE_ORDER_STATUS' && m.type !== 'TOGGLE_SOLD_OUT');
      if (generalGenMutations.length > 0) {
        const resGen = await fetch('/api/v1/sync/offline', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({ mutations: generalGenMutations })
        });
        if (!resGen.ok) {
          allOk = false;
          throw new Error(`General Sync failed with status ${resGen.status}`);
        }
      }

      // Sync KDS mutations
      const orderEvents = generalMutations.filter(m => m.type === 'UPDATE_ORDER_STATUS');
      if (orderEvents.length > 0) {
        const resOrder = await fetch('/api/pos/orders', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(orderEvents)
        });
        if (!resOrder.ok) {
          allOk = false;
          throw new Error(`Order Sync failed with status ${resOrder.status}`);
        }
      }

      const inventoryEvents = generalMutations.filter(m => m.type === 'TOGGLE_SOLD_OUT');
      if (inventoryEvents.length > 0) {
        const resInv = await fetch('/api/pos/inventory', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(inventoryEvents)
        });
        if (!resInv.ok) {
          allOk = false;
          throw new Error(`Inventory Sync failed with status ${resInv.status}`);
        }
      }

      if (allOk) {
        // Clear all successfully synced items
        for (const item of queue) {
           await removeAction(item.id);
        }
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
