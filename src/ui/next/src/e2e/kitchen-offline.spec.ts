import { test, expect } from '@playwright/test';

test.describe('Kitchen Command Center Offline-First Edge Client', () => {
  test('optimistically updates UI and queues offline events then syncs when back online', async ({ page, context, request }) => {
    await page.setViewportSize({ width: 375, height: 667 }); // Target Fatima's mobile context

    // Create a product to ensure we have a menu item to click on
    const loginRes = await request.post('/api/v1/auth/login', {
        data: { email: 'admin@ohc.local', password: 'admin' }
    });
    expect(loginRes.ok()).toBeTruthy();
    const { token, user } = await loginRes.json();

    const productTitle = 'E2E Offline Kitchen ' + Date.now();
    const createRes = await request.post('/api/v1/catalog/products', {
        headers: { Authorization: `Bearer ${token}` },
        data: {
            title: productTitle,
            price_cents: 800,
            inventory_count: 5
        }
    });
    expect(createRes.ok()).toBeTruthy();

    await page.goto('/kitchen');
    await page.evaluate(() => localStorage.setItem('tenant_id', 'tenant-e2e-kitchen'));

    // Wait for the Kitchen UI to load completely
    await expect(page.locator('text=Kitchen Command Center')).toBeVisible();

    const toggleButton = page.locator('button[id^="sold-out-toggle-"]').first();
    await expect(toggleButton).toBeVisible({ timeout: 15000 });

    // Go offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    const initialText = await toggleButton.textContent() || '';

    // Real user click
    await toggleButton.click();

    // Verify optimistic UI
    await expect(toggleButton).not.toHaveText(initialText);

    // Pending Sync Banner Check
    await expect(page.locator('id=queue-dashboard')).toBeVisible({ timeout: 10000 });

    // Go back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // The SyncManager should trigger and clear the queue, hiding the banner
    await expect(page.locator('id=queue-dashboard')).toBeHidden({ timeout: 15000 });
  });
});
