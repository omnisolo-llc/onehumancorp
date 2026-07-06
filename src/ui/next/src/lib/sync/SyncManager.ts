import { enqueueAction, getActions, removeAction } from '../../app/utils/offlineQueue';

export class SyncManager {
  private static instance: SyncManager;
  private syncInProgress = false;
  private retryDelayMs = 1000;
  private maxRetries = 5;

  private constructor() {
    this.connectWebSocket();

    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.sync());
    }
  }

  private connectWebSocket() {
    if (typeof window === 'undefined') return;
    const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/api/v1/sync/ws?tenant_id=${tenantId}`;

    const ws = new WebSocket(wsUrl);
    ws.onmessage = (event) => {
      if (typeof window !== 'undefined') {
        window.dispatchEvent(new Event('ohc_queue_updated'));
      }
    };
    ws.onclose = () => {
      setTimeout(() => this.connectWebSocket(), 5000);
    };
    ws.onerror = (err) => {
      console.error('Sync WebSocket error', err);
      ws.close();
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
        mutation.id = (typeof crypto !== 'undefined' && crypto.randomUUID) ? crypto.randomUUID() : Date.now().toString() + Math.random().toString().substring(2);
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

  public mapGeneralMutation(m: any): any {
    if (m.type === 'inventory_toggle') {
       return {
          timestamp: new Date(m.timestamp || Date.now()).toISOString(),
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
         timestamp: new Date(m.timestamp || Date.now()).toISOString(),
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
    } else if (m.type === 'agent_intent') {
      return {
         timestamp: new Date(m.timestamp || Date.now()).toISOString(),
         transaction_id: m.id,
         product_id: 'agent_intent',
         quantity_deducted: 0,
         amount: null,
         payment_method: null,
         payment_intent_id: null,
         currency: 'usd',
         mutation_type: 'agent_intent',
         payload: typeof m.payload === 'string' ? m.payload : JSON.stringify(m.payload)
      };
    } else if (m.type === 'UPDATE_ORDER_STATUS' || m.type === 'TOGGLE_SOLD_OUT' || m.type === 'update_quote' || m.type === 'approve_quote' || m.type === 'triage_action' || m.type === 'advisory_action' || m.type === 'field_ops_status' || m.type === 'generate_invoice') {
        return m; // keep them for specific APIs
    }
    return m;
  }

  public async sync(retryCount = 0) {
    if (typeof window === 'undefined' || this.syncInProgress || !navigator.onLine) return;

    let queue = await this.getQueue();
    if (queue.length === 0) return;

    this.syncInProgress = true;

    try {
      // Separate POS transactions from general offline sync
      const posTransactions = queue.filter(m => m.type === 'tap_to_pay' || m.type === 'cash_sale').map(m => {
        let storedDeviceId = 'terminal_client';
        if (typeof window !== 'undefined') {
            storedDeviceId = localStorage.getItem('ohc_pos_device_id') || 'terminal_client';
        }

        return {
          id: m.id,
          client_id: storedDeviceId, // Default fallback
          amount_cents: Math.round(m.amount),
          currency: m.currency || 'usd',
          payload: typeof m.payload === 'string' ? m.payload : JSON.stringify(m.payload || [{ product_id: m.product_id, quantity: m.quantity || 1 }]),
          timestamp: new Date(m.timestamp || Date.now()).toISOString(),
          device_signature: m.device_signature || `sig_offline_${storedDeviceId}_${m.id}`,
          mutation_type: m.type
        };
      });

      const generalMutations = queue.filter(m => m.type !== 'tap_to_pay' && m.type !== 'cash_sale').map(m => this.mapGeneralMutation(m));

      const crdtDeltas = queue.filter(m => m.type === 'CRDT_MUTATION').map(m => {
         return {
            id: m.id,
            entity_id: m.payload.entity_id || 'unknown',
            data: typeof m.payload.data === 'string' ? m.payload.data : JSON.stringify(m.payload.data || {}),
            updated_at: new Date(m.timestamp || Date.now()).toISOString()
         };
      });

      const tenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
      const spiffeId = `spiffe://ohc/org/${tenantId}/agent/ui`;

      let allOk = true;

      // Sync CRDT Deltas
      if (crdtDeltas.length > 0) {
        const resCrdt = await fetch('/api/v1/sync/mcp-deltas', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({ deltas: crdtDeltas })
        });
      }

      // Sync Quote Actions
      const quoteUpdates = generalMutations.filter(m => m.type === 'update_quote');
      for (const update of quoteUpdates) {
        const res = await fetch(`/api/quotes?id=${update.quoteId}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'x-tenant-id': tenantId },
          body: JSON.stringify(update.payload)
        });
      }

      const quoteApprovals = generalMutations.filter(m => m.type === 'approve_quote');
      for (const approval of quoteApprovals) {
        const res = await fetch(`/api/quotes/${approval.quoteId}/approve`, {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json', 'x-tenant-id': tenantId }
        });
      }

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
        try {
          const resPosData = await resPos.json();
          if (resPosData.pending_reconciliation && resPosData.pending_reconciliation.length > 0) {
             if (typeof window !== 'undefined') {
                window.dispatchEvent(new CustomEvent('ohc_sync_reconciliation', { detail: { pending_reconciliation: resPosData.pending_reconciliation } }));
             }
          }
        } catch (e) {
          console.error("Failed to parse POS Sync response", e);
        }
      }

      // Sync general mutations
      const generalGenMutations = generalMutations.filter(m => m.type !== 'UPDATE_ORDER_STATUS' && m.type !== 'TOGGLE_SOLD_OUT' && m.type !== 'update_quote' && m.type !== 'approve_quote' && m.type !== 'CRDT_MUTATION' && m.type !== 'triage_action' && m.type !== 'advisory_action' && m.type !== 'field_ops_status' && m.type !== 'generate_invoice');
      if (generalGenMutations.length > 0) {
        const resGen = await fetch('/api/v1/sync/offline', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'x-spiffe-id': spiffeId
          },
          body: JSON.stringify({ mutations: generalGenMutations })
        });
        try {
          const resGenData = await resGen.json();
          if (resGenData.pending_reconciliation && resGenData.pending_reconciliation.length > 0) {
             if (typeof window !== 'undefined') {
                window.dispatchEvent(new CustomEvent('ohc_sync_reconciliation', { detail: { pending_reconciliation: resGenData.pending_reconciliation } }));
             }
          }
        } catch (e) {
          console.error("Failed to parse General Sync response", e);
        }
      }

      // Sync triage actions
      const triageActions = generalMutations.filter(m => m.type === 'triage_action');
      for (const action of triageActions) {
        try {
          const res = await fetch(`/api/ui/triage/action?tenant_id=${tenantId}`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'x-tenant-id': tenantId,
              'Idempotency-Key': action.id
            },
            body: JSON.stringify(action.payload)
          });
          if (!res.ok) {
            console.error(`Triage Action Sync failed with status ${res.status}`);
            if (res.status >= 500) allOk = false;
          }
        } catch (err) {
          console.error("Triage Action Sync error:", err);
          allOk = false;
        }
      }

      // Sync advisory actions
      const advisoryActions = generalMutations.filter(m => m.type === 'advisory_action');
      for (const action of advisoryActions) {
        try {
          const res = await fetch(`/api/agents/approvals/${action.payload.id}`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'x-tenant-id': tenantId,
              'Idempotency-Key': action.id
            },
            body: JSON.stringify({ approved: action.payload.approved })
          });
          if (!res.ok) {
            console.error(`Advisory Action Sync failed with status ${res.status}`);
            if (res.status >= 500) allOk = false;
          }
        } catch (err) {
          console.error("Advisory Action Sync error:", err);
          allOk = false;
        }
      }


      // Sync generate invoice actions
      const invoiceActions = generalMutations.filter(m => m.type === 'generate_invoice');
      for (const action of invoiceActions) {
        try {
          const res = await fetch(`/api/v1/invoices/generate`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'x-tenant-id': tenantId,
              'Idempotency-Key': action.id
            },
            body: JSON.stringify(action.payload)
          });
          if (!res.ok) {
            console.error(`Generate Invoice Sync failed with status ${res.status}`);
            if (res.status >= 500) allOk = false;
          }
        } catch (err) {
          console.error("Generate Invoice Sync error:", err);
          allOk = false;
        }
      }

      // Sync field ops status actions
      const fieldOpsActions = generalMutations.filter(m => m.type === 'field_ops_status');
      for (const action of fieldOpsActions) {
        try {
          const res = await fetch(`/api/v1/field-ops/appointments`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'x-tenant-id': tenantId,
              'Idempotency-Key': action.id
            },
            body: JSON.stringify(action.payload)
          });
          if (!res.ok) {
            console.error(`Field Ops Status Sync failed with status ${res.status}`);
            if (res.status >= 500) allOk = false;
          }
        } catch (err) {
          console.error("Field Ops Status Sync error:", err);
          allOk = false;
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
      }

      const inventoryEvents = generalMutations.filter(m => m.type === 'TOGGLE_SOLD_OUT');
      if (inventoryEvents.length > 0) {
        const resInv = await fetch('/api/pos/inventory', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(inventoryEvents)
        });
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
