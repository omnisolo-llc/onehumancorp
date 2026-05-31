import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should navigate to Cost Dashboard from My Plan page', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await page.getByRole('button', { name: /View Cost Details/i }).click();
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
  });

  test('should display Total Costs', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Total Costs' })).toBeVisible();
    await expect(page.getByText(/^\$/)).first().toBeVisible(); // total cost text
  });

  test('should display LLM Usage breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
  });

  test('should display Storage Cost breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
  });

  test('should display Payment Fees breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });
});
