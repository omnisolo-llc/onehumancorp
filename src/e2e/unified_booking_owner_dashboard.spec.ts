import { test, expect } from '@playwright/test';

test.describe('Unified Booking & Owner Dashboard Integration', () => {
    test('Owner navigates to Bookings from Dashboard and handles a booking', async ({ page }) => {
        // 1. Start from Dashboard
        await page.goto('/dashboard.html?tenant=e2e-tenant');
        await page.waitForTimeout(500);

        // 2. Click the Bookings button
        const bookingsBtn = page.getByRole('link', { name: 'Bookings' });
        await expect(bookingsBtn).toBeVisible();
        await bookingsBtn.click();

        // 3. Wait for navigation to booking.html
        await page.waitForURL(/booking\.html/);
        await expect(page.locator('#description')).toBeVisible();

        // 4. Submit a new mock booking request
        await page.locator('#description').fill('I need a quote for an advanced piano lesson.');
        await page.getByRole('button', { name: 'Get a Quote' }).click();

        // 5. Verify success view
        await expect(page.getByText('Request Sent!')).toBeVisible();
    });
});
