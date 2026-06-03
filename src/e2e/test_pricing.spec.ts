import { test, expect } from '@playwright/test';

test('pricing page', async ({ page }) => {
  await page.goto('http://127.0.0.1:18789/pricing');
  await expect(page.locator('#pricing-screen')).toBeVisible();

  // Wait a bit to ensure it loads
  await page.waitForTimeout(2000);
});
