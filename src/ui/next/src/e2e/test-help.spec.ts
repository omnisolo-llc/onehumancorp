import { test, expect } from '@playwright/test';

test('Verify help center', async ({ page }) => {
  await page.goto('http://localhost:3000/help');
  await page.waitForTimeout(1000);

  await page.goto('http://localhost:3000/help/getting-started');
  await page.waitForTimeout(1000);
});
