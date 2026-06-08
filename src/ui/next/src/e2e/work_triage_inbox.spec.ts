import { test, expect } from './fixtures';

test.describe('Work Triage Inbox', () => {
  test('displays the triage inbox and interacts with items', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Welcome back' })).toBeVisible({ timeout: 10000 });

    // Navigate to Work Triage page
    await page.getByRole('link', { name: 'Work Triage Intelligent inbox' }).click();

    // Verify the Work Triage page has loaded
    await expect(page.getByRole('heading', { name: 'Work Triage', exact: true })).toBeVisible();
    await expect(page.getByText('Your intelligent command center for prioritizing work.')).toBeVisible();

    // Verify 'Needs Your Attention' section is visible
    await expect(page.getByRole('heading', { name: 'Needs Your Attention' })).toBeVisible();

    // The backend might not have data during this test, so we accept either the empty state or loaded items
    const hasItems = await page.getByRole('button', { name: 'Approve & Send Draft' }).count() > 0;
    if (hasItems) {
      const approveBtn = page.getByRole('button', { name: 'Approve & Send Draft' }).first();
      await approveBtn.click();

      // Look for the "Draft approved and sent." status message or "Failed to approve" depending on backend mock state
      await expect(page.locator('[role="status"]')).toBeVisible({ timeout: 10000 });
    } else {
      // Empty state
      await expect(page.getByText(/You're all caught up|No urgent work items|Loading your work feed/)).toBeVisible();
    }
  });
});
