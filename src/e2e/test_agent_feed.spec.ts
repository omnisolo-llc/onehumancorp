import { expect, test } from './fixtures';

test.describe('Unified Agent Feed Actions', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow approving an action card in the feed', async ({ page }) => {
    test.setTimeout(180000);

    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('#triage-queue').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // Look for approve buttons in the feed
    const approveButtons = feedContainer.locator('button:has-text("Approve")');
    // We expect there to be at least one card generated for triage
    await expect(approveButtons.first()).toBeVisible({ timeout: 15000 });

    // Store count to verify the count decreases
    const initialCount = await approveButtons.count();

    await approveButtons.first().click();

    // Expect the card to disappear or change state, count should be less
    await expect(async () => {
       const newCount = await page.locator('#triage-queue').locator('button:has-text("Approve")').count();
       expect(newCount).toBeLessThan(initialCount);
    }).toPass({ timeout: 10000 });
  });

  test('should allow dismissing an action card in the feed', async ({ page }) => {
    test.setTimeout(180000);

    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('#triage-queue').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // Look for dismiss/reject buttons in the feed
    const rejectButtons = feedContainer.locator('button:has-text("Dismiss"), button:has-text("Reject"), button:has-text("Deny")');
    // We expect there to be at least one card generated for triage
    await expect(rejectButtons.first()).toBeVisible({ timeout: 15000 });

    // Store count to verify the count decreases
    const initialCount = await rejectButtons.count();

    await rejectButtons.first().click();

    // Expect the card to disappear or change state, count should be less
    await expect(async () => {
       const newCount = await page.locator('#triage-queue').locator('button:has-text("Dismiss"), button:has-text("Reject"), button:has-text("Deny")').count();
       expect(newCount).toBeLessThan(initialCount);
    }).toPass({ timeout: 10000 });
  });
});
