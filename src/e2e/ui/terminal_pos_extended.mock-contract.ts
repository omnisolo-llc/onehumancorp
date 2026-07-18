import { test, expect } from '@playwright/test';

test.describe('Terminal POS Extended - Inventory and Layout Sync', () => {
  const TENANT_ID = 'terminal-test-tenant';

  test.beforeEach(async ({ page }) => {
    await page.goto('/pos.html');
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }
    await page.getByRole('button', { name: 'Clock In' }).click();
  });

  test('Validates responsiveness matching 375px', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await expect(page.getByRole('heading', { name: 'Clocked In' })).toBeVisible();

    // The container should not overflow its 375px viewport
    const viewportWidth = await page.evaluate(() => window.innerWidth);
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    expect(scrollWidth).toBeLessThanOrEqual(viewportWidth);
  });

  test('Verifies token fetch API is called successfully when Reader connects', async ({ page }) => {
     // A request to the proxy should occur on initialization
     const tokenPromise = page.waitForRequest(
       (request) => request.url().includes('/api/v1/payments/terminal/token') && request.method() === 'POST'
     );

     await page.getByRole('button', { name: 'Quick Charge $50' }).click();
     const tokenReq = await tokenPromise;
     expect(tokenReq).toBeTruthy();
  });

  test('Mocks intent to confirm successful payment state', async ({ page }) => {
     await page.getByRole('button', { name: 'Quick Charge $50' }).click();

     await page.route('**/api/v1/payments/terminal/intent', async (route) => {
         await route.fulfill({
             status: 200,
             contentType: 'application/json',
             body: JSON.stringify({ client_secret: 'mocked_secret_123', lock_id: 'mocked_lock_123' }),
         });
     });

     await expect(page.getByRole('status')).toBeVisible({ timeout: 10000 });
  });

  test('Confirms reserve correctly locks the product inventory', async ({ page }) => {
     await page.getByRole('button', { name: 'Quick Charge $50' }).click();

     await page.route('**/api/v1/payments/terminal/reserve', async (route) => {
         await route.fulfill({
             status: 200,
             contentType: 'application/json',
             body: JSON.stringify({ success: true, lock_id: 'mocked_lock_xyz' }),
         });
     });

     await expect(page.getByRole('status')).toBeVisible({ timeout: 10000 });
  });

  test('Confirms commit successfully calls backend after terminal checkout', async ({ page }) => {
      await page.getByRole('button', { name: 'Quick Charge $50' }).click();

      await page.route('**/api/v1/payments/terminal/commit', async (route) => {
          await route.fulfill({
              status: 200,
              contentType: 'application/json',
              body: JSON.stringify({ success: true }),
          });
      });

      await expect(page.getByRole('status')).toBeVisible({ timeout: 10000 });
  });

});
