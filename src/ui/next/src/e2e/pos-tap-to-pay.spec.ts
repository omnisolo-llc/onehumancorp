import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('POS Tap to Pay Integration', () => {
  test('Completes an optimistic Tap to Pay transaction successfully', async ({ browser }) => {
    // We launch a dedicated page for POS simulation instead of reusing adminPage to prevent bleeding state
    const page = await browser.newPage();
    await page.goto('/login');

    await page.fill('input[name="email"]', 'test@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Wait for the main app view
    await expect(page).toHaveURL(/.*dashboard.*/);


    // Navigate to the POS tab
    await page.goto('/pos.html');

    // Mock the backend API calls
    await page.route('**/api/v1/payments/terminal/token', async route => {
      const json = { secret: 'simulated_token_123' };
      await route.fulfill({ json });
    });

    await page.route('**/api/v1/payments/terminal/intent', async route => {
      const json = { client_secret: 'pi_simulated_secret_123' };
      await route.fulfill({ json });
    });

    await page.route('**/api/v1/payments/terminal/intent/capture', async route => {
      const json = { status: 'succeeded' };
      await route.fulfill({ json });
    });


    // Wait for POS UI to render
    await page.waitForSelector('.catalog-grid');

    // Click on the $10 POS Sync Product from seed data
    await page.click('text=POS Sync Product');

    // Ensure the charge button reflects the correct sum
    const chargeBtn = page.locator('#charge-btn').first();
    await expect(chargeBtn).toContainText('$10.00');

    // Mock the stripe terminal SDK initialization entirely in the test page context
    await page.evaluate(() => {
        window.StripeTerminal = {
            create: () => ({
                discoverReaders: async () => ({ discoveredReaders: [{id: 'simulated_123'}] }),
                connectInternetReader: async () => ({ reader: {id: 'simulated_123'} }),
                collectPaymentMethod: async (secret) => {
                    return { paymentIntent: { id: "pi_simulated_123", status: "requires_confirmation" } };
                },
                processPayment: async (pi) => {
                    return { paymentIntent: { id: "pi_simulated_123", status: "succeeded" } };
                }
            })
        };
        // Re-bind to ensure mock picks it up
        if (window.setupTerminal) window.setupTerminal();
    });

    // Click the charge button
    await chargeBtn.click();

    // Wait for the bottom sheet to slide up and click Tap to Pay
    const tapToPayBtn = page.locator('button.method-btn.contactless');
    await expect(tapToPayBtn).toBeVisible();
    await tapToPayBtn.click();

    // The tap overlay should automatically process, dismiss, and show the receipt screen
    const receiptScreen = page.locator('#receipt-screen');
    await expect(receiptScreen).toBeVisible();

    await page.close();
  });
});
