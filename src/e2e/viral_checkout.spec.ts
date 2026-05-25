import { test, expect } from './fixtures';

test.describe('Viral Checkout Share Loop', () => {
  test('should display share prompt after payment and reveal discount code upon sharing', async ({ page }) => {
    // Navigate to checkout
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // The payment form should be visible
    await expect(page.getByText('Please enter your payment details below.')).toBeVisible();

    // Click "Pay Now"
    await page.getByRole('button', { name: 'Pay Now' }).click();

    // Should now see the post-purchase celebration
    await expect(page.getByText('Payment Successful!')).toBeVisible();
    await expect(page.getByText('Thank you for your purchase. Want')).toBeVisible();

    // Verify share buttons exist
    const shareXBtn = page.getByRole('button', { name: 'Share on X' });
    const shareWhatsAppBtn = page.getByRole('button', { name: 'Share on WhatsApp' });

    await expect(shareXBtn).toBeVisible();
    await expect(shareWhatsAppBtn).toBeVisible();

    // Mock window.open to prevent new tabs opening during test
    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    // Click "Share on X"
    await shareXBtn.click();

    // Verify discount code is revealed
    await expect(page.getByText('Thanks for sharing! Here is your discount code:')).toBeVisible();
    await expect(page.getByText('VIRAL15')).toBeVisible();

    // Verify continue to dashboard button appears
    const continueBtn = page.getByRole('button', { name: 'Continue to Dashboard' });
    await expect(continueBtn).toBeVisible();

    // Click continue to dashboard
    await continueBtn.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
