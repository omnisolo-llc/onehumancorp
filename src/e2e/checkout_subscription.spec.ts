import { test, expect } from '@playwright/test';

test.describe('Checkout Subscription Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.route('/api/v1/billing/subscription/intent', async route => {
      const json = { checkout_url: 'https://checkout.stripe.com/pay/cs_test_mock_session_123' };
      await route.fulfill({ json });
    });
    await page.route('/api/v1/growth/referrals/generate', async route => {
      const json = { referral_link: 'http://ohc.store/join?ref=test-tenant' };
      await route.fulfill({ json });
    });
  });

  test('completes subscription payment successfully', async ({ page }) => {
    // Navigate to checkout with subscription flags
    await page.goto('http://localhost:3000/checkout?isSubscription=true&planId=plan_test_123');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Test the Pay Now button
    await page.getByRole('button', { name: 'Pay Now' }).click();

    // In e2e we mocked the flow, verify it hits the success UI state
    await expect(page.getByText('Payment Successful!')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Continue to Dashboard' })).toBeVisible();
  });
});
