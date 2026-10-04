import { test, expect } from "../../../../e2e/fixtures";
import { e2eDbQuery } from "../../../../e2e/db_utils";

test.describe("Field Service Routing & Dispatch Engine UI updates", () => {
  test("job status transitions persist and survive reloading the schedule", async ({ page, loginAs, adminUser, seedData }) => {
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
            `INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, location_address, location_lat, location_lng, updated_at)
             VALUES ($1, $2, $3, $4, 'Scheduled', NOW() + INTERVAL '1 hour', NOW() + INTERVAL '2 hours', '123 Main St', 40.7128, -74.0060, CURRENT_TIMESTAMP)`,
             [`appt-routing-${runId}-1`, tenantId, customerId, jtId]
        );

        await e2eDbQuery(
            `INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, location_address, location_lat, location_lng, updated_at)
             VALUES ($1, $2, $3, $4, 'Requested', NOW() + INTERVAL '2 hour', NOW() + INTERVAL '3 hours', '124 Main St', 40.7128, -74.0060, CURRENT_TIMESTAMP)`,
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
    const card = page.getByTestId(`job-card-appt-routing-${runId}-1`);
    const headingToJobBtn = card.getByRole('button', { name: 'Heading to Job', exact: true });
    await expect(headingToJobBtn).toBeVisible({ timeout: 5000 });
    await headingToJobBtn.click();

    // Check that we moved to 'Start Work'
    const startWorkBtn = card.getByRole('button', { name: 'Start Work', exact: true });
    await expect(startWorkBtn).toBeVisible({ timeout: 5000 });
    expect((await e2eDbQuery('SELECT status FROM appointments WHERE id = $1 AND tenant_id = $2', [`appt-routing-${runId}-1`, tenantId]))[0].status).toBe('En-Route');
    await startWorkBtn.click();

    // Check that we moved to 'Job Done'
    const jobDoneBtn = card.getByRole('button', { name: 'Job Done', exact: true });
    await expect(jobDoneBtn).toBeVisible({ timeout: 5000 });
    expect((await e2eDbQuery('SELECT status FROM appointments WHERE id = $1 AND tenant_id = $2', [`appt-routing-${runId}-1`, tenantId]))[0].status).toBe('In-Progress');
    await jobDoneBtn.click();

    await expect(card.getByText('COMPLETED', { exact: true })).toBeVisible({ timeout: 5000 });
    expect((await e2eDbQuery('SELECT status FROM appointments WHERE id = $1 AND tenant_id = $2', [`appt-routing-${runId}-1`, tenantId]))[0].status).toBe('Completed');
    await page.reload();
    await expect(card.getByText('COMPLETED', { exact: true })).toBeVisible();
    expect(scheduleErrors).toEqual([]);
  });
  test('delay preview leaves stored times unchanged and retains only stale failed approvals', async ({ page, loginAs, adminUser, seedData }) => {
    const tenantId = seedData.tenant.id;
    const runId = `delay-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    const templateId = `template-${runId}`;
    const ids = [0, 1, 2].map(index => `appointment-${runId}-${index}`);
    await e2eDbQuery("INSERT INTO job_templates (id, tenant_id, name) VALUES ($1, $2, 'Isolated delay test')", [templateId, tenantId]);
    for (const [index, id] of ids.entries()) {
      await e2eDbQuery(`INSERT INTO appointments (id, tenant_id, customer_id, job_template_id, status, scheduled_start_time, scheduled_end_time, updated_at)
        VALUES ($1, $2, $3, $4, 'Scheduled', '2030-01-01T10:00:00Z'::timestamptz + $5 * INTERVAL '1 hour', '2030-01-01T11:00:00Z'::timestamptz + $5 * INTERVAL '1 hour', CURRENT_TIMESTAMP)`, [id, tenantId, seedData.customer.id, templateId, index]);
    }
    const stored = async () => e2eDbQuery('SELECT id, status, scheduled_start_time, scheduled_end_time FROM appointments WHERE tenant_id = $1 AND id = ANY($2) ORDER BY id', [tenantId, ids]);
    const before = await stored();
    await loginAs(page, adminUser); await page.goto('/field-ops/jobs');
    await page.getByTestId(`job-card-${ids[0]}`).getByRole('button', { name: 'Running Late', exact: true }).click();
    const proposal = page.getByRole('region', { name: 'Schedule proposal' });
    await expect(proposal.getByRole('button', { name: 'Save schedule', exact: true })).toBeVisible();
    await expect(proposal).toContainText('No customer notifications are sent');
    expect(await stored()).toEqual(before);
    // A concurrent update invalidates only one original approval version.
    await e2eDbQuery("UPDATE appointments SET updated_at = updated_at + INTERVAL '1 second' WHERE id = $1 AND tenant_id = $2", [ids[2], tenantId]);
    await proposal.getByRole('button', { name: 'Save schedule', exact: true }).click();
    await expect(proposal.getByRole('alert')).toContainText('changed');
    await expect(proposal.getByRole('button', { name: 'Save schedule', exact: true })).toBeEnabled();
    const after = await stored();
    for (let index = 0; index < ids.length; index += 1) {
      const old = before.find(row => row.id === ids[index])!, saved = after.find(row => row.id === ids[index])!;
      const delta = new Date(String(saved.scheduled_start_time)).getTime() - new Date(String(old.scheduled_start_time)).getTime();
      expect(delta).toBe(index === 1 ? 30 * 60_000 : 0);
    }
    // Cancel explicitly discards the remaining preview; it never undoes a saved row.
    await proposal.getByRole('button', { name: 'Cancel', exact: true }).click();
    await expect(proposal).toHaveCount(0); await page.reload();
    await expect(page.getByTestId(`job-card-${ids[1]}`)).toBeVisible();
    expect(await stored()).toEqual(after);
  });

});
