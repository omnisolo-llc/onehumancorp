import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should navigate to Cost Dashboard from My Plan page', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' }).first()).toBeVisible();
    await page.getByRole('button', { name: /View Cost Details/i }).click();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' }).first()).toBeVisible();
  });

  test('should display Total Costs', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost Transparency' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Total Costs' }).first()).toBeVisible();
  });

  test('should display LLM Usage breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' }).first()).toBeVisible();
    await expect(page.getByText('LLM Usage').first()).toBeVisible();
  });

  test('should display Storage Cost breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' }).first()).toBeVisible();
    await expect(page.getByText('Storage', { exact: true }).first()).toBeVisible();
  });

  test('should display Payment Fees breakdown', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' }).first()).toBeVisible();
    await expect(page.getByText('Payment Fees').first()).toBeVisible();
  });
});
