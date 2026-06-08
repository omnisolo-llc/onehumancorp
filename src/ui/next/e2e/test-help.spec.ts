import { test, expect } from '@playwright/test';

test('Verify help center and navigation', async ({ page }) => {
  // Check Help Center
  await page.goto('/help');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'help-center.png' });
  await expect(page.locator('h1')).toContainText('Help Center');

  // Check specific article
  await page.goto('/help/getting-started-1');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'help-article.png' });
  await expect(page.locator('h1')).toContainText('Getting Started with Your Store');

  // Check API Docs
  await page.goto('/api-docs');
  await page.waitForTimeout(1000);
  await expect(page.locator('text=Advanced:')).toBeVisible();

  // Check Changelog
  await page.goto('/changelog');
  await page.waitForTimeout(1000);
  await expect(page.locator('h1')).toContainText('Release Notes & Changelog');
});
