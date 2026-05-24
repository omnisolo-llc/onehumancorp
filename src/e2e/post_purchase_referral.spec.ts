import { test, expect } from './fixtures';

test.describe('Post-Purchase Referral Flow', () => {
  test('displays referral modal after successful checkout', async ({ page }) => {
    // Navigate directly to the checkout page
    await page.goto('/checkout');

    // Make sure we're on the checkout page
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Click "Pay Now"
    await page.getByRole('button', { name: 'Pay Now' }).click();

    // Verify the "Thank You!" heading and "Payment Successful" appears
    await expect(page.getByRole('heading', { name: 'Thank You!' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Payment Successful' })).toBeVisible();

    // Verify the Referral Offer appears
    await expect(page.getByRole('heading', { name: 'Give $10, Get $10' })).toBeVisible();

    // Verify the copy link button is present
    await expect(page.getByRole('button', { name: 'Copy' })).toBeVisible();

    // Verify social share buttons are present
    await expect(page.getByRole('link', { name: /WhatsApp/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /X \(Twitter\)/i })).toBeVisible();

    // Verify "Continue to Dashboard" button exists and works
    const continueBtn = page.getByRole('button', { name: 'Continue to Dashboard' });
    await expect(continueBtn).toBeVisible();
    await continueBtn.click();

    // Verify it redirects to the dashboard (wait for URL change or heading)
    await expect(page).toHaveURL(/.*dashboard.*/);
  });
});
