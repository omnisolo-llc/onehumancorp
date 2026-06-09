import { test, expect } from './fixtures';

test('User can attempt to get shipping rates for an order', async ({ page }) => {
  await page.goto('http://localhost:3000/orders/e2e-shippo-order');

  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible({ timeout: 30000 });
  await expect(page.getByRole('heading', { name: 'Fulfillment' })).toBeVisible({ timeout: 30000 });

  // Verify address fields are present
  await expect(page.getByPlaceholder('Address Line 1')).toBeVisible({ timeout: 10000 });
  await expect(page.getByPlaceholder('City')).toBeVisible({ timeout: 10000 });
  await expect(page.getByPlaceholder('State')).toBeVisible({ timeout: 10000 });
  await expect(page.getByPlaceholder('Zip')).toBeVisible({ timeout: 10000 });

  // Input weight and dimensions
  await page.getByRole('spinbutton').fill('20');
  await page.getByPlaceholder('e.g. 10x8x6').fill('12x10x8');

  // Input address
  await page.getByPlaceholder('Address Line 1').fill('123 Main St');

  // Fetch rates
  await page.getByRole('button', { name: /Get Shipping Rates/ }).click();

  // Wait for the button to finish loading
  await expect(page.getByRole('button', { name: /Get Shipping Rates/ })).toBeEnabled({ timeout: 15000 });
});
