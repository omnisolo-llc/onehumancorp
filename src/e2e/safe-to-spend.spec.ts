import { test, expect } from '@playwright/test';
import { memberPage, e2eTenantId } from './fixtures';

test.describe('CFO Agent Safe To Spend Dashboard', () => {
  test('Dashboard displays Safe to Spend card and correct breakdown', async ({ memberPage: page }) => {
    // Navigate to the Dashboard
    await page.goto('/');

    // Wait for the "Safe to Spend" card to be visible
    const safeToSpendCard = page.locator('text=Safe to Spend');
    await expect(safeToSpendCard).toBeVisible({ timeout: 15000 });

    // Ensure there is some numerical value associated with it
    const balanceAmount = page.locator('text=/\\$\\d+/').first();
    await expect(balanceAmount).toBeVisible();

    // Verify CFO breakdown modal elements
    await expect(page.locator('text=Reserved for Taxes')).toBeVisible();
    await expect(page.locator('text=Upcoming Bills')).toBeVisible();
  });
});
