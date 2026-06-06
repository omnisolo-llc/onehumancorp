import { test, expect } from './fixtures';

test.describe('Business Analytics Widget Soft Paywall', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
  });

  test('should display the analytics widget with basic metrics', async ({ page }) => {
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    await expect(dashboard.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(dashboard.getByText('Total Sales')).toBeVisible();
    await expect(dashboard.getByText('Low Stock')).toBeVisible();
  });

  test('should display locked advanced AI insights with upgrade CTA', async ({ page }) => {
    const dashboard = page.locator('#dashboard-screen');
    await expect(dashboard).toBeVisible();

    await expect(dashboard.getByText('Advanced AI Insights')).toBeVisible();
    await expect(dashboard.getByText('Unlock predictive analytics')).toBeVisible();

    const upgradeBtn = dashboard.getByRole('button', { name: 'Upgrade to Pro' });
    await expect(upgradeBtn).toBeVisible();

    // Set up dialog handler
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Upgrade to Pro to access Advanced AI Insights?');
      await dialog.accept();
    });

    await upgradeBtn.click();

    // Verify it navigates to pricing-screen
    const pricingScreen = page.locator('#pricing-screen');
    await expect(pricingScreen).toBeVisible();
  });
});

test.describe('Analytics Empty States (Grandmother Test)', () => {
  test('business-analytics should show empty state for Revenue Forecast', async ({ page }) => {
    await page.goto('/business-analytics');
    await expect(page.getByRole('heading', { name: 'Revenue Forecast' })).toBeVisible();
    await expect(page.getByText('Not enough historical data to generate an AI forecast.')).toBeVisible();
  });

  test('business-analytics should show empty state for Customer Cohort Retention', async ({ page }) => {
    await page.goto('/business-analytics');
    await expect(page.getByRole('heading', { name: 'Customer Cohort Retention' })).toBeVisible();
    await expect(page.getByText('Customer retention data requires at least 3 months of history.')).toBeVisible();
  });

  test('analytics should show empty state for Traffic Sources', async ({ page }) => {
    await page.goto('/analytics');
    await expect(page.getByRole('heading', { name: 'Traffic Sources' })).toBeVisible();
    await expect(page.getByText('Waiting for real traffic data from visitors.')).toBeVisible();
  });

  test('analytics should show empty state for AI Buying Intent', async ({ page }) => {
    await page.goto('/analytics');
    await expect(page.getByRole('heading', { name: 'AI Buying Intent' })).toBeVisible();
    await expect(page.getByText('AI needs more customer behavior data to calculate intent.')).toBeVisible();
  });

  test('analytics pages should not contain mock data indicators', async ({ page }) => {
    await page.goto('/business-analytics');
    const pageContent = await page.content();
    expect(pageContent).not.toContain('Mock area chart');

    await page.goto('/analytics');
    const analyticsContent = await page.content();
    expect(analyticsContent).not.toContain('Mock Chart representation');
  });
});
