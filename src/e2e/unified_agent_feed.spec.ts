import { test, expect } from './fixtures';

test.describe('Unified Agent Feed', () => {
  test('should display agent feed and allow interaction', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Ensure we are using the seeded e2e tenant explicitly to fetch the seed data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to dashboard
    await page.goto('/dashboard');

    // Verify we are on dashboard and the Unified Agent Feed is present
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    // We expect seeded approvals to show up because of our seed data updates
    await expect(page.getByText('Draft email for review')).toBeVisible();
    await expect(page.getByText('Abandoned cart recovery: 10% discount for Sarah')).toBeVisible();

    // Check context data display
    await expect(page.getByText('Abandoned Carts:')).toBeVisible();
    await expect(page.getByText('3', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Potential Revenue:')).toBeVisible();
    await expect(page.getByText('$120.00')).toBeVisible();

    // Verify Edit button exists
    const editBtn = page.locator('button[aria-label="Edit proposal"]').first();
    await expect(editBtn).toBeVisible();

    // Click to decline the abandoned cart proposal
    const declineBtn = page.locator('button[aria-label="Reject proposal"]').last();
    await declineBtn.click();

    // Verify it was optimistically removed from the UI
    await expect(page.getByText('Abandoned cart recovery: 10% discount for Sarah')).not.toBeVisible();
  });
});
