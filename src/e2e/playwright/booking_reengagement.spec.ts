import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Automated Re-engagement Agent for Service Bookings', () => {
  test('should detect dormant customer and create re-engagement task', async ({ page }) => {
    // 1. Manually trigger the dormant logic or create seed data that represents a dormant user.
    await adminPage(page).goto('/');

    // Simulate DB insertion for a dormant customer
    const insertRes = await page.evaluate(async () => {
      try {
        const res = await fetch('/api/e2e/db', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            query: `
              INSERT INTO customers (id, tenant_id, name, email, phone)
              VALUES ('dormant-user-1', 'e2e-tenant', 'Leo Dormant Student', 'dormant@example.com', '+15555555555')
              ON CONFLICT (id) DO NOTHING;

              INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status)
              VALUES
                ('dormant-b1', 'e2e-tenant', 'dormant-user-1', CURRENT_TIMESTAMP - interval '20 days', CURRENT_TIMESTAMP - interval '19 days', 'confirmed'),
                ('dormant-b2', 'e2e-tenant', 'dormant-user-1', CURRENT_TIMESTAMP - interval '15 days', CURRENT_TIMESTAMP - interval '14 days', 'confirmed')
              ON CONFLICT (id) DO NOTHING;

              INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
              VALUES (gen_random_uuid()::text, 'e2e-tenant', 'booking_reengagement_check', '{"customer_id": "dormant-user-1"}', 'PENDING')
              ON CONFLICT DO NOTHING;
            `
          })
        });
        return res.ok;
      } catch (e) {
        return false;
      }
    });

    // If we're not running postgres, we use a fake endpoint to seed
    if (!insertRes) {
        await page.evaluate(async () => {
            try {
              await fetch('/api/e2e/db', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  query: `
                    INSERT INTO customers (id, tenant_id, name, email, phone)
                    VALUES ('dormant-user-1', 'e2e-tenant', 'Leo Dormant Student', 'dormant@example.com', '+15555555555')
                    ON CONFLICT (id) DO NOTHING;

                    INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status)
                    VALUES
                      ('dormant-b1', 'e2e-tenant', 'dormant-user-1', datetime('now', '-20 days'), datetime('now', '-19 days'), 'confirmed'),
                      ('dormant-b2', 'e2e-tenant', 'dormant-user-1', datetime('now', '-15 days'), datetime('now', '-14 days'), 'confirmed')
                    ON CONFLICT (id) DO NOTHING;

                    INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
                    VALUES (lower(hex(randomblob(16))), 'e2e-tenant', 'booking_reengagement_check', '{"customer_id": "dormant-user-1"}', 'PENDING')
                    ON CONFLICT DO NOTHING;
                  `
                })
              });
            } catch (e) {
              // ignore
            }
        });
    }

    // Check Agent Feed
    await page.goto('/feed');

    // Wait and reload if necessary since the background worker might take a few seconds
    let found = false;
    for (let i = 0; i < 5; i++) {
      try {
        await expect(page.locator('text=Approve Re-engagement for Leo Dormant Student')).toBeVisible({ timeout: 5000 });
        found = true;
        break;
      } catch (e) {
        await page.reload();
      }
    }

    expect(found).toBe(true);
  });
});
