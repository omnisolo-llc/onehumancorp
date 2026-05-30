import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  test('should display daily pulse and action cards', async ({ page }) => {
    await page.goto('http://localhost:3000/dashboard');

    await expect(page.locator('text="Today\'s Pulse"')).toBeVisible();
    await expect(page.locator('text="Action Required"')).toBeVisible();

    // Check for Operations card
    await expect(page.locator('text="Operations"')).toBeVisible();
    await expect(page.locator('text="2 Custom Cake Orders to Review"')).toBeVisible();

    // Check for Marketing card
    await expect(page.locator('text="Marketing"')).toBeVisible();
    await expect(page.locator('text="Approve Instagram post"')).toBeVisible();

    // Check for Advisor card
    await expect(page.locator('text="Advisor"')).toBeVisible();
    await expect(page.locator('text="Weekly Insights Available"')).toBeVisible();
  });
});
