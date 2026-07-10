import { test, expect } from "../../../e2e/fixtures";
import { e2eDbQuery } from "../../../e2e/db_utils";

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

    await test.step('Seed job templates and appointments', async () => {
        const jtRes = await e2eDbQuery(
            `INSERT INTO job_templates (id, tenant_id, name, estimated_duration_mins, base_price_cents)
             VALUES ('jt-routing-1', $1, 'Sink Repair', 60, 15000) RETURNING id`,
             [tenantId]
        );
        const jtId = jtRes.rows[0].id;

        await e2eDbQuery(
            `INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, location_address, location_lat, location_lng)
             VALUES ('appt-routing-1', $1, $2, $3, 'Scheduled', NOW() + INTERVAL '1 hour', NOW() + INTERVAL '2 hours', '123 Main St', 40.7128, -74.0060)`,
             [tenantId, customerId, jtId]
        );

        await e2eDbQuery(
            `INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, location_address, location_lat, location_lng)
             VALUES ('appt-routing-2', $1, $2, $3, 'Requested', NOW() + INTERVAL '2 hour', NOW() + INTERVAL '3 hours', '124 Main St', 40.7128, -74.0060)`,
             [tenantId, customerId, jtId]
        );
    });

    await loginAs(page, adminUser);

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

    // Check that we moved to 'Complete & Pay'
    const jobDoneBtn = page.locator("button", { hasText: "Complete & Pay" }).first();
    await expect(jobDoneBtn).toBeVisible({ timeout: 5000 });
    await jobDoneBtn.click();

    await expect(page.locator('span:has-text("COMPLETED")').first()).toBeVisible({ timeout: 5000 });
  });
});
