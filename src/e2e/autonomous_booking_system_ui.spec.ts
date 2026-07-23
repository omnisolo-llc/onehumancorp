import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `booking-ui-test-${Date.now()}`;

  test('Public Booking Form Flow', async ({ page }) => {
    // 1. Visit booking page
    await page.goto(`/booking?tenant=${tenantId}&service_id=mock-service`);
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    // 2. Fill the form
    await page.fill('input[type="text"]', 'Jane Doe');
    await page.fill('input[type="email"]', 'jane@example.com');
    await page.fill('textarea', 'I need a drum lesson.');

    // 3. Date Selection triggers slot loading
    const dateQuery = new Date().toISOString().split('T')[0];
    await page.fill('input[type="date"]', dateQuery);

    // Wait for the mock slots to load (9:00 AM, 11:00 AM, etc.)
    await page.waitForSelector('button:has-text("09:00 AM")');
    await page.click('button:has-text("09:00 AM")');

    // 4. Submit
    // Route mock to avoid actual backend errors if not fully seeded
    await page.route('/api/v1/booking/public/checkout', async (route) => {
        await route.fulfill({
            status: 200,
            json: {
                booking_id: 'mock-booking',
                stripe_url: 'https://checkout.stripe.com/pay/mock_session',
                status: 'pending_payment'
            }
        });
    });

    await page.click('button:has-text("Confirm Booking")');

    // 5. Verify deposit step
    await expect(page.getByTestId('booking-checkout-container')).toBeVisible();
    await expect(page.getByTestId('pay-deposit-btn')).toHaveAttribute('href', /checkout\.stripe\.com/);
  });

  test('Owner Admin Dashboard', async ({ page }) => {
    // 1. Visit admin bookings dashboard
    await page.goto(`/admin/bookings?tenant=${tenantId}`);
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();

    // Route mocks
    await page.route('/api/v1/booking/admin/resources', async (route) => {
        if (route.request().method() === 'GET') {
            await route.fulfill({
                status: 200,
                json: [{ id: 'res-1', name: 'Studio A', description: 'Main Studio', type: 'space' }]
            });
        } else {
            await route.fulfill({ status: 201, json: { id: 'new-res-1' } });
        }
    });

    await page.route('/api/v1/booking/admin/availability', async (route) => {
        if (route.request().method() === 'GET') {
            await route.fulfill({
                status: 200,
                json: [{ id: 'avail-1', resource_id: 'res-1', start_time: '2025-01-01T09:00:00Z', end_time: '2025-01-01T17:00:00Z' }]
            });
        } else {
            await route.fulfill({ status: 201, json: { id: 'new-avail-1' } });
        }
    });

    await page.reload();

    // 2. Check rendered content
    await expect(page.getByText('Studio A')).toBeVisible();

    // 3. Create Resource
    const newResNameInput = page.locator('input[type="text"]').first();
    await newResNameInput.fill('New Tutor Leo');
    await page.getByRole('button', { name: 'Add Resource' }).click();

    // 4. Create Availability Block
    // Wait for the select to be populated
    await page.selectOption('select', 'res-1');
    const timeInputs = page.locator('input[type="datetime-local"]');
    await timeInputs.nth(0).fill('2025-02-01T09:00');
    await timeInputs.nth(1).fill('2025-02-01T17:00');
    await page.getByRole('button', { name: 'Add Block' }).click();
  });
});
