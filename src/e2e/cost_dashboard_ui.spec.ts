import { test, expect } from '@playwright/test';

test('Cost Dashboard Premium UI verified', async ({ page }) => {
  await page.goto('/cost-dashboard');

  await page.waitForTimeout(1000);

  const glassContainers = await page.locator('.mac-glass-container').count();
  expect(glassContainers).toBeGreaterThan(0);
});
