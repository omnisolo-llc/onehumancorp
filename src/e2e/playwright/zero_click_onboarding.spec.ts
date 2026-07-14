import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to the real local server
    await page.goto('http://127.0.0.1:18789/setup.html');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Check if the input loaded
    await expect(page.locator('#instant-bio')).toBeVisible();

    // Type into the input
    await page.locator('#instant-bio').fill('I am a baker in Austin selling custom cakes');

    // Click the submit button
    await page.locator('#generate-storefront-btn').click();

<<<<<<< HEAD
    // Wait for the approval details screen
    await expect(page.locator('h1', { hasText: 'Ready to Launch' })).toBeVisible({ timeout: 30000 });
    await expect(page.locator('#approval-details')).toBeVisible();

    // Click Approve & Publish
    const approveBtn = page.locator('#approve-publish-btn-chat');
    await approveBtn.click();

    // The flow goes to the success/dashboard screen.
    await expect(page).toHaveURL(/.*dashboard\.html.*/, { timeout: 30000 });
=======
    // Wait for the loading indicators to appear then the flow goes to the success/dashboard screen.
    await expect(page).toHaveURL(/.*dashboard\.html.*/, { timeout: 60000 });
>>>>>>> d13f80d0c (security(ui): protect session key material)
  });
});
