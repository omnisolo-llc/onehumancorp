import { test, expect } from '@playwright/test';

test.describe('Finance Dashboard Loop', () => {
  test('Finance dashboard loads and displays cashflow forecast', async ({ page }) => {
    await page.goto('http://localhost:3000/finance-dashboard');

    await expect(page.locator('h1', { hasText: 'Financial Health Dashboard' })).toBeVisible({ timeout: 10000 });

    await expect(page.locator('h2', { hasText: 'Cashflow Forecast' })).toBeVisible();

    await expect(page.locator('span', { hasText: 'Projected Inflow' })).toBeVisible();

    await expect(page.locator('button', { hasText: 'Auto-send invoice reminders' })).toBeVisible();

    await page.locator('button', { hasText: 'Back to Dashboard' }).click();
    await expect(page).toHaveURL('http://localhost:3000/dashboard');
  });
});
