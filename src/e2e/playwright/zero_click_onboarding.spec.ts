import { test as base, expect } from '@playwright/test';

const test = base.extend({
  page: async ({ page }, use) => {
    // Ensure we do not inherit network block from fixtures
    await use(page);
  }
});

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('/setup.html');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Check if the chat assistant loaded
    await expect(page.locator('#instant-bio')).toBeVisible();

    // The user input should be visible
    const input = page.locator('#instant-bio');
    await expect(input).toBeVisible();

    // Type into the input
    await input.fill('I am a baker in Austin selling custom cakes');

    // Click the submit button
    const submitBtn = page.locator('#generate-storefront-btn');
    await submitBtn.click();

    // Verify Approval UI and Deposit Policy
    await expect(page.locator('h1', { hasText: 'Ready to Launch' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('#approval-details')).toBeVisible();
    await expect(page.locator('#approval-details')).toContainText('Suggested Deposit Policy');
    await expect(page.locator('#approval-details')).toContainText(/Requires a \\d+% upfront deposit/);

    // Click Approve & Publish
    const approveBtn = page.locator('#approve-publish-btn');
    await approveBtn.click();

    // Verify successful generation
    await expect(page).toHaveURL(/.*dashboard.html.*/, { timeout: 15000 });
  });
});
