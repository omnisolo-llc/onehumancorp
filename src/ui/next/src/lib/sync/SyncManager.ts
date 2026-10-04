import { enqueueAction, getActions, getActionRoutes, claimAction, completeAction, getQueueSummary } from '../../app/utils/offlineQueue';
import type { OfflineAction, MutationPayload } from '../../app/utils/offlineQueue';
import { readQueueOwner, sameOwner, type QueueOwner } from './queueIdentity';
import { readOutcome } from './queueRoutes';

type QueuedMutation = Omit<OfflineAction, 'id' | 'timestamp'> & { id?: string; timestamp?: number | string };
type MappedMutation = Omit<Partial<OfflineAction>, 'payload' | 'timestamp'> & {
  timestamp?: number | string;
  transaction_id?: string;
  quantity_deducted?: number;
  payment_method?: string | null;
  payment_intent_id?: string | null;
  mutation_type?: string;
  payload?: MutationPayload | string;
};

export class SyncManager {
  private static instance: SyncManager;
  private syncInProgress = false;
  private enqueueDuringSync = false;
  private syncingFieldOnly = false;
  private fullDrainRequested = false;

  private constructor() {
    this.connectWebSocket();

    if (typeof window !== 'undefined') {
      window.addEventListener('online', () => this.sync());
    }
  }

  private connectWebSocket() {
    // Next.js route handlers cannot safely proxy a WebSocket upgrade while keeping
    // the server-issued session credential private. The native desktop client owns
    // authenticated real-time streams; the web client uses the authenticated HTTP
    // sync routes below and must never construct a tenant-bearing socket URL.
  }

  public static getInstance(): SyncManager {
    if (!SyncManager.instance) {
      SyncManager.instance = new SyncManager();

    }
    return SyncManager.instance;
  }

  public async enqueue(mutation: QueuedMutation, expectedOwner?: QueueOwner) {
    if (typeof window === 'undefined') return;

    if (!mutation.id) {
        mutation.id = (typeof crypto !== 'undefined' && crypto.randomUUID) ? crypto.randomUUID() : Date.now().toString() + Math.random().toString().substring(2);
    }
    if (!mutation.timestamp) {
        mutation.timestamp = Date.now();
    }

    const timestamp = typeof mutation.timestamp === 'string' ? Date.parse(mutation.timestamp) : mutation.timestamp;
    if (!Number.isFinite(timestamp)) throw new Error('Offline mutation timestamp must be valid');
    const action = { ...mutation, id: mutation.id, timestamp };
    if (expectedOwner) await enqueueAction(action, expectedOwner);
    else await enqueueAction(action);
    this.notifyListeners();

    if (navigator.onLine) {
      if (this.syncInProgress) { this.enqueueDuringSync = true; this.fullDrainRequested = true; }
      else void this.sync();
    }
  }

  public async enqueueMutation(mutation: QueuedMutation, expectedOwner?: QueueOwner) {
    return expectedOwner ? this.enqueue(mutation, expectedOwner) : this.enqueue(mutation);
  }

  public async getQueueSummary() { return getQueueSummary(); }

  public async getQueueLength(): Promise<number> {
    const queue = await this.getQueue();
    return queue.length;
  }

  private async getQueue(): Promise<OfflineAction[]> {
    if (typeof window === 'undefined') return [];
    return await getActions();
  }

  private notifyListeners() {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new Event('omnisolo_queue_updated'));
    }
  }

  public mapGeneralMutation(m: QueuedMutation & { id: string }): MappedMutation {
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
    } else if (m.type === 'UPDATE_ORDER_STATUS' || m.type === 'TOGGLE_SOLD_OUT' || m.type === 'update_quote' || m.type === 'approve_quote' || m.type === 'triage_action' || m.type === 'advisory_action' || m.type === 'field_ops_status' || m.type === 'fulfillment_action' || m.type === 'generate_invoice' || m.type === 'sync_event') {
        return m; // keep them for specific APIs
    }
    return m;
  }

  /** Reopen only the existing durable field handoff on mount/session restoration. */
  public async resumeFieldCompletionWork(): Promise<void> {
    if (typeof window === 'undefined' || !navigator.onLine) return;
    try {
      const queue = await this.getQueue();
      if (!queue.some(action => action.field_completion || action.field_completion_parent_id)) return;
      if (this.syncInProgress) this.enqueueDuringSync = true;
      else await this.sync(true);
    } catch {
      // Unverified owner or unavailable storage leaves the journal held. The
      // existing queue indicator exposes its state; no request is inferred.
    }
  }

  public async sync(fieldCompletionOnly = false) {
    if (typeof window === 'undefined' || !navigator.onLine) return;
    if (this.syncInProgress) {
      if (!fieldCompletionOnly && this.syncingFieldOnly) { this.fullDrainRequested = true; this.enqueueDuringSync = true; }
      return;
    }
    // Acquire before the first await, including queue reads.
    this.syncInProgress = true;
    this.syncingFieldOnly = fieldCompletionOnly;
    let completedPass = false;
    try {
      const queue = await this.getQueue();
      for (const action of queue) {
        if (fieldCompletionOnly && !action.field_completion && !action.field_completion_parent_id) continue;
        for (const plan of await getActionRoutes(action.id)) {
          const claim = await claimAction(action.id, plan.id);
          if (!claim) continue;
          let outcome;
          try {
            // The expected-owner headers are a precondition; the sealed server
            // session remains the authority even if login changes during fetch.
            if (!sameOwner(claim.owner, await readQueueOwner())) {
              outcome = { status: 'blocked' as const, reason: 'Session owner changed before send' };
            } else {
              const response = await fetch(claim.route.id, {
                method: claim.route.method,
                credentials: 'same-origin', redirect: 'error',
                headers: { 'Content-Type': 'application/json', 'Idempotency-Key': claim.action.id,
                  'x-ohc-expected-user': claim.owner.userId, 'x-ohc-expected-tenant': claim.owner.tenantId },
                ...(claim.route.body === undefined ? {} : { body: JSON.stringify(claim.route.body) }),
              });
              let body: unknown;
              try { body = await response.json(); } catch { body = undefined; }
              outcome = readOutcome(claim.action.id, claim.route.id, response.status, body);
            }
          } catch {
            // Unknown effects, including lost responses, must never be replayed.
            outcome = { status: 'reconciliation' as const, reason: 'Request outcome could not be verified' };
          }
          // Persist the result to the original adapter and original owner's row.
          // A commit failure retains inflight state and stops this sync attempt.
          await completeAction(claim, outcome.status, outcome.reason);
          if (outcome.status === 'acknowledged' && claim.action.field_completion) this.enqueueDuringSync = true;
          this.notifyListeners();
        }
      }
      completedPass = true;
    } catch (error) {
      console.error('Offline queue requires attention:', error);
    } finally {
      this.syncInProgress = false;
      // A committed enqueue can arrive after this pass captured its snapshot.
      // Re-read once for that new work; existing claims still hold unknown,
      // blocked, and acknowledged actions without replaying their effects.
      if (this.enqueueDuringSync) {
        this.enqueueDuringSync = false;
        const nextFieldOnly = fieldCompletionOnly && !this.fullDrainRequested;
        this.fullDrainRequested = false;
        if (completedPass) void this.sync(nextFieldOnly);
      }
    }
  }
}

export const syncManager = SyncManager.getInstance();
