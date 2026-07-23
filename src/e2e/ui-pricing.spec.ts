import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Pricing UI', () => {
  test('should display pricing page successfully', async ({ page }) => {
    await adminPage(page);
    await page.goto('/pricing');
    await expect(page.locator('h1')).toContainText('Pricing Plans');
  });
});
