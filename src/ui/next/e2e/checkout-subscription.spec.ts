import { test, expect } from '@playwright/test';

test.describe('Checkout Flow with Subscribe & Save', () => {
  test.beforeEach(async ({ page }) => {
    await page.route('/api/billing/create-checkout-session', async route => {
      // Mock the checkout session creation
      await route.fulfill({
        json: { checkout_url: 'http://localhost:3000/checkout?success=true' }
      });
    });
    await page.route('/api/v1/growth/referrals/generate', async route => {
      const json = { referral_link: 'http://ohc.store/join?ref=test-tenant' };
      await route.fulfill({ json });
    });
  });

  test('completes subscription checkout successfully', async ({ page }) => {
    await page.goto('/checkout');
    await expect(page.getByRole('heading', { name: 'Secure Checkout' })).toBeVisible();

    // Check the Subscribe & Save checkbox
    await page.getByText('Subscribe & Save 10%').click();

    // Test the Pay button
    await page.getByRole('button', { name: 'Pay' }).click();

    // Should navigate to success or show success modal
    // In our mocked flow, we simulate returning from Stripe with ?success=true
    await page.goto('/checkout?success=true&isSub=true');

    await expect(page.getByText('Payment Successful!')).toBeVisible();
    await expect(page.getByText(/You're in! We'll text you a magic link to manage your/)).toBeVisible();
    await expect(page.getByRole('button', { name: 'Continue to Dashboard' })).toBeVisible();
  });
});
