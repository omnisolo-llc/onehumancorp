import { test, expect } from '@playwright/test';

test('basic test', async ({ page }) => {
  await page.goto(process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:8080');
});
