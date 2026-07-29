import { expect } from '@playwright/test';
import { test } from '../fixtures';

test.describe('Loyalty & Rewards Engine', () => {
  test('Dashboard should have a link to the loyalty widget', async ({ page }) => {
    await page.goto('/dashboard.html');
    const loyaltyLink = page.locator('a#loyalty-link');
    await expect(loyaltyLink).toBeVisible();
    await expect(loyaltyLink).toContainText('Viral Loyalty Engine');
  });
});
