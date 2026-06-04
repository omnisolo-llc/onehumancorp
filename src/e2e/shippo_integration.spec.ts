import { test, expect } from '@playwright/test';

test('User can purchase and print shipping labels for an order', async ({ page }) => {
<<<<<<< HEAD
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
=======
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
  // Navigate to the orders page
  await page.goto('/orders');

  // Check if the orders page loads correctly
<<<<<<< HEAD
  await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible({ timeout: 30000 });

  // Check if there are unfulfilled orders
  await expect(page.getByText('Unfulfilled').first()).toBeVisible({ timeout: 30000 });
=======
  await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();

  // Check if there are unfulfilled orders
  await expect(page.getByText('Unfulfilled').first()).toBeVisible();
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)

  // Click view on the first unfulfilled order
  await page.getByRole('button', { name: 'View' }).first().click();

  // Wait for the order details page to load
<<<<<<< HEAD
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible({ timeout: 30000 });

  // Verify fulfillment section
  await expect(page.getByRole('heading', { name: 'Fulfillment' })).toBeVisible({ timeout: 30000 });
  await expect(page.getByText('Powered by Shippo')).toBeVisible({ timeout: 30000 });
=======
  await expect(page.getByRole('heading', { name: /Order/ })).toBeVisible();

  // Verify fulfillment section
  await expect(page.getByRole('heading', { name: 'Fulfillment' })).toBeVisible();
  await expect(page.getByText('Powered by Shippo')).toBeVisible();
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)

  // Input weight and dimensions
  await page.getByRole('spinbutton').fill('20');
  await page.getByPlaceholder('e.g. 10x8x6').fill('12x10x8');

  // Fetch rates
  await page.getByRole('button', { name: /Get Shipping Rates/ }).click();

  // Wait for rates to appear
<<<<<<< HEAD
  await expect(page.getByText('Select a Service')).toBeVisible({ timeout: 30000 });
=======
  await expect(page.getByText('Select a Service')).toBeVisible();
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)

  // Select the first rate (USPS Priority Mail usually)
  await page.locator('input[name="shipping_rate"]').first().click();

  // Buy label
  await page.getByRole('button', { name: /Buy Label & Print/ }).click();

  // Wait for success status
<<<<<<< HEAD
  await expect(page.getByText('Label Purchased Successfully')).toBeVisible({ timeout: 30000 });
  await expect(page.getByRole('link', { name: /Print Label/ })).toBeVisible({ timeout: 30000 });
  await expect(page.getByText('Shipped', { exact: true }).first()).toBeVisible({ timeout: 30000 });
=======
  await expect(page.getByText('Label Purchased Successfully')).toBeVisible();
  await expect(page.getByRole('link', { name: /Print Label/ })).toBeVisible();
  await expect(page.getByText('Shipped', { exact: true }).first()).toBeVisible();
>>>>>>> b068d07b (feat: Implement instant build storefront wizard)
});
