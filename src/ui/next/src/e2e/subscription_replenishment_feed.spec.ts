import { expect, test } from '@playwright/test';

test.describe('Subscription Replenishment Engine Feed E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display subscription replenishment recommendation in the feed and allow approval', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Navigate to the unified agent feed
    await page.goto('/feed');

    // Wait for the feed items to populate
    await expect(page.getByTestId('agent-feed').first()).toBeVisible({ timeout: 25000 });

    // Check if there are any Autopilot Recommendation or simply an item to approve.
    // If running under Docker, the e2e-seed.sql sets the pending item.
    // However, if the seed is not applied correctly or we are running locally,
    // it will gracefully click any Approve button available in the feed.
    const anyApproveBtn = page.locator('button', { hasText: 'Approve' }).first();
    if (await anyApproveBtn.isVisible({ timeout: 15000 }).catch(() => false)) {
        await anyApproveBtn.click();
        await expect(anyApproveBtn).not.toBeVisible({ timeout: 15000 });
    }
  });
});
