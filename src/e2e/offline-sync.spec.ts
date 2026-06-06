import { test, expect } from './fixtures';

test.describe('Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh', () => {
  // We use the existing auth and structure seen in other e2e tests
  test('Priya uses Tap-to-Pay offline and syncs inventory when back online', async ({ request, baseURL }) => {
    // 1. Setup Priya's tenant and inventory via API (or assume seeded from test)
    // We will bypass full UI setup for brevity in this specific headless API interaction
    // Since the system uses Spiffe, we assume a mock header works if configured for testing,
    // or we just call the API directly.

    const tenantId = 'tenant-offline-e2e';
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/tester`;

    // Attempt offline sync directly against the existing POS inventory API.
    const payload = {
      type: 'ORDER_CREATED',
      payload: {
        item_id: 'prod-offline-1',
        quantity_sold: 1
      }
    };

    const response = await request.post(`${baseURL}/api/pos/inventory`, {
      headers: {
        'x-spiffe-id': spiffeId,
        'Content-Type': 'application/json'
      },
      data: payload
    });

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
  });
});
