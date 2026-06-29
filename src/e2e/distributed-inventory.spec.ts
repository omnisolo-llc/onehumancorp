import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Distributed Edge Caching and Dynamic Storefront SEO Engine', () => {

  test('High traffic event invalidates cache correctly without exposing stale data', async ({ browser }) => {
    const context = await browser.newContext();
    const page = await context.newPage();

    // Sign in using the documented flow
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');

    // Setup: Navigate to Inventory
    await page.goto('/inventory');
    await page.waitForLoadState('networkidle');

    // Create a new product for the high traffic test
    await page.click('button:has-text("Add Product")');
    await page.fill('input[name="title"]', 'Viral TikTok Cake');
    await page.fill('input[name="price"]', '45.00');
    await page.fill('input[name="inventory"]', '10'); // Starts with 10
    await page.click('button:has-text("Save")');
    await page.waitForSelector('text=Viral TikTok Cake');

    // Step 1: Simulate customer view (Edge Cache Hit expected)
    const customerPage = await context.newPage();
    await customerPage.goto('/store/products/viral-tiktok-cake');
    await expect(customerPage.locator('text=In Stock')).toBeVisible();

    // Step 2: High traffic event - Simulate rapid checkout on POS or backend
    await page.goto('/pos');
    await page.waitForLoadState('networkidle');

    // Add to cart and checkout (deduct 10)
    for (let i = 0; i < 10; i++) {
        await page.click('text=Viral TikTok Cake');
    }
    await page.click('button:has-text("Checkout")');
    await page.click('button:has-text("Pay Cash")');
    await page.waitForSelector('text=Transaction Complete');

    // Step 3: Verify Edge Cache Invalidation
    // The storefront should now show Sold Out/Waitlist without needing manual refresh
    await customerPage.reload(); // Simulate next customer hitting the page

    // The invalidation should have cleared the edge cache, forcing a fresh pull showing 0 inventory
    await expect(customerPage.locator('text=Sold Out')).toBeVisible();
    await expect(customerPage.locator('text=Join Waitlist')).toBeVisible();

  });
});
