import { test, expect } from '@playwright/test';

test.describe('Checkout Branding Growth Loop', () => {
    test('Powered by OHC footer is present and links correctly on successful checkout', async ({ page }) => {
        // Go to checkout page
        await page.goto('http://localhost:3000/checkout');

        // Fill out form
        await page.fill('input[placeholder="John Doe"]', 'Test User');
        await page.fill('input[type="email"]', 'test@example.com');
        await page.fill('input[placeholder="1234 5678 9101 1121"]', '1234 5678 9101 1121');
        await page.fill('input[placeholder="MM/YY"]', '12/25');
        await page.fill('input[placeholder="CVC"]', '123');

        // Submit form
        await page.click('button:has-text("Pay $45.00")');

        // Wait for modal to appear
        await page.waitForSelector('text=Payment Successful!', { timeout: 10000 });

        // Check for Powered by OHC footer link
        const footerLink = page.locator('a:has-text("⚡ Powered by OHC")');
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toHaveAttribute('href', /ohc\.store\/join\?ref=/);
    });
});
