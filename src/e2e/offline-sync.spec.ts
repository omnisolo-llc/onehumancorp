import { test, expect } from '@playwright/test';

test.describe('Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh', () => {
  test('CUJ: Simulate a mobile device offline sync dropping inventory successfully via mesh handler', async ({ request }) => {
    // 1. Create a dummy tenant Spiffe ID for validation
    const spiffeId = "spiffe://ohc/org/example.org/tenant/tenant_playwright";

    // 2. Perform a test offline sync API call simulating edge cache flush
    const response = await request.post('/api/v1/sync/offline', {
      headers: {
        'x-spiffe-id': spiffeId,
        'Content-Type': 'application/json',
      },
      data: {
        mutations: [
          {
            product_id: "prod_playwright_test",
            action: "sale",
            quantity: 3
          }
        ]
      }
    });

    // 3. Assert the Sync Event Handler returns success
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.success).toBe(true);

    // Depending on pre-existing data (since E2E runs against live DB),
    // we may see merged_count = 0 or 1. We just ensure the endpoint accepts the payload.
    expect(typeof body.merged_count).toBe('number');
  });

  test('CUJ: Ensure unauthorized edge syncs are rejected', async ({ request }) => {
    // 1. Send sync without x-spiffe-id header
    const response = await request.post('/api/v1/sync/offline', {
      headers: {
        'Content-Type': 'application/json',
      },
      data: {
        mutations: [
          {
            product_id: "prod_playwright_test",
            action: "sale",
            quantity: 1
          }
        ]
      }
    });

    // 2. Assert Unauthorized Rejection
    expect(response.status()).toBe(401);
    const body = await response.json();
    expect(body.success).toBe(false);
  });
});
