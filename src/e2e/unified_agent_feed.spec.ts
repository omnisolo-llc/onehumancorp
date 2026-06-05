import { test, expect } from './fixtures';

test.describe('Unified Agent Feed', () => {
  test('should display agent feed and allow interaction', async ({ page }) => {
    test.skip(true, 'Docker overlayfs bug breaks E2E test environments');

    // Ensure we are using the seeded e2e tenant explicitly to fetch the seed data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to dashboard
    await page.goto('/dashboard');

    // Verify we are on dashboard and the Unified Agent Feed is present
    await expect(page.getByRole('heading', { name: 'Agent Proposals' })).toBeVisible();

    // We expect seeded approvals to show up because of our seed data updates
    await expect(page.getByText('Draft email for review')).toBeVisible();
    await expect(page.getByText('Abandoned cart recovery: 10% discount for Sarah')).toBeVisible();

    // Click to approve the email draft
    const approveBtn = page.locator('button[aria-label="Approve proposal"]').first();
    await approveBtn.click();

    // Verify it was optimistically removed from the UI
    await expect(page.getByText('Draft email for review')).not.toBeVisible();
  });
});
