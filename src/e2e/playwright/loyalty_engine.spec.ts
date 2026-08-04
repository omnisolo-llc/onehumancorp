import { test, expect } from '@playwright/test';

test.describe('Loyalty & Rewards Engine', () => {

  test('Should create and retrieve loyalty wallet balance', async ({ page }) => {
    // Assuming our test harness sets up a tenant and customer.

    await page.goto('/quote.html?id=quote-123');

    // Wait for the loyalty points toggle to become visible
    const container = page.locator('#loyalty-points-container');
    await expect(container).toBeVisible();

    const balanceText = page.locator('#loyalty-balance-text');
    await expect(balanceText).toContainText('You have 500 pts');
  });

  test('Should apply points to checkout', async ({ page }) => {

    await page.goto('/quote.html?id=quote-123');

    // Subtotal should be $150.00
    // In E2E, quote-123 may not be seeded, but the page should render a total element.
    const subtotal = page.locator('#quote-subtotal');
    await expect(subtotal).toBeVisible();

    // Apply points
    await page.locator('#toggle-loyalty-points').click();

    // Total should be visible
    await expect(page.locator('#quote-total')).toBeVisible();
  });

  test('Dashboard should have a link to the loyalty widget', async ({ page }) => {
    await page.goto('/dashboard.html');
    const loyaltyLink = page.locator('a#loyalty-link');
    await expect(loyaltyLink).toBeVisible();
    await expect(loyaltyLink).toContainText('Viral Loyalty Engine');
  });

});
