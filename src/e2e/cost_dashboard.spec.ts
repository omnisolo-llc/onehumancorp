import { test, expect } from './fixtures';

test.describe('Cost Dashboard Page', () => {
  test('displays cost breakdown and revenue prominently', async ({ page }) => {
    // Navigate directly to the cost dashboard
    await page.goto('/cost-dashboard');

    // Verify main header
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();

    // Verify Advisory Summary
    await expect(page.getByRole('heading', { name: 'Advisory Summary' })).toBeVisible();

    // Verify Overview section including the new Total Revenue field
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByText('Total Revenue')).toBeVisible();
    await expect(page.getByText('Total Costs')).toBeVisible();

    // Verify Breakdown section
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });
});
