import { test, expect } from '@playwright/test';

test.describe('In-Person POS Payments via Tap-to-Pay', () => {
  // Test User Persona: Priya (Boutique Owner)
  // Scenario: Priya receives a customer in-store who wants to pay via Tap-to-Pay

  test('Priya completes a Tap-to-Pay transaction from checkout screen', async ({ page }) => {
    // Navigate directly to checkout in test mode
    await page.goto('/checkout?testMode=true');

    // Verify we are on the checkout page
    await expect(page).toHaveURL(/.*checkout.*/);
    await expect(page.locator('h1')).toHaveText('Checkout');

    // Intercept our newly created Stripe terminal endpoints to avoid actually hitting the network,
    // although they should be handled locally without stripe since we wrote fallback mocks in Rust.
    // However, playwright intercepts help guarantee test stability in CI environments.
    await page.route('**/api/v1/stripe/terminal/connection_token', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ secret: 'tok_terminal_mock' }),
      });
    });

    await page.route('**/api/v1/stripe/terminal/payment_intent', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          id: 'pi_mock_123',
          object: 'payment_intent',
          amount: 4500,
          currency: 'usd',
          client_secret: 'pi_mock_123_secret',
          status: 'requires_payment_method'
        }),
      });
    });

    await page.route('**/api/v1/stripe/terminal/capture', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          message: 'Payment captured and inventory updated'
        }),
      });
    });

    // Priya taps the terminal button
    await page.click('text="Tap to Pay (Stripe Terminal)"');

    // Wait for the success modal to appear which confirms the full sequence ran successfully
    await expect(page.locator('text="Payment Successful!"')).toBeVisible();
    await expect(page.locator('p', { hasText: 'Your order is confirmed.' })).toBeVisible();
  });
});
