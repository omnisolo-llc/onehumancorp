import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test('should display referral dashboard and generate link', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Referrals dashboard
    await page.goto('/referrals');
    await expect(page.locator('text=Viral Loop Dashboard')).toBeVisible();

    // 3. Generate a new referral link
    const newLinkButton = page.locator('button:has-text("New Link")');
    await newLinkButton.click();

    // 4. Assert link is generated and visible
    await expect(page.locator('text=/ohc:\\/\\/join\\?ref=.*&utm_source=standalone_desktop&utm_medium=team_share&inviter=.*/')).toBeVisible();

    // 5. Verify refresh button works
    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();

    // Final check for premium UI element
    await expect(page.locator('text=Your Referral Link')).toBeVisible();
  });
});
