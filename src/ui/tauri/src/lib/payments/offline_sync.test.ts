import { expect, test, beforeEach } from 'vitest';
import { OfflinePaymentEngine } from './offline_sync';

beforeEach(() => {
    localStorage.clear();
});

test('processes payment successfully when online', async () => {
    const engine = new OfflinePaymentEngine();
    engine.setOnlineStatus(true);

    const result = await engine.processPayment(5000, 'usd', 'idem-123');
    expect(result.status).toBe('succeeded');
    expect(engine.getPendingQueue().length).toBe(0);
});

test('queues payment when offline', async () => {
    const engine = new OfflinePaymentEngine();
    engine.setOnlineStatus(false);

    const result = await engine.processPayment(5000, 'usd', 'idem-456');
    expect(result.status).toBe('pending');
    expect(engine.getPendingQueue().length).toBe(1);
    expect(engine.getPendingQueue()[0].idempotency_key).toBe('idem-456');
});

test('syncs queued payments when back online', async () => {
    const engine = new OfflinePaymentEngine();
    engine.setOnlineStatus(false);

    await engine.processPayment(5000, 'usd', 'idem-789');
    expect(engine.getPendingQueue().length).toBe(1);

    // Simulate coming back online
    engine.setOnlineStatus(true);
    await engine.syncPendingPayments();

    expect(engine.getPendingQueue().length).toBe(0);
});
