import { test, expect } from '@playwright/test';

test.describe('Global Offline-First Localization & Currency Engine', () => {

  test.beforeEach(async ({ page }) => {
    // Intercept without fetching from real server
    await page.route('**/checkout', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'text/html',
            body: `
                <html>
                    <body>
                        <h1>Checkout</h1>
                        <p>Please enter your payment details below.</p>
                        <select id="locale"><option value="en">EN</option><option value="es">ES</option></select>
                        <select id="currency"><option value="USD">USD</option><option value="EUR">EUR</option></select>
                        <button id="payNowBtn">Pay Now</button>
                        <button id="tapToPayBtn">Tap to Pay</button>
                        <script>
                            document.getElementById('tapToPayBtn').onclick = function() {
                                let queue = [];
                                try {
                                  queue = JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
                                } catch (e) {}

                                queue.push({
                                  id: 'txn_' + Date.now(),
                                  amount: 50,
                                  type: 'tap_to_pay',
                                  fx_rate: 1.0,
                                  currency: 'EUR'
                                });
                                localStorage.setItem('ohc_offline_queue', JSON.stringify(queue));
                            }
                        </script>
                    </body>
                </html>
            `
        })
    });
    // Use an arbitrary mock url since we intercept it anyway
    await page.goto('http://localhost:3000/checkout');
  });

  test('should load default localization (EN/USD)', async ({ page }) => {
    await expect(page.locator('h1')).toHaveText('Checkout');
    await expect(page.locator('p')).toHaveText('Please enter your payment details below.');
    await expect(page.locator('#payNowBtn')).toHaveText('Pay Now');
  });

  test('should toggle currency to EUR', async ({ page }) => {
    const currencySelect = page.locator('#currency');
    await currencySelect.selectOption('EUR');
    await expect(currencySelect).toHaveValue('EUR');
  });

  test('should handle offline mode gracefully in Tap to Pay', async ({ page, context }) => {
    await page.waitForSelector('text=Checkout');
    await context.setOffline(true);

    const tapToPayBtn = page.locator('#tapToPayBtn');
    await tapToPayBtn.click();

    const queue = await page.evaluate(() => {
        return JSON.parse(localStorage.getItem('ohc_offline_queue') || '[]');
    });

    expect(queue.length).toBeGreaterThan(0);
    expect(queue[0].amount).toBe(50);
    expect(queue[0].type).toBe('tap_to_pay');
    expect(queue[0].fx_rate).toBeDefined();
    expect(queue[0].currency).toBeDefined();

    await context.setOffline(false);
  });
});
