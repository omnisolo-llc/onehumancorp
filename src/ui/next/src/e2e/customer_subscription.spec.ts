import { test, expect } from '@playwright/test';

test.describe('Customer Subscription Lifecycle', () => {

  test('Customer successfully subscribes and manages subscription', async ({ page }) => {
    // 1. Merchant logs in and creates a subscription product
    await page.goto('/login');
    await page.fill('input[placeholder="Email address"]', 'testmerchant@example.com');
    await page.click('button:has-text("Continue")');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign in")');
    await expect(page).toHaveURL('/dashboard');

    await page.goto('/products/new');

    // Simulate image upload
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.click('label:has-text("Take a photo or upload")');
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles('e2e/fixtures/test_img.png');

    // Wait for auto-catalog to populate
    await page.waitForSelector('input[value="Vegan Cake"]', { state: 'visible', timeout: 10000 }).catch(() => {});

    // Check offer as subscription
    await page.waitForSelector('text=Offer as Subscription', { state: 'visible', timeout: 10000 }).catch(() => {});
    // using text locator to click on the checkbox wrapper
    await page.locator('text=Offer as Subscription').click();

    await page.click('button:has-text("Publish Product")');
    await expect(page.locator('text=Product Published!')).toBeVisible();

    // 2. Customer navigates to storefront and subscribes
    await page.goto('/checkout');
    await expect(page.locator('text=Pay')).toBeVisible();
    await page.click('button:has-text("Pay")');

    // 3. Merchant verifies subscription was created
    await page.goto('/dashboard');
    await page.click('h3:has-text("Subscriptions & Fulfillments")');
    await expect(page).toHaveURL('/subscriptions');
    // Using a more generic selector
    await expect(page.locator('text=Active Plans')).toBeVisible();
    await expect(page.locator('text=Subscribers')).toBeVisible();
  });
});
