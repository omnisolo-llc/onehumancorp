import { test, expect } from '@playwright/test';
import { dbQuery } from '../db';
import { clearDb, seedUserAndTenant } from '../seed';

test.describe('Zero-Touch Smart Service Dispatch & Route Optimization', () => {
  // Test user persona: Carlos the Handyman (Field Service Operator)
  // Context: Carlos is running late for his first job and needs to notify subsequent clients.

  test.beforeEach(async ({ page, request }) => {
    // Seed some appointments for testing
    await clearDb();
    const { tenantId } = await seedUserAndTenant('carlos@ohc.local', 'password');

    // Create a staff member
    const staffId = 'staff-1';
    await dbQuery(`INSERT INTO staff_profiles (id, tenant_id, name) VALUES ('${staffId}', '${tenantId}', 'Carlos Handyman')`);

    // Create a customer
    const customerId = 'cust-1';
    await dbQuery(`INSERT INTO customers (id, tenant_id, name, email) VALUES ('${customerId}', '${tenantId}', 'John Doe', 'john@example.com')`);

    // Create a job template
    const templateId = 'template-1';
    await dbQuery(`INSERT INTO job_templates (id, tenant_id, name, estimated_duration_mins, base_price_cents) VALUES ('${templateId}', '${tenantId}', 'Fix Sink', 60, 10000)`);

    // Create 3 appointments
    await dbQuery(`INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, staff_profile_id, status, scheduled_start_time, scheduled_end_time, location_address)
      VALUES ('app-1', '${tenantId}', '${customerId}', '${templateId}', '${staffId}', 'Scheduled', CURRENT_TIMESTAMP + interval '1 hour', CURRENT_TIMESTAMP + interval '2 hours', '123 Main St')`);

    await dbQuery(`INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, staff_profile_id, status, scheduled_start_time, scheduled_end_time, location_address)
      VALUES ('app-2', '${tenantId}', '${customerId}', '${templateId}', '${staffId}', 'Scheduled', CURRENT_TIMESTAMP + interval '3 hours', CURRENT_TIMESTAMP + interval '4 hours', '456 Oak St')`);

    await dbQuery(`INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, staff_profile_id, status, scheduled_start_time, scheduled_end_time, location_address)
      VALUES ('app-3', '${tenantId}', '${customerId}', '${templateId}', '${staffId}', 'Scheduled', CURRENT_TIMESTAMP + interval '5 hours', CURRENT_TIMESTAMP + interval '6 hours', '789 Pine St')`);

    // Login
    await page.goto('/login');
    await page.fill('input[name="email"]', 'carlos@ohc.local');
    await page.fill('input[name="password"]', 'password');
    await page.click('button[type="submit"]');
    await page.waitForURL('/dashboard');
  });

  test('Carlos uses Running Late to notify next clients and update route', async ({ page }) => {
    await page.goto('/field-ops/jobs');

    // Wait for the page to load
    await expect(page.locator('h1', { hasText: "Today's Route" })).toBeVisible();

    // Look for a Running Late button.
    const runningLateBtn = page.getByRole('button', { name: 'Running Late' }).first();

    await expect(runningLateBtn).toBeVisible();

    // Grab the initial time to compare later
    const timeElements = await page.locator('.font-medium:has-text("⏱")').allInnerTexts();
    const firstJobTime = timeElements[0];

    await runningLateBtn.click();

    // Expect the Agent Suggestion Modal to appear
    const suggestionText = page.locator('text=/Notify the next \\d+ clients of a 30-minute delay\\?/');

    const approveBtn = page.getByRole('button', { name: 'Approve & Send' });
    await expect(approveBtn).toBeVisible();
    await expect(suggestionText).toBeVisible();
    await approveBtn.click();

    // The modal should close
    await expect(approveBtn).toBeHidden();
  });
});
