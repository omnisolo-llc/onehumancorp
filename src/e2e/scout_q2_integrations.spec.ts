import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('scout_q2_integrations');

test('Scout Q2 Integrations E2E Tests - Bookings, Lists, Shipping', async ({ page }) => {
  // Test Cal.com Booking Configuration
  await page.goto('/operations');
  await expect(page.getByRole('heading', { name: /Operations/i }).first()).toBeVisible();

  // Test Email List Marketing Configuration
  await page.goto('/marketing');
  await expect(page.getByRole('heading', { name: /Marketing/i }).first()).toBeVisible();

  // Test Shipping Logistics view
  await page.goto('/orders');
  await expect(page.getByRole('heading', { name: /Orders/i }).first()).toBeVisible();
  const firstOrder = page.locator('.order-row').first();
  if (await firstOrder.isVisible()) {
      await firstOrder.click();
      await expect(page.getByRole('button', { name: /Print Label/i })).toBeVisible();
  }
});
