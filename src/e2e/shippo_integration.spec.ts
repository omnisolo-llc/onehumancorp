import { test, expect } from './fixtures';

test('User can purchase and print shipping labels for an order', async ({ page }) => {
  // Navigate to the orders page
  await page.goto('/orders/e2e-shippo-order');

  // Wait for the order details page to load
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible({ timeout: 30000 });

  // Verify shipping section
  await expect(page.getByRole('heading', { name: 'Shipping' })).toBeVisible({ timeout: 30000 });

  // Input weight and dimensions
  await page.getByLabel('Package weight in ounces').fill('20');
  await page.getByLabel('Package dimensions').fill('12x10x8');

  // Fetch rates
  await page.getByRole('button', { name: 'Get Shipping Rates' }).click();

  // Wait for rates to appear
  await expect(page.locator('input[type="radio"]')).not.toHaveCount(0, { timeout: 30000 });

  // Select the first rate (USPS Priority Mail usually)
  await page.locator('input[type="radio"]').first().click();

  // Buy label
  await page.getByRole('button', { name: 'Ship Order' }).click();

  // Wait for success status
  await expect(page.getByRole('link', { name: 'Print Label' })).toBeVisible({ timeout: 30000 });
});

test('User encounters address validation error and corrects it', async ({ page }) => {
  await page.goto('/orders/e2e-shippo-order');
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible({ timeout: 30000 });

  await page.getByLabel('Package weight in ounces').fill('9999');
  await page.getByLabel('Package dimensions').fill('100x100x100');

  await page.getByRole('button', { name: 'Get Shipping Rates' }).click();
});
