import { test, expect } from '@playwright/test';

test.describe('Today Feed CUJ', () => {
  test('User logs in and sees Today feed with triage cards', async ({ page }) => {
    // Navigate to home which redirects to onboarding if not logged in
    await page.goto('/');

    // Set local storage to simulate onboarded user
    await page.evaluate(() => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('business_display_name', 'default');
    });

    // Reload page to trigger redirect to /today
    await page.reload();

    // Verify redirect to /today
    await expect(page).toHaveURL(/.*\/today/);

    // Ensure the AppShell and Today Feed title renders
    await expect(page.locator('h1').filter({ hasText: 'Today' }).first()).toBeVisible();
    await expect(page.getByText('Your actionable daily feed')).toBeVisible();

    // Check if empty state is visible or wait for items
    const emptyState = page.getByTestId('today-feed-empty');
    const firstCard = page.locator('[data-testid^="today-card-"]').first();

    await Promise.race([
      emptyState.waitFor({ state: 'visible' }),
      firstCard.waitFor({ state: 'visible' })
    ]);

    const hasItems = await firstCard.isVisible();

    if (hasItems) {
      // Test interactions on the first card
      await firstCard.locator('[data-testid^="today-card-header-"]').click();

      // Check if Review Draft or Approve & Send is visible
      const hasReview = await firstCard.locator('[data-testid^="today-review-btn-"]').isVisible();
      const hasApprove = await firstCard.locator('[data-testid^="today-approve-"]').isVisible();

      expect(hasReview || hasApprove).toBeTruthy();

      if (hasReview) {
        // Test Review Draft interaction
        await firstCard.locator('[data-testid^="today-review-btn-"]').click();
        await expect(firstCard.locator('textarea')).toBeVisible();
        await firstCard.locator('[data-testid^="today-cancel-btn-"]').click();
      }

      // We won't actually submit a real dismissal to avoid ruining test DB state,
      // just verify buttons are present and clickable.
      await expect(firstCard.locator('[data-testid^="today-dismiss-"]')).toBeVisible();
    } else {
      await expect(page.getByTestId('today-feed-empty')).toBeVisible();
      await expect(page.getByText('All caught up for today!')).toBeVisible();
    }
  });
});
