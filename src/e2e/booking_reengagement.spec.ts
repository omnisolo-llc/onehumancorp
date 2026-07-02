import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';
import { Pool } from 'pg';

test.describe('Automated Re-engagement Agent for Service Bookings', () => {
    let pool: Pool;
    let tenantId = '00000000-0000-0000-0000-000000000002'; // A fixed test tenant ID
    let customerId = randomUUID();
    let serviceId = randomUUID();

    test.beforeAll(async () => {
        // Playwright test environment setup - we use the DATABASE_URL passed by Bazel
        pool = new Pool({
            connectionString: process.env.DATABASE_URL || 'postgresql://postgres:postgres@localhost:5432/ohc',
        });

        // Setup initial data for the test
        await pool.query(`INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant Re-engagement') ON CONFLICT DO NOTHING`, [tenantId]);
        await pool.query(`INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, 'Dormant Customer', 'dormant@example.com') ON CONFLICT DO NOTHING`, [customerId, tenantId]);
        await pool.query(`INSERT INTO services (id, tenant_id, title, price_cents) VALUES ($1, $2, 'Test Service', 10000) ON CONFLICT DO NOTHING`, [serviceId, tenantId]);

        // Create past bookings to make the customer "dormant" (more than 1 booking, last one older than 14 days)
        await pool.query(`
            INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status)
            VALUES
            ($1, $2, $3, $4, CURRENT_TIMESTAMP - INTERVAL '30 days', CURRENT_TIMESTAMP - INTERVAL '30 days' + INTERVAL '1 hour', 'completed'),
            ($5, $2, $3, $4, CURRENT_TIMESTAMP - INTERVAL '20 days', CURRENT_TIMESTAMP - INTERVAL '20 days' + INTERVAL '1 hour', 'completed')
        `, [randomUUID(), tenantId, customerId, serviceId, randomUUID()]);
    });

    test.afterAll(async () => {
        if (pool) {
            await pool.query(`DELETE FROM bookings WHERE tenant_id = $1`, [tenantId]);
            await pool.query(`DELETE FROM customers WHERE id = $1`, [customerId]);
            await pool.query(`DELETE FROM services WHERE id = $1`, [serviceId]);
            await pool.end();
        }
    });

    test('detects dormant user and drafts a re-engagement message', async ({ page }) => {
        // Insert a job for the worker to process, simulating the scheduler
        const jobId = randomUUID();
        await pool.query(`
            INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
            VALUES ($1, $2, 'booking_reengagement_check', $3, 'PENDING', CURRENT_TIMESTAMP)
        `, [jobId, tenantId, JSON.stringify({ customer_id: customerId })]);

        // Wait for the worker to process the job (it polls every 10 seconds, but let's just wait a bit)
        let sharedTaskCreated = false;
        let attempts = 0;
        while (!sharedTaskCreated && attempts < 15) {
            await new Promise(r => setTimeout(r, 1000));
            const result = await pool.query(`SELECT * FROM shared_tasks WHERE organization_id = $1 AND title LIKE $2`, [tenantId, '%Approve Re-engagement%']);
            if (result.rows.length > 0) {
                sharedTaskCreated = true;
                break;
            }
            attempts++;
        }

        expect(sharedTaskCreated).toBe(true);

        // Now verify it in the UI
        // We'll mock the login by setting the tenant cookie
        await page.context().addCookies([{ name: 'ohc_tenant_id', value: tenantId, url: 'http://127.0.0.1:18789' }]);

        // Navigate to the agent feed
        await page.goto('/ui/unified-agent-feed.html');

        // Ensure that the header renders
        await expect(page.locator('h1').first()).toBeVisible();

        // Check that the re-engagement task is visible in the feed
        await expect(page.locator('text=Approve Re-engagement for Dormant Customer')).toBeVisible({ timeout: 15000 });
    });
});
