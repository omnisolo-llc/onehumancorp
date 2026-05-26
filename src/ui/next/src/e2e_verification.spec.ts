import { test, expect } from '@playwright/test';

test('verify Social Media Share Reward loop', async ({ page }) => {
  await page.goto('http://localhost:3000/dashboard');
  const bannerHeading = page.locator('h3', { hasText: 'Boost Your AI Power' });
  await expect(bannerHeading).toBeVisible({ timeout: 10000 });

  await page.screenshot({ path: '/home/jules/verification/verification.png' });
});
