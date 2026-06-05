import { test, expect } from '@playwright/test';

test.describe('Provider Booking Native App', () => {
    test('renders upcoming bookings empty state', async ({ page }) => {
        // Intercept API call to return empty array
        await page.route('**/api/v1/provider/bookings', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ bookings: [] }),
            });
        });

        await page.goto('/provider-dashboard/bookings');

        await expect(page.locator('h1').filter({ hasText: 'Upcoming Bookings' })).toBeVisible();
        await expect(page.getByText('No upcoming bookings')).toBeVisible();
    });

    test('renders upcoming bookings with data', async ({ page }) => {
        // Intercept API call to return mock data for test verification
        await page.route('**/api/v1/provider/bookings', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({
                    bookings: [
                        {
                            id: "test-1",
                            customerName: "Carlos Test",
                            serviceName: "Plumbing Fix",
                            startTime: new Date().toISOString(),
                            status: "confirmed",
                            depositStatus: "paid"
                        }
                    ]
                }),
            });
        });

        await page.goto('/provider-dashboard/bookings');

        await expect(page.locator('h1').filter({ hasText: 'Upcoming Bookings' })).toBeVisible();
        await expect(page.getByText('Carlos Test')).toBeVisible();
        await expect(page.getByText('Plumbing Fix')).toBeVisible();
        await expect(page.getByText('Confirmed')).toBeVisible();
    });

    test('renders settings empty schedule', async ({ page }) => {
        // Intercept API call
        await page.route('**/api/v1/provider/schedule', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({ schedule: null }),
            });
        });

        await page.goto('/provider-dashboard/settings');

        await expect(page.locator('h1').filter({ hasText: 'Settings' })).toBeVisible();
        await expect(page.getByText('No schedule configured.')).toBeVisible();
    });
});
