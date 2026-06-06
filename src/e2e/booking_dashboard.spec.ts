import { test, expect } from '@playwright/test';

test.describe('Booking Dashboard - Native Booking System', () => {
  test.beforeEach(async ({ request }) => {
    // Relying on the global E2E seed to have already run, or running it explicitly.
    // The E2E seed drops and seeds `e2e-tenant` with customers, products, and bookings.
    await request.post('/api/dev/seed', {
      data: { scenario: 'e2e-full-run' }
    });
  });

  test('Persona Carlos (Handyman) - Empty State Simulation', async ({ page, request }) => {
    // Clear out bookings specifically for this test or use a different tenant.
    // We'll mock the endpoint response for the empty state test just to simulate a new tenant.
    await page.route('**/api/ui/bookings*', async route => {
      await route.fulfill({ json: [] });
    });

    await page.goto('/dashboard');

    // Verify Upcoming Bookings section
    const section = page.locator('text=Upcoming Bookings');
    await expect(section).toBeVisible();

    // Verify empty state
    await expect(page.locator('text=No bookings found for this tenant.')).toBeVisible();
  });

  test('Persona Leo (Music Tutor) - Render Seeded Bookings', async ({ page }) => {
    // For this test, we DO NOT mock. We let the page fetch the real seeded data.
    // Log in as e2e-tenant (assumes standard auth/login flow or bypassing auth via test utility)
    // The dashboard fetches data via /api/ui/bookings?tenant_id=e2e-tenant

    // Wait for the Dashboard to load and verify real seeded data
    // Assuming we can pass tenant_id via query param for tests or we are logged in

    await page.goto('/dashboard?tenant_id=e2e-tenant');

    // In our seed, we added a booking for 'e2e-product-class' which represents 'Cake Decorating Class'
    await expect(page.locator('text=e2e-product-class')).toBeVisible();

    // Verify statuses
    await expect(page.locator('span.app-badge.good:has-text("confirmed")')).toBeVisible();
    await expect(page.locator('span.app-badge.warn:has-text("pending")')).toBeVisible();
  });

  test('Booking Section Header and Link', async ({ page }) => {
    await page.goto('/dashboard?tenant_id=e2e-tenant');
    const manageLink = page.locator('a.app-button', { hasText: 'Manage' }).filter({ has: page.locator('xpath=ancestor::div[contains(@class, "app-panel-header")][.//div[text()="Upcoming Bookings"]]') });
    await expect(manageLink).toHaveAttribute('href', '/booking');
  });

  test('Persona Maya (Baker) - View Custom Cake Order Flow Integration', async ({ page, request }) => {
     await page.route('**/api/ui/bookings*', async route => {
      const json = [
        {
          id: 'booking_125',
          tenant_id: 'e2e-tenant',
          customer_id: 'cust_003',
          product_id: 'Custom Vegan Cake Consultation',
          start_time: new Date(Date.now() + 3600000).toISOString(), // 1 hour from now
          status: 'confirmed'
        }
      ];
      await route.fulfill({ json });
    });

    await page.goto('/dashboard?tenant_id=e2e-tenant');
    await expect(page.locator('text=Custom Vegan Cake Consultation')).toBeVisible();
  });

  test('Booking List limit to 5 items', async ({ page, request }) => {
    await page.route('**/api/ui/bookings*', async route => {
      // Create 6 bookings
      const json = Array.from({ length: 6 }).map((_, i) => ({
        id: `booking_${i}`,
        tenant_id: 'e2e-tenant',
        customer_id: `cust_${i}`,
        product_id: `Service ${i}`,
        start_time: new Date(Date.now() + (i * 3600000)).toISOString(),
        status: 'confirmed'
      }));
      await route.fulfill({ json });
    });

    await page.goto('/dashboard?tenant_id=e2e-tenant');

    // Only 5 should be visible due to .slice(0, 5)
    await expect(page.locator('text=Service 0')).toBeVisible();
    await expect(page.locator('text=Service 4')).toBeVisible();
    await expect(page.locator('text=Service 5')).not.toBeVisible();
  });
});
