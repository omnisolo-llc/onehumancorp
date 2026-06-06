import { test, expect } from '@playwright/test';

test.describe('Smart Pricing Feature CUJ', () => {
  test('Owner configures smart pricing bounds', async ({ page }) => {
    // 1. Owner starts at dashboard
    await page.goto('/dashboard');

    await page.goto('/smart-pricing');

    // 3. Verify page loads
    await expect(page.getByText('Smart Pricing', { exact: true }).first()).toBeVisible();

    // 4. Enable smart pricing
    const enableToggle = page.getByTestId('enable-smart-pricing-toggle');
    await enableToggle.waitFor({ state: 'visible' });
    // NextJS client component might need a moment to be interactive
    await page.waitForTimeout(500);
    await enableToggle.click();

    // 5. Check config appears
    await expect(page.getByText('Configuration')).toBeVisible();

    // 6. Enable perishables discount and surge pricing
    await page.getByTestId('discount-perishables-toggle').click();
    await page.getByTestId('surge-pricing-toggle').click();

    // 7. Change bounds slider
    const slider = page.getByTestId('price-bounds-slider');
    await slider.fill('30');

    // 8. Verify preview updates
    await expect(page.getByTestId('preview-min-price')).toHaveText('$7.00');
    await expect(page.getByTestId('preview-max-price')).toHaveText('$13.00');
  });
});
