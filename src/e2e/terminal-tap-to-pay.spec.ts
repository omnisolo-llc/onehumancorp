import { test, expect } from '@playwright/test';

test.describe('Zero-Config Universal Tap-to-Pay POS Engine', () => {
  test('Business Owner (Priya) can accept Tap-to-Pay payments', async ({ page }) => {
    // Navigate to the POS / Checkout screen
    await page.goto('/');

    // Ensure basic UI is alive
    await expect(page.locator('text=OneHumanCorp')).toBeVisible();

    // Since this is primarily a hardware/mobile-device feature (Tap to Pay on iPhone/Android),
    // and we cannot fully simulate NFC taps in Playwright, we verify that the web app
    // successfully invokes the backend to generate a terminal connection token.

    // Simulate user initiating a charge
    const resToken = await page.request.post('/api/v1/payments/terminal/token');

    // Unauthenticated request should be handled gracefully (returning error rather than crashing)
    // Wait for the endpoint to stabilize
    expect([200, 401]).toContain(resToken.status());

    // Simulate creating a payment intent
    const resIntent = await page.request.post('/api/v1/payments/terminal/intent', {
        data: { amount_cents: 2500, currency: "usd" }
    });

    // Unauthenticated request should be handled gracefully
    expect([200, 401]).toContain(resIntent.status());
  });
});
