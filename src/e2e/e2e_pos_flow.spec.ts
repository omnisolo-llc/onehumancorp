import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction via checkout', async ({ page }) => {
    await page.goto('/checkout');

    // Tap to Pay button
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Enter amount to charge:');
      await dialog.accept('50');
    });

    await page.getByRole('button', { name: 'Tap to Pay (Stripe Terminal)' }).click();

    // The modal should appear
    await expect(page.getByRole('heading', { name: 'Tap to Pay' })).toBeVisible();

    // Verify Stripe Terminal is initializing/ready
    await expect(page.locator('text=Status:')).toBeVisible();

    // Click Discover Readers
    await page.getByRole('button', { name: 'Discover Readers' }).click();

    // Connect reader
    await page.getByRole('button', { name: 'Connect' }).first().click();

    // Charge button should appear
    await page.getByRole('button', { name: 'Charge $50.00' }).click();

    // Eventually status says successful
    await expect(page.locator('text=Status: Payment successful!')).toBeVisible({ timeout: 10000 });
  });
});
