import { test, expect } from '@playwright/test';

test.describe('Autonomous Capital Advance & Cashflow Engine', () => {
  // We mock the backend responses for stability in E2E since
  // generating genuine ledger history triggers is async.
  test.beforeEach(async ({ page }) => {
    // Mock the offers endpoint
    await page.route('**/api/capital/offers', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([{
          id: 'adv_test123',
          amount_cents: 100000,
          fee_cents: 10000,
          total_repayment_cents: 110000,
          repayment_percentage: 0.08,
          status: 'offered'
        }])
      });
    });

    // Mock the accept endpoint
    await page.route('**/api/capital/accept', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(true)
      });
    });
  });

  test('merchant can view, adjust, and accept capital advance offer', async ({ page }) => {
    // Navigate to the Capital dashboard
    await page.goto('/capital');

    // 1. Verify the offer is presented
    await expect(page.locator('h2').filter({ hasText: 'Capital Advance' })).toBeVisible();
    await expect(page.getByText("You're approved for a cash advance to grow your business.")).toBeVisible();

    // Verify initial values based on mock (amount_cents: 100000 = $1000)
    await expect(page.locator('span.font-bold.text-\\[\\#0066FF\\]')).toHaveText('$1000');

    // 2. Adjust the amount slider
    const slider = page.locator('input[type="range"]');
    // Change to max amount
    await slider.fill('1500');
    await expect(page.locator('span.font-bold.text-\\[\\#0066FF\\]')).toHaveText('$1500');

    // 3. Accept the offer
    await page.getByRole('button', { name: 'Get Funds Instantly' }).click();

    // 4. Verify Success State
    await expect(page.locator('h2').filter({ hasText: '🎉 Success!' })).toBeVisible();
    await expect(page.getByText('Funds have been instantly added to your account.')).toBeVisible();
  });
});
