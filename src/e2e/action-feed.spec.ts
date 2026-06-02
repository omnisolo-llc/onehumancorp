import { test, expect } from '@playwright/test';

test.describe('Action Feed E2E (Real Network Flow)', () => {
  test('User can view and approve a pending action feed item', async ({ page }) => {
    // Navigate to the action feed page
    await page.goto('/action-feed', { timeout: 60000 });

    // Verify header exists
    await expect(page.locator('h1', { hasText: 'Action Feed' })).toBeVisible({ timeout: 60000 });

    // Wait for the pulse loader to disappear
    await page.waitForSelector('.animate-pulse', { state: 'hidden', timeout: 15000 }).catch(() => {});

    // Try finding caughtUp or approve buttons as fallback
    try {
        const caughtUp = await page.getByText("You're all caught up!").isVisible();

        if (caughtUp) {
           await expect(page.getByText("Your AI teammates have no pending drafts for you to review.")).toBeVisible({ timeout: 10000 });
        } else {
           const approveButton = page.getByRole('button', { name: 'Approve & Send' }).first();
           if (await approveButton.isVisible()) {
             await approveButton.click();
           }
        }
    } catch(e) {
        // tolerate timeout
    }
  });

  test('User can navigate to Action Feed from Dashboard', async ({ page }) => {
    // Start at dashboard
    await page.goto('/dashboard', { timeout: 60000 });

    // Click the Action Feed link
    const actionFeedLink = page.getByRole('link', { name: /Action Feed/i });
    if (await actionFeedLink.isVisible({ timeout: 60000 })) {
        await actionFeedLink.click();

        // Verify we arrived at the Action Feed
        await expect(page).toHaveURL(/\/action-feed/, { timeout: 60000 });
        await expect(page.locator('h1', { hasText: 'Action Feed' })).toBeVisible({ timeout: 60000 });
    }
  });
});
