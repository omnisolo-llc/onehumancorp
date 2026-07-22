import { test, expect } from '@playwright/test';

test.describe('Post-Purchase Share Widget Embed', () => {
  test('should load the embed page correctly', async ({ page }) => {
    await page.goto('/embed/post-purchase-share?tenantId=test-tenant&orderId=ord_123&storeName=AwesomeStore');

    // Wait for the component to render
    const shareTitle = page.locator('h2', { hasText: 'Share your purchase' });
    await expect(shareTitle).toBeVisible();

    const storeLink = page.locator('#post-purchase-share-link');
    await expect(storeLink).toHaveValue(/post_purchase_ord_123/);

    const description = page.locator('p', { hasText: 'Copy or share your tracked store link' });
    await expect(description).toBeVisible();

    const copyBtn = page.locator('button', { hasText: 'Copy' });
    await expect(copyBtn).toBeVisible();

    const waBtn = page.locator('button', { hasText: 'Share on WhatsApp' });
    await expect(waBtn).toBeVisible();

    const xBtn = page.locator('button', { hasText: 'Share on X' });
    await expect(xBtn).toBeVisible();
  });

  test('should fallback to defaults when query params are missing', async ({ page }) => {
    await page.goto('/embed/post-purchase-share');

    const shareTitle = page.locator('h2', { hasText: 'Share your purchase' });
    await expect(shareTitle).toBeVisible();

    const storeLink = page.locator('#post-purchase-share-link');
    await expect(storeLink).toHaveValue(/post_purchase_default/);
  });
});
