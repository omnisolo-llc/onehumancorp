import { test, expect } from '@playwright/test';

test.describe('Financials Dashboard Card', () => {
  test('should display Total Balance and Recent Activity correctly', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for the ledger entries to be fetched and rendered
    const financialsCard = page.locator('.mac-glass-container').filter({ hasText: 'Total Balance' });
    await expect(financialsCard).toBeVisible({ timeout: 10000 });

    // Check if the Total Balance section is present
    await expect(financialsCard.getByText('Total Balance')).toBeVisible();
    await expect(financialsCard.locator('.text-4xl')).toBeVisible(); // The price element

    // Check if Recent Activity section is present
    await expect(financialsCard.getByText('Recent Activity')).toBeVisible();
  });
});
