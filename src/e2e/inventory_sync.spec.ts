import { test, expect } from '@playwright/test';

test.describe('Unified Multi-Channel Inventory Sync', () => {
  test('POS tap-to-pay double-booking conflict correctly handled', async ({ browser }) => {
    const context1 = await browser.newContext();
    const checkoutPage = await context1.newPage();
    const context2 = await browser.newContext();
    const posPage = await context2.newPage();

    await checkoutPage.goto('/checkout');
    await posPage.goto('/pos/terminal');

    // POS Login first
    await posPage.waitForSelector('text=Terminal Locked', { state: 'visible', timeout: 5000 }).catch(() => {});
    const pinDigits = ['1', '2', '3', '4'];
    for (const d of pinDigits) {
      await posPage.locator(`button:has-text("${d}")`).click().catch(() => {});
    }

    // 1. Click Pay Now on Checkout to acquire the 5-minute lock.
    const payNowBtn = checkoutPage.locator('button:has-text("Pay Now")');
    await payNowBtn.waitFor({ state: 'visible' });

    await payNowBtn.click();

    // 2. Try POS lock
    const response = await posPage.request.post('/api/v1/payments/terminal/reserve', {
      data: {
        product_id: 'demo_product',
        ttl_secs: 15
      }
    });

    const data = await response.json();

    // It should be false because Checkout locked it
    expect(data.success).toBe(false);

    await context1.close();
    await context2.close();
  });
});
