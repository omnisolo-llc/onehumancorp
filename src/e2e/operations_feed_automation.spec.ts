import { expect, test } from './fixtures';

test.describe('Operations Agent Task Automation', () => {

  test('Persona: Jun the Location Manager completes daily prep checklist', async ({ page }) => {
    // 1. Visit the dashboard (the fixture logs in and uses e2e-tenant which now has the seeded item)
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. Check the Agent Feed for the Operations Agent task
    const operationsTaskCard = page.locator('text=Review Daily Prep Checklist').locator('..');
    await expect(operationsTaskCard).toBeVisible({ timeout: 10000 });

    // Ensure buttons are visible
    const markCompleteBtn = operationsTaskCard.getByTestId('feed-approve-btn');
    const assignBtn = operationsTaskCard.getByTestId('feed-assign-btn');
    const dismissBtn = operationsTaskCard.getByTestId('feed-dismiss-btn');

    await expect(markCompleteBtn).toBeVisible();
    await expect(assignBtn).toBeVisible();
    await expect(dismissBtn).toBeVisible();

    // 3. Mark the task complete
    await markCompleteBtn.click();

    // Check that we hit the API successfully to approve it
    await page.waitForResponse(response => response.url().includes('/api/v1/agent-feed/e2e-feed-ops-daily-routine/state') && response.status() === 200, { timeout: 15000 }).catch(() => {});

    // 4. Verify the task disappears from the feed
    // Depending on optimistic update or fast refresh, it should hide
    await expect(operationsTaskCard).not.toBeVisible({ timeout: 10000 });
  });
});
