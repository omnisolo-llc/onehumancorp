import { test, expect } from '@playwright/test';

test.describe('Real-Time Multilingual KDS & Pre-Order Engine', () => {
  test('Fatima operates the KDS locally (offline first, Arabic UI, massive touch targets)', async ({ page, context }) => {
    // Navigate to the KDS route
    await page.goto('http://localhost:3000/kds');

    // 1. Multilingual Support
    // The initial language is English
    await expect(page.locator('h1').first()).toContainText('Kitchen Display');
    // Toggle to Arabic
    await page.click('#lang-toggle');
    // Ensure the title and layout switch to Arabic
    await expect(page.locator('h1').first()).toContainText('شاشة المطبخ');

    // Switch back to English to ease the rest of the test assertions
    await page.click('#lang-toggle');
    await expect(page.locator('h1').first()).toContainText('Kitchen Display');

    // 2. Offline queueing test
    await context.setOffline(true);

    await page.evaluate(() => {
        window.dispatchEvent(new Event('offline'));
    });

    // Verify offline banner shows
    await expect(page.locator('#network-status-indicator')).toHaveClass(/block/);

    // Toggle Falafel as sold out
    const falafelBtn = page.locator('#sold-out-toggle-falafel');
    await falafelBtn.click();
    await expect(falafelBtn).toContainText('Mark Available');

    // Verify that the action was queued locally
    await expect(page.locator('#queue-dashboard')).toHaveText(/1 Mutations Pending Sync/);

    // 3. Mark an order as preparing offline
    // Since there's an initial order we can mark it
    const preparingBtn = page.locator('button', { hasText: 'Start Preparing' }).first();
    await preparingBtn.click();

    await expect(page.locator('#queue-dashboard')).toHaveText(/2 Mutations Pending Sync/);

    // 4. Online sync recovery
    // IMPORTANT: Let it hit the real backend
    await context.setOffline(false);

    await page.evaluate(() => {
        window.dispatchEvent(new Event('online'));
    });

    // Wait for the sync to complete and the queue to hide
    await expect(page.locator('#queue-dashboard')).toBeHidden({ timeout: 10000 });
    await expect(page.locator('#network-status-indicator')).toHaveClass(/hidden/);

    // 5. Incoming pre-order via push
    await page.evaluate(() => {
        const pushEvent = new CustomEvent('push-notification');
        window.dispatchEvent(pushEvent);
    });

    // The order should be added to the queue
    const newOrderBadge = page.locator('span', { hasText: 'New Orders' }).first();
    await expect(newOrderBadge).toBeVisible();
  });
});
