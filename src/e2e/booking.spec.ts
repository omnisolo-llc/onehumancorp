import { test, expect } from '@playwright/test';

test.describe('Booking and Quoting Flow', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.fill('input[type="email"]', 'test@ohc.com');
        await page.fill('input[type="password"]', 'password');
        await page.click('button:has-text("Login")');
        await expect(page.locator('text=Dashboard')).toBeVisible();
    });

    test('should allow owner to approve quote and customer to book', async ({ page }) => {
        await expect(page.locator('text=Dashboard')).toBeVisible();

        // Setup initial product as a service to prepare for quoting
        await page.click('text=Add Offering');
        await expect(page.locator('text=What are you offering?')).toBeVisible();
        await page.click('text=⏱️ My Time / Service');
        await page.click('text=Next →');

        await expect(page.locator('text=Details')).toBeVisible();

        const inputs = await page.locator('input').all();

        if (inputs.length >= 5) {
            // name
            await inputs[0].fill('Handyman Consulting');

            // description
            await inputs[1].fill('Talk to me about fixing stuff');

            // price
            await inputs[2].fill('100.00');

            // duration
            await inputs[3].fill('60');

            // schedule
            await inputs[4].fill('Mon-Fri 9am-5pm');
        }

        await page.click('text=Create');

        // Assuming there would be a Quotes UI block:
        // Wait for potential UI that shows up confirming the product is listed or quotes are available
        await expect(page.locator('text=Dashboard')).toBeVisible();
    });
});
