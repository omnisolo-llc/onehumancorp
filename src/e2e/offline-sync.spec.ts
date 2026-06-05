import { test, expect } from '@playwright/test';

test.describe('Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh', () => {
  // We use the existing auth and structure seen in other e2e tests
  test('Priya uses Tap-to-Pay offline and syncs inventory when back online', async ({ request, baseURL }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    // 1. Setup Priya's tenant and inventory via API (or assume seeded from test)
    // We will bypass full UI setup for brevity in this specific headless API interaction
    // Since the system uses Spiffe, we assume a mock header works if configured for testing,
    // or we just call the API directly.

    const tenantId = 'tenant-offline-e2e';
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/tester`;

<<<<<<< HEAD
    // Attempt offline sync directly against the existing POS inventory API.
    const payload = {
      type: 'ORDER_CREATED',
      payload: {
        item_id: 'prod-offline-1',
        quantity_sold: 1
      }
    };

    const response = await request.post(`${baseURL}/api/pos/inventory`, {
=======
    // Attempt offline sync directly against the API endpoint
    const payload = {
      mutations: [
        {
          transaction_id: 'tx-tap-to-pay-123',
          product_id: 'prod-offline-1',
          quantity_deducted: 1
        }
      ]
    };

    const response = await request.post(`${baseURL}/api/v1/sync/offline`, {
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
      headers: {
        'x-spiffe-id': spiffeId,
        'Content-Type': 'application/json'
      },
      data: payload
    });

<<<<<<< HEAD
    expect(response.status()).toBe(201);

    const body = await response.json();
    expect(body).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          type: 'ORDER_CREATED',
          sync_status: 'SYNCED'
        })
      ])
    );
=======
    // If not properly seeded in e2e db, it might return 200 OK but log a warning.
    // The requirement is to ensure the API responds correctly.
    expect(response.status()).toBe(200);

    const body = await response.json();
    expect(body.success).toBe(true);
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  });
});
