import { test, expect } from '../../../../e2e/fixtures';

test.describe('Business Analytics Flow', () => {
  test('Dashboard contains link to Business Analytics', async ({ page }) => {
    await page.goto('/dashboard');
    const analyticsLink = page.locator('a', { hasText: 'Business Analytics' }).first();
    await expect(analyticsLink).toBeVisible();
    await expect(analyticsLink).toHaveAttribute('href', '/business-analytics');

    await analyticsLink.click();
    await page.waitForURL('**/business-analytics');
    await expect(page.locator('h1', { hasText: 'Business Analytics' }).first()).toBeVisible();
  });

  test('Displays business analytics and predictive growth section', async ({ page }) => {
    await page.goto('/business-analytics');
    await expect(page.getByRole('heading', { name: 'Business Analytics' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Predictive AI Growth Trends' })).toBeVisible();
    await expect(page.getByText(/Forecasts and cohort analytics are unavailable/i)).toBeVisible();
  });

  test('Back to dashboard link works', async ({ page }) => {
    await page.goto('/business-analytics');
    const backLink = page.getByRole('link', { name: 'Back to Dashboard' }).or(page.getByRole('button', { name: 'Back to Dashboard' })).first();
    await backLink.click();
    await page.waitForURL('**/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible();
  });
});
