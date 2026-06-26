import { test, expect } from '@playwright/test';

test.describe('Autonomous Intelligent Service Routing and Dispatch Engine', () => {
    test.use({ viewport: { width: 375, height: 812 } });

    test('should display daily run sheet and accept AI smart booking suggestion', async ({ page }) => {
        // Mock backend route for initial load to show empty state
        await page.route('**/api/v1/dispatch/run-sheet', async route => {
            const json = {
                id: 'mock-route-1',
                date: new Date().toISOString().split('T')[0],
                status: 'Active',
                stops: []
            };
            await route.fulfill({ json });
        });

        // Navigate to the Daily Run Sheet
        await page.goto('/operations/run-sheet');
        await expect(page.locator('h1:has-text("Today")')).toHaveText('Today');
        await expect(page.locator('text=Your daily run sheet.')).toBeVisible();
        await expect(page.locator('text=No stops scheduled for today.')).toBeVisible();

        // After 3 seconds, the suggestion should appear
        const suggestion = page.locator('text=New urgent request: Leak at 123 Main St.');
        await expect(suggestion).toBeVisible({ timeout: 5000 });

        // Mock backend for accepting the suggestion and updating the run sheet
        await page.route('**/api/v1/dispatch/inject-job', async route => {
            const json = {
                success: true,
                proposed_slot: '1:00 PM',
                impact: '+15m delay for PM jobs'
            };
            await route.fulfill({ json, status: 201 });
        });

        // Remock run sheet to simulate the updated route
        await page.route('**/api/v1/dispatch/run-sheet', async route => {
            const json = {
                id: 'mock-route-1',
                date: new Date().toISOString().split('T')[0],
                status: 'Active',
                stops: [
                    {
                        id: 'stop-1',
                        status: 'Pending',
                        estimated_arrival: new Date().toISOString(),
                        appointment: {
                            job_name: 'Emergency Leak Repair',
                            customer_name: 'John Doe',
                            location_address: '123 Main St.'
                        }
                    }
                ]
            };
            await route.fulfill({ json });
        });

        // Setup dialog handler to dismiss the native alert
        page.on('dialog', dialog => dialog.accept());

        // Click "Accept & Notify"
        await page.locator('button:has-text("Accept & Notify")').click();

        // The suggestion should disappear
        await expect(suggestion).toBeHidden();

        // The run sheet should now display the new job
        await expect(page.locator('text=Emergency Leak Repair')).toBeVisible();
        await expect(page.locator('text=John Doe • 123 Main St.')).toBeVisible();
        await expect(page.locator('text=Start Job')).toBeVisible();
    });
});
