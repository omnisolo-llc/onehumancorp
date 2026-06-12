import { test, expect } from '@playwright/test';

test.describe('Owner Referral Tier Dashboard', () => {
  test('should display owner referral tiers and generate referral link correctly', async ({ page, context }) => {
    // Navigate to dashboard first to simulate login and setting auth
    await page.goto('/dashboard');

    await page.evaluate(() => {
        localStorage.setItem('has_onboarded', 'true');
        localStorage.setItem('token', 'fake-token-for-test');
        localStorage.setItem('tenant', 'test-tenant');
    });

    // Go to the referral dashboard page (this relies on the real backend running via compose)
    await page.goto('/referrals');

    // Wait for the UI to load
    await expect(page.locator('h1')).toHaveText('Referrals & Rewards');

    // Check Tier Status
    await expect(page.locator('text=Your Tier')).toBeVisible();

    // Check Stats
    await expect(page.locator('text=Performance')).toBeVisible();

    // Check Copy button
    const copyButton = page.locator('button', { hasText: 'Copy Link' });
    await expect(copyButton).toBeVisible();

    // Copy the link to clipboard to ensure interaction
    await copyButton.click();
    await expect(page.locator('button', { hasText: 'Copied!' })).toBeVisible();
  });
});
