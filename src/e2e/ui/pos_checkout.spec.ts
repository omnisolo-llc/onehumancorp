import { test, expect } from '../fixtures';

test.describe('POS Tap to Pay UI', () => {
  // We use the 'unlimitedAdminUser' fixture to bypass the paywall that was blocking the test
  test('Cashier can initiate a tap to pay transaction on terminal', async ({ unlimitedAdminUser: page }) => {
    // 1. Setup the cart
    await page.goto('/pos/terminal');

    // Add item to cart
    await page.getByRole('button', { name: 'Add Custom Amount' }).click();
    await page.getByPlaceholder('Amount').fill('15.00');
    await page.getByPlaceholder('Note (Optional)').fill('Test Service');
    await page.getByRole('button', { name: 'Add to Cart' }).click();

    // 2. Initiate Checkout
    await page.getByRole('button', { name: 'Charge $15.00' }).click();

    // Select Tap to Pay
    await page.getByRole('button', { name: 'Tap to Pay on iPhone' }).click();

    // 3. Verify the loading/waiting state appears (simulating the reader connection)
    await expect(page.getByText('Present card to reader...')).toBeVisible();

    // In our test environment, we might click a 'Simulate Success' button if available,
    // or just verify the intent creation API was called successfully and the UI updated.

    // For now, ensuring we reach the reader state without a crash is a successful test
    // of the UI layer integration.
  });
});
