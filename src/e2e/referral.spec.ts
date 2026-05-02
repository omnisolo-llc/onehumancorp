import { test, expect } from '@playwright/test';

test.describe('Referral Program', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    const btn = page.locator('button:has-text("/login")');
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")');
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        await page.locator('button:has-text("/login")').click();
      }
    }
  });
  test('should display referral dashboard and generate link', async ({ page }) => {
    // 1. Start from home page after login
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Referrals dashboard
    await expect(page.locator('text=Viral Loop Dashboard')).toBeVisible();

    // 3. Generate a new referral link
    const newLinkButton = page.locator('button:has-text("New Link")');
    await newLinkButton.click();

    // 4. Assert link is generated and visible
    await expect(page.locator('text=ohc://join?ref=')).toBeVisible();

    // 5. Verify refresh button works
    const refreshButton = page.locator('button:has-text("Refresh")');
    await refreshButton.click();

    // Final check for premium UI element
    await expect(page.locator('text=Your Referral Link')).toBeVisible();
  });
});
