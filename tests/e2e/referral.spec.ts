import { test, expect } from '@playwright/test';

test.describe('Referral Program Full-Stack E2E', () => {
  test('should verify UI -> DB -> UI referral link generation', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('referral_test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In")').click();
    await page.waitForURL('**/*');

    // 2. Navigate to Referrals dashboard
    await page.goto('/referrals');
    await expect(page.locator('text=Viral Loop Dashboard')).toBeVisible();

    // 3. Generate a new referral link (This triggers the real DB call in the backend)
    const newLinkButton = page.locator('button:has-text("New Link")');
    await newLinkButton.click();

    // 4. Assert link is generated and visible (Fetched from DB)
    await expect(page.locator('text=ohc://join?ref=')).toBeVisible();

    // 5. Verify refresh button works and maintains the created data
    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();

    // Final check for premium UI element and persistence
    await expect(page.locator('text=Your Referral Link')).toBeVisible();
    await expect(page.locator('text=ohc://join?ref=')).toBeVisible();
  });
});
