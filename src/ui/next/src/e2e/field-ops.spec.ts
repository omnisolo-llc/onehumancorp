import { test, expect } from "../../../../e2e/fixtures";
import { e2eDbQuery } from "../../../../e2e/db_utils";

test.describe("Field Service Routing & Dispatch Engine UI updates", () => {
  test("Carlos can tap 'Heading to Job', 'Start Work', and 'Job Done' to update status without crashing", async ({
    page,
    context,
    loginAs,
    adminUser,
    seedData,
  }) => {
    const tenantId = seedData.tenant.id;
    const customerId = seedData.customer.id;
    const runId = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    const jobTemplateId = `jt-routing-${runId}`;

    await test.step('Seed job templates and appointments', async () => {
        const jtRes = await e2eDbQuery(
            `INSERT INTO job_templates (id, tenant_id, name)
             VALUES ($1, $2, 'Sink Repair') RETURNING id`,
             [jobTemplateId, tenantId]
        );
        const jtId = jtRes[0].id;

        await e2eDbQuery(
            `INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, location_address, location_lat, location_lng)
             VALUES ($1, $2, $3, $4, 'Scheduled', NOW() + INTERVAL '1 hour', NOW() + INTERVAL '2 hours', '123 Main St', 40.7128, -74.0060)`,
             [`appt-routing-${runId}-1`, tenantId, customerId, jtId]
        );

        await e2eDbQuery(
            `INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, location_address, location_lat, location_lng)
             VALUES ($1, $2, $3, $4, 'Requested', NOW() + INTERVAL '2 hour', NOW() + INTERVAL '3 hours', '124 Main St', 40.7128, -74.0060)`,
             [`appt-routing-${runId}-2`, tenantId, customerId, jtId]
        );
    });

    await loginAs(page, adminUser);

    const scheduleErrors: string[] = [];
    page.on('console', (message) => {
      if (message.type() === 'error' && /load appointments|schedule request failed/i.test(message.text())) {
        scheduleErrors.push(message.text());
      }
    });
    page.on('response', (response) => {
      if (response.url().includes('/api/v1/auth/powersync_token') && response.status() >= 400) {
        scheduleErrors.push(`PowerSync token request failed with HTTP ${response.status()}`);
      }
    });

    // Navigate to the field ops page
    await page.goto("/field-ops/jobs");

    // Verify online state
    await expect(page.locator("text=Today's Route")).toBeVisible();
    await expect(page.locator("text=Sink Repair").first()).toBeVisible();

    // Look for heading to job
    const headingToJobBtn = page.locator("button", { hasText: "Heading to Job" }).first();
    await expect(headingToJobBtn).toBeVisible({ timeout: 5000 });
    await headingToJobBtn.click();

    // Check that we moved to 'Start Work'
    const startWorkBtn = page.locator("button", { hasText: "Start Work" }).first();
    await expect(startWorkBtn).toBeVisible({ timeout: 5000 });
    await startWorkBtn.click();

    // Check that we moved to 'Job Done'
    const jobDoneBtn = page.locator("button", { hasText: "Job Done" }).first();
    await expect(jobDoneBtn).toBeVisible({ timeout: 5000 });
    await jobDoneBtn.click();

    await expect(page.locator('span:has-text("COMPLETED")').first()).toBeVisible({ timeout: 5000 });
    expect(scheduleErrors).toEqual([]);
  });
});
