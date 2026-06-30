import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Cart Recovery Feature', () => {
  test('should verify abandoned cart recovery flow without mock data', async ({ page, context }) => {
    await adminPage(page, context);

    // Navigate to cart recovery
    await page.goto('/cart-recovery.html');

    // Assert heading is visible
    await expect(page.locator('h1:has-text("Abandoned Cart Recovery")')).toBeVisible();

    // With true unmocked API and no seed data, cart count should be 0
    await expect(page.locator('#cart-count')).toHaveText('0');

    // Send button should be disabled when there are 0 carts
    const sendBtn = page.locator('button[id="send-btn"]');
    await expect(sendBtn).toBeDisabled();

    // Enter some details
    await page.fill('input#customer-name', 'Alice Tester');
    await page.fill('input#cart-value', '$120.00');

    // Click generate AI campaign
    const generateBtn = page.locator('button#generate-btn');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // Verify the generated draft is shown
    await expect(page.locator('#draft-preview')).toContainText('Alice Tester');
    await expect(page.locator('#draft-preview')).toContainText('$120.00');
    await expect(page.locator('#draft-preview')).toContainText('COMEBACK');
    await expect(page.locator('#draft-preview')).toContainText('Powered by OHC');
  });
});
