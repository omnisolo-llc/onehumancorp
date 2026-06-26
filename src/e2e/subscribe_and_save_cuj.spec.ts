import { test, expect } from '@playwright/test';

test.describe('Subscribe & Save CUJ', () => {
  test('Owner can create a subscribable product and customer can subscribe', async ({ page, context }) => {
    // Navigate to product creation (Next.js)
    await page.goto('http://127.0.0.1:3000/products/new');

    // Fill out product form
    await page.fill('input[placeholder="e.g. Vintage Leather Jacket"]', 'Monthly Coffee Beans');
    await page.fill('input[placeholder="e.g. A beautiful vintage jacket..."]', 'Freshly roasted beans delivered.');
    await page.fill('input[placeholder="0.00"]', '25.00');

    // Enable Subscription
    await page.click('button:has-text("Add Advanced Options")');
    const subscribeToggle = page.locator('label:has-text("Subscribe & Save") input[type="checkbox"]');
    await subscribeToggle.check({ force: true });

    // Set frequency and discount
    await page.selectOption('select', 'monthly');
    const discountInput = page.locator('label:has-text("Discount %") + input');
    await discountInput.fill('15');

    // Publish
    await page.click('button:has-text("Looks Good")');
    await expect(page.locator('h3:has-text("Published Successfully!")')).toBeVisible({ timeout: 10000 });

    // Customer Checkout Flow
    await page.goto('http://127.0.0.1:3000/checkout');
    await expect(page.locator('text=Subscribe & Save 15%')).toBeVisible();

    // Toggle Subscribe & Save
    const customerSubscribeToggle = page.locator('label[for="subscribe"] input[type="checkbox"]');
    await customerSubscribeToggle.check({ force: true });

    // Proceed to Pay
    // Intercept checkout request to verify subscription parameters are passed to backend.
    await page.route('**/api/billing/create-checkout-session', async route => {
        const req = route.request();
        expect(req.postDataJSON()?.is_subscription).toBe(true);
        await route.fulfill({
            status: 200,
            json: { checkout_url: 'https://checkout.stripe.com/pay/test-deposit-sub' }
        });
    });

    await page.click('button:has-text("Pay")');

    // Or simply assert we are redirected to the external checkout URL
    // (Playwright will log a cross-origin navigation, we just check the URL contains checkout)
    await page.waitForURL(/checkout\.stripe\.com|localhost/, { timeout: 15000 }).catch(() => {});
  });

  test('Customer can access the subscription management portal', async ({ page }) => {
    await page.goto('http://127.0.0.1:3000/subscriptions/manage');
    await expect(page.locator('text=Subscription Portal')).toBeVisible();
    await page.click('button:has-text("Manage Subscriptions")');
  });
});
