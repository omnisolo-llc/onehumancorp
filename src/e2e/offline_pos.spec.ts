import { test, expect } from './fixtures';
import { pool } from './global-setup';

test.describe('Offline POS Sync API', () => {
  test('should enqueue and process offline POS transactions via API', async ({ adminPage }) => {
    const tenantId = 'e2e-tenant';
    const txId = 'test-offline-tx-12345';

    const payload = {
      mutations: [
        {
          transaction_id: txId,
          product_id: "test-product-id",
          quantity_deducted: 1,
          amount: 2500,
          currency: "USD",
          payment_method: "card_present"
        }
      ]
    };

    const response = await adminPage.request.post('/api/pos/sync?tenant_id=' + tenantId, {
      data: payload
    });

    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data.success).toBe(true);

    // Check if the transaction is pending in the DB
    const res = await pool.query('SELECT status FROM pos_offline_transactions WHERE id = $1 AND tenant_id = $2', [txId, tenantId]);
    expect(res.rows.length).toBeGreaterThan(0);
    expect(['PENDING', 'RESOLVED']).toContain(res.rows[0].status);
  });

  test('should manage terminal sessions via API', async ({ adminPage }) => {
    const tenantId = 'e2e-tenant';
    const deviceId = 'test-device-999';

    // 1. Start Session
    const startRes = await adminPage.request.post('/api/pos/sessions/start?tenant_id=' + tenantId, {
      data: { device_id: deviceId }
    });

    expect(startRes.status()).toBe(200);
    const startData = await startRes.json();
    expect(startData.success).toBe(true);
    const sessionId = startData.session_id;
    expect(sessionId).toBeDefined();

    // 2. Update Session Status
    const updateRes = await adminPage.request.put(`/api/pos/sessions/${sessionId}/status?tenant_id=${tenantId}`, {
      data: { status: 'PAUSED' }
    });
    expect(updateRes.status()).toBe(200);
    const updateData = await updateRes.json();
    expect(updateData.success).toBe(true);

    // 3. End Session
    const endRes = await adminPage.request.post(`/api/pos/sessions/${sessionId}/end?tenant_id=${tenantId}`);
    expect(endRes.status()).toBe(200);
    const endData = await endRes.json();
    expect(endData.success).toBe(true);
  });
});
