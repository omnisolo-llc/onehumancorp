import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding Agent', () => {
  test('generates workspace from conversational input', async ({ page }) => {
    // Navigate to the onboarding page directly
    await page.goto('/onboarding');

    // Ensure mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });

    // Enter business description
    await page.fill('[data-testid="business-description-input"]', 'I bake custom cakes in Austin and sell them on Instagram.');

    // Click Analyze
    await page.click('[data-testid="process-button"]');

    // Wait for the thinking state to finish and present the proposed setup
    await expect(page.locator('[data-testid="proposed-setup-card"]')).toBeVisible({ timeout: 5000 });

    // Verify touch target size of the approve button
    const box = await page.locator('[data-testid="approve-button"]').boundingBox();
    expect(box!.width).toBeGreaterThanOrEqual(44);
    expect(box!.height).toBeGreaterThanOrEqual(44);

    // Wait for network response handling to redirect to dashboard
    // Note: in testing, the actual backend call might fail without valid auth/tenant,
    // but the spec verifies the agent flow works.

    // Let's mock the backend response for the test to verify the UI flow correctly navigates
    await page.route('/api/growth/zero-click-generate', async route => {
      const json = { organization_id: 'test-org', user_id: 'test-user', message: 'ok' };
      await route.fulfill({ json });
    });

    await page.click('[data-testid="approve-button"]');

    // Verify redirect
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
