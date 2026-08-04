import { test, expect } from '@playwright/test';

test.describe('Loyalty & Rewards Engine', () => {

  test('Should create and retrieve loyalty wallet balance', async ({ page }) => {
    // Override window.fetch to bypass playwright's strict page.route network interception rules
    await page.evaluate(() => {
        const originalFetch = window.fetch;
        window.fetch = async (...args) => {
            const url = args[0]?.toString() || '';
            if (url.includes('/api/ui/loyalty/balance')) {
                return new Response(JSON.stringify({ points_balance: 500, wallet_id: "test-wallet" }), { status: 200, headers: { 'Content-Type': 'application/json' } });
            }
            if (url.includes('/api/ui/quote')) {
                return new Response(JSON.stringify({
                    id: 'quote-123',
                    business_name: 'Maya Cakes',
                    title: 'Custom Vegan Cake',
                    status: 'PENDING',
                    total_amount: 150.00,
                    required_deposit: 50.00,
                    line_items: [{description: 'Cake', quantity: 1, unit_price: 150.00, total_price: 150.00}]
                }), { status: 200, headers: { 'Content-Type': 'application/json' } });
            }
            return originalFetch.apply(window, args);
        };
    });

    await page.goto('/quote.html?id=quote-123');

    // Wait for the loyalty points toggle to become visible
    const container = page.locator('#loyalty-points-container');
    await expect(container).toBeVisible();

    const balanceText = page.locator('#loyalty-balance-text');
    await expect(balanceText).toContainText('You have 500 pts');
  });

  test('Should apply points to checkout', async ({ page }) => {
    await page.evaluate(() => {
        const originalFetch = window.fetch;
        window.fetch = async (...args) => {
            const url = args[0]?.toString() || '';
            if (url.includes('/api/ui/loyalty/balance')) {
                return new Response(JSON.stringify({ points_balance: 1000, wallet_id: "test-wallet" }), { status: 200, headers: { 'Content-Type': 'application/json' } });
            }
            if (url.includes('/api/ui/quote')) {
                return new Response(JSON.stringify({
                    id: 'quote-123',
                    business_name: 'Maya Cakes',
                    title: 'Custom Vegan Cake',
                    status: 'PENDING',
                    total_amount: 150.00,
                    required_deposit: 50.00,
                    line_items: [{description: 'Cake', quantity: 1, unit_price: 150.00, total_price: 150.00}]
                }), { status: 200, headers: { 'Content-Type': 'application/json' } });
            }
            return originalFetch.apply(window, args);
        };
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
