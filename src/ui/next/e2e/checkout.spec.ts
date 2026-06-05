import { test, expect } from '@playwright/test';

test.describe('Checkout Flow', () => {
  // Use mock for the fetch request so it resolves correctly, in e2e mode we use interceptor
  test.beforeEach(async ({ page }) => {
    await page.route('/api/v1/growth/referrals/generate', async route => {
      const json = { referral_link: 'http://ohc.store/join?ref=test-tenant' };
      await route.fulfill({ json });
    });
  });

  test('completes payment successfully', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/checkout');
=======
    await page.goto('http://localhost:3000/checkout');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Test the Pay Now button
    await page.getByRole('button', { name: 'Pay Now' }).click();

    // In e2e, the fetch may be too fast to see Processing... text, we just verify success modal shows.
    // Should show success modal
    await expect(page.getByText('Payment Successful!')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Continue to Dashboard' })).toBeVisible();
  });
});
