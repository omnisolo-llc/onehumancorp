import { test, expect } from '@playwright/test';

test.describe('Finance Capital Engine CUJ', () => {

  test('Persona: Business Owner views and accepts cash flow advance', async ({ page }) => {
    // 1. Owner opens the Finance page
    await page.goto('/finance');

    // We expect the transparent glass card
    await expect(page.getByRole('heading', { name: /Cash Flow Alert/i })).toBeVisible();
    await expect(page.getByText('Looks like your ingredient costs are due')).toBeVisible();

    // 2. Setup interception for the API route
    await page.route('/api/v1/finance/offers', async route => {
      const json = [{
        offer_id: 'test-offer-123',
        amount: 1000,
        fee_percentage: 10.0,
        repayment_rate: 10.0,
        status: 'PENDING'
      }];
      await route.fulfill({ json });
    });

    await page.route('/api/v1/finance/offers/*/accept', async route => {
      const json = { status: 'success', message: 'Offer accepted', credited_amount: 1000.0 };
      await route.fulfill({ json });
    });

    // 3. The button should have the amount dynamically loaded or default
    const button = page.locator('#accept-btn');
    await expect(button).toBeVisible();

    // 4. Click accept
    await button.click();

    // 5. Verify result
    await expect(page.getByText('Funds added to ledger! ✅')).toBeVisible();
  });
});
