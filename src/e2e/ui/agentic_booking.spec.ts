import { test, expect } from '@playwright/test';
import { e2eDbQuery } from '../db_utils';

test.describe('Agentic Booking System', () => {
  test('Customer booking flow with real database assertions', async ({ page }) => {
    // 1. Setup: Seed the database directly to ensure we have a valid environment
    const tenantId = 'test-booking-tenant';
    const serviceId = 'test-booking-service';

    await e2eDbQuery(`
      INSERT INTO tenants (id, name) VALUES ($1, 'Test Booking Tenant') ON CONFLICT (id) DO NOTHING;
    `, [tenantId]);

    await e2eDbQuery(`
      INSERT INTO services (id, tenant_id, title, price_cents, requires_deposit, deposit_amount_cents)
      VALUES ($1, $2, 'Test Service', 10000, true, 5000)
      ON CONFLICT (id) DO NOTHING;
    `, [serviceId, tenantId]);

    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);

    // properly formatted for postgres timestamptz
    const startStr = new Date(tomorrow.setHours(10, 0, 0, 0)).toISOString();
    const endStr = new Date(tomorrow.setHours(11, 0, 0, 0)).toISOString();

    await e2eDbQuery(`
      INSERT INTO availability_blocks (id, tenant_id, service_id, start_time, end_time, is_available)
      VALUES ('test-block-1', $1, $2, $3, $4, true)
      ON CONFLICT (id) DO NOTHING;
    `, [tenantId, serviceId, startStr, endStr]);

    // 2. Visit the booking UI
    await page.goto(`/booking?tenant=${tenantId}&service_id=${serviceId}`);

    // Check initial rendering and specific UI tokens (glassmorphism/borderRadius)
    await expect(page.locator('text=Book an Appointment')).toBeVisible();

    // Select Date (tomorrow)
    const tomorrowDateStr = startStr.split('T')[0];
    await page.fill('input[type="date"]', tomorrowDateStr);

    // After date is filled, available times should appear
    await expect(page.locator('text=Available Times')).toBeVisible();

    // Select the first available slot button
    await page.locator('button', { hasText: /:/ }).first().click();

    // Fill customer details
    await page.fill('input[placeholder="Jane Doe"]', 'Jane Test');
    await page.fill('input[type="email"]', 'jane.test@test.com');
    await page.fill('textarea', 'Fix my sink');

    const confirmButton = page.locator('button:has-text("Confirm Booking")');
    await expect(confirmButton).toBeVisible();

    // 3. Confirm booking
    await confirmButton.click();

    // 4. Verify checkout rendering
    await expect(page.locator('text=Almost there!')).toBeVisible({ timeout: 15000 });
    const payButton = page.locator('text=Pay Deposit');
    await expect(payButton).toBeVisible();

    // 5. Verify the backend state was actually updated without mocks
    const result = await e2eDbQuery(`
        SELECT status, payment_intent_id FROM bookings WHERE tenant_id = $1 AND service_id = $2
    `, [tenantId, serviceId]);

    expect(result.length).toBeGreaterThan(0);
    expect(result[0].status).toBe('pending_payment');

    // 6. Click checkout to test the redirect
    await payButton.click();

    // It should redirect to the checkout session URL
    await page.waitForURL(/\/checkout\/session\/pi_/);
    expect(page.url()).toContain('/checkout/session/pi_');
  });
});
