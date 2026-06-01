import { test, expect } from '@playwright/test';

test('Cost Dashboard Premium UI verified', async ({ page }) => {
  // Mock auth if necessary or just load the page directly
  await page.goto('http://localhost:18789/cost-dashboard');

  // Wait for the UI to be fully rendered
  await page.waitForTimeout(1000);

  // Assert that mac-glass-container exists in the DOM
  const glassContainers = await page.locator('.mac-glass-container').count();
  expect(glassContainers).toBeGreaterThan(0);
});
