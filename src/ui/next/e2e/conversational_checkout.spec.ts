import { test, expect } from '@playwright/test';

test.describe('Conversational Checkout Flow', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the referral API that may be called in the success modal
    await page.route('/api/v1/growth/referrals/generate', async route => {
      const json = { referral_link: 'http://ohc.store/join?ref=test-tenant' };
      await route.fulfill({ json });
    });
  });

  test('successfully displays and interacts with Conversational Checkout Session after login', async ({ page }) => {
    // Login to application first
<<<<<<< HEAD
    await page.goto('/login');
=======
    await page.goto('http://localhost:3000/login');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Verify successful login
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Navigate to inbox
    await page.getByRole('link', { name: 'Inbox' }).first().click();

    // Trigger draft action to verify interaction in the actual flow
    await page.getByRole('button', { name: '✨ AI Draft' }).click();

    // The mock data is populated after click. We intercept conversational_checkout api:
    await page.route('/api/v1/booking/conversational_checkout', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          session_id: 'mock_session_123',
          tenant_id: 'tenant_123',
          customer_id: 'cust_456',
          amount_cents: 1000,
          inventory_lock_id: 'ohc:lock:tenant_123:inventory:prod_789:mock_session_123',
          checkout_url: 'https://checkout.stripe.com/pay/cs_test_mock_session_123',
          status: 'pending',
          expires_at_unix: Math.floor(Date.now() / 1000) + 900
        }),
      });
    });

    // Click "Generate Checkout Link"
    await page.getByRole('button', { name: 'Generate Checkout Link' }).first().click();

    // Wait for the modal and verify its content matches response
    await expect(page.getByRole('heading', { name: 'Checkout Session' })).toBeVisible();
    await expect(page.getByText('Session: mock_session_123')).toBeVisible();

    // Simulate user selecting the 'Pay Now' action presented by the Conversational AI
    await page.getByRole('button', { name: 'Pay Now' }).click();

    // Verification of the outcome state mapping to Conversational Checkout fulfillment
    await expect(page.getByText('Payment Successful!')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Close' })).toBeVisible();
  });
});
