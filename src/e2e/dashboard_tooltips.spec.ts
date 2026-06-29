import { test, expect } from './fixtures';

test.describe('Dashboard Tooltips', () => {

  test('hovering over dashboard widgets shows correct tooltips', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Hover over Cart Recovery widget
    const cartRecoveryLink = page.locator('a[href="/cart-recovery"]');
    await cartRecoveryLink.hover();

    // Check if tooltip appears
    const tooltip1 = page.getByText('Recover abandoned carts with personalized AI follow-ups.');
    await expect(tooltip1).toBeVisible();

    // Hover away to close
    await page.mouse.move(0, 0);
    await expect(tooltip1).not.toBeVisible();

    // Hover over Flash Sale widget
    const flashSaleLink = page.locator('a[href="/flash-sale-generator"]');
    await flashSaleLink.hover();

    const tooltip2 = page.getByText('Create high-converting flash sale countdown widgets.');
    await expect(tooltip2).toBeVisible();
  });
});
