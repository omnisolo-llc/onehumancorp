import { test, expect } from '@playwright/test';

test('User can purchase and print shipping labels for an order', async ({ page }) => {
  // Navigate to the orders page
  await page.goto('/orders');

  // Check if the orders page loads correctly
  await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();

  // Check if there are unfulfilled orders
  await expect(page.getByText('Unfulfilled').first()).toBeVisible();

  // Click view on the first unfulfilled order
  await page.getByRole('button', { name: 'View' }).first().click();

  // Wait for the order details page to load
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible();

  // Verify fulfillment section
  await expect(page.getByRole('heading', { name: 'Fulfillment' })).toBeVisible();
  await expect(page.getByText('Powered by Shippo')).toBeVisible();

  // Input weight and dimensions
  await page.getByRole('spinbutton').fill('20');
  await page.getByPlaceholder('e.g. 10x8x6').fill('12x10x8');

  // Fetch rates
  await page.getByRole('button', { name: /Get Shipping Rates/ }).click();

  // Wait for rates to appear
  await expect(page.getByText('Select a Service')).toBeVisible();

  // Select the first rate (USPS Priority Mail usually)
  await page.locator('input[name="shipping_rate"]').first().click();

  // Buy label
  await page.getByRole('button', { name: /Buy Label & Print/ }).click();

  // Wait for success status
  await expect(page.getByText('Label Purchased Successfully')).toBeVisible();
  await expect(page.getByRole('link', { name: /Print Label/ })).toBeVisible();
  await expect(page.getByText('Shipped', { exact: true }).first()).toBeVisible();
});
