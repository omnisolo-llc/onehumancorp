export interface OfflinePaymentIntent {
    intent_id: string;
    amount: number;
    currency: string;
    idempotency_key: string;
    status: 'pending' | 'succeeded' | 'failed';
    timestamp: number;
}

export class OfflinePaymentEngine {
    private queue: OfflinePaymentIntent[] = [];
    private _isOnline: boolean = true;

    constructor() {
        this.loadQueue();
    }

    private loadQueue() {
        if (typeof window !== 'undefined' && window.localStorage) {
            const stored = window.localStorage.getItem('ohc_offline_payments');
            if (stored) {
                try {
                    this.queue = JSON.parse(stored);
                } catch (e) {
                    this.queue = [];
                }
            }
        }
    }

    private saveQueue() {
        if (typeof window !== 'undefined' && window.localStorage) {
            window.localStorage.setItem('ohc_offline_payments', JSON.stringify(this.queue));
        }
    }

    public setOnlineStatus(isOnline: boolean) {
        this._isOnline = isOnline;
        if (isOnline) {
            this.syncPendingPayments();
        }
    }

    public isOnline(): boolean {
        return this._isOnline;
    }

    public async processPayment(amount: number, currency: string, idempotencyKey: string): Promise<OfflinePaymentIntent> {
        const intent: OfflinePaymentIntent = {
            intent_id: `pi_offline_${Date.now()}`,
            amount,
            currency,
            idempotency_key: idempotencyKey,
            status: 'pending',
            timestamp: Date.now()
        };

        if (this._isOnline) {
            // Simulated online immediate process
            intent.status = 'succeeded';
        } else {
            // Queue for offline
            this.queue.push(intent);
            this.saveQueue();
        }

        return intent;
    }

    public getPendingQueue(): OfflinePaymentIntent[] {
        return this.queue.filter(q => q.status === 'pending');
    }

    public async syncPendingPayments(): Promise<void> {
        if (!this._isOnline) return;

        const pending = this.getPendingQueue();
        if (pending.length === 0) return;

        // In a real scenario, we would send these to the backend /sync endpoint
        // Here we simulate successful sync
        for (const intent of pending) {
            intent.status = 'succeeded';
        }

        this.saveQueue();
    }
}
