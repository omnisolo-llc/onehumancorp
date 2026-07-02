import { expect, test } from '../fixtures';

test.describe('Booking Re-engagement Automation', () => {

  test('Persona: Leo the Music Tutor approves automated re-engagement message', async ({ page }) => {
    // 1. Visit the dashboard (the fixture logs in and uses e2e-tenant which has the seeded job)
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. Check the Agent Feed for the Re-engagement Agent task
    // The backend worker sets the message to "Approve Re-engagement for Ben Buyer"
    const feedCard = page.locator('text=Approve Re-engagement for Ben Buyer').locator('..');

    // Sometimes it takes a few seconds for the backend worker to process the job and the UI to refetch.
    await expect(feedCard).toBeVisible({ timeout: 20000 });

    // Verify the drafted message is shown
    await expect(feedCard.locator('text=jump back in this week').first()).toBeVisible();

    // Ensure approve button is visible
    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();

    // 3. Mark the task complete/approved
    await approveBtn.click();

    // 4. Verify the task disappears from the feed
    await expect(feedCard).not.toBeVisible({ timeout: 15000 });
  });
});
