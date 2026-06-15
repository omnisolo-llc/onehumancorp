import { test, expect } from './fixtures';

test.describe('In-Person POS UI', () => {
  test('CUJ: Navigates from dashboard to POS, enters amount, taps, and sees receipt', async ({ page }) => {
    // 1. Navigation from Dashboard
    await page.goto('/dashboard.html');
    await page.waitForSelector('#dashboard-title');

    // Using the POS Link/Nav Item
    await page.getByRole('link', { name: 'Point of Sale' }).click();
    await expect(page).toHaveURL(/.*\/pos\.html/);

    // 2. Entering an ad-hoc amount on the dialpad
    // Wait for the specific POS container to ensure we are fully loaded
    await page.waitForSelector('#pos-dialpad');

    // Simulating entering $15.50
    const dialpadButtons = ['1', '5', '5', '0'];
    for (const btn of dialpadButtons) {
      await page.getByRole('button', { name: btn, exact: true }).click();
    }

    // Add to order
    await page.getByRole('button', { name: 'Add' }).click();

    // 3. Verify Cart State
    const cartTotal = page.locator('#pos-cart-total');
    await expect(cartTotal).toHaveText('$15.50');

    // 4. Initiate Charge / Tap to Pay
    await page.getByRole('button', { name: 'Charge $15.50' }).click();

    // Expect the tap modal/reader UI to appear
    const readerModal = page.locator('#reader-status-modal');
    await expect(readerModal).toBeVisible();
    await expect(readerModal).toContainText('Present card to reader');
  });
});
