import { test, expect } from './fixtures';

test.describe('CUJ: Billing Cost Tracking', () => {
  test('should display cost breakdown on dashboard locally', async ({ page }) => {
    // User Input: Admin navigates to the "Costs" section.
    // In our UI, this is the cost-dashboard page.
    await page.goto('/cost-dashboard');

    // System Action: GET /api/costs is called (via the page load).
    // Outcome: A breakdown of tokens and USD cost is displayed.
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();

    // Admin inspects the per-model cost rows (in our UI these are broken down into LLM Usage, Storage, etc.)
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage', { exact: true })).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();
  });
});
