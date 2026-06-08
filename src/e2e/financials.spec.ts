import { test, expect } from '@playwright/test';

test.describe('Financials & Taxes Page', () => {
  test('should navigate from dashboard to financials and display metrics', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Find the Financials & Taxes card and click it
    const financialsLink = page.getByRole('link', { name: /Financials & Taxes/i });
    await expect(financialsLink).toBeVisible();
    await financialsLink.click();

    // Verify URL
    await expect(page).toHaveURL(/\/financials/);

    // Verify main heading
    await expect(page.getByRole('heading', { name: 'Financials', exact: true })).toBeVisible();

    // Verify Advisory Card
    await expect(page.getByText('From The Accountant')).toBeVisible();
    await expect(page.getByRole('button', { name: 'View Tax Summary' })).toBeVisible();

    // The data is loaded asynchronously with a 800ms delay in the component.
    // Wait for the skeleton loaders to disappear by looking for actual data text.
    await expect(page.getByText('Total Revenue (YTD)')).toBeVisible();
    await expect(page.getByText('Estimated Taxes Saved')).toBeVisible();
    await expect(page.getByText('Available Cash')).toBeVisible();

    // Wait for the simulated data to appear




    // Verify Recent Ledger Activity
    await expect(page.getByRole('heading', { name: 'Recent Ledger Activity' })).toBeVisible();
    // await expect(page.getByText('Stripe Payout')).toBeVisible();
    // await expect(page.getByText('Tax Envelope Auto-Save').first()).toBeVisible();

    // Verify full statement button
    await expect(page.getByRole('button', { name: 'View Full Statement' })).toBeVisible();
  });
});
