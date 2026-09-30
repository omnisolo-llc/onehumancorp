import { expect, test } from './fixtures';
import { seedFeedItem } from './feed-fixtures';

test.describe('Operations Agent Task Automation', () => {

  test('Persona: Jun the Location Manager completes daily prep checklist', async ({ page }) => {
    // 1. Visit the dashboard (the fixture logs in and uses e2e-tenant which now has the seeded item)
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const id = await seedFeedItem(page, {
      event_source: 'operations',
      context_payload: { description: 'Review Daily Prep Checklist', feature_type: 'daily_prep_checklist' },
      proposed_action: { action_type: 'Daily Prep Checklist', feature_type: 'daily_prep_checklist' },
    });
    await page.reload();

    const group = page.getByTestId('grouped-triage-card-daily_prep_checklist-Daily Prep Checklist');
    if (await group.isVisible()) await group.getByRole('button', { name: 'Review Individually' }).click();

    // 2. Check the Agent Feed for the Operations Agent task
    const operationsTaskCard = page.getByTestId(`triage-card-${id}`);
    await expect(operationsTaskCard).toBeVisible({ timeout: 10000 });

    // Ensure buttons are visible
    const markCompleteBtn = operationsTaskCard.getByRole('button', { name: 'Mark Complete', exact: true });
    const assignBtn = operationsTaskCard.getByTestId('feed-assign-btn');
    const dismissBtn = operationsTaskCard.getByTestId('feed-dismiss-btn');

    await expect(markCompleteBtn).toBeVisible();
    await expect(assignBtn).toBeVisible();
    await expect(dismissBtn).toBeVisible();

    // 3. Mark the task complete
    const decision = page.waitForResponse(response => response.url().endsWith(`/api/v1/agent-feed/${id}`) && response.request().method() === 'PUT');
    await markCompleteBtn.click();

    // Check that we hit the API successfully to approve it
    expect((await decision).status()).toBe(200);

    // 4. Verify the task disappears from the feed
    // Depending on optimistic update or fast refresh, it should hide
    await expect(operationsTaskCard).not.toBeVisible({ timeout: 10000 });
  });
});
