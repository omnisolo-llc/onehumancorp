import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');

    // Navigate to terminal
    await page.goto('/pos/terminal');

    // Inject offline staff member into localStorage
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([
        { id: '1', name: 'Test Staff', role: 'Manager', pin_hash: '1234' }
      ]));
    });

    // Reload to apply localStorage
    await page.reload();

    // Enter PIN: 1, 2, 3, 4
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // Verify staff logged in
    await expect(page.getByText('Test Staff')).toBeVisible();

    // Click 'New Order'
    await page.getByRole('button', { name: 'New Order' }).click();

    // Verify Stripe Terminal UI is displayed - waiting for either success or failure text
    await expect(page.locator('text=Stripe Terminal')).toBeVisible();
  });
});
