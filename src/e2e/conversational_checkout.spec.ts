import { test, expect } from './fixtures';

test.describe('Conversational Checkout & Instant Deposit Engine', () => {
  test('Autonomous AI Product Bundling intercepts cart addition and offers upsell', async ({ page }) => {
    // 1. Navigate to Checkout Page directly (we can test the new upsell engine here)
    await page.goto('/checkout');
    await page.evaluate(() => {
      localStorage.setItem('ohc_cart', JSON.stringify([{ name: 'Soy Candle', price: 20.00 }]));
    });
    await page.reload();

    // 2. Wait for the Upsell Engine drawer to appear
    await expect(page.getByText('Frequently Bought Together')).toBeVisible();

    // 3. Verify AI Recommended Match
    await expect(page.getByText('Premium Matches')).toBeVisible();
    await expect(page.getByText('$5.00')).toBeVisible();

    // 4. Click Add on the Upsell
    await page.getByRole('button', { name: 'Add' }).click();

    // 5. Checkout
    await page.getByRole('button', { name: 'Pay Now' }).click();
    await expect(page.getByText('Payment Successful!')).toBeVisible();
  });

  test('Sales AI generates conversational checkout link from inbox intent', async ({ page }) => {
    // 1. Navigate to Unified Inbox
    await page.goto('/inbox');

    // 2. Click Simulate Incoming Message
    await page.getByRole('button', { name: '🤖 Simulate Incoming Message' }).click();

    // 3. Verify Sales AI detects intent and generates checkout bubble (mocked via AI Replied)
    await expect(page.getByText('AI Replied')).toBeVisible({ timeout: 15000 });
  });
});
