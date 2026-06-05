import { test, expect } from '@playwright/test';
import { e2eAuth } from './fixtures';

test.describe('Zero-Configuration Subscription & Membership Engine', () => {
  test('Merchant can toggle recurring on a product and customer can subscribe via 1-tap checkout', async ({ page, request }) => {
    const tenantId = `tenant_${Date.now()}`;
    await e2eAuth(page, tenantId);

    // 1. Merchant adds a new product as a subscription
    await page.goto('/products/new');
    await page.fill('input[placeholder="e.g., Artisan Coffee Beans"]', 'Monthly VIP Coffee Box');
    await page.fill('input[placeholder="0.00"]', '49.99');

    // Toggle "Offer as Subscription"
    const subscribeToggle = page.locator('text=Offer as Subscription');
    await subscribeToggle.click();

    // Verify interval dropdown appears
    await expect(page.locator('select')).toContainText('Month');

    // Save Product
    await page.click('button:has-text("Save Product")');
    await expect(page.locator('text=Created Monthly VIP Coffee Box')).toBeVisible();

    // 2. Customer navigates to checkout (mocking storefront checkout link)
    await page.goto('/checkout');
    await expect(page.locator('text=Checkout')).toBeVisible();

    const subscribeButton = page.locator('button:has-text("Subscribe Monthly")');
    await expect(subscribeButton).toBeVisible();

    // Customer clicks to subscribe
    await subscribeButton.click();

    // Success modal appears with magic link copy
    await expect(page.locator('text=Payment Successful!')).toBeVisible();
    await expect(page.locator('text=magic link to manage your subscription')).toBeVisible();

    // Verify subscriber was recorded via backend API
    const authCookie = (await page.context().cookies()).find((c: { name: string; value: string; domain: string; path: string; expires: number; httpOnly: boolean; secure: boolean; sameSite: "Strict" | "Lax" | "None" }) => c.name === 'ohc_tenant')?.value;
    const subscribersRes = await request.get('/api/v1/subscription/subscribers', {
        headers: {
            'Cookie': `ohc_tenant=${authCookie}`
        }
    });

    expect(subscribersRes.ok()).toBeTruthy();
    const subscribers = await subscribersRes.json();
    expect(subscribers.length).toBeGreaterThan(0);
    expect(subscribers[0].status).toBe('Active');
  });
});
