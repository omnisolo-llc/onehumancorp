import { test, expect } from './fixtures';

test.describe('Business Analytics Page', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to business analytics page
    await page.goto('/business-analytics');
    await page.waitForLoadState('networkidle');
  });

  test('should display glassmorphism UI with database metrics', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Business Analytics 📊' })).toBeVisible();
    await expect(page.getByText('Total Revenue')).toBeVisible();
    await expect(page.getByText('Customers')).toBeVisible();
    await expect(page.getByText('Pending Orders')).toBeVisible();

    // Check if live data label is present indicating mock data is removed
    await expect(page.getByText('Live Data')).toHaveCount(3);

    // Wait for fetch to return values (since our E2E seeds data, total sales should be a number)
    // We expect a valid dollar amount format
    await expect(page.getByText(/\$\d+\.\d{2}/)).toBeVisible();

    // Check glassmorphism classes
    const metricsCards = page.locator('.glassmorphism');
    await expect(metricsCards.first()).toBeVisible();
    const classAttr = await metricsCards.first().getAttribute('class');
    expect(classAttr).toContain('glassmorphism');
    expect(classAttr).toContain('rounded-[16px]');
  });
});
