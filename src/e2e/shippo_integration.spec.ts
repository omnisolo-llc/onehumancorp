import { test, expect } from './fixtures';

test('User can purchase and print shipping labels for an order', async ({ page }) => {
  // Navigate to the orders page
  await page.goto('/orders/e2e-shippo-order');

  // Wait for the order details page to load
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible({ timeout: 30000 });

  // Verify fulfillment section
  await expect(page.getByRole('heading', { name: 'Shipping' })).toBeVisible({ timeout: 30000 });

  // Input weight and dimensions
  await page.getByRole('spinbutton', { name: /Package weight in ounces/i }).fill('20');
  await page.getByRole('textbox', { name: /Package dimensions/i }).fill('12x10x8');

  // Fetch rates
  await page.getByRole('button', { name: /Get Shipping Rates/ }).click();

  // Select the first rate (USPS Priority Mail usually)
  await page.locator('input[name="shipping-rate"]').first().click();

  // Buy label
  await page.getByRole('button', { name: /Buy Label/ }).click();

  // Wait for success status
  await expect(page.getByRole('link', { name: /Open Shipping Label/ })).toBeVisible({ timeout: 30000 });
});

test('User encounters address validation error and corrects it', async ({ page }) => {
  await page.goto('/orders/e2e-shippo-order');
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible({ timeout: 30000 });

  await page.getByRole('spinbutton', { name: /Package weight in ounces/i }).fill('9999');
  await page.getByRole('textbox', { name: /Package dimensions/i }).fill('100x100x100');

  await page.getByRole('button', { name: /Get Shipping Rates/ }).click();

  await expect(page.getByRole('alert')).toBeVisible({ timeout: 30000 });
});
