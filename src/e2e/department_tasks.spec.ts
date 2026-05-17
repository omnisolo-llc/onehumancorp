import { test, expect } from '@playwright/test';

test('dashboard order milestone is visible after marking an order ready', async ({ page }) => {
  await page.goto('/dashboard');
  await page.getByRole('button', { name: 'Mark Order Ready' }).click();

  await expect(page.locator('#milestone-card')).toBeVisible();
  await expect(page.locator('#milestone-card')).toContainText('First Sale!');
});
