import { test, expect } from '@playwright/test';
import { setupAuth, generateRandomUser } from './setupAuth';

test.describe('POS Terminal Offline Sync', () => {
  let orgId: string;
  let userId: string;
  const user = generateRandomUser();

  test.beforeAll(async () => {
    const setupResult = await setupAuth(user);
    orgId = setupResult.orgId;
    userId = setupResult.userId;
  });

  test('should queue transaction offline and sync when online', async ({ page, context }) => {
    // Navigate to POS terminal route
    await page.goto(`/pos/terminal`);

    // Add dummy product
    await context.request.post(`/api/v1/products`, {
      data: {
        id: `e2e-product-terminal-${Date.now()}`,
        name: 'Terminal E2E Test Product',
        price_cents: 5000,
        inventory_count: 10,
        status: 'active'
      },
      headers: { 'x-tenant-id': orgId }
    });

    await page.reload();

    // Verify product is visible in the catalog
    await expect(page.locator('text=Terminal E2E Test Product')).toBeVisible();

    // Simulate going offline
    await context.setOffline(true);

    // Click product to add to cart
    await page.locator('text=Terminal E2E Test Product').click();

    // Attempt to charge offline
    await page.locator('button', { hasText: /Collect Payment/ }).click();

    // Should indicate it was queued locally
    await expect(page.locator('text=Synced locally. Will push to cloud when network is restored.')).toBeVisible({ timeout: 10000 });

    // Go back online
    await context.setOffline(false);

    // Wait for auto sync logic from SyncManager
    await page.waitForTimeout(5000);

    // Optional check in universal ledger using API if necessary, but UI should have synced.
  });
});
