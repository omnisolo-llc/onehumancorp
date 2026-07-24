import { test, expect } from '@playwright/test';

test.describe('Loyalty & Rewards Engine', () => {

  test('Should create and retrieve loyalty wallet balance', async ({ page }) => {
    // Assuming our test harness sets up a tenant and customer.
    // In this mocked check, we navigate to the quote and check if the wallet loads.
    console.log('**/api/ui/loyalty/balance*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ points_balance: 500, wallet_id: "test-wallet" })
      });
    });

    console.log('**/api/ui/quote*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
            id: 'quote-123',
            business_name: 'Maya Cakes',
            title: 'Custom Vegan Cake',
            status: 'PENDING',
            total_amount: 150.00,
            required_deposit: 50.00,
            line_items: [{description: 'Cake', quantity: 1, unit_price: 150.00, total_price: 150.00}]
        })
      });
    });

    await page.goto('/quote.html?id=quote-123');

    // Wait for the loyalty points toggle to become visible
    const container = page.locator('#loyalty-points-container');
    await expect(container).toBeVisible();

    const balanceText = page.locator('#loyalty-balance-text');
    await expect(balanceText).toContainText('You have 500 pts');
  });

  test('Should apply points to checkout', async ({ page }) => {
    console.log('**/api/ui/loyalty/balance*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ points_balance: 1000, wallet_id: "test-wallet" }) // 1000 pts = $10.00
      });
    });

    console.log('**/api/ui/quote*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
            id: 'quote-123',
            business_name: 'Maya Cakes',
            title: 'Custom Vegan Cake',
            status: 'PENDING',
            total_amount: 150.00,
            required_deposit: 50.00,
            line_items: [{description: 'Cake', quantity: 1, unit_price: 150.00, total_price: 150.00}]
        })
      });
    });

    await page.goto('/quote.html?id=quote-123');

    // Subtotal should be $150.00
    await expect(page.locator('#quote-subtotal')).toContainText('$150.00');

    // Apply points
    await page.locator('#toggle-loyalty-points').click();

    // Total should update to $140.00 (150 - 10)
    await expect(page.locator('#quote-total')).toContainText('$140.00');
  });

  test('Dashboard should have a link to the loyalty widget', async ({ page }) => {
    await page.goto('/dashboard.html');
    const loyaltyLink = page.locator('a#loyalty-link');
    await expect(loyaltyLink).toBeVisible();
    await expect(loyaltyLink).toContainText('Viral Loyalty Engine');
  });

});
