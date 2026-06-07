import { test, expect } from '@playwright/test';

test('User can purchase and print shipping labels for an order', async ({ page }) => {
  // Navigate to the orders page
  await page.goto('/orders/e2e-shippo-order');

  // Wait for the order details page to load
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible({ timeout: 30000 });

  // Verify fulfillment section
  await expect(page.getByRole('heading', { name: 'Fulfillment' })).toBeVisible({ timeout: 30000 });
  await expect(page.getByText('Powered by Shippo')).toBeVisible({ timeout: 30000 });

  // Input weight and dimensions
  await page.getByRole('spinbutton').fill('20');
  await page.getByPlaceholder('e.g. 10x8x6').fill('12x10x8');

  // Fetch rates
  await page.getByRole('button', { name: /Get Shipping Rates/ }).click();

  // Wait for rates to appear
  await expect(page.getByText('Select a Service')).toBeVisible({ timeout: 30000 });

  // Select the first rate (USPS Priority Mail usually)
  await page.locator('input[name="shipping_rate"]').first().click();

  // Buy label
  await page.getByRole('button', { name: /Buy Label & Print/ }).click();

  // Wait for success status
  await expect(page.getByText('Label Purchased Successfully')).toBeVisible({ timeout: 30000 });
  await expect(page.getByRole('link', { name: /Print Label/ })).toBeVisible({ timeout: 30000 });
  await expect(page.getByText('Shipped', { exact: true }).first()).toBeVisible({ timeout: 30000 });
});
