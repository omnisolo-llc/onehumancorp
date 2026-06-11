import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Cart Recovery Feature', () => {
  test('should verify abandoned cart recovery flow', async ({ page, context }) => {
    // 1. Setup admin session
    await adminPage(page, context);

    // 2. Navigate to cart recovery
    await page.goto('/cart-recovery');

    // 3. Assert heading is visible
    await expect(page.locator('h1:has-text("Abandoned Cart Recovery")')).toBeVisible();

    // 4. Set Pro mode dynamically or by interacting with the UI.
    // The UI checks `localStorage.getItem('has_pro') === 'true'`.
    // We can inject this so we bypass the Twitter modal for test stability.
    await page.evaluate(() => {
      localStorage.setItem('has_pro', 'true');
    });
    // Reload the page so the state reads from localStorage
    await page.reload();

    // 5. Check we have abandoned carts from our seed data
    // The button might say "Send to 1 Abandoned Carts"
    await expect(page.locator('button:has-text("Send to 1 Abandoned Carts")').or(page.locator('button:has-text("Send to")'))).toBeVisible();

    // 6. Enter some details
    await page.fill('input#customer-name', 'Alice Tester');
    await page.fill('input#cart-value', '$120.00');

    // 7. Click generate AI campaign
    const generateBtn = page.locator('button:has-text("Generate AI Campaign")');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 8. Verify the generated draft is shown
    await expect(page.locator('pre')).toContainText('Alice Tester');
    await expect(page.locator('pre')).toContainText('$120.00');

    // 9. Send campaign
    const sendBtn = page.locator('button:has-text("Send to 1 Abandoned Carts")');
    await expect(sendBtn).toBeEnabled();
    await sendBtn.click();

    // 10. Verify success message
    await expect(page.locator('text=Campaign sent to 1 abandoned carts!')).toBeVisible();
  });
});
