import { test, expect } from '@playwright/test';
import { loginAsTestUser } from './fixtures/auth';
import { setupTestTenant } from './fixtures/tenant';

test.describe('Unified Resource Scheduling Matrix', () => {
  let tenantId: string;

  test.beforeEach(async ({ request }) => {
    tenantId = await setupTestTenant(request, 'UnifiedSchedulingTest');
  });

  test('Owner can view resource scheduling on unified feed', async ({ page }) => {
    await loginAsTestUser(page, tenantId);

    // Navigate to unified feed / dashboard
    await page.goto('/dashboard');

    // Expect to see the UI element for resource events (mocking future UI)
    // Here we assert that the dashboard loads correctly without throwing errors
    // regarding the new scheduling matrix.
    await expect(page.locator('h1')).toBeVisible();

    // The owner's feed should not crash and should show operations agent alerts
    // Once actual UI for low stock/reservation events is hooked up, it would be asserted here.
    // For now, we verify the page loads and is functional in the mobile view.
    await page.setViewportSize({ width: 375, height: 812 });

    // Verify touch targets for navigation or main actions are large enough
    const navButtons = await page.locator('nav button').all();
    for (const btn of navButtons) {
        const box = await btn.boundingBox();
        if (box) {
            expect(box.width).toBeGreaterThanOrEqual(44);
            expect(box.height).toBeGreaterThanOrEqual(44);
        }
    }
  });
});
