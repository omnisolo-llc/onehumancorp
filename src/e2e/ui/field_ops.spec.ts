import { test, expect } from '../fixtures';

test.describe('Agentic Field Service Scheduling & Quoting', () => {
  test('Owner can review and approve drafted quote from Today\'s Work feed', async ({ page, adminUser, loginAs }) => {
    // Navigate to the mobile UI feed
    // Rely on base URL set by the bazel playwright runner
    await loginAs(page, adminUser);
    await page.goto('/field-ops');

    // Open OHC mobile app feed
    await page.setViewportSize({ width: 375, height: 667 });

    // Assuming we have real data populated via e2e-seed.sql
    await expect(page.locator('h1')).toHaveText("Today's Work");

    // We expect the app to load the seeded appointment
    const card = page.locator('.appointment-card').first();
    await expect(card).toBeVisible();

    // The status should initially be Quote Pending or Requested
    const statusBadge = card.locator('.status-badge');
    await expect(statusBadge).toHaveText('Quote Pending');

    // Click Approve
    const approveBtn = card.locator('.approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Validate the resulting state
    await expect(statusBadge).toHaveText('Approved');
  });
});
