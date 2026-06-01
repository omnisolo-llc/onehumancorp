import { test, expect } from './fixtures';

test.describe('Viral Checkout Referral Loop on Checkout Page', () => {
  test('should display Payment Successful modal and generate a referral link', async ({ page }) => {
    await page.goto('/checkout?tier=Starter');

    // Click pay now
    await page.getByRole('button', { name: 'Pay with Mercado Pago' }).click();

    // The dialog tells us "Redirecting to Mercado Pago...", accept it
    page.on('dialog', dialog => dialog.accept());

    // Verify modal content
    await expect(page.getByRole('heading', { name: 'Payment Successful!' })).toBeVisible();
    await expect(page.getByText('Your order is confirmed. Love what you bought?')).toBeVisible();

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy' }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
