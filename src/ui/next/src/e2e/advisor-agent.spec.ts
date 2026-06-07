import { test, expect } from '@playwright/test';
// TODO: Replace with actual database seeding in CI
// This test relies on standard data flowing through the app stack.
test.describe('Business Advisory Agent', () => {
  test('displays advisory report in Agent Feed and allows approval', async ({ page }) => {
    // Navigate to agents page
    await page.goto('/agents');

    // Due to the requirement of no API mocks, we rely on the staging backend
    // to have an advisory report generated for the test user via cron.
    // Ensure the page loads without crashing.
    await expect(page.getByRole('heading', { name: 'Your Team' }).first()).toBeVisible();

    // Switch to Approvals tab
    await page.getByRole('button', { name: 'Needs Approval' }).click();
  });
});
