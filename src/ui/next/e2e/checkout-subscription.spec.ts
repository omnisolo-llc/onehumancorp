import { test, expect } from './fixtures';

test.describe('Checkout Flow with Subscribe & Save', () => {
  test('completes subscription checkout successfully', async ({ page }) => {
    // Navigate to checkout with a seeded product id that supports subscribe & save
    // "e2e-product-cake" or similar from e2e-seed.sql, but we can just use the default mock in nextjs for now,
    // though the real backend will handle it. Wait, the Next.js page defaults to prod_123 if not given.
    await page.goto('/checkout?product_id=e2e-product-cake');
    await expect(page.getByRole('heading', { name: 'Secure Checkout' })).toBeVisible();

    // The backend should return the checkout URL.
    // Intercept window.location.assign so we don't actually navigate to Stripe in the E2E test.
    await page.addInitScript(() => {
      let interceptedUrl = '';
      Object.defineProperty(window, 'location', {
        configurable: true,
        enumerable: true,
        get: () => {
          return {
            assign: (url) => { interceptedUrl = url; window['_interceptedCheckoutUrl'] = url; },
            href: window.location.href,
            search: window.location.search,
            pathname: window.location.pathname,
          };
        }
      });
    });

    // Check the Subscribe & Save checkbox (Wait for it to be visible first)
    const subscribeCheckbox = page.getByText('Subscribe & Save 10%');
    await expect(subscribeCheckbox).toBeVisible();
    await subscribeCheckbox.click();

    // Test the Pay button
    await page.getByRole('button', { name: 'Pay' }).click();

    // Wait for the simulated navigation to be captured
    await expect(async () => {
      const url = await page.evaluate(() => window['_interceptedCheckoutUrl']);
      expect(url).toContain('checkout.stripe.com');
    }).toPass({ timeout: 10000 });

    // Should navigate to success or show success modal
    await page.goto('/checkout?success=true&isSub=true');

    await expect(page.getByText('Payment Successful!')).toBeVisible();
    await expect(page.getByText(/You're in! We'll text you a magic link to manage your/)).toBeVisible();
    await expect(page.getByRole('button', { name: 'Continue to Dashboard' })).toBeVisible();
  });
});
