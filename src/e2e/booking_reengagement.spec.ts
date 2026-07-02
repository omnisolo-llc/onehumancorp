import { test, expect } from '@playwright/test';
import { e2eSeedApp } from './fixtures';
import { executeSql } from './db_utils';

test.describe('Automated Re-engagement Agent for Service Bookings', () => {
  test('Detects dormant booking and drafts follow-up for approval', async ({ page, request }) => {
    // 1. Setup tenant and login
    const tenantId = `tenant_reengage_${Date.now()}`;
    const ownerEmail = `owner_${tenantId}@example.com`;
    const password = 'Password123!';

    await e2eSeedApp(request, tenantId, ownerEmail, password);

    await page.goto('/login');
    await page.getByPlaceholder(/email/i).fill(ownerEmail);
    await page.getByPlaceholder(/password/i).fill(password);
    await page.getByRole('button', { name: /log in/i }).click();
    await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 10000 });

    // 2. Setup database state to simulate a dormant customer
    // We need a service and a customer
    const serviceId = `svc_${Date.now()}`;
    await executeSql(`
      INSERT INTO products (id, tenant_id, name, type, is_service, price)
      VALUES ('${serviceId}', '${tenantId}', 'Music Lesson', 'service', true, 50.00)
    `);

    const customerId = `cust_${Date.now()}`;
    await executeSql(`
      INSERT INTO customers (id, tenant_id, name, email)
      VALUES ('${customerId}', '${tenantId}', 'Leo Student', 'leo.student@example.com')
    `);

    // Insert two past bookings > 14 days ago to make them dormant
    const pastDate1 = new Date();
    pastDate1.setDate(pastDate1.getDate() - 20);
    const pastDate1Str = pastDate1.toISOString();

    const pastDate2 = new Date();
    pastDate2.setDate(pastDate2.getDate() - 15);
    const pastDate2Str = pastDate2.toISOString();

    await executeSql(`
      INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status)
      VALUES ('bk1_${Date.now()}', '${tenantId}', '${customerId}', '${serviceId}', '${pastDate1Str}', '${pastDate1Str}', 'completed')
    `);

    await executeSql(`
      INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status)
      VALUES ('bk2_${Date.now()}', '${tenantId}', '${customerId}', '${serviceId}', '${pastDate2Str}', '${pastDate2Str}', 'completed')
    `);

    // 3. Trigger the job queue (simulate the reserve endpoint behavior)
    // We insert a job with `next_retry_at` in the past so it runs immediately
    const jobId = `job_${Date.now()}`;
    const payload = JSON.stringify({
      customer_id: customerId,
      product_id: serviceId
    });

    await executeSql(`
      INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, next_retry_at)
      VALUES ('${jobId}', '${tenantId}', 'booking_reengagement_check', '${payload}', 'PENDING', CURRENT_TIMESTAMP)
    `);

    // 4. Navigate to Agent Feed and verify the drafted message
    // Mobile view to ensure 375px responsiveness
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/feed');

    // Wait for the worker to process the job and push to shared_tasks
    await page.waitForTimeout(3000);

    // Look for the "Approve Re-engagement" task
    const reengageCard = page.locator('text=Approve Re-engagement for Leo Student').first();
    await expect(reengageCard).toBeVisible({ timeout: 15000 });

    // The message should mention they haven't had a session in a while
    const messageContent = page.locator('text=haven\'t had a session in a while').first();
    await expect(messageContent).toBeVisible();

    // 5. Approve the draft
    const approveBtn = page.getByRole('button', { name: /Approve/i }).first();
    await approveBtn.click();

    // The card should disappear or show success state
    await expect(reengageCard).toBeHidden({ timeout: 5000 });
  });
});
