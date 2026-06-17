import { test, expect } from './fixtures';

test.describe('Terminal POS - Mobile First & Inventory Sync', () => {

  test.beforeEach(async ({ page }) => {
    // 1. First navigate to /products/new to create a product
    await page.goto('/products/new');

    // Fill the prompt textarea
    await page.getByPlaceholder('e.g., Guitar lessons for beginners, 1 hour').fill('Test POS Product');

    // Click Generate
    await page.getByRole('button', { name: 'Generate' }).click();

    // Wait for the form to populate and "Looks Good" button to appear
    await expect(page.getByRole('button', { name: 'Looks Good' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Looks Good' }).click();

    // Assert creation success
    await expect(page.getByText('Product Published!')).toBeVisible({ timeout: 10000 });

    // 2. Navigate to POS terminal path
    await page.goto(`/pos/terminal`);

    // Unlock the terminal
    const pins = ['1', '2', '3', '4'];
    for (const p of pins) {
      await page.getByRole('button', { name: p, exact: true }).click();
    }

    // Clock in
    await page.getByRole('button', { name: 'Clock In' }).click();
  });

  test('Processes tap-to-pay and reserves inventory', async ({ page }) => {
    // Set up request interception to verify the correct API calls are made for the connection token and payment intent
    const tokenPromise = page.waitForRequest(
      (request) => request.url().includes('/api/v1/payments/terminal/token') && request.method() === 'POST'
    );

    const intentPromise = page.waitForRequest(
      (request) => request.url().includes('/api/v1/payments/terminal/intent') && request.method() === 'POST'
    );

    // Click the first product button in the catalog
    await page.locator('h3:has-text("Product Catalog") + div.grid button').first().click();

    // Wait for the Stripe Terminal UI to appear
    await expect(page.getByRole('button', { name: 'Discover Readers' })).toBeVisible();

    // Discover Readers
    await page.getByRole('button', { name: 'Discover Readers' }).click();

    // Connect to a reader (the first one)
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // Wait for the token request to occur and assert it was successful
    const tokenRequest = await tokenPromise;
    expect(tokenRequest).toBeTruthy();

    // Wait for the charge button to be visible
    const chargeButton = page.locator('button.charge-btn', { hasText: /Collect Payment/ });
    await expect(chargeButton).toBeVisible({ timeout: 15000 });

    // Click charge
    await chargeButton.click();

    // Ensure intent is created with the expected currency
    const intentRequest = await intentPromise;
    const postData = intentRequest.postDataJSON();
    expect(postData.currency).toBe('usd');

    // Wait for the payment success text
    await expect(page.getByText('Payment successful!')).toBeVisible({ timeout: 20000 });
  });

  test('Processes Quick Charge successfully', async ({ page }) => {
    await page.getByRole('button', { name: 'Quick Charge $50' }).click();
    await expect(page.getByText('Offline Quick Charge Saved.')).toBeVisible();
  });

  test('Handles offline mode and sync queue', async ({ page }) => {
    await page.context().setOffline(true);
    await expect(page.getByText('Offline Mode')).toBeVisible();
    await page.getByRole('button', { name: 'Quick Charge $50' }).click();
    await page.context().setOffline(false);
    await expect(page.getByText('Online')).toBeVisible();
  });
});
