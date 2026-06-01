import { test, expect } from '@playwright/test';
import { setupTestOrganization, createTestUser } from './fixtures'; // Ensure standard setup

test.describe('Autonomous Local Delivery', () => {
  let orgId: string;

  test.beforeEach(async () => {
    // Standard real business owner setup
    orgId = await setupTestOrganization('Fatima Halal Cart');
  });

  test('Business Owner enables local delivery', async ({ page }) => {
    // Authenticate and navigate as Fatima
    await page.goto('/login');
    // Using simple mock flow
    await page.evaluate(() => localStorage.setItem('has_onboarded', 'true'));

    // Check initial state
    await page.goto('/delivery');
    await expect(page.locator('h1')).toHaveText('Local Delivery');

    // Toggle delivery
    const toggle = page.locator('input[type="checkbox"]');
    await toggle.check();
    await expect(toggle).toBeChecked();

    // Verify UI reacts properly
    await expect(page.locator('text=Pending Orders')).toBeVisible();
    await expect(page.locator('text=Active Routes')).toBeVisible();
  });

  test('Driver UI functions properly', async ({ page }) => {
    await page.goto('/driver');

    await expect(page.locator('text=OHC Driver')).toBeVisible();
    await expect(page.locator('text=Online')).toBeVisible();
    await expect(page.locator('text=Waiting for routes...')).toBeVisible();
  });
});
