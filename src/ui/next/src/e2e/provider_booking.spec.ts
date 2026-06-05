import { test, expect } from '@playwright/test';

test.describe('Native Booking System for Service Businesses', () => {

    test('Provider Dashboard - Upcoming Bookings', async ({ page }) => {
        // Mock the authentication cookie
        await page.context().addCookies([
            {
                name: 'ohc_session',
                value: 'test_token',
                domain: 'localhost',
                path: '/',
            }
        ]);

        // Navigate to the Calendar page
        await page.goto('/calendar');

        // Verify the Header is present
        await expect(page.locator('h1').filter({ hasText: 'Schedule' })).toBeVisible();

        // Check that the mocked booking appears on the page
        await expect(page.locator('h3').filter({ hasText: 'Carlos Handyman' })).toBeVisible();
        await expect(page.locator('p').filter({ hasText: 'Plumbing Fix' })).toBeVisible();
        await expect(page.locator('span').filter({ hasText: 'confirmed' })).toBeVisible();

        // Because we mocked a booking tomorrow in our backend route, the relative date should appear
        const tomorrow = new Date(Date.now() + 86400000);
        const dayString = tomorrow.toLocaleDateString(undefined, { weekday: 'short', month: 'short', day: 'numeric' });
        await expect(page.locator('span').filter({ hasText: dayString })).toBeVisible();
    });

});
