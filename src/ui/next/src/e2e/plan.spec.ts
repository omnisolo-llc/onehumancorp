import { test, expect } from '../../../../e2e/fixtures';

// NOTE: This test requires a docker-sandbox fix to run properly in CI
// due to pgvector pull permissions in the Bazel test sandbox environment.
test.describe('My Plan Page Loop', () => {
  test('My Plan page loads and displays user tier, usage data, and upgrade paths', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' }).first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'AI Actions Used' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage Used' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Plan:' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Estimated Next Bill' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade' })).toBeVisible();
  });

  test('My Plan page displays View Detailed Costs button', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('button', { hasText: 'View Detailed Costs' })).toBeVisible();
  });

  test('My Plan page Upgrade button navigates to pricing', async ({ page }) => {
    await page.goto('/plan');
    await page.locator('button', { hasText: 'Upgrade' }).click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });

  test('My Plan page Manage Billing button navigates correctly', async ({ page }) => {
    await page.goto('/plan');
    await page.locator('button', { hasText: 'Manage Billing' }).click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });
});
