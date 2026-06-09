import { test, expect } from '@playwright/test';

// NOTE: This test requires a docker-sandbox fix to run properly in CI
// due to pgvector pull permissions in the Bazel test sandbox environment.
test.describe('My Plan Page Loop', () => {
  test('My Plan page loads and displays user tier, usage data, and upgrade paths', async ({ page }) => {
    // Navigate to the plan page
    await page.goto('/plan');

    // Wait for the main heading to appear, indicating successful load
    await expect(page.locator('h1', { hasText: 'My Plan' }).first()).toBeVisible({ timeout: 10000 });

    // Check that Current Usage is present
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();

    // Check that AI Actions Used section is present
    await expect(page.locator('span', { hasText: 'AI Actions Used' })).toBeVisible();

    // Check that Storage Used section is present
    await expect(page.locator('span', { hasText: 'Storage Used' })).toBeVisible();

    // Check that Status Snapshot includes Plan and Estimated Next Bill
    await expect(page.locator('h2', { hasText: 'Current Plan' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Estimated Next Bill' })).toBeVisible();

    // Check for "Upgrade Plan" or "View Upgrade Plans" buttons
    await expect(page.locator('button', { hasText: 'Upgrade Plan' })).toBeVisible();

    // Check navigation buttons work correctly
    await page.locator('button', { hasText: 'Upgrade Plan' }).click();
    await expect(page).toHaveURL('/pricing');
  });
});
