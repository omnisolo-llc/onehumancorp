import { test, expect } from '@playwright/test';

test.describe('Delivery Dispatch Engine CUJ', () => {
  // Use a predictable tenant/user
  test.use({ extraHTTPHeaders: { 'x-tenant-id': 'e2e_delivery_tenant' } });

  test('merchant can view delivery manifest and start delivery', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    // 1. Merchant goes to dashboard and sees "Start Deliveries" action
    await page.goto('http://localhost:3000/dashboard');

    const dispatchLink = page.getByText('Start Deliveries');
    await expect(dispatchLink).toBeVisible();
    await dispatchLink.click();

    // 2. Navigates to the delivery dispatch view
    await expect(page).toHaveURL(/.*\/delivery-dispatch/);
    await expect(page.getByRole('heading', { name: 'Local Delivery Manifest' })).toBeVisible({ timeout: 10000 });

    // Seed database before asserting
    const date = new Date().toISOString().split('T')[0];

    // Create a mock delivery task to test the UI flow
    await page.route('/api/delivery/itinerary', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({
                tasks: [{
                    id: 'test-task-1',
                    order_id: 'ORD-TEST-01',
                    status: 'PENDING',
                    estimated_arrival_unix: Date.now() / 1000 + 3600,
                    delivery_location_lat: 40.7128,
                    delivery_location_lng: -74.0060
                }]
            })
        });
    });

    await page.route('/api/delivery/update-status', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({}) });
    });

    // We need to trigger the fetch again after mocking
    await page.goto('http://localhost:3000/delivery-dispatch');

    // 3. Ensure data is loaded
    await page.waitForSelector('.mac-glass-container');

    // 4. Verify PENDING task and start it
    const startDeliveryBtn = page.getByRole('button', { name: 'Start Delivery' }).first();
    await expect(startDeliveryBtn).toBeVisible();
    await startDeliveryBtn.click();

    // Wait for state to change to IN_TRANSIT, so Mark Delivered appears
    const markDeliveredBtn = page.getByRole('button', { name: 'Mark Delivered' }).first();
    await expect(markDeliveredBtn).toBeVisible();

    // 5. Button should change to "Mark Delivered" as task moves to IN_TRANSIT
    await markDeliveredBtn.click();

    // Wait for the specific element that we clicked to disappear
    // Note: since it's the first button, it might just disappear leaving no buttons, or another button becomes first.
    // In our UI, DELIVERED tasks have no buttons.
    await expect(page.locator('.mac-glass-container').first().getByRole('button', { name: 'Mark Delivered' })).toHaveCount(0);
  });
});
