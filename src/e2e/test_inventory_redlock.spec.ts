import { test, expect } from './fixtures';

test.describe('Distributed POS Redlock Inventory Sync', () => {
  test('should prevent concurrent checkout via redlock in UI', async ({ adminPage: page, serverBaseUrl, browser }) => {
    // Navigate to the storefront
    await page.goto(`${serverBaseUrl}/store`);

    // Simulating Priya logging into POS in another browser context
    const posContext = await browser.newContext();
    const posPage = await posContext.newPage();

    // POS login flow
    await posPage.goto(`${serverBaseUrl}/login`);
    await posPage.fill('input[type="email"]', 'test@example.com');
    await posPage.fill('input[type="password"]', 'password123');
    await posPage.click('button[type="submit"]');
    await posPage.waitForURL(`${serverBaseUrl}/dashboard`);

    // Navigate to POS terminal interface
    await posPage.goto(`${serverBaseUrl}/pos`);

    // Add item to POS cart
    await posPage.click('text=POS Sync Product');

    // In parallel, the online customer tries to buy the same item
    await page.goto(`${serverBaseUrl}/store`);
    await page.click('text=POS Sync Product');
    await page.click('button:has-text("Add to Cart")');
    await page.goto(`${serverBaseUrl}/checkout`);

    // POS initiates payment (acquires lock) first
    await posPage.click('button:has-text("Charge")');
    // Ensure POS lock is acquired before triggering customer
    await posPage.waitForTimeout(500);

    // Customer attempts to place order right after
    await page.click('button:has-text("Place Order")');


    // Online checkout should fail gracefully due to lock
    await expect(page.locator('text=Item is currently being checked out by another customer.')).toBeVisible();

    // POS checkout should succeed
    await expect(posPage.locator('text=Payment Successful')).toBeVisible();

    // Verify Operations Agent Feed Integration
    await posPage.goto(`${serverBaseUrl}/dashboard`);
    await expect(posPage.locator('text=Review and approve restock order')).toBeVisible();

    await posContext.close();
  });
});
