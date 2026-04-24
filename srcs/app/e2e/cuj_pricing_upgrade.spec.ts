import { test, expect } from '@playwright/test';

test.describe('CUJ: Billing Pricing Upgrade', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.fill('[data-testid="login-email"]', 'ceo@onehumancorp.com');
    await page.fill('[data-testid="login-password"]', 'password123');
    await page.click('[data-testid="login-button"]');
    await expect(page.locator('text=Dashboard')).toBeVisible();
  });

  test('admin views cost dashboard, sees plan, and navigates to pricing', async ({ page }) => {
    await page.click('text=Cost & Usage');
    await expect(page.locator('text=My Plan')).toBeVisible();
    await expect(page.locator('text=Free Plan')).toBeVisible();
    await page.click('text=Upgrade Plan');
    await expect(page.locator('text=Choose Your Plan')).toBeVisible();
    await expect(page.locator('text=Starter')).toBeVisible();
    await expect(page.locator('text=Pro')).toBeVisible();
    await expect(page.locator('text=Business')).toBeVisible();
    const upgradeButton = page.locator('text=Upgrade to Starter').first();
    await expect(upgradeButton).toBeVisible();
  });
});
