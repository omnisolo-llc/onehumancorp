import { test, expect } from '@playwright/test';

test('simple check', async ({ page }) => {
  await page.goto('http://localhost:3000/help');
  await expect(page).toHaveTitle(/OneHumanCorp/i);
});
