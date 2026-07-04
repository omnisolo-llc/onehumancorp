import { test, expect } from '@playwright/test';
import { Pool } from 'pg';

test.describe('Automated Re-engagement Agent for Service Bookings', () => {
    let pool: Pool;
    const testTenant = 'e2e-tenant-reengagement';
    const testCustomerId = 'e2e-cust-123';
    const testCustomerName = 'Leo Customer';

    test.beforeAll(async () => {
        const dbUrl = process.env.DATABASE_URL as string;
        pool = new Pool({ connectionString: dbUrl });

        try {
            await pool.query('SELECT 1');
        } catch (e) {
            console.log('Database not available, skipping setup');
            return;
        }

        // Setup tenant
        await pool.query(`INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant') ON CONFLICT DO NOTHING`, [testTenant]);

        // Clean up previous test data
        await pool.query(`DELETE FROM bookings WHERE tenant_id = $1`, [testTenant]);
        await pool.query(`DELETE FROM customers WHERE tenant_id = $1`, [testTenant]);
        await pool.query(`DELETE FROM ohc_job_queue WHERE tenant_id = $1`, [testTenant]);
        await pool.query(`DELETE FROM agent_feed_items WHERE tenant_id = $1`, [testTenant]);

        // Insert test customer
        await pool.query(`INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, $3, 'leo@example.com')`,
            [testCustomerId, testTenant, testCustomerName]);

        // Insert two past bookings to make them qualify for re-engagement
        const pastDate1 = new Date();
        pastDate1.setDate(pastDate1.getDate() - 30);
        const pastDate2 = new Date();
        pastDate2.setDate(pastDate2.getDate() - 20); // More than 14 days ago

        await pool.query(`
            INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status)
            VALUES ($1, $2, $3, $4, $5, 'completed')`,
            ['booking-1', testTenant, testCustomerId, pastDate1, pastDate1]);

        await pool.query(`
            INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status)
            VALUES ($1, $2, $3, $4, $5, 'completed')`,
            ['booking-2', testTenant, testCustomerId, pastDate2, pastDate2]);

        // Insert a booking_reengagement_check job to simulate the daily routine worker finding this customer
        await pool.query(`
            INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
            VALUES ($1, $2, 'booking_reengagement_check', $3, 'PENDING', CURRENT_TIMESTAMP)`,
            ['job-reengage', testTenant, JSON.stringify({ customer_id: testCustomerId })]);
    });

    test.afterAll(async () => {
        if (pool) {
            await pool.end();
        }
    });

    test('should generate a context-aware re-engagement message for dormant customers', async ({ page }) => {
        // Set tenant context for local storage
        await page.goto('/ui/dashboard.html');
        await page.evaluate((tenant) => {
            localStorage.setItem('tenant_id', tenant);
        }, testTenant);
        await page.reload();

        // Navigate to Unified Agent Feed
        await expect(page.locator('h2:has-text("Command Center")')).toBeVisible({ timeout: 15000 });
        const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');

        // Verify the card appears
        await expect(feedSection.locator(`text=AI detected Leo Customer is a dormant customer`)).toBeVisible({ timeout: 15000 });

        // Verify the drafted message exists (or part of it)
        const approveButton = feedSection.locator('button:has-text("Approve")').first();
        await expect(approveButton).toBeVisible();

        // Clean up
        await page.evaluate(() => {
            localStorage.removeItem('tenant_id');
        });
    });
});
