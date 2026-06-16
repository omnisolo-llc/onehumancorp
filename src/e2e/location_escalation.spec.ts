import { test, expect } from '@playwright/test';
import { e2eTenantId, e2eAdminEmail } from './fixtures';
import { Pool } from 'pg';

test.describe('Location Escalation CUJ (Jun and Owner)', () => {

  test('Jun escalates an issue and Owner reviews it', async ({ page }) => {

    const pool = new Pool({
        connectionString: process.env.DATABASE_URL || 'postgres://ohc:ohc@localhost:5432/ohc'
    });

    // Seed Location and Tasks DB data natively and correctly
    const locationIdRes = await pool.query(`INSERT INTO locations (tenant_id, name) VALUES ($1, $2) RETURNING id`, ['e2e-tenant', 'Location A']);
    const locationId = locationIdRes.rows[0].id;

    await pool.query(`INSERT INTO shared_tasks (id, organization_id, title, status, location_id) VALUES ($1, $2, $3, $4, $5)`, ['t2', 'e2e-tenant', '3 customer complaints regarding slow pickup in the last hour', 'ALERT', locationId]);

    // Set localStorage so location is tracked for UI request
    await page.goto('/api/ui/location-manager.html');
    await page.evaluate((locId) => {
        localStorage.setItem('location_id', locId);
        localStorage.setItem('tenant_id', 'e2e-tenant');
    }, locationId);
    await page.reload();

    // Stage 1: Jun (Location Manager) View
    // Verify Jun's dashboard loaded
    await expect(page.locator('text=Location Manager Dashboard')).toBeVisible();

    // Wait for JS load
    await page.waitForTimeout(1000);

    // Verify task is present
    await expect(page.locator('text=3 customer complaints regarding slow pickup in the last hour')).toBeVisible();

    // Click Escalate
    await page.click('button:has-text("Escalate to Owner")');

    // Verify modal and summary
    await expect(page.locator('h2:has-text("Escalate to Owner")')).toBeVisible();
    await expect(page.locator('textarea#escalate-summary')).toHaveValue('Spike in pickup complaints at Location A. Staffing appears adequate, but the kitchen printer is offline. Requesting IT support.');

    // Submit escalation
    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Send to Owner")');

    // Stage 2: Owner View
    // We navigate to the actual HTML file for testing
    await page.goto('/api/ui/dashboard.html');
    await page.evaluate(() => {
        localStorage.setItem('tenant_id', 'e2e-tenant');
    });

    // Wait for JS load
    await page.waitForTimeout(1500);

    // Verify Regional Escalations feed
    await expect(page.locator('h2:has-text("Regional Escalations")').first()).toBeVisible();

    // Verify escalation card content
    await expect(page.locator('h3:has-text("Escalation")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Approve IT Request")').first()).toBeVisible();
  });

});
