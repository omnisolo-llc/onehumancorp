import { test, expect } from '@playwright/test';

test('Dashboard Premium UI verified', async ({ page }) => {
  // Use login bypass hook
  await page.goto('/dashboard');

  // Wait for the UI to be fully rendered
  await page.waitForTimeout(1000);

  // Assert that mac-glass-container exists in the DOM
  const glassContainers = await page.locator('.mac-glass-container').count();
  expect(glassContainers).toBeGreaterThan(0);
});
