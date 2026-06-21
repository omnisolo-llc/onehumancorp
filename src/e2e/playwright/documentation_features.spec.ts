import { test, expect } from '@playwright/test';

test.describe('Documentation Features CUJ', () => {
  test('User can access help center, use chat, run walkthroughs, view changelog, and API docs', async ({ page }) => {
    // 1. Visit Help Center
    await page.goto('/help');
    await expect(page.locator('h1')).toContainText('Help Center');

    // 2. Open AI Help Chat
    const helpChatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(helpChatButton).toBeVisible();
    await helpChatButton.click();
    await expect(page.locator('text="Ask AI Help"').first()).toBeVisible();

    // 3. View Changelog from Dashboard
    await page.goto('/dashboard');
    const changelogLink = page.locator('a', { hasText: 'Changelog' }).first();
    await expect(changelogLink).toBeVisible();
    await changelogLink.click();
    await expect(page).toHaveURL(/.*\/changelog/);
    await expect(page.locator('h1')).toHaveText('Release Notes & Changelog');

    // 4. Trigger Walkthrough (Dashboard has a walkthrough button)
    await page.goto('/dashboard');
    const walkthroughBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkthroughBtn).toBeVisible();
    await walkthroughBtn.click();
    await expect(page.locator('.ohc-walkthrough-bubble')).toBeVisible();
    await page.locator('.ohc-walkthrough-close').click();
    await expect(page.locator('.ohc-walkthrough-bubble')).not.toBeVisible();

    // 5. View API Docs
    await page.goto('/api-docs');
    await expect(page.locator('text="Advanced:"').first()).toBeVisible();
    // Wait for swagger to load
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });
  });
});
