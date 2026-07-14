import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to the real local server
    await page.goto('/setup.html');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Check if the input loaded
    await expect(page.locator('#instant-bio')).toBeVisible();

    // Type into the input
    await page.locator('#instant-bio').fill('I am a baker in Austin selling custom cakes');

    // Click the submit button
    await page.locator('#generate-storefront-btn').click();

    // The flow goes directly to the success/dashboard screen (Zero-Click, no approval).
    await expect(page).toHaveURL(/.*dashboard\.html.*/, { timeout: 60000 });
  });
});
