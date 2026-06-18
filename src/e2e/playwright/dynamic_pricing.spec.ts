import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Dynamic Pricing Engine UX', () => {
  test('should approve dynamic pricing rule and verify it applied in cart', async ({ adminPage }) => {
    // 1. Navigate to the dashboard to view the Unified Agent Feed
    await adminPage.goto('/dashboard');

    // Ensure the feed is visible
    const feedSection = adminPage.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // 2. Find the dynamic pricing recommendation in the feed
    // This looks for the text added in e2e-seed.sql for the Yield Agent
    const pricingCard = feedSection.locator('div.bg-\\[rgba\\(255\\,255\\,255\\,0\\.65\\)\\]', {
      hasText: 'High Demand Surge: Vegan Celebration Cake'
    });

    // It should exist
    await expect(pricingCard).toBeVisible();

    // 3. Approve the pricing rule
    const approveButton = pricingCard.locator('[data-testid="approve-proposal"], [data-testid="approve-send-proposal"]');
    await approveButton.click();

    // Verify it moves to activity or disappears
    await expect(pricingCard).not.toBeVisible();

    // 4. Verify the pricing applied by hitting the cart API directly
    // Since the frontend storefront UI might not be fully functional for E2E cart logic out-of-the-box,
    // we use a direct API call to add the item and verify its price is higher than base price.
    // Base price in DB is 3999 cents ($39.99). A 15% surge should be 4598 cents.

    const cartRes = await adminPage.request.post('/api/cart', {
      data: {
        channel: 'online',
        currency: 'usd'
      }
    });
    expect(cartRes.status()).toBe(200);
    const cart = await cartRes.json();
    const cartId = cart.id;

    const addRes = await adminPage.request.post(`/api/cart/${cartId}/items`, {
      data: {
        product_id: 'e2e-product-cake',
        quantity: 1,
        unit_price_cents: 3999 // pass the base price
      }
    });

    expect(addRes.status()).toBe(200);

    const verifyRes = await adminPage.request.get(`/api/cart/${cartId}`);
    expect(verifyRes.status()).toBe(200);
    const verifyCart = await verifyRes.json();

    // The cart total should be greater than the 3999 base price due to the dynamic pricing rule we approved
    expect(verifyCart.total_amount_cents).toBeGreaterThan(3999);
  });
});
